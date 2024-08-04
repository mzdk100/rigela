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

use crate::{call_proc, library::setup_library};
use log::{error, info};
use std::{
    ffi::{c_void, CString},
    sync::Once,
    time::Duration,
};
use tokio::time::sleep;
use win_wrap::common::{free_library, get_proc_address, load_library, FARPROC, HMODULE};
use crate::library::get_rigela_library_path;

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
    ($module:expr,stream_put_file_data,$handle:expr,$data:expr) => {
        call_proc!(
            $module,
            BASS_StreamPutFileData,
            extern "system" fn(i32, *const u8, i32) -> i32,
            $handle,
            $data.as_ptr(),
            $data.len() as i32
        )
    };
    ($module:expr,channel_is_active,$handle:expr) => {
        call_proc!(
            $module,
            BASS_ChannelIsActive,
            extern "system" fn(i32) -> i32,
            $handle
        )
    };
    ($module:expr,channel_set_sync,$handle:expr,$type:expr,$param:expr,$proc:expr,$user:expr) => {
        call_proc!(
            $module,
            BASS_ChannelSetSync,
            extern "system" fn(i32, u32, i64, fn(i32, i32, i32, i32), i32) -> i32,
            $handle,
            $type,
            $param,
            $proc,
            $user
        )
    };
    ($module:expr,channel_remove_sync,$handle:expr,$h_sync:expr) => {
        call_proc!(
            $module,
            BASS_ChannelRemoveSync,
            extern "system" fn(i32, i32) -> bool,
            $handle,
            $h_sync
        )
    };
    ($module:expr,stream_create_file,$mem:expr,$data:expr,$offset:expr,$length:expr,$flags:expr) => {
        call_proc!(
            $module,
            BASS_StreamCreateFile,
            extern "system" fn(bool, *const c_void, u64, u64, i32) -> i32,
            $mem,
            $data as *const c_void,
            $offset,
            $length,
            $flags
        )
    };
    ($module:expr,channel_slide_attribute,$handle:expr,$attrib:expr,$value:expr,$time:expr) => {
        call_proc!(
            $module,
            BASS_ChannelSlideAttribute,
            extern "system" fn(i32, i32, f32, i32) -> bool,
            $handle,
            $attrib,
            $value,
            $time
        )
    };
}

#[cfg(target_arch = "x86_64")]
const LIB_NAME: &str = "bass-64.dll";
#[cfg(target_arch = "x86")]
const LIB_NAME: &str = "bass-32.dll";

//noinspection SpellCheckingInspection
const STREAMPROC_PUSH: usize = usize::MAX;

/// The channel is not active, or a handle is not a valid channel.
const BASS_ACTIVE_STOPPED: i32 = 0;
/// The channel is playing (or recording).
#[allow(unused)]
const BASS_ACTIVE_PLAYING: i32 = 1;
/// Playback of the stream has been stalled due to a lack of sample data.
/// Playback will automatically resume once there is sufficient data to do so.
const BASS_ACTIVE_STALLED: i32 = 2;
/// The channel is paused.
const BASS_ACTIVE_PAUSED: i32 = 3;
/// The channel's device is paused.
#[allow(unused)]
const BASS_ACTIVE_PAUSED_DEVICE: i32 = 4;

// BASS_ChannelSetSync types
#[allow(unused)]
const BASS_SYNC_POS: u32 = 0;
#[allow(unused)]
const BASS_SYNC_END: u32 = 2;
#[allow(unused)]
const BASS_SYNC_META: u32 = 4;
#[allow(unused)]
const BASS_SYNC_SLIDE: u32 = 5;
#[allow(unused)]
const BASS_SYNC_STALL: u32 = 6;
#[allow(unused)]
const BASS_SYNC_DOWNLOAD: u32 = 7;
#[allow(unused)]
const BASS_SYNC_FREE: u32 = 8;
//noinspection SpellCheckingInspection
#[allow(unused)]
const BASS_SYNC_SETPOS: u32 = 11;
//noinspection SpellCheckingInspection
#[allow(unused)]
const BASS_SYNC_MUSICPOS: u32 = 10;
//noinspection SpellCheckingInspection
#[allow(unused)]
const BASS_SYNC_MUSICINST: u32 = 1;
//noinspection SpellCheckingInspection
#[allow(unused)]
const BASS_SYNC_MUSICFX: u32 = 3;
#[allow(unused)]
const BASS_SYNC_OGG_CHANGE: u32 = 12;
#[allow(unused)]
const BASS_SYNC_DEV_FAIL: u32 = 14;
#[allow(unused)]
const BASS_SYNC_DEV_FORMAT: u32 = 15;
/// flag: call sync in another thread
#[allow(unused)]
const BASS_SYNC_THREAD: u32 = 0x20000000;
//noinspection SpellCheckingInspection
/// flag: sync at mixtime, else at playtime
#[allow(unused)]
const BASS_SYNC_MIXTIME: u32 = 0x40000000;
#[allow(unused)]
const BASS_SYNC_ONETIME: u32 = 0x80000000; // flag: sync only once, else continuously

