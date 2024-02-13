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
pub(crate) mod ttsable;
pub(crate) mod vvtts;

use super::tts::ttsable::{Direction, TtsProperty, Ttsable, ValueChange};
use crate::configs::config_operations::update_tts_config;
use crate::configs::tts::TtsConfig;
use crate::context::Context;
use crate::performer::tts::sapi5::Sapi5;
use crate::performer::tts::vvtts::Vvtts;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::DerefMut;
use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;

type TtsAble = Arc<dyn Ttsable + Send + Sync + 'static>;

///  语音TTS的抽象实现
#[allow(unused)]
pub(crate) struct Tts {
    all_tts: Mutex<Vec<TtsAble>>,
    all_voices: Mutex<HashMap<usize, (usize, usize, String)>>,
    tts_index: Mutex<i32>,
    voice_index: Mutex<usize>,
    tts: Mutex<TtsAble>,
    context: OnceLock<Arc<Context>>,
    cur_prop: Mutex<TtsProperty>,
}

impl Tts {
    /// 构建一个Tts实例
    pub(crate) async fn build(context: Arc<Context>) -> Self {
        // 必须有一个默认的语音库
        let sapi5 = Sapi5::default();
        sapi5.set_context(context.clone());
        let ttsable: TtsAble = Arc::new(sapi5);
        let tts = ttsable.clone().into();

        let _self = Self {
            all_tts: vec![ttsable].into(),
            all_voices: HashMap::new().into(),
            tts_index: 0.into(),
            voice_index: 0.into(),
            tts,
            context: OnceLock::new(),
            cur_prop: TtsProperty::Speed.into(),
        };
        _self.context.set(context.clone()).unwrap();

        //  在这里添加所有可用的语音库
        _self.add_ttsable(Arc::new(Vvtts::default())).await;

        _self.init_voices().await;
        let cfg = context.config_manager.get_config().tts_config.clone();
        _self.apply_config(&cfg).await;

        _self
    }

    /// 朗读
    pub(crate) async fn speak<S: Into<String>>(&self, text: S) {
        let text = text.into();
        self.tts.lock().await.speak(text.as_str()).await;
    }

    /// 移动当前操作的TTS属性
    pub(crate) async fn move_tts_prop(&self, direction: Direction) {
        *self.cur_prop.lock().await.deref_mut() = match direction {
            Direction::Next => self.cur_prop.lock().await.next(),
            Direction::Prev => self.cur_prop.lock().await.prev(),
        };
    }

    /// 获取当前TTS操作属性
    pub(crate) async fn get_cur_prop(&self) -> TtsProperty {
        self.cur_prop.lock().await.clone()
    }

    /// 设置当前TTS属性的值
    pub(crate) async fn set_tts_prop_value(&self, value_change: ValueChange) {
        let set_val = |x| match value_change {
            ValueChange::Increment => x + 1,
            ValueChange::Decrement => x - 1,
        };

        let ctx = self.context.get().unwrap();
        let config = ctx.config_manager.get_config().tts_config.clone();

        let cur_val = match self.get_cur_prop().await {
            TtsProperty::Speed => config.speed,
            TtsProperty::Pitch => config.pitch,
            TtsProperty::Volume => config.volume,
            TtsProperty::Voice => config.voice_index,
        };

        update_tts_config(ctx.clone(), self.get_cur_prop().await, set_val(cur_val)).await;
        self.apply_config(&ctx.config_manager.get_config().tts_config.clone())
            .await;
    }

    /// 获取当前TTS属性值, 暂不使用，直接从配置获取
    #[allow(unused)]
    pub(crate) fn get_tts_prop_value(&self) -> i32 {
        todo!()
    }

    // 添加语音库
    async fn add_ttsable(&self, tts: TtsAble) {
        tts.set_context(self.context.get().unwrap().clone());
        self.all_tts.lock().await.push(tts);
    }

    // 初始化所有语音角色
    async fn init_voices(&self) {
        let mut index = 0;
        self.all_voices.lock().await.clear();

        for (i, tts) in self.all_tts.lock().await.iter().enumerate() {
            for (j, v) in tts.get_all_voices().await.iter().enumerate() {
                self.all_voices
                    .lock()
                    .await
                    .insert(index, (i, j, v.to_string()));
                index += 1;
            }
        }
    }

    // 应用配置到TTS
    async fn apply_config(&self, config: &TtsConfig) {
        for tts in self.all_tts.lock().await.iter() {
            tts.set_value_by_prop(TtsProperty::Speed, config.speed)
                .await;
            tts.set_value_by_prop(TtsProperty::Pitch, config.pitch)
                .await;
            tts.set_value_by_prop(TtsProperty::Volume, config.volume)
                .await;
            tts.set_value_by_prop(TtsProperty::Voice, 0).await;
        }
        // *self.tts_index.lock().await.deref_mut() = config.voice_index;
    }

    async fn set_voice(&self) {}

    async fn next_voice(&self) -> usize {
        let index = self.voice_index.lock().await.clone();
        let max_index = self.all_voices.lock().await.len().clone() - 1;
        if index + 1 > max_index {
            0
        } else {
            index + 1
        }
    }

    async fn prev_voice(&self) -> usize {
        let index = self.voice_index.lock().await.clone();
        let max_index = self.all_voices.lock().await.len().clone() - 1;
        if index as i32 - 1 < 0 {
            max_index
        } else {
            index - 1
        }
    }
}

impl Debug for Tts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tts").field("tts", &"Tts").finish()
    }
}

unsafe impl Send for Tts {}
unsafe impl Sync for Tts {}
