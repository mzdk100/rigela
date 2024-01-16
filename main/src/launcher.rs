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
use std::{sync::Arc, time::Duration};
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

        // 加载32位的主程序代理模块（为了启动速度，此模块可以延迟加载）
        self.context.clone().main_handler.spawn(load_proxy32());

        // 朗读当前桌面
        speak_desktop(self.context.clone()).await;

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
    let root = context.ui_automation.get_root_element();
    context.performer.speak(&root).await;

    sleep(Duration::from_millis(1000)).await;
}

#[cfg(target_arch = "x86_64")]
async fn load_proxy32() {
    use crate::utils::get_program_directory;
    use tokio::{fs::OpenOptions, io::AsyncWriteExt, process::Command};

    // 获取proxy32.exe的二进制数据并写入到用户目录中，原理是在编译时把proxy32的数据使用include_bytes!内嵌到64位的主程序内部，在运行时释放到磁盘。
    // 注意：这里使用条件编译的方法，确保include_bytes!仅出现一次，不能使用if语句，那样会多次包含bytes，main.exe的大小会成倍增长。
    #[cfg(not(debug_assertions))]
    let proxy32_bin = include_bytes!("../../target/i686-pc-windows-msvc/release/proxy32.exe");
    #[cfg(debug_assertions)]
    let proxy32_bin = include_bytes!("../../target/i686-pc-windows-msvc/debug/proxy32.exe");
    let proxy32_path = get_program_directory().join("proxy32.exe");
    {
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(&proxy32_path)
            .await
            .unwrap()
            .write(proxy32_bin)
            .await
            .unwrap();
    }

    // 启动32位的代理模块。
    let mut cmd = Command::new(&proxy32_path).spawn();
    while cmd.is_err() {
        // 因为proxy32.exe刚刚释放到磁盘，很可能被微软杀毒锁定，这时候启动会失败（另一个程序正在使用此文件，进程无法访问。）
        sleep(Duration::from_millis(5000)).await;
        // 5秒之后重新尝试启动
        cmd = Command::new(&proxy32_path).spawn();
    }

    // 等待到程序结束
    cmd.unwrap()
        .wait()
        .await
        .unwrap();
}

#[cfg(target_arch = "x86")]
async fn load_proxy32() {
    // 如果主程序本身就是32位，则无需执行此操作（proxy32模块没有用武之地）
    use log::info;
    info!("Loaded proxy32.");
}
