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

pub(crate) mod sapi5;
//noinspection SpellCheckingInspection
pub(crate) mod vvtts;

use crate::performer::text_processing::transform_single_char;
use crate::{
    configs::tts::{TtsConfig, TtsPropertyItem},
    context::Context,
};
use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
    sync::Arc,
    time::Duration,
};
use tokio::{sync::Mutex, time::sleep};

#[derive(Clone, Debug)]
pub(crate) struct VoiceInfo {
    pub(crate) engine: String,
    pub(crate) id: String,
    pub(crate) name: String,
}

/// TTS的属性枚举
#[derive(Debug, Clone)]
pub(crate) enum TtsProperty {
    Speed(i32),
    Voice(VoiceInfo),
    Pitch(i32),
    Volume(i32),
}

/// 语音TTS的抽象接口
#[async_trait::async_trait]
pub(crate) trait TtsEngine {
    async fn speak(&self, text: &str);
    async fn wait(&self);
    fn stop(&self);
    fn get_name(&self) -> String;
    async fn get_all_voices(&self) -> Vec<(String, String)>;
    async fn set_speed(&self, value: i32);
    async fn set_volume(&self, value: i32);
    async fn set_pitch(&self, value: i32);
    async fn set_voice(&self, id: String);
}

/// 移动TTS属性的方向
#[derive(Debug, Clone, Copy)]
pub(crate) enum Direction {
    Next,
    Prev,
}

/// 增减TTS属性的值
#[derive(Debug, Clone, Copy)]
pub(crate) enum ValueChange {
    Increment,
    Decrement,
}

///  语音TTS的抽象实现
pub(crate) struct Tts {
    default_engine: Mutex<Option<String>>,
    is_cancelled: Mutex<bool>,
    all_engines: Mutex<HashMap<String, Arc<dyn TtsEngine + Sync + Send>>>,
    all_voices: Mutex<Vec<VoiceInfo>>,
    context: Arc<Context>,
}

impl Tts {
    /// 构建一个Tts实例
    pub(crate) fn new(context: Arc<Context>) -> Self {
        Self {
            default_engine: None.into(),
            is_cancelled: false.into(),
            all_engines: HashMap::new().into(),
            all_voices: vec![].into(),
            context,
        }
    }

    //noinspection StructuralWrap
    /**
     * 朗读文字，如果当前有朗读的任务，则进行排队。
     * 本方法会等待朗读完毕，如果朗读成功，则返回true；如果中途通过stop函数停止，或者朗读失败，则返回false。
     * `text` 需要朗读的文本。
     * */
    pub(crate) async fn speak(&self, text: String) -> bool {
        let mut text = text.clone();

        // 单个字符的预处理，utf8的字节数一般在4个字节以内， 如果字节数小于5个字符，就进行预处理
        if text.len() < 5 {
            text = transform_single_char(&text);
        }

        let engine = self
            .context
            .config_manager
            .get_config()
            .tts_config
            .voice
            .0
            .clone();
        let lock = self.all_engines.lock().await;
        if let Some(x) = lock.get(&engine) {
            let engine = x.clone();
            drop(lock);
            engine.speak(text.as_str()).await;
            {
                *self.is_cancelled.lock().await = false;
            }
            engine.wait().await;
            return !*self.is_cancelled.lock().await;
        };
        drop(lock);
        loop {
            if let Some(default_engine) = { self.default_engine.lock().await.clone() } {
                let lock = self.all_engines.lock().await;
                if let Some(x) = lock.get(&default_engine) {
                    let engine = x.clone();
                    drop(lock);
                    engine.speak(text.as_str()).await;
                    {
                        *self.is_cancelled.lock().await = false;
                    }
                    engine.wait().await;
                    return !*self.is_cancelled.lock().await;
                };
                drop(lock);
            }
            sleep(Duration::from_millis(100)).await;
        }
    }

    /**
     * * 停止当前的朗读任务。
     * */
    pub(crate) async fn stop(&self) {
        {
            *self.is_cancelled.lock().await = true;
        }
        let engine = self
            .context
            .config_manager
            .get_config()
            .tts_config
            .voice
            .0
            .clone();
        let lock = self.all_engines.lock().await;
        if let Some(x) = lock.get(&engine) {
            x.stop();
            return;
        };
        if let Some(default_engine) = self.default_engine.lock().await.as_ref() {
            if let Some(x) = lock.get(default_engine) {
                x.stop();
            };
        }
    }

    /**
     * 停止所有语音引擎的朗读。
     * */
    pub(crate) async fn stop_all(&self) {
        {
            *self.is_cancelled.lock().await = true;
        }
        for x in self.all_engines.lock().await.iter() {
            x.1.stop();
        }
    }

