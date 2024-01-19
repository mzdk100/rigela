/*
 * Copyright (c) 2024. The RigelA open source project team and
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

pub mod pipe;

use home::home_dir;
use std::fs::create_dir;
use std::path::PathBuf;
use tokio::io::AsyncReadExt;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

const DIR_NAME: &str = ".rigela";

/// 获取程序存储目录
pub fn get_program_directory() -> PathBuf {
    let program_dir = home_dir()
        .expect("Can't get the current user directory.")
        .join(DIR_NAME);

    if !program_dir.exists() {
        create_dir(&program_dir).expect("Can't create the root directory.");
    }

    program_dir
}

/**
 * 把数据完整写入到文件，这会冲洗现有文件，覆盖写入。
 * `path` 文件路径。
 * `data` 需要写入的数据。
 * */
pub async fn write_file(path: &PathBuf, data: &[u8]) -> Result<(), String> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&path)
        .await;
    if file.is_err() {
        return Err(format!("Can't open the file ({}).", path.display()));
    }
    let res = file
        .unwrap()
        .write_all(data)
        .await;
    if res.is_err() {
        return Err("Can't write the data to file.".to_string());
    }
    Ok(())
}

/// 异步读取文件
pub async fn read_file(path: &PathBuf) -> Result<String, std::io::Error> {
    let mut result = String::new();

    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&path)
        .await?
        .read_to_string(&mut result)
        .await?;

    Ok(result)
}
