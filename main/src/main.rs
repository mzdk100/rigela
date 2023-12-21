/*
 * Copyright (c) 2023. The RigelA open source project team and
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

mod commander;
mod launcher;
mod performer;
mod terminator;

use launcher::Launcher;
use win_wrap::com::co_initialize_multi_thread;

#[tokio::main]
async fn main() {
    // 初始化COM线程模型。
    co_initialize_multi_thread()
        .expect("Can't initialize the com environment.");
    // peeper 可以监控远进程中的信息
    peeper::mount();
    // 使用发射台启动主程序
    let mut launcher = Launcher::new();
    launcher.launch().await;
    // 解除远进程监控
    peeper::unmount();
}
