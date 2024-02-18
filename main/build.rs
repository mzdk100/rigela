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

extern crate cargo_rigela;

use embed_manifest::{embed_manifest, new_manifest};
use serde::{Deserialize, Serialize};
use std::fs::{write, File};
use std::io::Read;

//noinspection SpellCheckingInspection
fn main() {
    println!("cargo:rerun-if-changed=locale");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build.rs");

    cargo_rigela::make_version();

    // 如果帮助文件或者更新日志有变动，更新哈希值
    println!("cargo:rerun-if-changed=../docs/help.txt");
    println!("cargo:rerun-if-changed=../docs/update_log.txt");
    update_docs_md5();

    let _ = embed_manifest(new_manifest("Contoso.Sample"));
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct DocsMd5 {
    help_md5: String,
    update_log_md5: String,
}

fn update_docs_md5() {
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
        toml::to_string(&docs_md5).expect("Can't write docs_md5.toml."),
    )
    .expect("Can't write docs_md5.toml.")
}
