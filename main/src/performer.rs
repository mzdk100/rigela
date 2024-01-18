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

use crate::configs::tts::TtsProperty;
use crate::context::Context;
use std::sync::Arc;
use tokio::sync::RwLock;
use win_wrap::tts::Tts;

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
    pub(crate)  cur_tts_prop: RwLock<TtsProperty>,
}

impl Performer {
    /**
     * 创建表演者对象。
     * */
    pub(crate) fn new() -> Self {
        let tts = Tts::new();
        Self {
            tts: tts.into(),
            cur_tts_prop: RwLock::new(TtsProperty::Speed),
        }
    }

    /**
     * 设置表演者的参数。
     * `context` 框架的上下文环境。
     * `slot` 一个用于修改参数的函数或闭包。
     * */
    pub(crate) async fn apply_config<FN>(&self, context: Arc<Context>, slot: FN)
    where
        FN: (FnOnce() -> i32) + Send + Sync + 'static,
    {
        let tts = self.tts.clone();
        let mut config = context.config_manager.read().await;
        let mut tts_config = config.tts_config.clone().unwrap();

        let diff = slot();
        if diff != 0 {
            let mut value = match *self.cur_tts_prop.read().await {
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

            match *self.cur_tts_prop.read().await {
                TtsProperty::Speed => tts_config.speed.replace(value),
                TtsProperty::Volume => tts_config.volume.replace(value),
                TtsProperty::Pitch => tts_config.pitch.replace(value),
            };
        }
        tts.set_prop(
            2.0 + (tts_config.speed.unwrap_or(50) as f64 - 50.0) * 0.02,
            0.5 + (tts_config.volume.unwrap_or(100) as f64 - 50.0) * 0.01,
            0.5 + (tts_config.pitch.unwrap_or(50) as f64 - 50.0) * 0.01,
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
        *self.cur_tts_prop.write().await = self.cur_tts_prop.read().await.next();
    }

    pub(crate) async fn prev_tts_prop(&self) {
        *self.cur_tts_prop.write().await = self.cur_tts_prop.read().await.prev();
    }
}
