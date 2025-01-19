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

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "dll")]
mod dll;
#[cfg(feature = "handler")]
mod handler;
#[cfg(feature = "model")]
pub mod model;
#[cfg(feature = "server")]
pub mod server;
#[cfg(feature = "utils")]
mod utils;

#[cfg(not(feature = "dll"))]
use log::{debug, error};
#[cfg(not(feature = "dll"))]
use rigela_utils::fs::get_rigela_program_directory;
use std::sync::OnceLock;
#[cfg(not(feature = "dll"))]
use std::{
    thread::{self, sleep},
    time::Duration,
};
#[cfg(not(feature = "dll"))]
use win_wrap::{
    common::{
        get_proc_address, load_library, set_windows_hook_ex, unhook_windows_hook_ex, LPARAM, WPARAM,
    },
    ext::FarProcExt,
    hook::{HOOK_TYPE_CALL_WND_PROC, HOOK_TYPE_GET_MESSAGE},
    message::{
        message_loop, register_window_message, send_message_timeout, HWND_BROADCAST,
        SMTO_ABORTIFHUNG, SMTO_BLOCK,
    },
    threading::{get_current_thread_id, ThreadNotify},
};

// 此字段保存钩子的线程，在主进程中有效，所有远进程都不会被初始化
#[cfg(not(feature = "dll"))]
static HOOK_THREAD: OnceLock<ThreadNotify> = OnceLock::new();

// 此字段保存一个自定义的窗口消息值并在所有进程中都需要使用，用于在主进程中通知所有远进程钩子需要初始化，这能确保所有远进程收到通知并处理后主进程才能进行下一步操作
static HOOK_INIT: OnceLock<u32> = OnceLock::new();

// 此字段保存一个自定义的窗口消息值并在所有进程中都需要使用，用于在主进程中通知所有远进程钩子将要被卸载，这能确保所有远进程收到通知并处理后主进程才能进行下一步操作
static HOOK_UNINIT: OnceLock<u32> = OnceLock::new();

/**
启动亏叹气，这会把当前模块作为dll注入到远进程中，这是通过set_windows_hook机制实现的。
为什么选择使用windows hook的方法注入呢？这是因为很多安全防护软件会监控读屏的行为，如果使用create_remote_thread的方法，很容易被拦截，而windows hook机制是通过系统这一个媒介来完成dll注入，防护软件一般无能为力。
注意： 当main引用本模块并构建时，会自动生成此dll。
*/
#[cfg(not(feature = "dll"))]
pub fn mount() {
    debug!("mounted.");
    thread::spawn(move || {
        #[cfg(target_arch = "x86_64")]
        let dll_path = get_rigela_program_directory().join(format!("libs/{}.dll", module_path!()));
        #[cfg(target_arch = "x86")]
        let dll_path =
            get_rigela_program_directory().join(format!("libs/{}32.dll", module_path!()));

        debug!("Module path: {}", dll_path.display());
        let handle = match load_library(dll_path.to_str().unwrap()) {
            Ok(h) => h,
            Err(e) => {
                error!("{}", e);
                return;
            }
        };
        debug!("Module handle: {:?}", handle.0);

        // 安装消息队列钩子
        let h_hook_get_message = loop {
            let proc = get_proc_address(handle, "hook_proc_get_message");
            if let Ok(h) = set_windows_hook_ex(
                HOOK_TYPE_GET_MESSAGE,
                proc.to_hook_proc(),
                Some(handle.into()),
                0,
            ) {
                break h;
            }
            error!("Can't set the `get message` hook.");
            sleep(Duration::from_millis(1000));
        };
        debug!(
            "The hook of get message is ok, and it is {:?}.",
            h_hook_get_message.0
        );

        // 安装窗口过程钩子
        let h_hook_call_wnd_proc = loop {
            let proc = get_proc_address(handle, "hook_proc_call_wnd_proc");
            if let Ok(h) = set_windows_hook_ex(
                HOOK_TYPE_CALL_WND_PROC,
                proc.to_hook_proc(),
                Some(handle.into()),
                0,
            ) {
                break h;
            }
            error!("Can't set the `call wnd proc` hook.");
            sleep(Duration::from_millis(1000));
        };
        debug!(
            "The hook of call wnd proc is ok, and it is {:?}.",
            h_hook_call_wnd_proc.0
        );

        // 通知所有进程需要初始化
        send_message_timeout(
            HWND_BROADCAST,
            wm!(HOOK_INIT),
            WPARAM::default(),
            LPARAM::default(),
            SMTO_BLOCK | SMTO_ABORTIFHUNG,
            1000,
        );

        let notify = ThreadNotify::new(get_current_thread_id());
        if let Ok(_) = HOOK_THREAD.set(notify.clone()) {
            debug!("The thread of the hook is ready.");
        }
        message_loop(|_| ());

        // 在卸载钩子之前，我们必须先通知所有的远进程即将卸载钩子，让他们有机会清理资源，否则将可能引起系统不稳定
        send_message_timeout(
            HWND_BROADCAST,
            wm!(HOOK_UNINIT),
            WPARAM::default(),
            LPARAM::default(),
            SMTO_BLOCK | SMTO_ABORTIFHUNG,
            1000,
        );
        // 因为send_message把消息发送到窗口过程函数中，所以他是同步运行，当他返回（也就是运行到这里）时，理论上所有的远进程都已经清理资源完毕（除非有一些意外或超时），这时候无论如何都应该卸载钩子了
        if let Err(e) = unhook_windows_hook_ex(h_hook_get_message) {
            error!("Can't unhook, because {}.", e);
        }
        if let Err(e) = unhook_windows_hook_ex(h_hook_call_wnd_proc) {
            error!("Can't unhook, because {}.", e);
        }
        notify.finish();
    });
}

/** 停止亏叹气。 */
#[cfg(not(feature = "dll"))]
pub fn unmount() {
    match HOOK_THREAD.get() {
        None => {
            debug!("Exiting.");
        }
        Some(x) => {
            x.quit();
            debug!("Waiting the thread of the hook exit.");
            x.join(5000);
        }
    }
    debug!("unmounted.")
}
