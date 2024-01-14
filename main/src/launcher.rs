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
    context::Context,
    gui::FrameUi,
    terminator::{TerminationWaiter, Terminator},
};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use win_wrap::com::co_initialize_multi_thread;

/// 启动器对象
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

        // 上下文对象的创建，需要传入终结器，上下文对象通过终结器对象响应终结消息
        let ctx = Context::new(terminator);
        let ctx_ref: Arc<Context> = ctx.into();

        // 调用上下文对象的应用到每一个组件的方法
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
        speak_desktop(Arc::clone(&self.context)).await;

        // 启动事件监听
        self.context.event_core.run(self.context.clone()).await;

        // 等待程序退出的信号
        self.waiter.as_deref_mut().unwrap().wait().await;
        self.context.dispose();

        // 解除远进程监控
        peeper::unmount();
    }
}

/// 朗读桌面
async fn speak_desktop(context: Arc<Context>) {
    if let Ok(root) = context.ui_automation.get_root_element() {
        context.performer.speak(&root).await;
    }

    sleep(Duration::from_millis(1000)).await;
}
