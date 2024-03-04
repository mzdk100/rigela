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

use std::path::PathBuf;
use rigela_utils::library::setup_library;

//noinspection RsModuleNaming
pub(crate) mod IAccessible2Lib;
//noinspection RsModuleNaming
#[allow(non_snake_case)]
pub(crate) mod JabLib;

pub mod ia2;
pub mod jab;


/**
 * 安装动态库。
 * */
pub fn setup() -> (PathBuf, PathBuf) {
    // 注册IAccessible2Proxy.dll
    let ia2_path = setup_library("IAccessible2Proxy.dll", include_bytes!("../lib/IAccessible2Proxy.dll"));

    // 释放windowsaccessbridge-64.dll
    let jab_path = setup_library("windowsaccessbridge-64.dll", include_bytes!("../lib/windowsaccessbridge-64.dll"));
    (ia2_path, jab_path)
}
