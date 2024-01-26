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

use log::error;
use std::{
    alloc::{alloc_zeroed, dealloc, Layout},
    ffi::{c_char, CString},
    future::Future,
    mem::transmute,
    pin::Pin,
    ptr::null_mut,
    sync::RwLock,
    task::{Context, Poll},
};
use win_wrap::common::{free_library, get_proc_address, load_library, HMODULE};

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

#[allow(unused)]
const MSG_WAVEFORM_BUFFER: u32 = 0;
#[allow(unused)]
const MSG_PHONEME_BUFFER: u32 = 1;
#[allow(unused)]
const MSG_INDEX_REPLY: u32 = 2;
#[allow(unused)]
const MSG_PHONEME_INDEX_REPLY: u32 = 3;
#[allow(unused)]
const MSG_WORD_INDEX_REPLY: u32 = 4;
#[allow(unused)]
const RETURN_DATA_NOT_PROCESSED: u32 = 0;
#[allow(unused)]
const RETURN_DATA_PROCESSED: u32 = 1;
#[allow(unused)]
const RETURN_DATA_ABORT: u32 = 2;

//noinspection SpellCheckingInspection
pub(crate) struct Ibmeci {
    buffer_layout: Layout,
    buffer_ptr: *mut u8,
    data: RwLock<Vec<u8>>,
    h_module: HMODULE,
    h_eci: isize,
}

impl Ibmeci {
    fn callback_internal(
        #[allow(unused_variables)] h_eci: isize,
        msg: isize,
        param: isize,
        data: &Self,
    ) -> u32 {
        if msg != MSG_WAVEFORM_BUFFER as isize {
            return RETURN_DATA_PROCESSED;
        }
        let self_ = data;
        let mut lock = self_.data.write().unwrap();
        unsafe {
            for i in 0..(param * 2) {
                lock.push(*self_.buffer_ptr.wrapping_add(i as usize));
            }
        }
        RETURN_DATA_PROCESSED
    }

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
            Some(x) => x,
        };
        let buffer_layout = Layout::new::<[u8; 8192]>();
        let buffer_ptr = unsafe { alloc_zeroed(buffer_layout) };
        let self_ = Self {
            buffer_layout,
            buffer_ptr,
            data: vec![].into(),
            h_module,
            h_eci,
        };
        call_proc!(
            h_module,
            eciRegisterCallback,
            fn(isize, fn(isize, isize, isize, &Self) -> u32, &Self),
            h_eci,
            Self::callback_internal,
            &self_
        );
        call_proc!(
            h_module,
            eciSetOutputBuffer,
            fn(isize, usize, *mut u8),
            h_eci,
            buffer_layout.size() / 2,
            buffer_ptr
        );
        self_
    }

    /**
     * 创建一个空实例。
     * */
    pub fn null() -> Self {
        Self {
            buffer_layout: Layout::new::<u8>(),
            buffer_ptr: null_mut(),
            data: vec![].into(),
            h_module: HMODULE::default(),
            h_eci: 0,
        }
    }

    pub(crate) async fn synth(&self, text: &str) -> Vec<u8> {
        call_proc!(
            self.h_module,
            eciAddText,
            fn(isize, *mut c_char),
            self.h_eci,
            CString::new(text).unwrap().into_raw()
        );
        {
            self.data.write().unwrap().clear();
        }
        call_proc!(self.h_module, eciSynthesize, fn(isize), self.h_eci);
        self.data.read().unwrap().clone()
    }
}

impl Drop for Ibmeci {
    fn drop(&mut self) {
        call_proc!(self.h_module, eciDelete, fn(isize), self.h_eci);
        unsafe {
            dealloc(self.buffer_ptr, self.buffer_layout);
        }
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
