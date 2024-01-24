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

use crate::gui::welcome::show_form;
use crate::{
    context::Context,
    terminator::{TerminationWaiter, Terminator},
};
use log::error;
use rigela_utils::{get_program_directory, write_file};
use std::{sync::Arc, thread, time::Duration};
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
        // 初始化COM线程模型。
        co_initialize_multi_thread().expect("Can't initialize the com environment.");

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
        // peeper 可以监控远进程中的信息
        put_peeper().await;
        peeper::mount();
        let peeper_server = self.context.peeper_server.clone();
        self.context.work_runtime.spawn(async move {
            peeper_server.run().await;
        });

        // 显示欢迎页面。
        thread::spawn(|| show_form());

        // 加载32位的主程序代理模块（为了启动速度，此模块可以延迟加载）
        let proxy32 = self.context.proxy32.clone();
        self.context.work_runtime.spawn(async move {
            proxy32.spawn().await;
        });

        // 朗读当前桌面
        speak_desktop(self.context.clone()).await;

        // 启动事件监听
        self.context.event_core.run(self.context.clone()).await;

        // 等待程序退出的信号
        self.waiter.as_deref_mut().unwrap().wait().await;
        self.context.dispose();

        // 杀死32位代理模块
        self.context.proxy32.kill().await.wait().await;

        // 解除远进程监控
        peeper::unmount();
    }
}

/// 朗读桌面
async fn speak_desktop(context: Arc<Context>) {
    let root = context.ui_automation.get_root_element();
    context.performer.speak(&root).await;

    sleep(Duration::from_millis(1000)).await;
}

/**
 * 安装peeper.dll文件。
 * */
async fn put_peeper() {
    // 获取peeper.dll的二进制数据并写入到用户目录中，原理是在编译时把peeper.dll的数据使用include_bytes!内嵌到主程序内部，在运行时释放到磁盘。
    // 注意：这里使用条件编译的方法，确保include_bytes!仅出现一次，不能使用if语句，那样会多次包含bytes，main.exe的大小会成倍增长。
    #[cfg(not(debug_assertions))]
    let peeper_dll = include_bytes!("../../target/x86_64-pc-windows-msvc/release/peeper.dll");
    #[cfg(debug_assertions)]
    let peeper_dll = include_bytes!("../../target/x86_64-pc-windows-msvc/debug/peeper.dll");
    let peeper_path = get_program_directory().join("peeper.dll");
    if let Err(e) = write_file(&peeper_path, peeper_dll).await {
        error!("{}", e);
    };
}
