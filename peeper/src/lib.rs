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

use std::ffi::c_void;
use std::mem::transmute;
use std::thread;
use log::debug;
use win_wrap::common::{BOOL, DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH, FALSE, HMODULE, set_windows_hook_ex, TRUE, WPARAM, LPARAM, LRESULT, HOOKPROC, call_next_hook_ex, unhook_windows_hook_ex, load_library, beep, get_proc_address};
use win_wrap::hook::HOOK_TYPE_GET_MESSAGE;
use win_wrap::message::message_loop;

#[no_mangle]
unsafe extern "system" fn hook_get_message(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    beep(400,40);
    call_next_hook_ex(code, w_param, l_param)
}

#[no_mangle]
pub extern "system" fn mount() {
    debug!("mounted");
    thread::spawn(|| {
        let path = format!("{}.dll", module_path!());
        debug!("p:{}", path);
        let handle = load_library(path.as_str());
        debug!("Module handle is {}", handle.0);
        let proc = get_proc_address(handle, "hook_get_message")
            .map(|x| unsafe {
                transmute::<_, unsafe extern "system" fn(i32, WPARAM, LPARAM) -> LRESULT>(x)
            });
        let h_hook = set_windows_hook_ex(HOOK_TYPE_GET_MESSAGE, proc, handle.into(), 0);
        message_loop();
        unhook_windows_hook_ex(h_hook);
    });
}

#[no_mangle]
pub extern "system" fn unmount() {
    debug!("unmounted")
}

#[no_mangle]
extern "system" fn DllMain(h_module: HMODULE, dw_reason: u32, _lp_reserved: *const c_void) -> BOOL {
    match dw_reason {
        DLL_PROCESS_ATTACH => {}
        DLL_PROCESS_DETACH => {}
        DLL_THREAD_ATTACH => {}
        DLL_THREAD_DETACH => {}
        _ => return FALSE
    }
    TRUE
}

#[cfg(test)]
mod tests {}
