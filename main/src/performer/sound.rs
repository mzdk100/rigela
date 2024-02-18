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
    sync::{Arc, OnceLock},
    time::Duration,
};
use tokio::{sync::Mutex, time::sleep};

/// 音效播放器
#[derive(Debug)]
pub(crate) struct Sound {
    context: OnceLock<Arc<Context>>,
    sound_streams: Mutex<HashMap<String, Arc<BassChannelOutputStream>>>,
}

impl Sound {
    pub(crate) fn new() -> Self {
        Self {
            context: OnceLock::new(),
            sound_streams: HashMap::new().into(),
        }
    }

    pub(crate) fn apply(&self, context: Arc<Context>) {
        self.context.set(context.clone()).unwrap_or(());
    }

    //noinspection StructuralWrap
    /**
     * 播放一个音效，并等待音效播放完毕。
     * `res_name` 资源名称。
     * */
    pub(crate) async fn play(&self, res_name: &str) {
        let context = loop {
            if let Some(x) = self.context.get() {
                break x;
            }
            sleep(Duration::from_millis(100)).await;
        };
        let mut lock = self.sound_streams.lock().await;
        let stream = match lock.get(res_name) {
            None => {
                context.resource_accessor.open(res_name).await.unwrap();
                let s = BassChannelOutputStream::from_disk_file(
                    context.resource_accessor.get_path(res_name).as_str(),
                );
                lock.insert(res_name.to_string(), s.into());
                lock.get(res_name).unwrap()
            }
            Some(s) => s,
        }
        .clone();
        drop(lock);
        stream.play(true);
        stream.wait_until_stopped().await;
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
