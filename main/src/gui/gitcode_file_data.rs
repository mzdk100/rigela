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

// YApi QuickType插件生成，具体参考文档:https://plugins.jetbrains.com/plugin/18847-yapi-quicktype/documentation

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GitcodeFileData {
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
