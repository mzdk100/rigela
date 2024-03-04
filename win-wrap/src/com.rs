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

use crate::{common::HRESULT, ext::VecExt};
use std::ffi::c_void;
use windows::Win32::System::{
    Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED, SAFEARRAY},
    Ole::{SafeArrayDestroy, SafeArrayGetElement, SafeArrayGetLBound, SafeArrayGetUBound},
};

/**
 * 使用多线程模型套间（Multi Thread Apartment, MTA）初始化COM调用。
 * MTA能充分利用多核CPU，提高程序性能，但要注意线程之间同步的安全问题。
 * */
pub fn co_initialize_multi_thread() -> HRESULT {
    unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }
}

/**
 * 关闭当前线程的COM库,卸载线程加载的所有dll,释放任何其他的资源,关闭在线程上维护所有的RPC连接。
 * */
pub fn co_uninitialize() {
    unsafe { CoUninitialize() }
}

impl<T> VecExt<T> for *const SAFEARRAY {
    fn to_vec(self) -> Vec<T> {
        unsafe {
            let mut v = vec![];
            let a = SafeArrayGetLBound(self, 1).unwrap();
            let b = SafeArrayGetUBound(self, 1).unwrap();
            for i in a..=b {
                let mut e: T = std::mem::zeroed();
                SafeArrayGetElement(self, &i, &mut e as *mut T as *mut c_void).unwrap_or(());
                v.push(e);
            }
            SafeArrayDestroy(self).unwrap_or(());
            v
        }
    }
}
