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
use rigela_utils::library::{get_library_path, setup_library};

//noinspection RsModuleNaming
pub(crate) mod IAccessible2Lib;
//noinspection RsModuleNaming
#[allow(non_snake_case)]
pub(crate) mod JabLib;

pub mod ia2;
pub mod jab;

const IA2_LIB_NAME: &str = "IAccessible2Proxy.dll";
const JAB_LIB_NAME: &str = "windowsaccessbridge-64.dll";

/**
 * 安装动态库。
 * */
pub fn setup() {
    // 注册IAccessible2Proxy.dll
    setup_library(IA2_LIB_NAME, include_bytes!("../lib/IAccessible2Proxy.dll"));

    // 释放windowsaccessbridge-64.dll
    setup_library(JAB_LIB_NAME, include_bytes!("../lib/windowsaccessbridge-64.dll"));
}

/**
 * 获取IA2动态库的安装路径。
 * */
pub fn get_ia2_lib_path() -> PathBuf {
    get_library_path(IA2_LIB_NAME)
}

/**
 * 获取JAB动态库的安装路径。
 * */
pub fn get_jab_lib_path() -> PathBuf {
    get_library_path(JAB_LIB_NAME)
}