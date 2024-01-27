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

use crate::{call_proc, get_program_directory, SERVER_HOME_URI};
use log::{error, info};
use rigela_resources::clone_resource;
use std::time::Duration;
use tokio::time::sleep;
use win_wrap::common::{get_proc_address, load_library, FARPROC, HMODULE};

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
                Option<extern "system" fn(i32, i32, i32, i32) -> i32>,
                i32,
            ) -> i32,
            $freq,
            $chans,
            $flags,
            $proc,
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
}

pub struct BassChannelStream {
    h_bass: i32,
    h_module: HMODULE,
}

impl BassChannelStream {
    pub async fn new(sample_rate: u32, num_channels: u32) -> Self {
        const LIB_NAME: &str = "bass.dll";
        let url = format!("{}/{}", SERVER_HOME_URI, LIB_NAME);

        let eci_path = get_program_directory().join(LIB_NAME);
        let file = clone_resource(url, eci_path.clone()).await;
        if let Err(e) = file {
            error!("{}", e);
            return Self::null();
        }
        drop(file);
        let h_module = loop {
            // 文件刚释放可能被安全软件锁定，推迟加载他
            sleep(Duration::from_millis(1000)).await;
            match load_library(eci_path.to_str().unwrap()) {
                Ok(h) => break h,
                Err(e) => error!("Can't open the library ({}). {}", eci_path.display(), e),
            }
        };
        info!("{} loaded.", eci_path.display());
        bass!(h_module, init, -1, 44100, 0, 0, 0);
        let h_bass = bass!(
            h_module,
            stream_create,
            sample_rate as i32,
            num_channels as i32,
            0,
            None,
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
    pub fn dispose(&self) {
        bass!(self.h_module, stream_free, self.h_bass);
        bass!(self.h_module, free);
    }
}
