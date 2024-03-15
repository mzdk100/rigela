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

use crate::context::Context;
use rigela_utils::bass::BassChannelOutputStream;
use std::{
    collections::HashMap,
    sync::{
        Arc,
        OnceLock,
        Weak,
    },
    time::Duration,
};
use tokio::{sync::Mutex, time::sleep};

/// 声音参数
pub(crate) enum SoundArgument {
    Single(&'static str),
    WithFreq(&'static str, f32),
}

/// 音效播放器
#[derive(Debug)]
pub(crate) struct Sound {
    context: OnceLock<Weak<Context>>,
    sound_streams: Mutex<HashMap<String, Arc<BassChannelOutputStream>>>,
}

impl Sound {
    pub(crate) fn new() -> Self {
        Self {
            context: OnceLock::new(),
            sound_streams: HashMap::new().into(),
        }
    }

    pub(crate) fn apply(&self, context: Weak<Context>) {
        self.context.set(context).unwrap_or(());
    }

    //noinspection StructuralWrap
    /**
     * 播放一个音效，并等待音效播放完毕。
     * `arg` 声音参数。
     * */
    pub(crate) async fn play(&self, arg: SoundArgument) {
        let context = loop {
            if let Some(x) = self.context.get() {
                break unsafe { &*x.as_ptr() };
            }
            sleep(Duration::from_millis(100)).await;
        };
        let res_name = match arg {
            SoundArgument::Single(n) => n,
            SoundArgument::WithFreq(n, _) => n,
        };
        let lock = self.sound_streams.lock().await;
        let stream = match lock.get(res_name) {
            None => {
                drop(lock);
                context.resource_provider.open(&res_name).await.unwrap();
                loop {
                    let s = BassChannelOutputStream::from_disk_file(
                        context.resource_provider.get_path(&res_name).as_str(),
                    );
                    if s.is_valid() {
                        let mut lock = self.sound_streams.lock().await;
                        lock.insert(res_name.to_string(), s.into());
                        break;
                    }
                    sleep(Duration::from_millis(100)).await;
                }
                let lock = self.sound_streams.lock().await;
                lock.get(res_name).unwrap().clone()
            }
            Some(s) => {
                let s = s.clone();
                drop(lock);
                s
            }
        };
        if let SoundArgument::WithFreq(_, f) = arg {
            stream.set_freq(f);
        }
        stream.play(true);
        stream.wait_until_stopped_or_stalled().await;
    }

    /**
     * 停止所有正在播放的音效。
     * */
    pub(crate) async fn stop_all(&self) {
        for x in self.sound_streams.lock().await.iter() {
            x.1.stop();
        }
    }
}
