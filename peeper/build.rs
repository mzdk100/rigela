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

use std::{
    env,
    fs::{copy, read_dir},
    path::Path,
};

fn copy_deps(target: &str) {
    let cargo = env::var("CARGO").unwrap();
    let lib_path = Path::new(&cargo)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("lib")
        .join("rustlib")
        .join(target)
        .join("lib");
    let lib_dir = read_dir(&lib_path).unwrap();
    let target_dir = Path::new(&env::var("USERPROFILE").unwrap()).join(".rigela");
    for i in lib_dir {
        let i = i.unwrap();
        let f = i.file_name().into_string().unwrap().to_lowercase();
        if f.ends_with(".dll") && f.starts_with("std") {
            let t = target_dir.join(&i.file_name());
            if t.exists() {
                continue;
            }
            println!("cargo:rerun-if-changed={}", t.display());
            copy(i.path(), t).unwrap();
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build.rs");
    if env::var("DEBUG").unwrap_or("false".to_string()) != "true" {
        return;
    }

    // 此脚本用于调试模式下构建，因为rust编译器的优化策略问题，debug版本的peeper.dll需要依赖rust标准库的dll
    copy_deps("x86_64-pc-windows-msvc");
    copy_deps("i686-pc-windows-msvc");
}
