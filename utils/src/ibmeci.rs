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

use crate::{call_proc, common::SafeModuleHandle, library::setup_library};
use encoding_rs::GBK;
use log::info;
use std::{
    alloc::{alloc_zeroed, dealloc, Layout},
    borrow::Cow,
    ffi::{c_char, CString},
    sync::OnceLock,
    thread,
};
use tokio::sync::oneshot::{self, channel, Sender};
use win_wrap::{
    common::{free_library, get_proc_address, load_library, FARPROC, LPARAM, WPARAM},
    message::{message_loop, post_thread_message, register_window_message},
    threading::get_current_thread_id,
    wm,
};
use crate::library::get_rigela_library_path;

macro_rules! eci {
    ($module:expr,new) => {
        call_proc!($module, eciNew, extern "system" fn() -> i32,)
    };
    ($module:expr,delete,$handle:expr) => {
        call_proc!($module, eciDelete, extern "system" fn(i32) -> i32, $handle)
    };
    ($module:expr,speaking,$handle:expr) => {
        call_proc!(
            $module,
            eciSpeaking,
            extern "system" fn(i32) -> bool,
            $handle
        )
    };
    ($module:expr,stop,$handle:expr) => {
        call_proc!($module, eciStop, extern "system" fn(i32) -> bool, $handle)
    };
    ($module:expr,register_callback,$handle:expr,$cb:expr,$data:expr) => {
        call_proc!(
            $module,
            eciRegisterCallback,
            extern "system" fn(i32, extern "system" fn(u32, u32, u32, u32) -> u32, u32),
            $handle,
            $cb,
            $data
        )
    };
    ($module:expr,set_output_buffer,$handle:expr,$samples:expr,$buffer:expr) => {
        call_proc!(
            $module,
            eciSetOutputBuffer,
            extern "system" fn(i32, u32, *mut u8),
            $handle,
            $samples,
            $buffer
        )
    };
    ($module:expr,add_text,$handle:expr,$text:expr) => {{
        if let Ok(p) = CString::new($text) {
            call_proc!(
                $module,
                eciAddText,
                extern "system" fn(i32, *const c_char) -> bool,
                $handle,
                p.as_ptr()
            )
        } else {
            None
        }
    }};
    ($module:expr,speak_text,$text:expr) => {{
        if let Ok(p) = CString::new($text) {
            call_proc!(
                $module,
                eciSpeakText,
                extern "system" fn(*mut c_char),
                p.as_ptr()
            )
        } else {
            None
        }
    }};
    ($module:expr,synthesize,$handle:expr) => {
        call_proc!($module, eciSynthesize, extern "system" fn(i32), $handle)
    };
    ($module:expr,synchronize,$handle:expr) => {
        call_proc!($module, eciSynchronize, extern "system" fn(i32), $handle)
    };
    ($module:expr,set_voice_param,$handle:expr,$voice:expr,$key:expr,$value:expr) => {
        call_proc!(
            $module,
            eciSetVoiceParam,
            extern "system" fn(i32, i32, u32, i32),
            $handle,
            $voice,
            $key,
            $value
        )
    };
    ($module:expr,get_voice_param,$handle:expr,$voice:expr,$key:expr) => {
        call_proc!(
            $module,
            eciGetVoiceParam,
            extern "system" fn(i32, i32, u32) -> i32,
            $handle,
            $voice,
            $key
        )
    };
    ($module:expr,copy_voice,$handle:expr,$copy_from:expr,$copy_to:expr) => {
        call_proc!(
            $module,
            eciCopyVoice,
            extern "system" fn(i32, u32, u32),
            $handle,
            $copy_from,
            $copy_to
        )
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

// Voice params
pub const VP_GENDER: u32 = 0;
pub const VP_HEAD_SIZE: u32 = 1;
pub const VP_PITCH_BASELINE: u32 = 2;
pub const VP_PITCH_FLUCTUATION: u32 = 3;
pub const VP_ROUGHNESS: u32 = 4;
//noinspection SpellCheckingInspection
pub const VP_BREATHINESS: u32 = 5;
pub const VP_SPEED: u32 = 6;
pub const VP_VOLUME: u32 = 7;

//noinspection SpellCheckingInspection
static mut IBMECI: OnceLock<Ibmeci> = OnceLock::new();
static SYNTH_TASK: OnceLock<u32> = OnceLock::new();

extern "system" fn _callback_internal(
    #[allow(unused_variables)] h_eci: u32,
    msg: u32,
    param: u32,
    #[allow(unused_variables)] data: u32,
) -> u32 {
    if msg != MSG_WAVEFORM_BUFFER {
        return RETURN_DATA_PROCESSED;
    }
    unsafe {
        let eci = IBMECI.get_mut();
        if eci.is_none() {
            return RETURN_DATA_PROCESSED;
        }

        let eci = eci.unwrap();
        let mut vec = vec![];
        for i in 0..(param * 2) {
            vec.push(*eci.buffer_ptr.wrapping_add(i as usize));
        }
        eci.data.extend(vec);
    }
    RETURN_DATA_PROCESSED
}

//noinspection SpellCheckingInspection
#[derive(Debug)]
pub struct Ibmeci {
    buffer_layout: Layout,
    buffer_ptr: *mut u8,
    data: Vec<u8>,
    h_module: SafeModuleHandle,
    h_eci: i32,
    thread: u32,
}

impl Ibmeci {
    //noinspection SpellCheckingInspection
    /**
    获取一个实例。
    */
    pub async fn get<'a>() -> Result<&'a Self, String> {
        unsafe {
            // 单例模式
            if let Some(self_) = IBMECI.get() {
                return Ok(self_);
            }
        }
        const LIB_NAME: &str = "ibmeci.dll";
        let eci_path = get_rigela_library_path().join(LIB_NAME);
        setup_library(&eci_path, include_bytes!("../lib/ibmeci.dll"));

        let h_module = match load_library(eci_path.to_str().unwrap()) {
            Ok(h) => SafeModuleHandle::new(h),
            Err(e) => {
                return Err(format!(
                    "Can't open the library ({}). {}",
                    eci_path.display(),
                    e
                ))
            }
        };
        info!("{} loaded.", eci_path.display());
        let (tx, rx) = oneshot::channel();
        thread::spawn(move || {
            let h_eci = eci!(*h_module, new).unwrap_or(0);
            let buffer_layout = Layout::new::<[u8; 8192]>();
            let buffer_ptr = unsafe { alloc_zeroed(buffer_layout) };

            let self_ = Self {
                buffer_layout,
                buffer_ptr,
                data: vec![],
                h_module: h_module.clone(),
                h_eci,
                thread: get_current_thread_id(),
            };

            eci!(*h_module, register_callback, h_eci, _callback_internal, 0);
            eci!(
                *h_module,
                set_output_buffer,
                h_eci,
                (buffer_layout.size() / 2) as u32,
                buffer_ptr
            );
            info!("Module handle: {:?}, eci handle: {}", h_module.0, h_eci);
            unsafe {
                IBMECI.set(self_).unwrap();
                tx.send(IBMECI.get().unwrap()).unwrap();
            }
            message_loop(|m| {
                if wm!(SYNTH_TASK) == m.message {
                    let b = unsafe { Box::from_raw(m.wParam.0 as *mut Cow<[u8]>) };
                    eci!(*h_module, add_text, h_eci, *b);
                    eci!(*h_module, synthesize, h_eci);
                    eci!(*h_module, synchronize, h_eci);
                    let b = unsafe { Box::from_raw(m.lParam.0 as *mut Sender<()>) };
                    b.send(()).unwrap_or(());
                }
            });
        });
        match rx.await {
            Err(e) => Err(format!("Can't get the instance. ({})", e)),
            Ok(x) => Ok(x),
        }
    }

    /**
    合成语音。
    `text` 要合成的文字。
    */
    pub async fn synth(&self, text: &str) -> Vec<u8> {
        eci!(*self.h_module, stop, self.h_eci);
        let (text, _, unmapped) = GBK.encode(text);
        let text = if unmapped {
            // 如果有不能被编码成gbk的字符，我们需要过滤他们
            let mut v = vec![];
            let mut u = vec![];
            let mut has_html_char = false;
            let mut last_char = 0u8;
            for i in text.iter() {
                let i = i.clone();
                if last_char == 38 {
                    has_html_char = i == 35u8;
                    if has_html_char {
                        u.clear();
                        u.push(last_char);
                        u.push(i);
                    } else {
                        v.push(last_char);
                        v.push(i);
                    }
                } else {
                    if has_html_char {
                        u.push(i);
                        if i == 59u8 {
                            has_html_char = false;
                        } else if !(i >= 48u8 && i <= 57u8) {
                            v.extend(&u);
                            has_html_char = false;
                        }
                    } else if i != 38 {
                        v.push(i);
                    }
                }
                last_char = i;
            }
            Cow::from(v)
        } else {
            text
        };
        if let Some(eci) = unsafe { IBMECI.get_mut() } {
            eci.data.clear();
            let (tx, rx) = channel();
            let tx = Box::new(tx);
            post_thread_message(
                eci.thread,
                wm!(SYNTH_TASK),
                WPARAM(Box::into_raw(Box::new(text)) as usize),
                LPARAM(Box::into_raw(tx) as isize),
            );
            rx.await.unwrap_or(());
            eci.data.clone()
        } else {
            vec![]
        }
    }

    /**
    设置语音参数。
    `vp` 参数key。
    `value` 参数值。
    */
    pub fn set_voice_param(&self, vp: u32, value: i32) {
        eci!(*self.h_module, set_voice_param, self.h_eci, 0, vp, value);
    }

    /**
    获取语音参数。
    `vp` 参数key。
    */
    pub fn get_voice_param(&self, vp: u32) -> i32 {
        eci!(*self.h_module, get_voice_param, self.h_eci, 0, vp).unwrap_or(0)
    }

    /**
    获取发音人列表。
    */
    pub fn get_voices(&self) -> Vec<(u32, String)> {
        vec![
            (1, "Adult Male 1"),
            (2, "Adult Female 1"),
            (3, "Child 1"),
            (4, "Adult Male 2"),
            (5, "Adult Male 3"),
            (6, "Adult Female 2"),
            (7, "Elderly Female 1"),
            (8, "Elderly Male 1"),
        ]
        .iter()
        .map(|i| (i.0, i.1.to_string()))
        .collect()
    }

    /**
    设置当前发音人。
    `voice_id` 声音id。
    */
    pub fn set_voice(&self, voice_id: u32) {
        eci!(*self.h_module, copy_voice, self.h_eci, voice_id, 0);
    }
}

impl Drop for Ibmeci {
    fn drop(&mut self) {
        if !self.h_module.is_invalid() {
            eci!(*self.h_module, delete, self.h_eci);
            free_library(*self.h_module);
        }
        unsafe {
            dealloc(self.buffer_ptr, self.buffer_layout);
        }
    }
}

unsafe impl Sync for Ibmeci {}

unsafe impl Send for Ibmeci {}

#[cfg(all(test, target_arch = "x86"))]
mod test_eci {
    use super::Ibmeci;
    use super::super::logger::init_logger;

    #[tokio::test]
    async fn main() {
        init_logger(Some("test.log"));
        let eci = Ibmeci::get().await.unwrap();
        for _ in 0..1000 {
            let data = eci.synth("abc‎").await;
            assert_eq!(data.len(), 21978);
        }
    }
}
