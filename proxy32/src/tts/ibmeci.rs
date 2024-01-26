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

use std::ffi::{c_char, CString};
use std::future::Future;
use std::mem::transmute;
use std::pin::Pin;
use std::task::{Context, Poll};
use log::error;
use win_wrap::common::{free_library, get_proc_address, HMODULE, load_library};

macro_rules! call_proc {
    ($module:expr,$name:ident,$def:ty,$($arg:expr),*) => {
        match get_proc_address($module, stringify!($name)) {
            None => None,
            Some(f) => unsafe {
                Some(transmute::<_, $def>(f)($($arg),*))
            }
        }
    };
}

//noinspection SpellCheckingInspection
pub(crate) struct Ibmeci {
    h_module: HMODULE,
    h_eci: isize
}

impl Ibmeci {
    //noinspection SpellCheckingInspection
    /**
     * 创建一个实例。
     * `dll_path` ibmeci.dll的路径。
     * */
    pub fn new(dll_path: &str) -> Self {
        let h_module = load_library(dll_path);
        if h_module.is_err() {
            error!("Can't open the library ({}).", dll_path);
            return Self::null();
        }
        let h_module = h_module.unwrap();
        let h_eci = match call_proc!(h_module, eciNew, fn() -> isize,) {
            None => 0,
            Some(x) => x
        };
        Self {
            h_module,
            h_eci
        }
    }

    /**
     * 创建一个空实例。
     * */
    pub fn null() -> Self {
        Self {
            h_module: HMODULE::default(),
            h_eci: 0
        }
    }

    pub(crate) async fn synth(&self, text: &str) -> Vec<u8> {
        call_proc!(self.h_module, eciAddText, fn(isize, *mut c_char), self.h_eci, CString::new(text).unwrap().into_raw());
        call_proc!(self.h_module, eciSynthesize, fn(isize), self.h_eci);
        let data = Vec::new();
        data
    }
}

impl Drop for Ibmeci {
    fn drop(&mut self) {
        call_proc!(self.h_module, eciDelete, fn(isize),self.h_eci);
        free_library(self.h_module);
    }
}

impl Future for Ibmeci {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let is_speaking = call_proc!(self.h_module, eciSpeaking, fn(isize) -> bool, self.h_eci);
        if let Some(x) = is_speaking {
            if !x {
                return Poll::Ready(());
            }
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}
