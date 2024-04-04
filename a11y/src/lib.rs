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

#![doc = include_str!("../README.md")]

#[cfg(any(feature = "JabLib", feature = "IAccessible2Lib"))]
use rigela_utils::library::{get_library_path, setup_library};
#[cfg(any(feature = "JabLib", feature = "IAccessible2Lib"))]
use std::path::PathBuf;

//noinspection RsModuleNaming
#[cfg(feature = "IAccessible2Lib")]
#[doc(hidden)]
pub(crate) mod IAccessible2Lib;
//noinspection RsModuleNaming
#[cfg(feature = "JabLib")]
#[allow(non_snake_case)]
#[doc(hidden)]
pub(crate) mod JabLib;

#[cfg(feature = "ia2")]
#[cfg_attr(docsrs, doc(cfg(feature = "ia2")))]
pub mod ia2;
#[cfg(feature = "jab")]
#[cfg_attr(docsrs, doc(cfg(feature = "jab")))]
pub mod jab;

#[cfg(feature = "IAccessible2Lib")]
const IA2_LIB_NAME: &str = "IAccessible2Proxy.dll";

#[cfg(all(target_arch = "x86", feature = "JabLib"))]
const JAB_LIB_NAME: &str = "windowsaccessbridge-32.dll";
#[cfg(all(target_arch = "x86_64", feature = "JabLib"))]
const JAB_LIB_NAME: &str = "windowsaccessbridge-64.dll";

/**
 * 安装动态库。
 * */
pub fn setup() {
    // 注册IAccessible2Proxy.dll
    #[cfg(feature = "IAccessible2Lib")]
    setup_library(IA2_LIB_NAME, include_bytes!("../lib/IAccessible2Proxy.dll"));

    // 释放windowsaccessbridge.dll
    // 二进制提取自https://builds.openlogic.com/downloadJDK/openlogic-openjdk/8u402-b06/openlogic-openjdk-8u402-b06-windows-x32.zip
    #[cfg(all(target_arch = "x86", feature = "JabLib"))]
    setup_library(
        JAB_LIB_NAME,
        include_bytes!("../lib/WindowsAccessBridge-32.dll"),
    );

    // 二进制提取自https://corretto.aws/downloads/resources/17.0.8.7.1/amazon-corretto-17.0.8.7.1-windows-x64-jdk.zip
    #[cfg(all(target_arch = "x86_64", feature = "JabLib"))]
    setup_library(
        JAB_LIB_NAME,
        include_bytes!("../lib/windowsaccessbridge-64.dll"),
    );
}

/**
 * 获取IA2动态库的安装路径。
 * */
#[cfg(feature = "IAccessible2Lib")]
pub fn get_ia2_lib_path() -> PathBuf {
    get_library_path(IA2_LIB_NAME)
}

/**
 * 获取JAB动态库的安装路径。
 * */
#[cfg(feature = "JabLib")]
pub fn get_jab_lib_path() -> PathBuf {
    get_library_path(JAB_LIB_NAME)
}
