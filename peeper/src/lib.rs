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

mod client;
mod model;
mod utils;

use log::{debug, error, info};
use rigela_utils::get_program_directory;
use std::{
    path::Path,
    ffi::c_void,
    thread,
    sync::RwLock
};
use tokio::runtime::Handle;
use rigela_utils::logger::init_logger;
use win_wrap::{
    common::{
        call_next_hook_ex,
        get_proc_address,
        load_library,
        set_windows_hook_ex,
        unhook_windows_hook_ex,
        BOOL,
        DLL_PROCESS_ATTACH,
        DLL_PROCESS_DETACH,
        DLL_THREAD_ATTACH,
        DLL_THREAD_DETACH,
        FALSE,
        HMODULE,
        LPARAM,
        LRESULT,
        TRUE,
        WPARAM,
        get_module_file_name
    },
    ext::{
        FarProcExt,
        LParamExt
    },
    hook::{CwpStruct, HOOK_TYPE_CALL_WND_PROC, HOOK_TYPE_GET_MESSAGE},
    message::{message_loop, register_window_message, send_message, HWND_BROADCAST},
    threading::{get_current_thread_id, ThreadNotify},
};
use crate::client::PeeperClient;

// 此字段保存钩子的线程，在主进程中有效，所有远进程都为None
static HOOK_THREAD: RwLock<Option<ThreadNotify>> = RwLock::new(None);

// 此字段保存一个自定义的窗口消息值并在所有进程中都需要使用，用于在主进程中通知所有远进程钩子需要初始化，这能确保所有远进程收到通知并处理后主进程才能进行下一步操作
static WM_HOOK_INIT: RwLock<u32> = RwLock::new(0);

// 此字段保存一个自定义的窗口消息值并在所有进程中都需要使用，用于在主进程中通知所有远进程钩子将要被卸载，这能确保所有远进程收到通知并处理后主进程才能进行下一步操作
static WM_HOOK_UNINIT: RwLock<u32> = RwLock::new(0);

// peeper的client实例。
static CLIENT: RwLock<Option<PeeperClient>> = RwLock::new(None);

macro_rules! wm {
    ($field:ident) => {{
        let mut message = $field.write().unwrap();
        if *message < 1 {
            *message = register_window_message(format!("{}_{}", module_path!(), stringify!($field)).as_str());
        }
        *message
    }};
}

/**
 * 窗口过程钩子，在此钩子中可以处理来自send_message的消息。
 * `code` 钩子代码，用于call_next_hook_ex函数。
 * `w_param` 钩子参数，取决于钩子类型。
 * `l_param` 钩子参数，取决于钩子类型。
 * */
#[no_mangle]
unsafe extern "system" fn hook_proc_call_wnd_proc(
    code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    let msg: &CwpStruct = l_param.to();
    if msg.message == wm!(WM_HOOK_INIT) {
        // 主进程发来的初始化命令
        let module = get_module_file_name(HMODULE::default());
        if let Some(n) = Path::new(&module).file_name() {
            if let Some(n) = n.to_str() {
                init_logger(Some(format!("{}.log", n).as_str()));
            }
        }
        info!("Injected into {}.", module);
        let mut client = CLIENT.write().unwrap();
        if client.is_none() {
            *client = Some(Handle::current().block_on(PeeperClient::new()));
            info!("Hooked.");
        }
    } else if msg.message == wm!(WM_HOOK_UNINIT) {
        // 主进程发来的卸载命令
        let mut client = CLIENT.write().unwrap();
        if !client.is_none() {
            client.as_ref().unwrap().quit();
            *client = None;
            info!("Unhooked.");
        }
    }
    call_next_hook_ex(code, w_param, l_param)
}

/**
 * 消息队列钩子，在此钩子中可以处理来自get_message/post_message/peek_message的消息。
 * `code` 钩子代码，用于call_next_hook_ex函数。
 * `w_param` 钩子参数，取决于钩子类型。
 * `l_param` 钩子参数，取决于钩子类型。
 * */
#[no_mangle]
unsafe extern "system" fn hook_proc_get_message(
    code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    call_next_hook_ex(code, w_param, l_param)
}

/**
 * 启动亏叹气，这会把当前模块作为dll注入到远进程中，这是通过set_windows_hook机制实现的。
 * 为什么选择使用windows hook的方法注入呢？这是因为很多安全防护软件会监控读屏的行为，如果使用create_remote_thread的方法，很容易被拦截，而windows hook机制是通过系统这一个媒介来完成dll注入，防护软件一般无能为力。
 * 注意： 当main引用本模块并构建时，会自动生成此dll。
 * */
pub fn mount() {
    debug!("mounted");
    thread::spawn(|| {
        let dll_path = get_program_directory().join(format!("{}.dll", module_path!()));
        debug!("Module path: {}", dll_path.display());
        let handle = match load_library(dll_path.to_str().unwrap()) {
            Ok(h) => h,
            Err(e) => {
                error!("{}", e);
                return;
            }
        };
        debug!("Module handle: {}", handle.0);

        // 动态获取dll中钩子函数的地址
        let proc_get_message = get_proc_address(handle, "hook_proc_get_message");
        let proc_call_wnd_proc = get_proc_address(handle, "hook_proc_call_wnd_proc");

        // 安装消息队列钩子
        let h_hook_get_message = set_windows_hook_ex(
            HOOK_TYPE_GET_MESSAGE,
            proc_get_message.to_hook_proc(),
            handle.into(),
            0,
        );
        // 安装窗口过程钩子
        let h_hook_call_wnd_proc = set_windows_hook_ex(
            HOOK_TYPE_CALL_WND_PROC,
            proc_call_wnd_proc.to_hook_proc(),
            handle.into(),
            0,
        );

        // 通知所有进程需要初始化
        send_message(
            HWND_BROADCAST,
            wm!(WM_HOOK_INIT),
            WPARAM::default(),
            LPARAM::default(),
        );

        let notify = ThreadNotify::new(get_current_thread_id());
        let mut lock = HOOK_THREAD.write().unwrap();
        *lock = Some(notify.clone());
        drop(lock);
        message_loop();

        // 在卸载钩子之前，我们必须先通知所有的远进程即将卸载钩子，让他们有机会清理资源，否则将可能引起系统不稳定
        send_message(
            HWND_BROADCAST,
            wm!(WM_HOOK_UNINIT),
            WPARAM::default(),
            LPARAM::default(),
        );
        // 因为send_message把消息发送到窗口过程函数中，所以他是同步运行，当他返回（也就是运行到这里）时，理论上所有的远进程都已经清理资源完毕（除非有一些意外），这时候无论如何都应该卸载钩子了
        unhook_windows_hook_ex(h_hook_get_message);
        unhook_windows_hook_ex(h_hook_call_wnd_proc);
        notify.finish();
    });
}

/** 停止亏叹气。 */
pub fn unmount() {
    let mut lock = HOOK_THREAD.write().unwrap();
    match lock.as_ref() {
        None => {}
        Some(x) => {
            x.quit();
            x.join(5000);
        }
    }
    *lock = None;
    drop(lock);
    debug!("unmounted")
}

#[no_mangle]
extern "system" fn DllMain(_: HMODULE, dw_reason: u32, _lp_reserved: *const c_void) -> BOOL {
    match dw_reason {
        DLL_PROCESS_ATTACH => {}
        DLL_PROCESS_DETACH => {}
        DLL_THREAD_ATTACH => {}
        DLL_THREAD_DETACH => {}
        _ => return FALSE,
    }
    TRUE
}
