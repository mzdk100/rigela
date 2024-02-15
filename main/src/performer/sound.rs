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
    io::SeekFrom,
    sync::{Arc, OnceLock},
};
use tokio::{
    io::{AsyncReadExt, AsyncSeekExt},
    sync::Mutex,
};

const SOUND_LIST: [&str; 2] = ["boundary.wav", "tip.wav"];

/// 音效播放器
#[derive(Debug)]
pub(crate) struct Sound {
    output_stream: BassChannelOutputStream,
    context: OnceLock<Arc<Context>>,
    sound_table: Mutex<HashMap<String, Vec<u8>>>,
}

impl Sound {
    pub(crate) fn new() -> Self {
        Self {
            output_stream: BassChannelOutputStream::new(16000, 1),
            context: OnceLock::new(),
            sound_table: HashMap::new().into(),
        }
    }

    pub(crate) async fn apply(&self, context: Arc<Context>) {
        self.context.set(context.clone()).unwrap_or(());

        // 初始化音效播放器
        self.load_data().await;
    }

    /// 播放一个音效。 目前仅支持16位深16K采样率单通道的音频。
    pub(crate) async fn play(&self, res_name: &str) {
        self.output_stream.stop();
        self.output_stream.start();

        let lock = self.sound_table.lock().await;
        let data = lock.get(res_name).unwrap().clone();
        drop(lock);

        self.output_stream.put_data(&data);
        self.output_stream.wait_until_stalled().await;
    }

    async fn load_data(&self) {
        let context = self.context.get().unwrap();

        for i in &SOUND_LIST {
            let mut data = Vec::<u8>::new();
            let mut file = context.resource_accessor.open(i).await.unwrap();
            file.seek(SeekFrom::Start(44)).await.unwrap();
            file.read_to_end(&mut data).await.unwrap();
            self.sound_table.lock().await.insert(i.to_string(), data);
        }
    }
}
