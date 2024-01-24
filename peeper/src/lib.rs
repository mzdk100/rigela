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
mod handler;
mod model;
pub mod server;
mod utils;

use log::{debug, error};
use rigela_utils::get_program_directory;
use std::{
    ffi::c_void,
    thread
};
use std::sync::{OnceLock, RwLock};
use std::thread::sleep;
use std::time::Duration;
use once_cell::sync::Lazy;
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
    input::WM_CHAR,
    message::{message_loop, register_window_message, send_message, HWND_BROADCAST, MSG},
    threading::{get_current_thread_id, ThreadNotify},
};
use crate::{
    client::PeeperClient,
    handler::input_char
};

macro_rules! wm {
    ($field:ident) => {
        register_window_message(format!("{}_{}", module_path!(), stringify!($field)).as_str())
    }
}

// 此字段保存钩子的线程，在主进程中有效，所有远进程都不会被初始化
static mut HOOK_THREAD: OnceLock<ThreadNotify> = OnceLock::new();

// 此字段保存一个自定义的窗口消息值并在所有进程中都需要使用，用于在主进程中通知所有远进程钩子需要初始化，这能确保所有远进程收到通知并处理后主进程才能进行下一步操作
static WM_HOOK_INIT: Lazy<u32> = Lazy::new(|| wm!(HOOK_INIT));

// 此字段保存一个自定义的窗口消息值并在所有进程中都需要使用，用于在主进程中通知所有远进程钩子将要被卸载，这能确保所有远进程收到通知并处理后主进程才能进行下一步操作
static WM_HOOK_UNINIT: Lazy<u32> = Lazy::new(|| wm!(HOOK_UNINIT));

// peeper的client实例。
static CLIENT: RwLock<Option<PeeperClient>> = RwLock::new(None);

fn activate_transaction() {
    let mut lock = CLIENT.write().unwrap();
    lock.get_or_insert({
        let module = get_module_file_name(HMODULE::default());
        debug!("Injected into {}.", module.as_str());
        let client = PeeperClient::new(module);
        debug!("Hooked.");
        client
    });
    drop(lock);
}

fn deactivate_transaction() {
    let mut lock = CLIENT.write().unwrap();
    if let Some(c) = lock.as_mut() {
        c.quit();
        debug!("Unhooked.");
    }
    *lock = None;
    drop(lock);
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
    if code < 0 {
        return call_next_hook_ex(code, w_param, l_param);
    }
    let msg: &CwpStruct = l_param.to();
    if WM_HOOK_INIT.clone() == msg.message {
        // 主进程发来的初始化命令
        activate_transaction();
    } else if WM_HOOK_UNINIT.clone() == msg.message {
        // 主进程发来的卸载命令
        deactivate_transaction();
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
    if code < 0 {
        return call_next_hook_ex(code, w_param, l_param);
    }
    let msg: &MSG = l_param.to();
    match msg.message {
        WM_CHAR => {
            activate_transaction();
            let lock = CLIENT.read().unwrap();
            input_char(lock.as_ref().unwrap(), msg);
            drop(lock);
        }
        _ => {}
    }

    call_next_hook_ex(code, w_param, l_param)
}

/**
 * 启动亏叹气，这会把当前模块作为dll注入到远进程中，这是通过set_windows_hook机制实现的。
 * 为什么选择使用windows hook的方法注入呢？这是因为很多安全防护软件会监控读屏的行为，如果使用create_remote_thread的方法，很容易被拦截，而windows hook机制是通过系统这一个媒介来完成dll注入，防护软件一般无能为力。
 * 注意： 当main引用本模块并构建时，会自动生成此dll。
 * */
pub fn mount() {
    debug!("mounted.");
    thread::spawn(|| {
        #[cfg(target_arch = "x86_64")]
            let dll_path = get_program_directory().join(format!("{}.dll", module_path!()));
        #[cfg(target_arch = "x86")]
            let dll_path = get_program_directory().join(format!("{}32.dll", module_path!()));

        debug!("Module path: {}", dll_path.display());
        let handle = match load_library(dll_path.to_str().unwrap()) {
            Ok(h) => h,
            Err(e) => {
                error!("{}", e);
                return;
            }
        };
        debug!("Module handle: {}", handle.0);

        // 安装消息队列钩子
        let h_hook_get_message = loop {
            let proc = get_proc_address(handle, "hook_proc_get_message");
            if let Ok(h) = set_windows_hook_ex(HOOK_TYPE_GET_MESSAGE, proc.to_hook_proc(), handle.into(), 0) {
                break h;
            }
            error!("Can't set the `get message` hook.");
            sleep(Duration::from_millis(1000));
        };
        debug!("The hook of get message is ok, and it is {}.", h_hook_get_message.0);

        // 安装窗口过程钩子
        let h_hook_call_wnd_proc = loop {
            let proc = get_proc_address(handle, "hook_proc_call_wnd_proc");
            if let Ok(h) = set_windows_hook_ex(HOOK_TYPE_CALL_WND_PROC, proc.to_hook_proc(), handle.into(), 0) {
                break h;
            }
            error!("Can't set the `call wnd proc` hook.");
            sleep(Duration::from_millis(1000));
        };
        debug!("The hook of call wnd proc is ok, and it is {}.", h_hook_call_wnd_proc.0);

        // 通知所有进程需要初始化
        send_message(
            HWND_BROADCAST,
            WM_HOOK_INIT.clone(),
            WPARAM::default(),
            LPARAM::default(),
        );

        let notify = ThreadNotify::new(get_current_thread_id());
        unsafe { HOOK_THREAD.set(notify.clone()) }.unwrap_or(());
        message_loop();

        // 在卸载钩子之前，我们必须先通知所有的远进程即将卸载钩子，让他们有机会清理资源，否则将可能引起系统不稳定
        send_message(
            HWND_BROADCAST,
            WM_HOOK_UNINIT.clone(),
            WPARAM::default(),
            LPARAM::default(),
        );
        // 因为send_message把消息发送到窗口过程函数中，所以他是同步运行，当他返回（也就是运行到这里）时，理论上所有的远进程都已经清理资源完毕（除非有一些意外），这时候无论如何都应该卸载钩子了
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
pub fn unmount() {
    unsafe {
        match HOOK_THREAD.get() {
            None => {}
            Some(x) => {
                x.quit();
                x.join(5000);
            }
        }
    }
    debug!("unmounted.")
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