// Channel attributes
const BASS_ATTRIB_FREQ: i32 = 1;
#[allow(unused)]
const BASS_ATTRIB_VOL: i32 = 2;
#[allow(unused)]
const BASS_ATTRIB_PAN: i32 = 3;
//noinspection SpellCheckingInspection
#[allow(unused)]
const BASS_ATTRIB_EAXMIX: i32 = 4;
//noinspection SpellCheckingInspection
#[allow(unused)]
const BASS_ATTRIB_NOBUFFER: i32 = 5;
#[allow(unused)]
const BASS_ATTRIB_VBR: i32 = 6;
#[allow(unused)]
const BASS_ATTRIB_CPU: i32 = 7;
#[allow(unused)]
const BASS_ATTRIB_SRC: i32 = 8;
#[allow(unused)]
const BASS_ATTRIB_NET_RESUME: i32 = 9;
//noinspection SpellCheckingInspection
#[allow(unused)]
const BASS_ATTRIB_SCANINFO: i32 = 10;
//noinspection SpellCheckingInspection
#[allow(unused)]
const BASS_ATTRIB_NORAMP: i32 = 11;
#[allow(unused)]
const BASS_ATTRIB_BITRATE: i32 = 12;
#[allow(unused)]
const BASS_ATTRIB_BUFFER: i32 = 13;
#[allow(unused)]
const BASS_ATTRIB_GRANULE: i32 = 14;
#[allow(unused)]
const BASS_ATTRIB_USER: i32 = 15;
#[allow(unused)]
const BASS_ATTRIB_TAIL: i32 = 16;
#[allow(unused)]
const BASS_ATTRIB_PUSH_LIMIT: i32 = 17;
//noinspection SpellCheckingInspection
#[allow(unused)]
const BASS_ATTRIB_DOWNLOADPROC: i32 = 18;
//noinspection SpellCheckingInspection
#[allow(unused)]
const BASS_ATTRIB_VOLDSP: i32 = 19;
//noinspection SpellCheckingInspection
#[allow(unused)]
const BASS_ATTRIB_VOLDSP_PRIORITY: i32 = 20;
#[allow(unused)]
const BASS_ATTRIB_MUSIC_AMPLIFY: i32 = 0x100;
//noinspection SpellCheckingInspection
#[allow(unused)]
const BASS_ATTRIB_MUSIC_PANSEP: i32 = 0x101;
#[allow(unused)]
const BASS_ATTRIB_MUSIC_PSCALER: i32 = 0x102;
#[allow(unused)]
const BASS_ATTRIB_MUSIC_BPM: i32 = 0x103;
#[allow(unused)]
const BASS_ATTRIB_MUSIC_SPEED: i32 = 0x104;
#[allow(unused)]
const BASS_ATTRIB_MUSIC_VOL_GLOBAL: i32 = 0x105;
#[allow(unused)]
const BASS_ATTRIB_MUSIC_ACTIVE: i32 = 0x106;
#[allow(unused)]
const BASS_ATTRIB_MUSIC_VOL_CHAN: i32 = 0x200;
// + channel #
#[allow(unused)]
const BASS_ATTRIB_MUSIC_VOL_INST: i32 = 0x300; // + instrument #

#[derive(Debug)]
pub struct BassChannelOutputStream {
    h_bass: i32,
    h_module: HMODULE,
}

impl BassChannelOutputStream {
    fn create(slot: impl FnOnce(HMODULE) -> Option<i32>) -> Self {
        let bass_path = get_rigela_library_path().join(LIB_NAME);
        #[cfg(target_arch = "x86_64")]
        setup_library(&bass_path, include_bytes!("../lib/bass-64.dll"));
        #[cfg(target_arch = "x86")]
        setup_library(&bass_path, include_bytes!("../lib/bass-32.dll"));
        let h_module = match load_library(bass_path.to_str().unwrap()) {
            Ok(h) => h,
            Err(e) => {
                error!("Can't open the library ({}). {}", bass_path.display(), e);
                return Self::null();
            }
        };
        static _LOADED: Once = Once::new();
        _LOADED.call_once(|| {
            info!(
                "{} loaded, library handle is {:?}.",
                bass_path.display(),
                h_module.0
            )
        });
        bass!(h_module, init, -1, 44100, 0, 0, 0);
        let h_bass = slot(h_module).unwrap();
        Self { h_bass, h_module }
    }

    //noinspection RsUnresolvedReference
    /**
    创建一个通道输出流。
    `sample_rate` 采样率。
    `num_channels` 声道数量。
    */
    pub fn new(sample_rate: u32, num_channels: u32) -> Self {
        Self::create(|h_module| {
            bass!(
                h_module,
                stream_create,
                sample_rate as i32,
                num_channels as i32,
                0,
                STREAMPROC_PUSH,
                0
            )
        })
    }

