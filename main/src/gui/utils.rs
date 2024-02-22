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

use log::error;
use rigela_utils::{get_program_directory, read_file, write_file};
use select::{document::Document, predicate::Class};
use serde::{Deserialize, Serialize};
use std::env::args;
use std::error::Error;

const HELP_URL: &str =
    "https://gitcode.net/mzdk100/rigela/-/blob/dev/main/docs/help.txt?format=json&viewer=simple";
const UPDATE_LOG_URL: &str =
    "https://gitcode.net/mzdk100/rigela/-/blob/dev/main/docs/update_log.txt?format=json&viewer=simple";
//noinspection HttpUrlsUsage
const UPDATE_BIN_URL: &str = "http://api.zhumang.vip:8080/rigela/rigela_x64/updater.exe";

pub(crate) const HELP_DIR: &str = "resources/RigelA 帮助文档.txt";
pub(crate) const UPDATE_LOG_DIR: &str = "resources/RigelA 更新日志.txt";
const DOCS_MD5_FILE: &str = "resources/docs_md5.toml";
pub(crate) const UPDATE_BIN_DIR: &str = "libs/update.exe";

/// 文档哈希结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct DocsMd5 {
    help_md5: String,
    update_log_md5: String,
}

pub(crate) enum UpdateState {
    None,
    Updated,

    HasNewest,
}
/// 检测是否需要更新
pub(crate) async fn check_update() -> UpdateState {
    let newest_md5 = get_newest_docs_md5().await;
    let md5_file = get_program_directory().join(DOCS_MD5_FILE);

    // 如果尚未保存哈希值，则直接首次保存，返回未检测到更新，否则再比较哈希值
    if !md5_file.exists() {
        save_docs_md5(&newest_md5).await;
        save_docs(true).await;
        save_docs(false).await;

        return UpdateState::None;
    }

    let old_md5: DocsMd5 = match read_file(&md5_file).await {
        Ok(ref d) => toml::from_str(d).expect("Can't parse docs_md5.toml."),
        Err(e) => {
            error!("Can't read docs_md5.toml. {}", e);
            return UpdateState::None;
        }
    };

    // 如果更新日志有变动，立即返回可以更新
    if old_md5.update_log_md5 != newest_md5.update_log_md5 {
        // 检查当前进程是否被更新器调用，如果更新器成功更新了主程序，会使用--updated命令行参数调用
        if !args().find(|i| i == "--updated").is_none() {
            save_docs(false).await;
            save_docs_md5(&newest_md5).await;
            // 更新流程完毕
            return UpdateState::Updated;
        }
        return UpdateState::HasNewest;
    }

    // 如果帮助文档有变动，保存更新后的帮助文档，
    if old_md5.help_md5 != newest_md5.help_md5 {
        save_docs(true).await;
        save_docs_md5(&newest_md5).await;
    }

    UpdateState::None
}

/// 获取最新的docs_md5
pub(crate) async fn get_newest_docs_md5() -> DocsMd5 {
    let help_md5 = match get_gitcode_data(HELP_URL).await {
        Ok(x) => x.last_commit_sha,
        Err(e) => {
            error!("Can't get help data. {}", e);
            String::new()
        }
    };
    let update_log_md5 = match get_gitcode_data(UPDATE_LOG_URL).await {
        Ok(x) => x.last_commit_sha,
        Err(e) => {
            error!("Can't get update_log data. {}", e);
            String::new()
        }
    };

    DocsMd5 {
        help_md5,
        update_log_md5,
    }
}

/// 保存新的docs_md5
pub(crate) async fn save_docs_md5(md5: &DocsMd5) {
    let docs_md5_path = get_program_directory().join(DOCS_MD5_FILE);
    let data = toml::to_string(md5).expect("Can't parse to docs_md5.toml.");

    write_file(&docs_md5_path, data.as_bytes())
        .await
        .expect("write docs_md5.toml error.");
}

/// 保存最新的文档。
pub(crate) async fn save_docs(help_or_update_log: bool) {
    let path = if help_or_update_log {
        get_program_directory().join(HELP_DIR)
    } else {
        get_program_directory().join(UPDATE_LOG_DIR)
    };
    let url = if help_or_update_log {
        HELP_URL
    } else {
        UPDATE_LOG_URL
    };

    write_file(&path, parse_html_node(url, "blob-content").await.as_bytes())
        .await
        .expect("Can't write docs file.");
}

// 异步获取gitcode文件数据
async fn get_gitcode_data(url: &str) -> Result<GitcodeFileData, Box<dyn Error>> {
    Ok(reqwest::get(url).await?.json().await?)
}

// 解析网页的指定节点
async fn parse_html_node(url: &str, node: &str) -> String {
    let html = match get_gitcode_data(url).await {
        Ok(data) => data.html,
        Err(_) => "".to_string(),
    };

    Document::from(html.as_str())
        .find(Class(node))
        .map(|node| node.text())
        .collect::<Vec<String>>()
        .join("\n")
}

/// 确认更新器存在
pub(crate) async fn confirm_update_exists() -> Result<(), Box<dyn Error>> {
    let path = get_program_directory().join(UPDATE_BIN_DIR);
    if path.exists() {
        Ok(())
    } else {
        let resp = reqwest::get(UPDATE_BIN_URL).await?;
        write_file(&path, &resp.bytes().await?).await?;
        Ok(())
    }
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
