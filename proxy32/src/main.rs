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

#![windows_subsystem = "windows"]

mod model;
mod server;
mod tts;

#[cfg(target_arch = "x86")]
#[tokio::main]
async fn main() {
    use crate::server::Proxy32Server;
    use peeper;
    use rigela_utils::logger;
    use std::env;

    logger::init_logger(Some(format!("{}.log", module_path!()).as_str()));
    put_peeper32().await;
    peeper::mount();

    let mut server = Proxy32Server::new(env::args().nth(1).unwrap().as_str()).await;
    server.run().await;
    peeper::unmount();
}

#[cfg(not(target_arch = "x86"))]
fn main() {
    panic!("X86 arch target only!");
}

/**
 * 安装peeper32.dll文件。
 * */
#[cfg(target_arch = "x86")]
async fn put_peeper32() {
    use log::error;
    use rigela_utils::fs::{get_program_directory, write_file};

    // 获取peeper.dll的二进制数据并写入到用户目录中，原理是在编译时把peeper.dll的数据使用include_bytes!内嵌到主程序内部，在运行时释放到磁盘。
    // 注意：这里使用条件编译的方法，确保include_bytes!仅出现一次，不能使用if语句，那样会多次包含bytes，proxy32.exe的大小会成倍增长。
    #[cfg(not(debug_assertions))]
        let peeper_dll = include_bytes!("../../target/i686-pc-windows-msvc/release/peeper.dll");
    #[cfg(debug_assertions)]
        let peeper_dll = include_bytes!("../../target/i686-pc-windows-msvc/debug/peeper.dll");
    let peeper_path = get_program_directory().join("libs/peeper32.dll");
    if let Err(e) = write_file(&peeper_path, peeper_dll).await {
        error!("{}", e);
    };
}
