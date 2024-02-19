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
use rigela_utils::{get_program_directory, write_file};
use select::{document::Document, predicate::Class};
use serde::{Deserialize, Serialize};

const HELP_URL: &str =
    "https://gitcode.net/mzdk100/rigela/-/blob/dev/docs/help.txt?format=json&viewer=simple";
const UPDATE_LOG_URL: &str =
    "https://gitcode.net/mzdk100/rigela/-/blob/dev/docs/update_log.txt?format=json&viewer=simple";
const DOCS_MD5_URL: &str =
    "https://gitcode.net/mzdk100/rigela/-/blob/dev/docs/docs_md5.toml?format=json&viewer=simple";

pub(crate) const HELP_DIR: &str = "resources/RigelA 帮助文档.txt";
pub(crate) const UPDATE_LOG_DIR: &str = "resources/RigelA 更新日志.txt";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct DocsMd5 {
    help_md5: String,
    update_log_md5: String,
}

pub(crate) async fn update_docs() -> bool {
    let md5_path = get_program_directory().join("resources/docs_md5.toml");
    if !md5_path.exists() {
        save_docs_md5(&get_docs_md5().await).await;
        save_docs(true).await;
        save_docs(false).await;
        return true;
    }
    false
}

/// 获取最新的docs_md5
pub(crate) async fn get_docs_md5() -> DocsMd5 {
    let _docs_md5_text = parse_html_node(DOCS_MD5_URL, "blob-content").await;
    // Todo: 解析docs_md5.toml

    let help_md5 = "".to_string();
    let update_log_md5 = "".to_string();
    DocsMd5 {
        help_md5,
        update_log_md5,
    }
}

/// 保存新的docs_md5
pub(crate) async fn save_docs_md5(md5: &DocsMd5) {
    let docs_md5_path = get_program_directory().join("resources/docs_md5.toml");
    let data = toml::to_string(md5).expect("Can't write docs_md5.toml.");
    write_file(&docs_md5_path, data.as_bytes())
        .await
        .expect("write docs_md5.toml error.");
}

/// 保存最新的文档。
pub(crate) async fn save_docs(help_or_update_log: bool) {
    if help_or_update_log {
        let help_path = get_program_directory().join(HELP_DIR);
        let help_text = parse_html_node(HELP_URL, "blob-content").await;
        write_file(&help_path, help_text.as_bytes())
            .await
            .expect("Can't write help.txt.");
    } else {
        let update_log_path = get_program_directory().join(UPDATE_LOG_DIR);
        let update_log_text = parse_html_node(UPDATE_LOG_URL, "blob-content").await;
        write_file(&update_log_path, update_log_text.as_bytes())
            .await
            .expect("Can't write update_log.txt.");
    }
}

/// 异步获取网页正文
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

    let mut result = String::new();
    for i in Document::from(html.as_str()).find(Class(node)) {
        result += format!("{}\r\n", i.text().trim()).as_str();
    }

    result
}
