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
use std::future::Future;
use std::sync::Arc;
use tokio::runtime::Handle;
use crate::performer::{Performer, Speakable};
use crate::terminator::{TerminationWaiter, Terminator};
use win_wrap::{uia::*};
use crate::commander::Commander;

impl Speakable for UiAutomationElement {
    fn get_sentence(&self) -> String {
        format!("{}: {}", self.get_name(), self.get_localized_control_type())
    }
}

pub struct Launcher {
    commander: Arc<Commander>,
    pub(crate) main_handler: Arc<Handle>,
    performer: Arc<Performer>,
    pub(crate) terminator: Arc<Terminator>,
    ui_automation: Arc<UiAutomation>,
    waiter: Option<Box<TerminationWaiter>>
}
impl Clone for Launcher {
    fn clone(&self) -> Self {
        Self{
            commander: self.commander.clone(),
            main_handler: self.main_handler.clone(),
            performer: self.performer.clone(),
            terminator: self.terminator.clone(),
            ui_automation: self.ui_automation.clone(),
            // 只有主线程可以独有等待程序结束的权限，不可以克隆
            waiter: None
        }
    }
}

impl Launcher {
    /**
     * 创建一个发射台，通常一个进程只有一个实例。
     * */
    pub(crate) fn new() -> Self {
        // 创建一个终结者对象，main方法将使用他异步等待程序退出
        let (terminator, waiter) = Terminator::new();
        // 创建一个指挥官，用于下发操作命令
        let mut commander =Commander::new();
        // 创建表演者对象（用于把各种信息转换成用户可以感知的形式，例如语音、音效等）
        let performer = Performer::new();
        // 获取一个主线程携程处理器，可以在子线程中调度任务到主线程
        let main_handler = Handle::current();
        // 创建UiAutomation
        let ui_automation = UiAutomation::new();
        let launcher = Self {
            commander: commander.clone().into(),
            main_handler: main_handler.into(),
            performer: performer.into(),
            terminator: terminator.into(),
            ui_automation: ui_automation.into(),
            waiter: Some(waiter.into())
        };
        let launcher_ref = Arc::new(launcher.clone());
        commander.apply(launcher_ref);
        launcher
    }

    /**
     * 发射操作，这会启动整个框架，异步方式运行，直到程序结束。
     * */
    pub(crate) fn launch(&mut self) -> impl Future + '_ {
        async {
            let performer = self.performer.clone();
            let main_handler = self.main_handler.clone();
            // 朗读当前桌面
            performer.speak(&self.ui_automation.get_root_element()).await;
            // 订阅UIA的焦点元素改变事件
            self.ui_automation.add_focus_changed_listener(move |x| {
                let performer = performer.clone();
                main_handler.spawn(async move {
                    performer.speak(&x).await
                });
            });
            // 等待程序退出的信号
            self.waiter
                .as_deref_mut()
                .unwrap()
                .wait()
                .await;
            self.commander.dispose();
        }
    }
}