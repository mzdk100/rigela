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

#[cfg(any(feature = "jab_lib", feature = "ia2_lib"))]
use rigela_utils::library::{get_rigela_library_path, setup_library};
#[cfg(any(feature = "jab_lib", feature = "ia2_lib"))]
use std::{path::PathBuf, sync::OnceLock};


#[cfg(feature = "ia2")]
pub mod ia2;
#[cfg(feature = "jab")]
pub mod jab;
#[cfg(feature = "scintilla")]
pub mod scintilla;

#[cfg(feature = "ia2_lib")]
const IA2_LIB_NAME: &str = "IAccessible2Proxy.dll";

#[cfg(all(target_arch = "x86", feature = "jab_lib"))]
const JAB_LIB_NAME: &str = "windowsaccessbridge-32.dll";
#[cfg(all(target_arch = "x86_64", feature = "jab_lib"))]
const JAB_LIB_NAME: &str = "windowsaccessbridge-64.dll";

static LIBRARY_DIRECTORY: OnceLock<PathBuf> = OnceLock::new();

/**
 获取动态库的路径。
 `lib_name` 库名称。
 */
pub fn get_library_path(lib_name: &str) -> PathBuf {
    LIBRARY_DIRECTORY.get().unwrap().join(lib_name)
}

/**
 给RigelA程序安装动态库。
 */
pub fn setup_for_rigela() {
    let path = get_rigela_library_path();
    setup(&path)
}

/**
 安装动态库。
 `path` 存放动态库的目录。
 */
pub fn setup(path: &PathBuf) {
    // 注册IAccessible2Proxy.dll
    #[cfg(feature = "ia2_lib")]
    setup_library(&path.join(IA2_LIB_NAME), include_bytes!("../lib/IAccessible2Proxy.dll"));

    // 释放windowsaccessbridge.dll
    // 二进制提取自https://builds.openlogic.com/downloadJDK/openlogic-openjdk/8u402-b06/openlogic-openjdk-8u402-b06-windows-x32.zip
    #[cfg(all(target_arch = "x86", feature = "jab_lib"))]
    setup_library(&path.join(JAB_LIB_NAME), include_bytes!("../lib/WindowsAccessBridge-32.dll"));

    // 二进制提取自https://corretto.aws/downloads/resources/17.0.8.7.1/amazon-corretto-17.0.8.7.1-windows-x64-jdk.zip
    #[cfg(all(target_arch = "x86_64", feature = "jab_lib"))]
    setup_library(&path.join(JAB_LIB_NAME), include_bytes!("../lib/windowsaccessbridge-64.dll"));
    let _ = LIBRARY_DIRECTORY.set(path.clone());
}

/**
 获取IA2动态库的安装路径。
 */
#[cfg(feature = "ia2_lib")]
pub fn get_ia2_lib_path() -> PathBuf {
    LIBRARY_DIRECTORY.get().unwrap().join(IA2_LIB_NAME)
}

/**
 获取JAB动态库的安装路径。
 */
#[cfg(feature = "jab_lib")]
pub fn get_jab_lib_path() -> PathBuf {
    LIBRARY_DIRECTORY.get().unwrap().join(JAB_LIB_NAME)
}
