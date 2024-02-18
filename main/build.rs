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

//noinspection SpellCheckingInspection
fn main() {
    println!("cargo:rerun-if-changed=locale");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build.rs");

    cargo_rigela::make_version();

    // 如果帮助文件或者更新日志有变动，更新哈希值
    println!("cargo:rerun-if-changed=../docs/help.txt");
    println!("cargo:rerun-if-changed=../docs/update_log.txt");
    rigela_utils::docs_check_update::update_docs_md5();

    let _ = embed_manifest(new_manifest("Contoso.Sample"));
}
