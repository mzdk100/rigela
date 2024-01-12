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

use crate::{
    browser::uia,
    context::Context,
    gui::FrameUi,
    terminator::{TerminationWaiter, Terminator},
};
use std::sync::Arc;
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
    pub(crate) async fn launch(&mut self) {
        // 初始化COM线程模型。
        co_initialize_multi_thread().expect("Can't initialize the com environment.");

        // peeper 可以监控远进程中的信息
        peeper::mount();

        // 显示欢迎页面。
        self.context
            .gui_accessor
            .get_welcome_frame_ui()
            .show(self.context.clone());

        // 朗读当前桌面
        uia::speak_desktop(Arc::clone(&self.context)).await;

        // 订阅UIA的焦点元素改变事件
        uia::speak_focus_item(Arc::clone(&self.context)).await;

        // 监听前台窗口变动
        uia::watch_foreground_window(Arc::clone(&self.context)).await;

        // 等待程序退出的信号
        self.waiter.as_deref_mut().unwrap().wait().await;
        self.context.dispose();

        // 解除远进程监控
        peeper::unmount();
    }
}
