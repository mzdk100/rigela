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

use reqwest::header::{CONTENT_LENGTH, RANGE};
use select::{document::Document, predicate::Class};
use serde::{Deserialize, Serialize};
use std::env::temp_dir;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

const UPDATE_LOG_URL: &str =
    "https://gitcode.net/mzdk100/rigela/-/blob/dev/docs/update_log.txt?format=json&viewer=simple";

//noinspection HttpUrlsUsage
const BIN_URL: &str = "http://api.zhumang.vip:8080/rigela/rigela_x64/rigela-main.exe";
const BIN_NAME: &str = "RigelA_main.exe";

/// 下载的二进制临时文件路径
pub(crate) fn bin_path() -> PathBuf {
    temp_dir().join(BIN_NAME)
}

/// 获取更新日志
pub(crate) async fn get_update_log() -> Result<String, Box<dyn std::error::Error>> {
    Ok(parse_html_node(UPDATE_LOG_URL, "blob-content").await)
}

/// 下载, 替换二进制文件
pub(crate) async fn download_and_replace_bin(
    cb: impl Fn(u32),
) -> Result<(), Box<dyn std::error::Error>> {
    const CHUNK_SIZE: usize = 10240;

    // Test
    cb(5);

    let client = reqwest::Client::new();
    let response = client.head(BIN_URL).send().await?;
    let length = response
        .headers()
        .get(CONTENT_LENGTH)
        .ok_or("response doesn't include the content length")?;
    let length = u64::from_str(length.to_str()?).map_err(|_| "invalid Content-Length header")?;

    let mut output_file = File::create(bin_path())?;

    for range in (0..length).step_by(CHUNK_SIZE) {
        let response = client.get(BIN_URL).header(RANGE, range).send().await?;
        output_file.write(&response.bytes().await?)?;
        cb(((range + CHUNK_SIZE as u64) / length) as u32);
    }

    // 替换

    cb(101);

    Ok(())
}

/// 异步获取gitcode文件数据
async fn get_gitcode_data(url: &str) -> Result<GitcodeFileData, Box<dyn std::error::Error>> {
    Ok(reqwest::get(url).await?.json().await?)
}

/// 解析网页的指定节点
async fn parse_html_node(url: &str, node: &str) -> String {
    let html = match get_gitcode_data(url).await {
        Ok(data) => data.html,
        Err(_) => "".to_string(),
    };

    Document::from(html.as_str())
        .find(Class(node))
        .map(|node| node.text())
        .collect::<Vec<String>>()
        .join("\r\n")
}

//noinspection DuplicatedCode
// YApi QuickType插件生成，具体参考文档:https://plugins.jetbrains.com/plugin/18847-yapi-quicktype/documentation
#[derive(Serialize, Deserialize)]
struct GitcodeFileData {
    #[serde(rename = "tree_path")]
    tree_path: String,

    #[serde(rename = "extension")]
    extension: String,

    #[serde(rename = "last_commit_sha")]
    pub last_commit_sha: String,

    #[serde(rename = "blame_path")]
    blame_path: String,

    #[serde(rename = "simple_viewer")]
    simple_viewer: String,

    #[serde(rename = "show_viewer_switcher")]
    show_viewer_switcher: bool,

    #[serde(rename = "path")]
    path: String,

    #[serde(rename = "size")]
    size: i64,

    #[serde(rename = "mime_type")]
    mime_type: String,

    #[serde(rename = "binary")]
    binary: bool,

    #[serde(rename = "commits_path")]
    commits_path: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "html")]
    pub html: String,

    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "permalink")]
    permalink: String,

    #[serde(rename = "raw_path")]
    raw_path: String,
}
