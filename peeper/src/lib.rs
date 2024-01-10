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

use log::debug;
use std::{ffi::c_void, sync::RwLock, thread};
use win_wrap::{
    common::{
        call_next_hook_ex, get_proc_address, load_library, set_windows_hook_ex,
        unhook_windows_hook_ex, BOOL, DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH,
        DLL_THREAD_DETACH, FALSE, HMODULE, LPARAM, LRESULT, TRUE, WPARAM,
    },
    ext::FarProcExt,
    hook::HOOK_TYPE_GET_MESSAGE,
    message::message_loop,
    threading::{get_current_thread_id, ThreadNotify},
};

static HOOK_THREAD: RwLock<Option<ThreadNotify>> = RwLock::new(None);

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
pub extern "system" fn mount() {
    debug!("mounted");
    thread::spawn(|| {
        let path = format!("{}.dll", module_path!());
        debug!("Module path: {}", path);
        let handle = load_library(path.as_str());
        debug!("Module handle: {}", handle.0);
        let proc = get_proc_address(handle, "hook_proc_get_message");
        let h_hook =
            set_windows_hook_ex(HOOK_TYPE_GET_MESSAGE, proc.to_hook_proc(), handle.into(), 0);
        let notify = ThreadNotify::new(get_current_thread_id());
        let mut lock = HOOK_THREAD.write().unwrap();
        *lock = Some(notify.clone());
        drop(lock);
        message_loop();
        unhook_windows_hook_ex(h_hook);
        notify.finish();
    });
}

/** 停止亏叹气。 */
pub extern "system" fn unmount() {
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
