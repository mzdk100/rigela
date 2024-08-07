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

use crate::{
    configs::items::tts::{TtsConfig, TtsPropertyItem},
    context::{Context, ContextAccessor},
    performer::text_processing::transform_single_char,
};
use arc_swap::ArcSwapAny;
use parking_lot::RwLock;
use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Weak,
    },
    time::Duration,
};
use tokio::{sync::OnceCell, time::sleep};

#[derive(Debug, Clone, Default)]
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
    default_engine: OnceCell<String>,
    is_cancelled: AtomicBool,
    all_engines: RwLock<HashMap<String, Arc<dyn TtsEngine + Sync + Send>>>,
    all_voices: ArcSwapAny<Arc<Vec<VoiceInfo>>>,
    context: Weak<Context>,
}

impl Tts {
    /// 构建一个Tts实例
    pub(crate) fn new(context: Weak<Context>) -> Self {
        Self {
            default_engine: OnceCell::new(),
            is_cancelled: false.into(),
            all_engines: HashMap::new().into(),
            all_voices: ArcSwapAny::from(Arc::new(Vec::new())),
            context,
        }
    }

    //noinspection StructuralWrap
    /**
    朗读文字，如果当前有朗读的任务，则进行排队。
    本方法会等待朗读完毕，如果朗读成功，则返回true；如果中途通过stop函数停止，或者朗读失败，则返回false。
    `text` 需要朗读的文本。
    */
    pub(crate) async fn speak(&self, text: String) -> bool {
        assert!(text.len() > 0);

        if let Some(engine) = self.get_engine().await.upgrade() {
            // 这里Chars是迭代器，没有计算，不损耗性能，确保text不为空,unwrap不会失败
            let mut chars = text.chars();
            let first_char = chars.next().unwrap();
            match chars.next() {
                Some(_) => engine.speak(&text).await,

                None => {
                    let text = transform_single_char(&first_char);
                    engine.speak(&text).await;
                }
            }

            {
                self.is_cancelled.store(false, Ordering::Release);
            }

            engine.wait().await;
        }

        return !self.is_cancelled.load(Ordering::Acquire);
    }

    async fn get_engine(&self) -> Weak<dyn TtsEngine + Sync + Send> {
        let ttc_cfg = self.context.get_config_manager().get_config().tts_config;
        let engine_name = ttc_cfg.voice.0.clone();

        match { self.all_engines.read().get(&engine_name) } {
            Some(x) => Arc::downgrade(x),

            None => loop {
                let Some(engine_name) = self.default_engine.get() else {
                    sleep(Duration::from_millis(100)).await;
                    continue;
                };
                match { self.all_engines.read().get(engine_name) } {
                    Some(x) => break Arc::downgrade(x),
                    None => sleep(Duration::from_millis(100)).await,
                }
            },
        }
    }

    /**
    停止当前的朗读任务。
    */
    pub(crate) async fn stop(&self) {
        self.is_cancelled.store(true, Ordering::Release);
        let engine = self
            .context
            .get_config_manager()
            .get_config()
            .tts_config
            .voice
            .0
            .clone();

        let lock = self.all_engines.read();
        if let Some(x) = lock.get(&engine) {
            x.stop();
            return;
        };
        if let Some(default_engine) = self.default_engine.get() {
            if let Some(x) = lock.get(default_engine) {
                x.stop();
            };
        }
    }

    /**
    停止所有语音引擎的朗读。
    */
    pub(crate) async fn stop_all(&self) {
        self.is_cancelled.store(true, Ordering::Release);
        self.all_engines.read().iter().for_each(|(_, engine)| {
            engine.stop();
        });
    }

    /// 设置当前TTS属性的值
    pub(crate) async fn set_tts_prop_value(&self, value_change: ValueChange) {
        let set_val = |x| match value_change {
            ValueChange::Increment if x < 99 => x + 1,
            ValueChange::Increment => 100,
            ValueChange::Decrement if x > 2 => x - 1,
            ValueChange::Decrement => 1,
        };

        let TtsConfig {
            speed,
            pitch,
            volume,
            voice: (engine, id),
            item,
        } = self
            .context
            .get_config_manager()
            .get_config()
            .tts_config
            .clone();

        let mut cfg = self
            .context
            .get_config_manager()
            .get_config()
            .tts_config
            .clone();
        match item {
            TtsPropertyItem::Speed => cfg.speed = set_val(speed),
            TtsPropertyItem::Pitch => cfg.pitch = set_val(pitch),
            TtsPropertyItem::Volume => cfg.volume = set_val(volume),
            TtsPropertyItem::Voice => {
                self.stop_all().await;
                let voice = self.switch_voice(engine, id, value_change).await;
                cfg.voice = (voice.engine, voice.id)
            }
        };

        self.apply_config(&cfg).await;

        let mut root = self.context.get_config_manager().get_config();
        root.tts_config = cfg;
        self.context.get_config_manager().set_config(&root);
    }

