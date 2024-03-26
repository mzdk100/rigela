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

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate rust_i18n;
// 初始化I18N国际化多语言支持
i18n!("locale");

/**
 * 入口程序。
 * 本读屏程序的框架的设计类似于舞台表演模式，大体结构如下：
 * 1. launcher 发射台，负责启动整个框架；
 * 2. commander 指挥官，负责从用户那里收集命令请求，例如键盘命令；
 * 3. configs 配置模块，可以读写toml格式的配置文件，并实现所有的配置文件的数据结构；
 * 4. ext 一些扩展操作的函数；
 * 5. event_core用于订阅和处理各种辅助功能事件；
 * 6. gui GUI图形界面模块，实现所有的UI交互逻辑；
 * 7. navigator UI界面元素导航模块，通过“线性”、“平面”和“树状”等模式在UI元素中访问和浏览界面信息，并可以针对元素进行操作；
 * 8. performer 表演者，负责把信息转换成用户可以感知的形式，例如语音；
 * 9. resources 资源模块，可以读写资源文件，内部实现了自动增量更新；
 * 10. talent 能力模块，定义所有能力，这些能力通常绑定到输入设备上，例如（键盘、鼠标、触摸屏等）；
 * 11. tasks 任务模块，针对一些异步的任务进行管理；
 * 12. terminator 终结者，用于控制和等待程序结束；
 * 13. context 上下文环境，可以贯穿整个框架的环境，让每一个模块之间可以互相访问。
 * */
mod commander;
mod configs;
mod context;
mod event_core;
mod ext;
mod gui;
mod impls;
mod launcher;
mod navigator;
mod performer;
mod resources;
mod talent;
mod tasks;
mod terminator;

use launcher::Launcher;
use log::info;
use rigela_utils::{killer::kill, logger::init_logger};

#[tokio::main]
async fn main() {
    // 通知其他的读屏进程退出，防止多开
    kill().await;

    // 初始化日志库
    init_logger(None);

    // 使用发射台启动主程序
    info!("Launching RigelA...");
    Launcher::new().launch().await;
}
