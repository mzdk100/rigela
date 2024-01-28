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

use crate::{call_proc, get_program_directory};
use log::{error, info};
use win_wrap::common::{free_library, get_proc_address, load_library, FARPROC, HMODULE};

macro_rules! bass {
    ($module:expr,init,$device:expr,$freq:expr,$flags:expr,$win:expr,$clsid:expr) => {
        call_proc!(
            $module,
            BASS_Init,
            extern "system" fn(i32, i32, i32, i32, i32) -> bool,
            $device,
            $freq,
            $flags,
            $win,
            $clsid
        )
    };
    ($module:expr,free) => {
        call_proc!($module, BASS_Free, extern "system" fn() -> bool,)
    };
    ($module:expr,stream_create,$freq:expr,$chans:expr,$flags:expr,$proc:expr,$user:expr) => {
        call_proc!(
            $module,
            BASS_StreamCreate,
            extern "system" fn(
                i32,
                i32,
                i32,
                *const extern "system" fn(i32, i32, i32, i32) -> i32,
                i32,
            ) -> i32,
            $freq,
            $chans,
            $flags,
            $proc as *const extern "system" fn(i32, i32, i32, i32) -> i32,
            $user
        )
    };
    ($module:expr,stream_free,$handle:expr) => {
        call_proc!(
            $module,
            BASS_StreamFree,
            extern "system" fn(i32) -> bool,
            $handle
        )
    };
    ($module:expr,channel_play,$handle:expr,$restart:expr) => {
        call_proc!(
            $module,
            BASS_ChannelPlay,
            extern "system" fn(i32, bool) -> bool,
            $handle,
            $restart
        )
    };
    ($module:expr,channel_pause,$handle:expr) => {
        call_proc!(
            $module,
            BASS_ChannelPause,
            extern "system" fn(i32) -> bool,
            $handle
        )
    };
    ($module:expr,channel_stop,$handle:expr) => {
        call_proc!(
            $module,
            BASS_ChannelStop,
            extern "system" fn(i32) -> bool,
            $handle
        )
    };
    ($module:expr,channel_start,$handle:expr) => {
        call_proc!(
            $module,
            BASS_ChannelStart,
            extern "system" fn(i32) -> bool,
            $handle
        )
    };
    ($module:expr,stream_put_data,$handle:expr,$data:expr) => {
        call_proc!(
            $module,
            BASS_StreamPutData,
            extern "system" fn(i32, *const u8, i32) -> i32,
            $handle,
            $data.as_ptr(),
            $data.len() as i32
        )
    };
}

const LIB_NAME: &str = "bass.dll";

//noinspection SpellCheckingInspection
const STREAMPROC_PUSH: usize = usize::MAX;

#[derive(Debug)]
pub struct BassChannelOutputStream {
    h_bass: i32,
    h_module: HMODULE,
}

impl BassChannelOutputStream {
    //noinspection RsUnresolvedReference
    /**
     * 创建一个通道输出流。
     * `sample_rate` 采样率。
     * `num_channels` 声道数量。
     * */
    pub fn new(sample_rate: u32, num_channels: u32) -> Self {
        #[cfg(target_arch = "x86_64")]
        use std::{fs::OpenOptions, io::Write};

        let bass_path = get_program_directory().join(LIB_NAME);
        #[cfg(target_arch = "x86_64")]
        if !bass_path.exists() {
            let lib_bin = include_bytes!("../lib/bass.dll");
            OpenOptions::new()
                .write(true)
                .create(true)
                .open(&bass_path)
                .unwrap()
                .write_all(lib_bin)
                .unwrap();
        }
        let h_module = match load_library(bass_path.to_str().unwrap()) {
            Ok(h) => h,
            Err(e) => {
                error!("Can't open the library ({}). {}", bass_path.display(), e);
                return Self::null();
            }
        };
        info!("{} loaded.", bass_path.display());
        bass!(h_module, init, -1, 44100, 0, 0, 0);
        let h_bass = bass!(
            h_module,
            stream_create,
            sample_rate as i32,
            num_channels as i32,
            0,
            STREAMPROC_PUSH,
            0
        )
        .unwrap();
        Self { h_bass, h_module }
    }

    fn null() -> Self {
        Self {
            h_bass: 0,
            h_module: HMODULE::default(),
        }
    }

    /**
     * 清理释放。
     * */
    pub fn dispose(&self) {
        bass!(self.h_module, stream_free, self.h_bass);
        bass!(self.h_module, free);
    }

    /**
     * 播放操作。
     * `restart` 重新开始。
     * */
    pub fn play(&self, restart: bool) {
        bass!(self.h_module, channel_play, self.h_bass, restart);
    }

    /**
     * 暂停操作。
     * */
    pub fn pause(&self) {
        bass!(self.h_module, channel_pause, self.h_bass);
    }

    /**
     * 停止操作。
     * */
    pub fn stop(&self) {
        bass!(self.h_module, channel_stop, self.h_bass);
    }

    /**
     * 开始或继续播放操作。
     * */
    pub fn start(&self) {
        bass!(self.h_module, channel_start, self.h_bass);
    }

    /**
     * 写入数据。
     * `data` 音频数据。
     * */
    pub fn put_data(&self, data: &[u8]) {
        bass!(self.h_module, stream_put_data, self.h_bass, data);
    }
}

impl Drop for BassChannelOutputStream {
    fn drop(&mut self) {
        if !self.h_module.is_invalid() {
            free_library(self.h_module);
        }
    }
}
