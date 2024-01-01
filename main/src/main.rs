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

/**
 * 入口程序。
 * 本读屏程序的框架的设计类似于舞台表演模式，大体结构如下：
 * 1. launcher 发射台，负责启动整个框架；
 * 2. commander 指挥官，负责从用户那里收集命令请求，例如键盘命令；
 * 3. performer 表演者，负责把信息转换成用户可以感知的形式，例如语音；
 * 4. peeper 窥探器，可以收集远进程中的信息，例如输入法和gdi绘图信息；
 * 5. resource_accessor 资源访问器，可以读写资源文件，内部实现了自动增量更新；
 * 6. talent 才能，一些功能的实现；
 * 7. terminator 终结者，用于控制和等待程序结束；
 * 8. context 上下文环境，可以贯穿整个框架的环境，让每一个模块之间可以互相访问。
 * */

mod commander;
mod context;
mod gui;
mod launcher;
mod performer;
mod resources;
mod talent;
mod terminator;
mod consts;

use launcher::Launcher;

#[tokio::main]
async fn main() {
    // 使用发射台启动主程序
    let mut launcher = Launcher::new();
    launcher.launch().await;
}
