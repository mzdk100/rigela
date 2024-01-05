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

use crate::context::Context;
use crate::gui::welcome::show_welcome;
use crate::terminator::{TerminationWaiter, Terminator};
use std::future::Future;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;
use win_wrap::browser::get_foreground_window;
use win_wrap::com::co_initialize_multi_thread;

pub struct Launcher {
    context: Arc<Context>,
    waiter: Option<Box<TerminationWaiter>>,
}

impl Launcher {
    /**
     * 创建一个发射台，通常一个进程只有一个实例。
     * */
    pub(crate) fn new() -> Self {
        // 创建一个终结者对象，main方法将使用他异步等待程序退出
        let (terminator, waiter) = Terminator::new();
        let ctx = Context::new(terminator);
        let ctx_ref: Arc<Context> = ctx.into();
        ctx_ref.apply();
        Self {
            context: ctx_ref,
            waiter: Some(waiter.into()),
        }
    }

    /**
     * 发射操作，这会启动整个框架，异步方式运行，直到程序结束。
     * */
    pub(crate) fn launch(&mut self) -> impl Future + '_ {
        // 初始化COM线程模型。
        co_initialize_multi_thread().expect("Can't initialize the com environment.");
        async {
            // peeper 可以监控远进程中的信息
            peeper::mount();
            let ctx = self.context.clone();
            let ctx2 = self.context.clone();

            let ctx3 = self.context.clone();
            let ctx4 = Arc::new(Mutex::new(self.context.clone()));

            // 显示欢迎页面。
            thread::spawn(|| show_welcome(ctx2));
            let performer = ctx.performer.clone();
            let main_handler = ctx.main_handler.clone();
            // 朗读当前桌面
            performer.speak(&ctx.ui_automation.get_root_element()).await;
            sleep(Duration::from_millis(1000)).await;
            // 订阅UIA的焦点元素改变事件
            ctx.ui_automation.add_focus_changed_listener(move |x| {
                let performer = performer.clone();
                main_handler.spawn(async move { performer.speak(&x).await });

                let hwnd = get_foreground_window();
                if hwnd != ctx3.form_browser.get_hwnd() {
                    let ctx = ctx4.lock().unwrap();
                    let ctx = *ctx.clone();
                    ctx.form_browser.set_hwnd(hwnd);
                    // 在这里接着往form_browser里面添加元素
                }
            });
            // 等待程序退出的信号
            self.waiter.as_deref_mut().unwrap().wait().await;
            self.context.dispose();
            // 解除远进程监控
            peeper::unmount();
        }
    }
}
