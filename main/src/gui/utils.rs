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

use crate::gui::gitcode_file_data::GitcodeFileData;
use rigela_utils::{get_program_directory, read_file, write_file};
use select::{document::Document, predicate::Class};
use serde::{Deserialize, Serialize};

const HELP_URL: &str =
    "https://gitcode.net/mzdk100/rigela/-/blob/dev/docs/help.txt?format=json&viewer=simple";
const UPDATE_LOG_URL: &str =
    "https://gitcode.net/mzdk100/rigela/-/blob/dev/docs/update_log.txt?format=json&viewer=simple";
// const DOCS_MD5_URL: &str = "https://gitcode.net/mzdk100/rigela/-/blob/dev/docs/docs_md5.toml?format=json&viewer=simple";

pub(crate) const HELP_DIR: &str = "resources/RigelA 帮助文档.txt";
pub(crate) const UPDATE_LOG_DIR: &str = "resources/RigelA 更新日志.txt";
const DOCS_MD5_DIR: &str = "resources/docs_md5.toml";

/// 文档哈希结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct DocsMd5 {
    help_md5: String,
    update_log_md5: String,
}

/// 检测是否需要更新文档。
pub(crate) async fn check_update_docs() -> bool {
    let newest_md5 = get_newest_docs_md5().await;

    // 如果尚未保存哈希值，则直接首次保存，返回未检测到更新，否则再比较哈希值
    if !get_program_directory().join(DOCS_MD5_DIR).exists() {
        save_docs_md5(&newest_md5).await;
        save_docs(true).await;
        save_docs(false).await;

        return false;
    }

    let data = read_file(&get_program_directory().join(DOCS_MD5_DIR))
        .await
        .expect("Can't read docs_md5.toml.");
    let old_md5: DocsMd5 = toml::from_str(&data).expect("Can't parse docs_md5.toml.");

    // 如果更新日志有变动，立即返回可以更新，不做其他操作
    if old_md5.update_log_md5 != newest_md5.update_log_md5 {
        return true;
    }

    // 如果帮助文档有变动，保存更新后的帮助文档，
    // 更新日志需要更新程序后在做更新，程序更新之前不会保存变动的更新日志
    // 这里更新日志和帮助文档的更新混在一起，逻辑稍微有些混乱，后期在优化
    if old_md5.help_md5 != newest_md5.help_md5 {
        save_docs(true).await;
        save_docs_md5(&newest_md5).await;
    }

    false
}

/// 获取最新的docs_md5
pub(crate) async fn get_newest_docs_md5() -> DocsMd5 {
    let help_data = get_gitcode_data(HELP_URL)
        .await
        .expect("Can't get help data.");
    let update_log_data = get_gitcode_data(UPDATE_LOG_URL)
        .await
        .expect("Can't get update_log data.");

    DocsMd5 {
        help_md5: help_data.last_commit_sha,
        update_log_md5: update_log_data.last_commit_sha,
    }
}

/// 保存新的docs_md5
pub(crate) async fn save_docs_md5(md5: &DocsMd5) {
    let docs_md5_path = get_program_directory().join(DOCS_MD5_DIR);
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

/// 异步获取gitcode文件数据
pub(crate) async fn get_gitcode_data(
    url: &str,
) -> Result<GitcodeFileData, Box<dyn std::error::Error>> {
    Ok(reqwest::get(url).await?.json().await?)
}

/// 解析网页的指定节点
pub(crate) async fn parse_html_node(url: &str, node: &str) -> String {
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
