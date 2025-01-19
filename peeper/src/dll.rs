/*
 * Copyright (c) 2024. The RigelA open source project team and
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
    client::PeeperClient,
    handler::{on_ime, on_input_char},
    wm, HOOK_INIT, HOOK_UNINIT,
};
use log::debug;
use std::{ffi::c_void, sync::RwLock};
use win_wrap::{
    common::{
        call_next_hook_ex, get_module_file_name, DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH,
        DLL_THREAD_ATTACH, DLL_THREAD_DETACH, HMODULE, LPARAM, LRESULT, WPARAM,
    },
    ext::LParamExt,
    hook::CwpStruct,
    input::{WM_CHAR, WM_IME_NOTIFY},
    message::{register_window_message, MSG},
};

macro_rules! handle_event {
    ($name:ident, $($arg:expr),*) => {{
        activate_transaction();
        let lock = CLIENT.read().unwrap();
        $name(lock.as_ref().unwrap(), $($arg),*);
    }};
}

// peeper的client实例。
static CLIENT: RwLock<Option<PeeperClient>> = RwLock::new(None);

fn activate_transaction() {
    let mut lock = CLIENT.write().unwrap();
    if let None = lock.as_ref() {
        let module = get_module_file_name(None);
        debug!("Injected into {}.", module.as_str());
        let client = PeeperClient::new(module);
        debug!("Hooked.");
        *lock = Some(client);
    };
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
    if wm!(HOOK_INIT) == msg.message {
        // 主进程发来的初始化命令
        activate_transaction();
    } else if wm!(HOOK_UNINIT) == msg.message {
        // 主进程发来的卸载命令
        deactivate_transaction();
    } else {
        match msg.message {
            WM_IME_NOTIFY => handle_event!(on_ime, msg.hwnd, msg.wParam, msg.lParam),
            _ => {}
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
    if code < 0 {
        return call_next_hook_ex(code, w_param, l_param);
    }
    let msg: &MSG = l_param.to();
    match msg.message {
        WM_CHAR => handle_event!(on_input_char, msg.wParam),
        WM_IME_NOTIFY => handle_event!(on_ime, msg.hwnd, msg.wParam, msg.lParam),
        _ => {}
    }

    call_next_hook_ex(code, w_param, l_param)
}

#[no_mangle]
extern "system" fn DllMain(_: HMODULE, dw_reason: u32, _lp_reserved: *const c_void) -> i32 {
    match dw_reason {
        DLL_PROCESS_ATTACH => {}
        DLL_PROCESS_DETACH => {}
        DLL_THREAD_ATTACH => {}
        DLL_THREAD_DETACH => {}
        _ => return 0,
    }
    1
}