    /// 设置当前TTS属性的值
    pub(crate) async fn set_tts_prop_value(&self, value_change: ValueChange) {
        let set_val = |x| match value_change {
            ValueChange::Increment => {
                if x < 99 {
                    x + 1
                } else {
                    100
                }
            }
            ValueChange::Decrement => {
                if x > 2 {
                    x - 1
                } else {
                    1
                }
            }
        };

        let mut root = self.context.config_manager.get_config();
        match root.tts_config.item {
            TtsPropertyItem::Speed => root.tts_config.speed = set_val(root.tts_config.speed),
            TtsPropertyItem::Pitch => root.tts_config.pitch = set_val(root.tts_config.pitch),
            TtsPropertyItem::Volume => root.tts_config.volume = set_val(root.tts_config.volume),
            TtsPropertyItem::Voice => {
                self.stop_all().await;
                let voice = self
                    .switch_voice(
                        root.tts_config.voice.0,
                        root.tts_config.voice.1,
                        value_change,
                    )
                    .await;
                root.tts_config.voice = (voice.engine, voice.id)
            }
        };

        self.apply_config(&root.tts_config).await;
        self.context.config_manager.set_config(&root);
    }

    /// 获取当前TTS属性值
    pub(crate) async fn get_tts_prop_value(&self, item: Option<TtsPropertyItem>) -> TtsProperty {
        let config = self.context.config_manager.get_config().tts_config.clone();
        match item.map(|x| x).unwrap_or(config.item) {
            TtsPropertyItem::Speed => TtsProperty::Speed(config.speed),
            TtsPropertyItem::Pitch => TtsProperty::Pitch(config.pitch),
            TtsPropertyItem::Volume => TtsProperty::Volume(config.volume),
            TtsPropertyItem::Voice => {
                let lock = self.all_voices.lock().await;
                if let Some(v) = lock
                    .iter()
                    .find(|i| i.engine == config.voice.0 && i.id == config.voice.1)
                {
                    TtsProperty::Voice(v.clone())
                } else {
                    TtsProperty::Voice(lock.first().unwrap().clone())
                }
            }
        }
    }

    /**
     * 设置默认引擎。
     * `engine` 实现了TtsEngine特征的语音引擎对象。
     * */
    pub(crate) async fn put_default_engine<T>(&self, engine: T) -> &Self
    where
        T: TtsEngine + Sync + Send + 'static,
    {
        self.default_engine.lock().await.replace(engine.get_name());
        self.add_engine(engine).await
    }

    /**
     * 增加一个引擎。
     * `engine` 实现了TtsEngine特征的语音引擎对象。
     * */
    pub(crate) async fn add_engine<T>(&self, engine: T) -> &Self
    where
        T: TtsEngine + Sync + Send + 'static,
    {
        for (id, name) in engine.get_all_voices().await.iter() {
            self.all_voices.lock().await.push(VoiceInfo {
                engine: engine.get_name(),
                id: id.clone(),
                name: name.clone(),
            });
        }
        self.all_engines
            .lock()
            .await
            .insert(engine.get_name(), Arc::new(engine));
        self.apply_config(&self.context.config_manager.get_config().tts_config)
            .await;
        self
    }

    // 应用配置到TTS
    pub(crate) async fn apply_config(&self, config: &TtsConfig) {
        for (_, tts) in self.all_engines.lock().await.iter() {
            if config.voice.0 == tts.get_name() {
                tts.set_voice(config.voice.1.clone()).await;
            }
            tts.set_speed(config.speed).await;
            tts.set_volume(config.volume).await;
            tts.set_pitch(config.pitch).await;
        }
    }

    pub(crate) async fn move_tts_prop(&self, direction: Direction) {
        let mut root = self.context.config_manager.get_config();
        root.tts_config.item = match direction {
            Direction::Next => match root.tts_config.item {
                TtsPropertyItem::Speed => TtsPropertyItem::Pitch,
                TtsPropertyItem::Pitch => TtsPropertyItem::Volume,
                TtsPropertyItem::Volume => TtsPropertyItem::Voice,
                TtsPropertyItem::Voice => TtsPropertyItem::Speed,
            },
            Direction::Prev => match root.tts_config.item {
                TtsPropertyItem::Speed => TtsPropertyItem::Voice,
                TtsPropertyItem::Pitch => TtsPropertyItem::Speed,
                TtsPropertyItem::Volume => TtsPropertyItem::Pitch,
                TtsPropertyItem::Voice => TtsPropertyItem::Volume,
            },
        };
        self.apply_config(&root.tts_config).await;
        self.context.config_manager.set_config(&root);
    }

    async fn switch_voice(
        &self,
        engine: String,
        id: String,
        value_change: ValueChange,
    ) -> VoiceInfo {
        let mut voices = { self.all_voices.lock().await.clone() };
        if let ValueChange::Decrement = &value_change {
            voices.reverse();
        }
        let mut iter = voices.iter();
        while let Some(i) = iter.next() {
            if i.engine == engine && i.id == id {
                if let Some(v) = iter.next() {
                    return v.clone();
                }
            }
        }
        voices.first().unwrap().clone()
    }

    pub(crate) async fn get_all_voiceinfo(&self) -> Vec<VoiceInfo> {
        self.all_voices.lock().await.clone()
    }
}

impl Debug for Tts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tts").field("tts", &"Tts").finish()
    }
}

unsafe impl Send for Tts {}

unsafe impl Sync for Tts {}
