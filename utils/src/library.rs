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

use crate::fs::get_rigela_program_directory;
use log::error;
use std::{
    fs::{create_dir, OpenOptions},
    io::Write,
    path::PathBuf,
};

/**
 获取RigelA库目录的路径。
 */
pub fn get_rigela_library_path() -> PathBuf {
    let path = get_rigela_program_directory().join("libs");
    if !path.exists() {
        create_dir(&path).unwrap();
    }
    path
}

/**
安装一个动态库文件到指定的目录。
`path` 动态库路径。
`lib_bin` 库的二进制数据。
*/
pub fn setup_library(path: &PathBuf, lib_bin: &[u8]) {
    if path.exists() {
        return;
    }

    match OpenOptions::new().write(true).create(true).open(&path) {
        Ok(mut f) => match f.write_all(lib_bin) {
            Ok(_) => {}
            Err(e) => error!("Can't setup the `{}` library. {}", path.display(), e),
        },
        Err(e) => error!("Can't setup the `{}` library. {}", path.display(), e),
    }
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
