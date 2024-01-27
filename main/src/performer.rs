/*
 * Copyright (c) 2023. The RigelA open source project team and
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

use crate::{configs::config_operations::apply_tts_config, context::Context};
use rigela_utils::resample::resample_audio;
use std::{
    collections::HashMap,
    io::SeekFrom,
    sync::{Arc, OnceLock},
};
use tokio::{
    io::{AsyncReadExt, AsyncSeekExt},
    sync::Mutex,
};
use win_wrap::{audio::AudioOutputStream, tts::Tts};

const SAMPLE_RATE: u32 = 16000;
const NUM_CHANNELS: u32 = 1;
const CHUNK_SIZE: usize = 3200;

/**
 * 表演者语音信息收集接口。
 * 实现此接口的对象可以调用表演者的speak方法进行输出。
 * */
pub(crate) trait Speakable {
    fn get_sentence(&self) -> String;
}

/**
 * 表演者对象结构。
 * 可以进行语音输出或音效提示。
 * */
#[derive(Debug)]
pub(crate) struct Performer {
    context: OnceLock<Arc<Context>>,
    pub(crate) tts: Tts,
    sound_table: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    output_stream: Arc<AudioOutputStream>,
}

impl Performer {
    /**
     * 创建表演者对象。
     * */
    pub(crate) fn new() -> Self {
        let output_stream = AudioOutputStream::new(SAMPLE_RATE, NUM_CHANNELS);
        let tts = Tts::new();
        Self {
            context: OnceLock::new(),
            tts,
            sound_table: Arc::new(HashMap::new().into()),
            output_stream: output_stream.into(),
        }
    }

    /**
     * 使用SAPI5语音输出，播报对象的信息。
     * `speakable` 实现了Speakable特征的对象。
     * */
    pub(crate) async fn speak_with_sapi5(&self, speakable: impl Speakable) {
        let str = speakable.get_sentence();
        self.tts.speak(str.as_str()).await;
    }

    //noinspection SpellCheckingInspection
    /**
     * 使用VVTTS语音输出，播报对象的信息。
     * `speakable` 实现了Speakable特征的对象。
     * */
    pub(crate) async fn speak_with_vvtts(&self, speakable: impl Speakable) {
        let ctx = self.context.get();
        if ctx.is_none() {
            return;
        }
        let str = speakable.get_sentence();
        let data = ctx.unwrap().proxy32.eci_synth(str.as_str()).await;
        self.output_stream.flush();
        self.output_stream.stop();
        self.output_stream.start();

        let data = resample_audio(data, 11025, SAMPLE_RATE as usize).await;

        self.output_stream.write(&data).await;
    }

    /**
     * 播放一个音效。
     * 目前仅支持16位深16K采样率单通道的音频。
     * */
    pub(crate) async fn play_sound(&self, res_name: &str) {
        let lock = self.sound_table.lock().await;
        let data = lock.get(res_name).unwrap().clone();
        drop(lock);

        self.output_stream.flush();
        self.output_stream.stop();
        self.output_stream.start();

        let len = data.len();
        for i in (0..len).step_by(CHUNK_SIZE) {
            if i + CHUNK_SIZE >= len {
                self.output_stream.write(&data[i..len]).await;
                break;
            }
            self.output_stream.write(&data[i..i + CHUNK_SIZE]).await;
        }
    }

    /**
     * 配置表演者。
     * `context` 上下文环境。
     * */
    pub(crate) async fn apply(&self, context: Arc<Context>) {
        self.context.set(context.clone()).unwrap_or(());
        // 初始化TTS属性
        apply_tts_config(context.clone(), 0).await;

        // 初始化音效播放器
        let list = vec!["boundary.wav"];

        for i in &list {
            let mut data = Vec::<u8>::new();
            let mut file = context.resource_accessor.open(i).await.unwrap();
            file.seek(SeekFrom::Start(44)).await.unwrap();
            file.read_to_end(&mut data).await.unwrap();
            self.sound_table.lock().await.insert(i.to_string(), data);
        }
    }
}
