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

pub mod bass;
pub mod docs_check_update;
pub mod logger;
pub mod pipe;
pub mod resample;

use clipboard::{ClipboardContext, ClipboardProvider};
use home::home_dir;
use select::{document::Document, predicate::Class};
use std::{fs::create_dir, io::Error, path::PathBuf};
use tokio::{
    fs::{metadata, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

#[macro_export]
macro_rules! call_proc {
    ($module:expr,$name:ident,$def:ty,$($arg:expr),*) => {{
        let f = get_proc_address($module, stringify!($name));
        if !f.is_none() {
            unsafe {
                let r = (&*((&f) as *const FARPROC as *const $def)) ($($arg),*);
                Some(r)
            }
        } else {
            None
        }
    }};
}

pub const DIR_NAME: &str = ".rigela";

//noinspection HttpUrlsUsage
pub const SERVER_HOME_URI: &str = "http://api.zhumang.vip:8080/rigela";

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
 * 获取文件已修改的时长（单位是秒），如果文件不存在或遇到其他错误则返回u64::MAX。
 * `path` 文件路径。
 * */
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
 * 把数据完整写入到文件，这会冲洗现有文件，覆盖写入。
 * `path` 文件路径。
 * `data` 需要写入的数据。
 * */
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

/// 异步读取文件
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

/// 异步获取网页正文
pub async fn download_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    match reqwest::get(url).await {
        Ok(response) => {
            let html = response.text().await?;
            Ok(html)
        }
        Err(err) => Err(Box::new(err)),
    }
}

/// 解析网页的指定节点
pub async fn parse_html_node(url: &str, node: &str) -> String {
    let html = download_html(url).await.unwrap();
    let document = Document::from(html.as_str());
    let mut result = String::new();
    for i in document.find(Class(node)) {
        result += (i.text().trim().to_owned() + "\r\n").as_str();
    }
    result
}

/**
 * 获取剪贴板文本数据。
 * */
pub fn get_clipboard_text() -> String {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.get_contents().unwrap()
}

/**
 * 设置剪贴板文本数据。
 * `text` 数据内容。
 * */
pub fn set_clipboard_text(text: String) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(text).unwrap()
}
