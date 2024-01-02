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
use futures_util::StreamExt;
use std::collections::HashMap;
use std::future;
use std::path::Path;
use bitar::chunker::Config::RollSum;
use bitar::chunker::FilterConfig;
use bitar::{Compression, HashSum};
use bitar::chunk_dictionary::{ChunkDescriptor, ChunkDictionary, ChunkerParameters};
use bitar::chunk_dictionary::chunker_parameters::ChunkingAlgorithm;
use blake2::{Blake2b512, Digest};
use tokio::{fs::{File, OpenOptions}, io::AsyncWriteExt};
use tokio::fs::{create_dir, read_dir};
use tokio::task::spawn_blocking;

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const COMPRESSED_DIR: &str = "compressed";
const FILES_DIR: &str = "dev";
async fn compress(resource_name: &str) {
    let output = Path::new(COMPRESSED_DIR)
        .join(format!("{}.cba", resource_name));
    let input = Path::new(FILES_DIR).join(resource_name);
    let mut input_file = File::open(&input)
        .await
        .expect(
            format!("Failed to open input file {}.", input.display()).as_str()
        );
    let hash_config = FilterConfig::default();
    let chunkier_config = RollSum(hash_config.clone());
    let compression = Compression::brotli(6).unwrap();
    let chunkier = chunkier_config.new_chunker(&mut input_file);
    let mut source_hasher = Blake2b512::new();
    let mut source_size: u64 = 0;
    let mut unique_chunks = HashMap::new();
    let mut unique_chunk_index: usize = 0;
    let mut chunk_order = Vec::new();
    let mut archive_chunks = Vec::new();
    let mut archive_body = Vec::new();
    {  // 需要单独的块来限制对chunk_order变量的可变引用时间不能太长，否则在最后生成file_header的时候无法对chunk_order进行不可变引用
        let mut chunk_stream = chunkier
            .map(|r| {
                let (offset, chunk) = r
                    .expect("error while chunking");
                source_hasher
                    .update(chunk.data());
                source_size += chunk.len() as u64;
                spawn_blocking(move || (offset, chunk.verify()))
            })
            .buffered(4)
            .filter_map(|r| {
                let (offset, verified) = r
                    .expect("error while hashing chunk");
                let (unique, chunk_index) = if unique_chunks.contains_key(verified.hash()) {
                    (false, *unique_chunks.get(verified.hash()).unwrap())
                } else {
                    let chunk_index = unique_chunk_index;
                    unique_chunks.insert(verified.hash().clone(), chunk_index);
                    unique_chunk_index += 1;
                    (true, chunk_index)
                };
                // 将指针（作为索引）存储到每个区块的唯一区块索引
                chunk_order.push(chunk_index);
                future::ready(if unique {
                    Some((chunk_index, offset, verified))
                } else {
                    None
                })
            })
            .map(|(chunk_index, offset, verified)| {
                spawn_blocking(move || {
                    let compressed = verified
                        .chunk()
                        .clone()
                        .compress(Some(compression))
                        .expect("compress chunk");
                    (chunk_index, offset, verified, compressed)
                })
            })
            .buffered(4);
        let mut archive_offset: u64 = 0;
        while let Some(r) = chunk_stream.next().await {
            let (index, offset, verified, compressed) = r
                .expect("Error compressing.");
            let chunk_len = verified.len();
            let use_uncompressed = compressed.len() >= chunk_len;
            println!(
                "Chunk {}, '{}', offset: {}, size: {}, {}",
                index, verified.hash(), offset, chunk_len,
                if use_uncompressed {
                    "left uncompressed".to_owned()
                } else {
                    format!("compressed to: {}", compressed.len())
                },
            );
            let (mut hash, chunk) = verified.into_parts();
            let use_data = if use_uncompressed {
                chunk.data()
            } else {
                compressed.data()
            };
            hash.truncate(HashSum::MAX_LEN);
            // 存储引用压缩数据的描述符
            archive_chunks.push(ChunkDescriptor {
                checksum: hash.to_vec(),
                source_size: chunk_len as u32,
                archive_offset,
                archive_size: use_data.len() as u32,
            });
            archive_offset += use_data.len() as u64;
            // 写入压缩区块
            archive_body.extend(use_data);
        }
    }
    let mut output_file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(true)
        .create_new(false)
        .open(output.clone())
        .await
        .expect(
            format!("Failed to open the output file {}.", output.display()).as_str()
        );
    let chunkier_params = ChunkerParameters {
        chunk_filter_bits: hash_config.filter_bits.bits(),
        min_chunk_size: hash_config.min_chunk_size as u32,
        max_chunk_size: hash_config.max_chunk_size as u32,
        rolling_hash_window_size: hash_config.window_size as u32,
        chunk_hash_length: HashSum::MAX_LEN as u32,
        chunking_algorithm: ChunkingAlgorithm::Rollsum as i32
    };
    // 构建最终存档
    let file_header = ChunkDictionary {
        rebuild_order: chunk_order.iter().map(|x| *x as u32).collect(),
        application_version: PKG_VERSION.to_string(),
        chunk_descriptors: archive_chunks,
        source_checksum: source_hasher.finalize().to_vec(),
        chunk_compression: Some(Some(compression).into()),
        source_total_size: source_size,
        chunker_params: Some(chunkier_params),
    };
    let header_buf = bitar::header::build(&file_header, None).unwrap();
    output_file
        .write_all(&header_buf)
        .await
        .expect(
            format!("Failed to write the header to output file {}.", output.display()).as_str()
        );
    output_file
        .write_all(&archive_body[..])
        .await
        .expect("Failed to write the body to output file.");
}

#[tokio::main]
async fn main() {
    let path = Path::new(COMPRESSED_DIR);
    if !path.exists() {
        create_dir(path)
            .await
            .expect("Can't create the compressed output directory.");
    }
    let mut dir = read_dir(Path::new(FILES_DIR))
        .await
        .expect("Can't read the dev directory.");
    while let Ok(Some(x)) = dir.next_entry().await {
        let filename = x.file_name();
        let name = filename.as_os_str()
            .to_str()
            .expect("Can't read the filename.");
        compress(name).await;
    }
}