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

use serde::{Deserialize, Serialize};
use std::fs::{write, File};
use std::io::Read;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocsMd5 {
    pub help_md5: String,
    pub update_log_md5: String,
}

pub fn update_docs_md5() {
    let root_dir = std::env::current_dir().unwrap();
    let docs_dir = root_dir.join("../docs");

    let mut help_buf = Vec::<u8>::new();
    let mut update_log_buf = Vec::<u8>::new();

    File::open(docs_dir.join("help.txt"))
        .unwrap()
        .read_to_end(&mut help_buf)
        .unwrap();
    File::open(docs_dir.join("update_log.txt"))
        .unwrap()
        .read_to_end(&mut update_log_buf)
        .unwrap();

    let docs_md5 = DocsMd5 {
        help_md5: format!("{:x}", md5::compute(help_buf)),
        update_log_md5: format!("{:x}", md5::compute(update_log_buf)),
    };

    write(
        docs_dir.join("docs_md5.toml"),
        toml::to_string(&docs_md5).unwrap(),
    )
    .unwrap();
}
