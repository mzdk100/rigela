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
use std::collections::HashMap;
use std::io::SeekFrom;

use std::sync::Arc;

use crate::context::Context;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio::sync::Mutex;
use win_wrap::audio::AudioOutputStream;

pub(crate) struct Sounder {
    data_table: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    output_stream: Arc<AudioOutputStream>,
}

impl Sounder {
    /**
     * 创建一个新的音效播放器。
     * */
    pub(crate) fn new() -> Self {
        let output_stream = AudioOutputStream::new(16000, 1);
        Self {
            data_table: Arc::new(HashMap::new().into()),
            output_stream: output_stream.into(),
        }
    }

    /**
     * 播放一个音效。
     * 目前仅支持16位深16K采样率单通道的音频。
     * */
    pub(crate) async fn play(&self, res_name: &str) {
        let lock = self.data_table.lock().await;
        let data = lock.get(res_name).unwrap().clone();
        drop(lock);
        self.output_stream.flush();
        self.output_stream.stop();
        self.output_stream.start();
        for i in (0..data.len()).step_by(3200) {
            self.output_stream.write(&data[i..i + 3200]).await;
        }
    }

    /**
     * 配置音效播放器。
     * `context` 上下文环境。
     * */
    pub(crate) async fn apply(&self, context: Arc<Context>) {
        let list = vec!["boundary.wav"];
        for i in &list {
            let mut data = Vec::<u8>::new();
            let mut file = context.resource_accessor.open(i).await.unwrap();
            file.seek(SeekFrom::Start(44)).await.unwrap();
            file.read_to_end(&mut data).await.unwrap();
            self.data_table.lock().await.insert(i.to_string(), data);
        }
    }
}
