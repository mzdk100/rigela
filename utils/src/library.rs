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

use crate::fs::get_program_directory;
use log::error;
use std::{
    fs::{create_dir, OpenOptions},
    io::Write,
    path::PathBuf,
};

/**
 * 获取库的路径。
 * `lib_name` 库名称。
 * */
pub fn get_library_path(lib_name: &str) -> PathBuf {
    let path = get_program_directory().join("libs");
    if !path.exists() {
        create_dir(&path).unwrap();
    }

    path.join(lib_name)
}

/**
 * 安装一个动态库文件到个人文件夹中。
 * `lib_name` 库名称。
 * `lib_bin` 库的二进制数据。
 * */
pub fn setup_library(lib_name: &str, lib_bin: &[u8]) -> PathBuf {
    let path = get_library_path(lib_name);
    if path.exists() {
        return path;
    }

    match OpenOptions::new().write(true).create(true).open(&path) {
        Ok(mut f) => match f.write_all(lib_bin) {
            Ok(_) => {}
            Err(e) => error!("Can't setup the `{}` library. {}", path.display(), e),
        },
        Err(e) => error!("Can't setup the `{}` library. {}", path.display(), e),
    }
    path
}

#[macro_export]
macro_rules! call_proc {
    ($module:expr,$name:ident,$def:ty,$($arg:expr),*) => {{
        let f = get_proc_address($module, stringify!($name));
        if !f.is_none() {
            unsafe {
                let r = (&*((&f) as *const FARPROC as *const $def)) ($($arg),*);
                Some(r)
            }
        } else {
            None
        }
    }};
}
