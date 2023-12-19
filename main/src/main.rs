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
mod performer;
use performer::{Performer, Speakable};
use win_wrap::{common::*, com::*, hook::*, uia::*};
use std::time::Duration;
use tokio::{time};

impl Speakable for UiAutomationElement {
    fn get_sentence(&self) -> String {
        format!("{}: {}", self.get_name(), self.get_localized_control_type())
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    // peeper 可以监控远进程中的信息
    peeper::mount();
    // 安装键盘钩子
    let keyboard_hook = WindowsHook::new(HOOK_TYPE_KEYBOARD_LL, |w_param, l_param, next| {
        let info: &KbdLlHookStruct = l_param.to();
        println!("{}", info.vkCode);
        next()
    });
    // 获取主线程携程处理器
    let main_handler = tokio::runtime::Handle::current();
    // 初始化COM线程模型。
    co_initialize_multi_thread()?;
    // 创建表演者对象
    let performer = Performer::new();
    // 获取Automation
    let automation = UiAutomation::new();
    // 朗读当前桌面
    performer.speak(&automation.get_root_element()).await?;
    // 订阅UIA的焦点元素改变事件
    automation.add_focus_changed_listener(move |x| {
        let performer2 = performer.clone();
        main_handler.spawn(async move {
            performer2.speak(&x).await
        });
    });
    // 无限循环
    let mut is_needed_quit = false;
    while !is_needed_quit {
        // 需要使用携程框架中的sleep函数，不可以使用线程级别的sleep，否则主线程无法处理任何携程任务
        time::sleep(Duration::from_millis(1000)).await;
    }
    // 解除键盘钩子
    keyboard_hook.unhook();
    // 解除远进程监控
    peeper::unmount();
    Ok(())
}
