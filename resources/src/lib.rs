/*
 * Copyright (c) 2023. The RigelA open source project team and
 * its contributors reserve all rights.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and limitations under the License.
 */

use bitar::{archive_reader::HttpReader, Archive, ChunkIndex, CloneOutput, HashSum, VerifiedChunk};
use blake2::{Blake2b512, Digest};
use futures_util::StreamExt;
use log::{debug, info};
use std::{io::SeekFrom, path::PathBuf, time::Duration};
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncSeekExt},
    task::spawn_blocking,
};
use url::Url;

/**
 * 增量方式克隆一个资源。
 * `resource_url` 资源文件的url，提供文件的http server必须有断点续传能力。
 * `save_path` 保存资源文件的本地路径。
 * */
pub async fn clone_resource(resource_url: String, save_path: PathBuf) -> Result<File, String> {
    let url = format!("{}.cba", resource_url);
    let reader = HttpReader::from_url(Url::parse(url.as_str()).unwrap())
        .retries(5)
        .retry_delay(Duration::from_millis(5000));
    let archive = Archive::try_init(reader).await;
    if let Err(x) = archive {
        return Err(format!("Failed to read archive at {}.\n{}", url, x));
    }
    let mut archive = archive.unwrap();
    let clone_index = archive.build_source_index();
    // 创建或打开输出文件
    let mut output_file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .create_new(false)
        .open(&save_path)
        .await
        .expect(format!("Failed to open {}", save_path.display()).as_str());
    // 建立输出文件块的索引
    let output_index = {
        info!("Building chunk index of {}...", save_path.display());
        let mut chunk_stream = archive
            .chunker_config()
            .new_chunker(&mut output_file)
            .map(|r| spawn_blocking(|| r.map(|(offset, chunk)| (offset, chunk.verify()))))
            .buffered(4);
        let mut index = ChunkIndex::new_empty(archive.chunk_hash_length());
        while let Some(r) = chunk_stream.next().await {
            let (chunk_offset, verified) = r.unwrap().unwrap();
            let (hash, chunk) = verified.into_parts();
            index.add_chunk(hash, chunk.len(), &[chunk_offset]);
        }
        Some(index)
    };
    let mut output_clone = CloneOutput::new(output_file, clone_index);
    let mut total_read_from_seed = 0u64;
    if let Some(output_index) = output_index {
        debug!("Re-ordering chunks of {}...", save_path.display());
        let used_from_self = output_clone
            .reorder_in_place(output_index)
            .await
            .expect("Failed to clone in place");
        info!("Used {} from {}", used_from_self, save_path.display());
        total_read_from_seed += used_from_self;
    }
    // 从存档中读取剩余内容
    info!("Fetching {} chunks from {}...", output_clone.len(), url);
    let total_read_from_remote = {
        let mut total_fetched = 0u64;
        let output_bytes = {
            let mut chunk_stream = archive
                .chunk_stream(output_clone.chunks())
                .map(|r| {
                    if let Ok(compressed) = &r {
                        total_fetched += compressed.len() as u64;
                    }
                    spawn_blocking(move || -> VerifiedChunk {
                        let compressed = r.expect("Can't read the archive.");
                        let verified = compressed
                            .decompress()
                            .expect("Can't decompress the chunk.")
                            .verify()
                            .expect("Failed to verify the chunk.");
                        verified
                    })
                })
                .buffered(4);
            let mut output_bytes = 0;
            while let Some(result) = chunk_stream.next().await {
                let verified = result.unwrap();
                let wc = output_clone.feed(&verified).await.unwrap();
                if wc > 0 {
                    info!("Chunk '{}', size {} used", verified.hash(), verified.len());
                }
                output_bytes += wc as u64;
            }
            output_bytes
        };
        info!(
            "Fetched {} from archive and decompressed to {}.",
            total_fetched, output_bytes
        );
        total_fetched
    };
    let mut output_file = output_clone.into_inner();
    // 将输出文件的大小调整为与存档源的大小相同
    output_file
        .set_len(archive.total_source_size())
        .await
        .expect(format!("Failed to resize {}", save_path.display()).as_str());
    info!("Verifying checksum of {}...", save_path.display());
    let sum = {
        output_file.seek(SeekFrom::Start(0)).await.unwrap();
        let mut output_hasher = Blake2b512::new();
        let mut buffer: Vec<u8> = vec![0; 4 * 1024 * 1024];
        loop {
            let rc = output_file.read(&mut buffer).await.unwrap();
            if rc == 0 {
                break;
            }
            output_hasher.update(&buffer[0..rc]);
        }
        HashSum::from(&output_hasher.finalize()[..])
    };
    let expected_checksum = archive.source_checksum();
    if sum != *expected_checksum {
        return Err(format!(
            "Checksum mismatch ({}: {}, {}: {})",
            save_path.display(),
            sum,
            url,
            expected_checksum
        ));
    }
    info!(
        "Successfully cloned archive using {} from archive and {} from seeds.",
        total_read_from_remote, total_read_from_seed
    );
    output_file.seek(SeekFrom::Start(0)).await.unwrap();
    Ok(output_file)
}
