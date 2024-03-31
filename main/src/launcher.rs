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
    context::{Context, ContextAccessor},
    ext::window::AccessibleWindowExt,
    performer::sound::SoundArgument::Single,
    talent::Talented,
    terminator::Terminator,
};
use a11y::{get_ia2_lib_path, setup};
use log::{error, info};
use rigela_utils::{killer::wait_until_killed, library::setup_library};
use std::sync::{Arc, Weak};
use tokio::{process::Command, runtime::Runtime};
use win_wrap::{com::co_initialize_multi_thread, msaa::object::AccessibleObject};

/// 发射台对象
pub(crate) struct Launcher {
    context: Arc<Context>,
}

impl Launcher {
    /**
     * 创建一个发射台，通常一个进程只有一个实例。
     * */
    pub(crate) fn new(work_runtime: Weak<Runtime>, terminator: Weak<Terminator>) -> Self {
        // 初始化COM线程模型。
        let res = co_initialize_multi_thread();
        if res.is_err() {
            error!("Can't initialize the com environment. {}", res.message());
        }
        // 安装a11y的运行时
        setup();

        // 上下文对象的创建，需要传入终结器，上下文对象通过终结器对象响应终结消息
        let context = Context::new(
            unsafe { &*work_runtime.as_ptr() },
            terminator.upgrade().unwrap(),
        );
        let context = Arc::new(context);
        // 调用上下文对象的应用到每一个组件的方法
        context.apply();

        Self { context }
    }

    //noinspection RsUnresolvedPath
    /**
     * 发射操作，这会启动整个框架，异步方式运行，直到程序结束。
     * */
    pub(crate) async fn launch(&self) {
        // 监听外部进程请求主程序退出，这是一种安全杀死主进程的方案
        let ctx = Arc::downgrade(&self.context);
        self.context.get_work_runtime().spawn(async move {
            wait_until_killed().await;
            ctx.get_talent_provider()
                .get_exit_talent()
                .perform(ctx)
                .await;
        });

        // 播放启动时的音效
        let ctx = self.context.clone();
        self.context.get_work_runtime().spawn(async move {
            ctx.get_performer().play_sound(Single("launch.wav")).await;
        });

        // 注册com组件库
        self.context.get_work_runtime().spawn(async move {
            register_service((&get_ia2_lib_path()).to_str().unwrap()).await;
        });

        // peeper 可以监控远进程中的信息
        put_peeper();
        peeper::mount();
        let ctx = self.context.clone();
        self.context.get_work_runtime().spawn(async move {
            ctx.get_peeper_server().run().await;
        });

        #[cfg(target_arch = "x86_64")]
        {
            // 加载32位的主程序代理模块（为了启动速度，此模块可以延迟加载）
            let ctx = self.context.clone();
            self.context.get_work_runtime().spawn(async move {
                ctx.get_proxy32process().spawn().await;
            });
        }

        // 朗读当前桌面
        self.context
            .get_performer()
            .speak(&self.context.get_ui_automation().get_root_element())
            .await;

        // 朗读当前前景窗口
        if let Ok(o) = AccessibleObject::from_foreground_window() {
            self.context.get_performer().speak(&(o, 0)).await;
        }

        // 启动事件监听
        self.context
            .get_event_core()
            .run(Arc::downgrade(&self.context))
            .await;

        // 更新自定义热键, 这个调用放在apply里面不会生效
        self.context
            .get_talent_provider()
            .update_custom_combo_key_map(Arc::downgrade(&self.context));
    }

    //noinspection RsUnresolvedPath
    /**
     * 退出程序。
     * */
    pub(crate) async fn exit(&self) {
        // 杀死32位代理模块
        #[cfg(target_arch = "x86_64")]
        self.context.get_proxy32process().kill().await.wait().await;

        // 播放退出音效
        self.context
            .get_performer()
            .play_sound(Single("exit.wav"))
            .await;

        // 清理上下文
        self.context.dispose();

        // 解除远进程监控
        peeper::unmount();
    }
}

async fn register_service(path: &str) {
    match Command::new("regsvr32").arg("/s").arg(path).spawn() {
        Ok(mut p) => match p.wait().await {
            Ok(_) => info!("Register {} is successfully.", path),
            Err(e) => error!("Can't register the dll server ({}). {}", path, e),
        },
        Err(e) => error!("Can't register the dll server ({}). {}", path, e),
    }
}

/**
 * 安装peeper.dll文件。
 * */
fn put_peeper() {
    // 获取peeper.dll的二进制数据并写入到用户目录中，原理是在编译时把peeper.dll的数据使用include_bytes!内嵌到主程序内部，在运行时释放到磁盘。
    // 注意：这里使用条件编译的方法，确保include_bytes!仅出现一次，不能使用if语句，那样会多次包含bytes，main.exe的大小会成倍增长。
    #[cfg(not(debug_assertions))]
    let peeper_dll = include_bytes!("../../target/x86_64-pc-windows-msvc/release/peeper.dll");
    #[cfg(debug_assertions)]
    let peeper_dll = include_bytes!("../../target/x86_64-pc-windows-msvc/debug/peeper.dll");
    setup_library("peeper.dll", peeper_dll);
}