    /**
    从内存文件创建实例。
    `data` 文件数据。
    */
    pub fn from_memory_file(data: &[u8]) -> Self {
        Self::create(|h_module| bass!(h_module, stream_create_file, true, data.as_ptr(), 0, 0, 0))
    }

    /**
    从磁盘文件创建实例。
    `path` 文件路径。
    */
    pub fn from_disk_file(path: &str) -> Self {
        let path = CString::new(path).unwrap();
        Self::create(|h_module| bass!(h_module, stream_create_file, false, path.as_ptr(), 0, 0, 0))
    }

    /**
    创建一个空实例。
    */
    fn null() -> Self {
        Self {
            h_bass: 0,
            h_module: HMODULE::default(),
        }
    }

    /**
    判断输出流是否可用。
    */
    pub fn is_valid(&self) -> bool {
        (!self.h_module.is_invalid()) && self.h_bass != 0
    }
    /**
    清理释放。
    */
    pub fn dispose(&self) {
        bass!(self.h_module, stream_free, self.h_bass);
        bass!(self.h_module, free);
    }

    /**
    播放操作，此方法和start方法的功能一样，但具有从头开始播放的选项。
    `restart` 重新开始。
    */
    pub fn play(&self, restart: bool) {
        bass!(self.h_module, channel_play, self.h_bass, restart);
    }

    /**
    暂停操作。
    */
    pub fn pause(&self) {
        bass!(self.h_module, channel_pause, self.h_bass);
    }

    /**
    停止操作。
    */
    pub fn stop(&self) {
        bass!(self.h_module, channel_stop, self.h_bass);
    }

    /**
    开始或继续播放操作。
    */
    pub fn start(&self) {
        bass!(self.h_module, channel_start, self.h_bass);
    }

    /**
    写入数据。
    `data` 音频数据。
    */
    pub fn put_data(&self, data: &[u8]) -> i32 {
        bass!(self.h_module, stream_put_data, self.h_bass, data).unwrap_or(0)
    }

    //noinspection StructuralWrap
    /**
    写入文件数据。
    `data` 音频文件数据。
    */
    pub fn put_file_data(&self, data: &[u8]) -> i32 {
        bass!(self.h_module, stream_put_file_data, self.h_bass, data).unwrap_or(0)
    }

    //noinspection StructuralWrap
    /**
    设置播放频率。
    `value` 要播放的频率。
    */
    pub fn set_freq(&self, value: f32) {
        bass!(
            self.h_module,
            channel_slide_attribute,
            self.h_bass,
            BASS_ATTRIB_FREQ,
            value,
            100
        );
    }

    /**
    检查样本、流或MOD音乐是否处于活动状态（正在播放）或暂停状态。还可以检查是否正在录制。
    */
    pub fn is_active(&self) -> i32 {
        bass!(self.h_module, channel_is_active, self.h_bass).unwrap_or(BASS_ACTIVE_STOPPED)
    }

    /**
    判断播放状态是否已经停止。
    */
    pub fn is_stopped(&self) -> bool {
        self.is_active() == BASS_ACTIVE_STOPPED
    }

    /**
    等待直到停止状态或没有数据可以播放。
    */
    pub async fn wait_until_stopped_or_stalled(&self) {
        self.wait(|active| active == BASS_ACTIVE_STOPPED || active == BASS_ACTIVE_STALLED)
            .await;
    }

    /**
    等待直到暂停。
    */
    pub async fn wait_until_paused(&self) {
        self.wait(|active| active == BASS_ACTIVE_PAUSED).await;
    }

    async fn wait(&self, condition: impl Fn(i32) -> bool) {
        loop {
            let Some(active) = bass!(self.h_module, channel_is_active, self.h_bass) else {
                break;
            };
            if condition(active) {
                break;
            }
            sleep(Duration::from_millis(100)).await;
        }
    }
}

unsafe impl Send for BassChannelOutputStream {}
unsafe impl Sync for BassChannelOutputStream {}

impl Drop for BassChannelOutputStream {
    fn drop(&mut self) {
        if !self.h_module.is_invalid() {
            free_library(self.h_module);
        }
    }
}

#[cfg(test)]
mod test_bass {
    use crate::bass::BassChannelOutputStream;

    #[tokio::test]
    async fn main() {
        let out = BassChannelOutputStream::new(16000, 1);
        let mut data = vec![];
        for i in 0..8000 {
            data.push(((i as f64).sin() * 127f64 + 128f64) as u8)
        }
        out.start();
        out.put_data(&data);
        out.wait_until_stopped_or_stalled().await;
        let out = BassChannelOutputStream::from_disk_file(
            r"C:\Users\Administrator\.rigela\resources\tip.wav",
        );
        for _ in 0..5 {
            out.start();
            out.wait_until_stopped_or_stalled().await;
        }
    }
}
