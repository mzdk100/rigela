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

use crate::configs::mouse::MouseConfig;
use crate::configs::tts::TtsConfig;
use crate::{configs::tts::TtsProperty, context::Context};
use std::sync::OnceLock;
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
#[derive(Debug)]
pub(crate) struct Performer {
    tts: Tts,
    pub(crate) cur_tts_prop: RwLock<TtsProperty>,
    sound_table: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    output_stream: Arc<AudioOutputStream>,
    context: OnceLock<Arc<Context>>,
}

impl Performer {
    /**
     * 创建表演者对象。
     * */
    pub(crate) fn new() -> Self {
        let output_stream = AudioOutputStream::new(SAMPLE_RATE, NUM_CHANNELS);
        let tts = Tts::new();
        Self {
            tts,
            cur_tts_prop: RwLock::new(TtsProperty::Speed),
            sound_table: Arc::new(HashMap::new().into()),
            output_stream: output_stream.into(),
            context: OnceLock::new(),
        }
    }

    /**
     * 设置表演者TTS的参数。
     * `context` 框架的上下文环境。
     * `diff` 属性值的差值， 传0初始化tts属性值
     * */
    pub(crate) async fn apply_tts_config(&self, context: Arc<Context>, diff: i32) {
        let tts = &self.tts;
        let mut config = context.config_manager.get_config().await;
        let mut tts_config = config.tts_config.clone();

        // 如果差值等于0，直接设置TTS属性值参数，返回
        if diff == 0 {
            tts.set_prop(
                3.0 + (tts_config.speed as f64 - 50.0) * 0.06,
                0.5 + (tts_config.volume as f64 - 50.0) * 0.01,
                1.0 + (tts_config.pitch as f64 - 50.0) * 0.01,
            );
            return;
        }

        let restrict = |x| match x {
            i if i > 100 => 100,
            i if i < 0 => 0,
            i => i,
        };

        tts_config = match self.get_cur_tts_prop().await {
            TtsProperty::Speed => TtsConfig {
                speed: restrict(tts_config.speed + diff),
                ..tts_config
            },
            TtsProperty::Volume => TtsConfig {
                volume: restrict(tts_config.volume + diff),
                ..tts_config
            },
            TtsProperty::Pitch => TtsConfig {
                pitch: restrict(tts_config.pitch + diff),
                ..tts_config
            },
        };

        tts.set_prop(
            3.0 + (tts_config.speed as f64 - 50.0) * 0.06,
            0.5 + (tts_config.volume as f64 - 50.0) * 0.01,
            1.0 + (tts_config.pitch as f64 - 50.0) * 0.01,
        );

        config.tts_config = tts_config;
        context.config_manager.set_config(config).await;
    }

    /// 设置是否开启朗读鼠标
    pub(crate) async fn apply_mouse_config(&self, context: Arc<Context>, is_read: bool) {
        let mut config = context.config_manager.get_config().await;
        config.mouse_config = MouseConfig { is_read };
        context.config_manager.set_config(config).await;
    }

    /**
     * 使用SAPI5语音输出，播报对象的信息。
     * `speakable` 实现了Speakable特征的对象。
     * */
    pub(crate) async fn speak_with_sapi5(&self, speakable: &(dyn Speakable + Sync)) {
        let str = speakable.get_sentence();
        self.tts.speak(str.as_str()).await;
    }

    /**
     * 使用VVTTS语音输出，播报对象的信息。
     * `speakable` 实现了Speakable特征的对象。
     * */
    pub(crate) async fn speak_with_vvtts(&self, speakable: &(dyn Speakable + Sync)) {
        let ctx = self.context.get();
        if ctx.is_none() {
            return;
        }
        let str = speakable.get_sentence();
        let data = ctx.unwrap().proxy32.eci_synth(str.as_str()).await;
        self.output_stream.write(&data).await;
    }

    /// 获取当前调节的TTS属性
    pub(crate) async fn get_cur_tts_prop(&self) -> TtsProperty {
        self.cur_tts_prop.read().await.clone()
    }

    /// 后移当前需调节的TTS属性
    pub(crate) async fn next_tts_prop(&self) {
        let prop = self.cur_tts_prop.read().await.next();
        let mut cur = self.cur_tts_prop.write().await;
        *cur = prop;
    }

    /// 前移当前需调节的TTS属性
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
        self.context.set(context.clone()).unwrap();
        // 读取配置项，应用配置到程序实例
        self.apply_tts_config(context.clone(), 0).await;

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
