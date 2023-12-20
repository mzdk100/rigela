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
mod terminator;

use performer::{Performer, Speakable};
use terminator::Terminator;
use win_wrap::{common::*, com::*, hook::*, uia::*};

impl Speakable for UiAutomationElement {
    fn get_sentence(&self) -> String {
        format!("{}: {}", self.get_name(), self.get_localized_control_type())
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    // 创建一个终结者对象，main方法将使用他异步等待程序退出
    let (terminator, mut waiter) = Terminator::new();
    // peeper 可以监控远进程中的信息
    peeper::mount();
    // 准备安装键盘钩子，并获取一个主线程的携程处理器，用于在钩子函数中调度任务到主线程
    let main_handler = tokio::runtime::Handle::current();
    let keyboard_hook = WindowsHook::new(HOOK_TYPE_KEYBOARD_LL, move |w_param, l_param, next| {
        let info: &KbdLlHookStruct = l_param.to();
        println!("{}", info.vkCode);
        if info.vkCode == 163 && info.flags.contains(LLKHF_EXTENDED) {
            let terminator = terminator.clone();
            main_handler.spawn(async move {
                terminator.exit().await;
            });
            return LRESULT::default()
        }
        next()
    });
    // 初始化COM线程模型。
    co_initialize_multi_thread()?;
    // 创建表演者对象
    let performer = Performer::new();
    // 获取Automation
    let automation = UiAutomation::new();
    // 朗读当前桌面
    performer.speak(&automation.get_root_element()).await?;
    // 订阅UIA的焦点元素改变事件，并获取主线程的携程处理器，用于在uia线程中调度任务到主线程
    let main_handler = tokio::runtime::Handle::current();
    automation.add_focus_changed_listener(move |x| {
        let performer2 = performer.clone();
        main_handler.spawn(async move {
            performer2.speak(&x).await
        });
    });
    // 等待程序退出的信号
    waiter.wait().await;
    // 解除键盘钩子
    keyboard_hook.unhook();
    // 解除远进程监控
    peeper::unmount();
    Ok(())
}
