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

use std::{
    sync::Arc,
};
use crate::{
    context::Context,
    ext::AccessibleObjectExt,
    performer::sound::SoundArgument::Single,
    terminator::{TerminationWaiter, Terminator},
};
use log::{error, info};
use rigela_utils::{killer::{kill, listen_to_killing}, get_program_directory, SERVER_HOME_URI, write_file, get_file_modified_duration};
use tokio::process::Command;
use rigela_resources::clone_resource;
use win_wrap::{com::co_initialize_multi_thread, msaa::object::AccessibleObject};

/// 启动器对象
pub(crate) struct Launcher {
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
        // 通知其他的读屏进程退出，防止多开
        kill().await;

        // 监听外部进程请求主程序退出，这是一种安全杀死主进程的方案
        let terminator = self.context.terminator.clone();
        listen_to_killing(async move {
            terminator.exit().await;
        });

        // 播放启动时的音效
        let performer = self.context.performer.clone();
        self.context.work_runtime.spawn(async move {
            performer.play_sound(Single("launch.wav")).await;
        });

        // 注册一些com组件库
        self.context.work_runtime.spawn(async move {
            register_service("IAccessible2Proxy.dll").await;
        });

        // peeper 可以监控远进程中的信息
        put_peeper().await;
        peeper::mount();
        let peeper_server = self.context.peeper_server.clone();
        self.context.work_runtime.spawn(async move {
            peeper_server.run().await;
        });

        #[cfg(target_arch = "x86_64")]
        {
            // 加载32位的主程序代理模块（为了启动速度，此模块可以延迟加载）
            let proxy32 = self.context.proxy32.clone();
            self.context.work_runtime.spawn(async move {
                proxy32.spawn().await;
            });
        }

        // 初始化GUI窗口界面
        self.context.window_manager.init(self.context.clone());

        // 朗读当前桌面
        self.context
            .performer
            .speak(self.context.ui_automation.get_root_element())
            .await;

        // 朗读当前前景窗口
        if let Ok(o) = AccessibleObject::from_foreground_window() {
            self.context.performer.speak((o, 0)).await;
        }

        // 启动事件监听
        self.context.event_core.run(self.context.clone()).await;

        // 等待程序退出的信号
        self.waiter.as_deref_mut().unwrap().wait().await;
        self.context.dispose();

        // 杀死32位代理模块
        #[cfg(target_arch = "x86_64")]
        self.context.proxy32.kill().await.wait().await;

        // 解除远进程监控
        peeper::unmount();

        // 退出Gui界面
        self.context.window_manager.uninit();

        // 播放退出音效
        self.context.performer.play_sound(Single("exit.wav")).await;
    }
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
    let peeper_path = get_program_directory().join("libs/peeper.dll");
    if let Err(e) = write_file(&peeper_path, peeper_dll).await {
        error!("{}", e);
    };
}

/**
 * 注册类库。
 * `dll_name` 库名称。
 * */
async fn register_service(dll_name: &str) {
    let path = get_program_directory().join("libs").join(dll_name);
    if get_file_modified_duration(&path).await > 3600 * 6 {
        match clone_resource(format!("{}/{}", SERVER_HOME_URI, dll_name), &path).await {
            Ok(_) => {}
            Err(e) => {
                error!("Can't register {}. {}", dll_name,e);
                return;
            }
        }
    }

    match Command::new("regsvr32").arg("/s").arg(path).spawn() {
        Ok(mut p) => match p.wait().await {
            Ok(_) => info!("Register {} is successfully.", dll_name),
            Err(e) => error!("Can't register the dll server ({}). {}", dll_name,e)
        },
        Err(e) => error!("Can't register the dll server ({}). {}", dll_name,e)
    }
}