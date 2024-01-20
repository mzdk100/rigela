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

use crate::{configs::tts::TtsProperty, context::Context};
use std::{collections::HashMap, io::SeekFrom, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncSeekExt},
    sync::{Mutex, RwLock},
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
pub(crate) struct Performer {
    tts: Arc<Tts>,
    pub(crate) cur_tts_prop: RwLock<TtsProperty>,
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
            tts: tts.into(),
            cur_tts_prop: RwLock::new(TtsProperty::Speed),
            sound_table: Arc::new(HashMap::new().into()),
            output_stream: output_stream.into(),
        }
    }

    /**
     * 设置表演者的参数。
     * `context` 框架的上下文环境。
     * `slot` 一个用于修改参数的函数或闭包。
     * */
    pub(crate) async fn apply_config(&self, context: Arc<Context>, diff: i32) {
        let tts = self.tts.clone();
        let mut config = context.config_manager.read().await;
        let mut tts_config = config.tts_config.clone().unwrap();

        if diff != 0 {
            let prop = self.cur_tts_prop.read().await;

            let mut value = match *prop {
                TtsProperty::Speed => tts_config.speed.unwrap(),
                TtsProperty::Volume => tts_config.volume.unwrap(),
                TtsProperty::Pitch => tts_config.pitch.unwrap(),
            };

            value = value + diff;

            let value = match value {
                i if i > 100 => 100,
                i if i < 0 => 0,
                i => i,
            };

            match *prop {
                TtsProperty::Speed => tts_config.speed.replace(value),
                TtsProperty::Volume => tts_config.volume.replace(value),
                TtsProperty::Pitch => tts_config.pitch.replace(value),
            };
        }

        tts.set_prop(
            3.0 + (tts_config.speed.unwrap() as f64 - 50.0) * 0.06,
            0.5 + (tts_config.volume.unwrap() as f64 - 50.0) * 0.01,
            1.0 + (tts_config.pitch.unwrap() as f64 - 50.0) * 0.01,
        );

        config.tts_config.replace(tts_config);
        context.config_manager.write(&config).await;
    }

    /**
     * 使用语音输出，播报对象的信息。
     * */
    pub(crate) async fn speak(&self, speakable: &(dyn Speakable + Sync)) {
        let str = speakable.get_sentence();
        self.tts.speak(str.as_str()).await;
    }

    /// 简单朗读文本
    pub(crate) async fn speak_text(&self, text: &str) {
        self.tts.speak(text).await;
    }

    pub(crate) async fn next_tts_prop(&self) {
        let prop = self.cur_tts_prop.read().await.next();
        let mut cur = self.cur_tts_prop.write().await;
        *cur = prop;
    }

    pub(crate) async fn prev_tts_prop(&self) {
        let prop = self.cur_tts_prop.read().await.prev();
        let mut cur = self.cur_tts_prop.write().await;
        *cur = prop;
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
        // 读取配置项，应用配置到程序实例
        self.apply_config(context.clone(), 0).await;

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
