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

use crate::browser::FORM_BROWSER;
use crate::context::Context;
use crate::gui::welcome::show_welcome;
use crate::terminator::{TerminationWaiter, Terminator};
use std::{future::Future, sync::Arc, thread, time::Duration};
use tokio::time::sleep;
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

            // 显示欢迎页面。
            let ctx2 = self.context.clone();
            thread::spawn(|| show_welcome(ctx2));

            let performer = ctx.performer.clone();
            let main_handler = ctx.main_handler.clone();

            // 朗读当前桌面
            performer.speak(&ctx.ui_automation.get_root_element()).await;
            sleep(Duration::from_millis(1000)).await;

            let ctx3 = Arc::clone(&self.context);

            // 订阅UIA的焦点元素改变事件
            ctx.ui_automation.add_focus_changed_listener(move |x| {
                let performer1 = Arc::clone(&performer);
                let handle1 = Arc::clone(&main_handler);
                // let performer2 = Arc::clone(&performer);
                // let handle2 = Arc::clone(&main_handler);

                handle1.spawn(async move { performer1.speak(&x).await });

                let mut fb = FORM_BROWSER.lock().expect("Can't lock the form browser.");
                if !fb.is_foreground_window_changed() {
                    return;
                }
                fb.update_hwnd();

                let elements = ctx3.ui_automation.get_foreground_window_elements();

                for ele in elements {
                    fb.add(Box::new(ele));
                }

                // 测试是否监测到前台窗口更新，测试使用，待删除。
                // handle2.spawn(async move {
                //     performer2.speak_text("hello test").await;
                // });
            });
            // 等待程序退出的信号
            self.waiter.as_deref_mut().unwrap().wait().await;
            self.context.dispose();
            // 解除远进程监控
            peeper::unmount();
        }
    }
}
