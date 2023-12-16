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

mod safety;
mod tts;

use safety::{com::*, uia::*};
use std::time::Duration;
use tokio::{time};
use windows::core::Result;
use crate::tts::Tts;


#[tokio::main]
async fn main() -> Result<()> {
    // 用于测试DLL
    println!("Hello, {}!", peeper::add(3, 4));
    // 获取主线程携程处理器
    let main_handler = tokio::runtime::Handle::current();
    // 初始化COM线程模型。
    co_initialize_multi_thread()?;
    // 创建tts
    let tts = Tts::new();
    // 获取Automation
    let automation = UiAutomation::new();
    // 获取UI根元素
    let root_element = automation.get_root_element();
    // 朗读当前桌面
    tts.speak(root_element.get_name().as_str()).await?;
    // 订阅UIA的焦点元素改变事件
    automation.add_focus_changed_listener(move |x| {
        let tts2 = tts.clone();
        main_handler.spawn(async move {
            tts2.speak(x.get_name().as_str()).await
        });
    });
    // 无限循环
    loop {
        // 需要使用携程框架中的sleep函数，不可以使用线程级别的sleep，否则主线程无法处理任何携程任务
        time::sleep(Duration::from_millis(1000)).await;
    }
}
