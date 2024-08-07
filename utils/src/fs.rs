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

use std::{
    fs::create_dir,
    io::Error,
    path::{Path, PathBuf},
};
use tokio::{
    fs::{metadata, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};
use win_wrap::shell::{get_known_folder_path, FOLDERID_Profile, KF_FLAG_DEFAULT};

pub const DIR_NAME: &str = ".rigela";

/// 获取程序存储目录
pub fn get_rigela_program_directory() -> PathBuf {
    let home_path = get_known_folder_path(&FOLDERID_Profile, KF_FLAG_DEFAULT, None).unwrap();
    let program_dir = Path::new(&home_path).join(DIR_NAME);

    if !program_dir.exists() {
        create_dir(&program_dir).expect("Can't create the root directory.");
    }

    program_dir
}

/**
 获取文件已修改的时长（单位是秒），如果文件不存在或遇到其他错误则返回u64::MAX。
 `path` 文件路径。
 */
pub async fn get_file_modified_duration(path: &PathBuf) -> u64 {
    let Ok(attr) = metadata(&path).await else {
        return u64::MAX;
    };
    let Ok(modified) = attr.modified() else {
        return u64::MAX;
    };
    match modified.elapsed() {
        Ok(d) => d.as_secs(),
        Err(_) => u64::MAX,
    }
}

/**
 把数据完整写入到文件，这会冲洗现有文件，覆盖写入。
 `path` 文件路径。
 `data` 需要写入的数据。
 */
pub async fn write_file(path: &PathBuf, data: &[u8]) -> Result<(), Error> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&path)
        .await?
        .write_all(data)
        .await
}

/**
 异步读取文件
 `path` 文件路径。
 */
pub async fn read_file(path: &PathBuf) -> Result<String, Error> {
    let mut result = String::new();
    OpenOptions::new()
        .read(true)
        .open(&path)
        .await?
        .read_to_string(&mut result)
        .await?;
    Ok(result)
}
