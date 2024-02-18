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

use crate::{get_program_directory, parse_html_node, write_file};
use serde::{Deserialize, Serialize};
use std::fs::{write, File};
use std::io::Read;

const HELP_URL: &str = "https://gitcode.net/mzdk100/rigela/-/blob/dev/docs/help.txt";
const UPDATE_LOG_URL: &str = "https://gitcode.net/mzdk100/rigela/-/blob/dev/docs/update_log.txt";
const DOCS_MD5_URL: &str = "https://gitcode.net/mzdk100/rigela/-/blob/dev/docs/docs_md5.toml";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocsMd5 {
    pub help_md5: String,
    pub update_log_md5: String,
}

pub fn update_docs_md5() {
    let root_dir = std::env::current_dir().expect("Can't get the current directory.");
    let docs_dir = root_dir.join("../docs");

    let mut help_buf = Vec::<u8>::new();
    let mut update_log_buf = Vec::<u8>::new();

    File::open(docs_dir.join("help.txt"))
        .expect("Can't open help.txt.")
        .read_to_end(&mut help_buf)
        .expect("Can't read help.txt.");
    File::open(docs_dir.join("update_log.txt"))
        .expect("Can't open update_log.txt.")
        .read_to_end(&mut update_log_buf)
        .expect("Can't read update_log.txt.");

    let docs_md5 = DocsMd5 {
        help_md5: format!("{:x}", md5::compute(help_buf)),
        update_log_md5: format!("{:x}", md5::compute(update_log_buf)),
    };

    write(
        docs_dir.join("docs_md5.toml"),
        toml::to_string(&docs_md5).unwrap(),
    )
    .expect("Can't write docs_md5.toml.")
}

/// 获取最新的docs_md5
pub async fn get_docs_md5() -> DocsMd5 {
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
pub async fn save_docs_md5(md5: &DocsMd5) {
    let docs_md5_path = get_program_directory().join("resources/docs_md5.toml");
    let data = toml::to_string(md5).expect("Can't write docs_md5.toml.");
    write_file(&docs_md5_path, data.as_bytes())
        .await
        .expect("write docs_md5.toml error.");
}

/// 保存最新的文档。
pub async fn save_docs(help_or_update_log: bool) {
    if help_or_update_log {
        let help_path = get_program_directory().join("resources/help.txt");
        let help_text = parse_html_node(HELP_URL, "blob-content").await;
        write_file(&help_path, help_text.as_bytes())
            .await
            .expect("Can't write help.txt.");
    } else {
        let update_log_path = get_program_directory().join("resources/update_log.txt");
        let update_log_text = parse_html_node(UPDATE_LOG_URL, "blob-content").await;
        write_file(&update_log_path, update_log_text.as_bytes())
            .await
            .expect("Can't write update_log.txt.");
    }
}
