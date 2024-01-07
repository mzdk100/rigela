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
use std::sync::RwLock;
use log::debug;
use windows::Win32::{
    Foundation::{BOOL, FALSE, TRUE, HMODULE},
    System::SystemServices::{DLL_PROCESS_DETACH, DLL_PROCESS_ATTACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH}
};

#[allow(dead_code)]
static H_INSTANCE: RwLock<HMODULE> = RwLock::new(HMODULE(0));

#[allow(dead_code)]
fn setup(hmodule: HMODULE) {
    let mut x = H_INSTANCE
        .write()
        .unwrap();
    if x.is_invalid() {
        return;
    }
    *x = hmodule;
}

#[no_mangle]
pub extern "system" fn mount() {
    debug!("mounted")
}

#[no_mangle]
pub extern "system" fn unmount() {
    debug!("unmounted")
}

#[no_mangle]
extern "system" fn DllMain(h_module: HMODULE, dw_reason: u32, _lp_reserved: *const c_void) -> BOOL {
    match dw_reason {
        DLL_PROCESS_ATTACH => {
            println!("hm: {}", h_module.0)
        }
        DLL_PROCESS_DETACH => {}
        DLL_THREAD_ATTACH => {}
        DLL_THREAD_DETACH => {}
        _ => return FALSE
    }
    TRUE
}

#[cfg(test)]
mod tests {}