    /// 获取当前TTS属性值
    pub(crate) async fn get_tts_prop_value(&self, item: Option<TtsPropertyItem>) -> TtsProperty {
        let config = self
            .context
            .get_config_manager()
            .get_config()
            .tts_config
            .clone();

        match item.map(|x| x).unwrap_or(config.item) {
            TtsPropertyItem::Speed => TtsProperty::Speed(config.speed),
            TtsPropertyItem::Pitch => TtsProperty::Pitch(config.pitch),
            TtsPropertyItem::Volume => TtsProperty::Volume(config.volume),

            TtsPropertyItem::Voice => {
                let (engine, id) = config.voice;
                let all_voices = self.all_voices.load();
                match all_voices.iter().find(|v| v.engine == engine && v.id == id) {
                    Some(v) => TtsProperty::Voice(v.clone()),
                    None => TtsProperty::Voice(all_voices.first().unwrap().clone()),
                }
            }
        }
    }

    /**
    设置默认引擎。
    `engine` 实现了TtsEngine特征的语音引擎对象。
    */
    pub(crate) async fn put_default_engine<T>(&self, engine: T) -> &Self
    where
        T: TtsEngine + Sync + Send + 'static,
    {
        self.default_engine
            .get_or_init(|| async { engine.get_name() })
            .await;
        self.add_engine(engine).await;
        self
    }

    /**
    增加一个引擎。
    `engine` 实现了TtsEngine特征的语音引擎对象。
    */
    pub(crate) async fn add_engine<T>(&self, engine: T) -> &Self
    where
        T: TtsEngine + Sync + Send + 'static,
    {
        let mut all_voices = self.all_voices.load().as_ref().clone();
        for (id, name) in engine.get_all_voices().await.iter() {
            all_voices.push(VoiceInfo {
                engine: engine.get_name(),
                id: id.clone(),
                name: name.clone(),
            });
        }
        self.all_voices.store(Arc::new(all_voices));

        {
            self.all_engines
                .write()
                .insert(engine.get_name(), Arc::new(engine));
        }
        let cfg = self
            .context
            .get_config_manager()
            .get_config()
            .tts_config
            .clone();
        self.apply_config(&cfg).await;

        self
    }

    // 应用配置到TTS
    pub(crate) async fn apply_config(&self, config: &TtsConfig) {
        for (_, engine) in { self.all_engines.read().clone() }.iter() {
            let (engine_name, id) = config.voice.clone();
            if engine.get_name() == engine_name {
                engine.set_voice(id).await;
            }

            engine.set_speed(config.speed).await;
            engine.set_volume(config.volume).await;
            engine.set_pitch(config.pitch).await;
        }
    }

    pub(crate) async fn move_tts_prop(&self, direction: Direction) {
        let mut cfg = self
            .context
            .get_config_manager()
            .get_config()
            .tts_config
            .clone();
        cfg.item = match direction {
            Direction::Next => match cfg.item {
                TtsPropertyItem::Speed => TtsPropertyItem::Pitch,
                TtsPropertyItem::Pitch => TtsPropertyItem::Volume,
                TtsPropertyItem::Volume => TtsPropertyItem::Voice,
                TtsPropertyItem::Voice => TtsPropertyItem::Speed,
            },
            Direction::Prev => match cfg.item {
                TtsPropertyItem::Speed => TtsPropertyItem::Voice,
                TtsPropertyItem::Pitch => TtsPropertyItem::Speed,
                TtsPropertyItem::Volume => TtsPropertyItem::Pitch,
                TtsPropertyItem::Voice => TtsPropertyItem::Volume,
            },
        };

        self.apply_config(&cfg).await;

        let mut root = self.context.get_config_manager().get_config();
        root.tts_config = cfg;
        self.context.get_config_manager().set_config(&root);
    }

    async fn switch_voice(
        &self,
        engine: String,
        id: String,
        value_change: ValueChange,
    ) -> VoiceInfo {
        let all_voices = self.all_voices.load();
        let mut voices = vec![];
        for v in all_voices.iter() {
            voices.push(v.clone());
        }
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
        self.all_voices.load().as_ref().clone()
    }
}

impl Debug for Tts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tts").field("tts", &"Tts").finish()
    }
}

unsafe impl Send for Tts {}

unsafe impl Sync for Tts {}
