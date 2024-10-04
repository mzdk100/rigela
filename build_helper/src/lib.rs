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

extern crate embed_resource;

use cargo_emit::rerun_if_changed;
use chrono::{Datelike, Local};
use std::env;
use std::env::current_dir;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

const VERSION: &str = r#"
#include <windows.h>

#ifndef DEBUG
#define VER_DEBUG 0
#else
#define VER_DEBUG VS_FF_DEBUG
#endif

/*
 * 版本信息定义。
 * 参考： https://learn.microsoft.com/zh-cn/windows/win32/menurc/versioninfo-resource?redirectedfrom=MSDN
 * */
VS_VERSION_INFO VERSIONINFO
FILEVERSION {VERSION}
PRODUCTVERSION {VERSION}
FILEFLAGSMASK VS_FFI_FILEFLAGSMASK
FILEFLAGS VER_DEBUG
FILEOS VOS__WINDOWS32
FILETYPE VFT_DLL
FILESUBTYPE VFT2_UNKNOWN
BEGIN
    BLOCK "StringFileInfo"
    BEGIN
        BLOCK "040904E4"
        BEGIN
            VALUE "CompanyName", "sscn"
            VALUE "FileDescription", "{DESCRIPTION}"
            VALUE "FileVersion", "{VERSION_NAME}\0"
            VALUE "InternalName", "{PRODUCT}"
            VALUE "LegalCopyright", "{COPYRIGHT}"
            VALUE "OriginalFilename", "{PRODUCT}.exe"
            VALUE "ProductName", "{NAME}"
            VALUE "ProductVersion", "{VERSION_NAME}\0"
        END
    END

    BLOCK "VarFileInfo"

    /*
     * 以下行只能针对本地化版本进行修改。
     * 它由任意数量的WORD、WORD对组成，每对描述文件支持的语言、代码页组合。
     * 例如，文件可能具有值“0x409,1252”，表示它支持Windows ANSI代码页（1252）中的英语（0x409）。
     * */
    BEGIN
        VALUE "Translation", 0x804, 1252
    END
END

/*
 * 图标资源
 * */
IDI_ICON1 ICON DISCARDABLE "{LOGO}"
"#;
pub fn make_version() {
    let path = Path::new(env::var("OUT_DIR").unwrap().as_str()).join("version.rc");
    let logo_path = current_dir().unwrap().parent().unwrap().join("images").join("logo.ico");
    let mut description = env::var("CARGO_PKG_DESCRIPTION").unwrap();
    if description.is_empty() {
        description = "RigelA screen reader (rsr).".to_string()
    }
    let mut version = String::from(VERSION);
    version = version.replace("{NAME}", env::var("CARGO_PKG_NAME").unwrap().as_str());
    version = version.replace("{DESCRIPTION}", description.as_str());
    version = version.replace(
        "{VERSION}",
        env::var("CARGO_PKG_VERSION")
            .unwrap()
            .replace(".", ",")
            .as_str(),
    );
    version = version.replace("{VERSION_NAME}", get_vcs_last_commit_hash().as_str());
    version = version.replace("{PRODUCT}", "rigela");
    version = version.replace("{COPYRIGHT}", format!("Copyright (C) 2023-{} The RigelA Open Source Project. RigelA and all its contributors. All rights reserved.", Local::now().year()).as_str());
    version = version.replace("{LOGO}", &logo_path.to_str().unwrap().replace(r"\", r"\\"));
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .unwrap()
        .write_all(version.as_bytes())
        .unwrap();
    embed_resource::compile(&path, embed_resource::NONE);
    rerun_if_changed!(logo_path.display(), "build.rs");
}

fn get_vcs_last_commit_hash() -> String {
    let mut cmd = Command::new("git")
        .stdout(Stdio::piped())
        .args(["log", "-n", "1", r#"--pretty=format:"%H""#])
        .spawn()
        .unwrap();
    cmd.wait().unwrap();
    let mut out = String::new();
    cmd.stdout.unwrap().read_to_string(&mut out).unwrap_or(0);
    out.replace(r#"""#, "")
}
