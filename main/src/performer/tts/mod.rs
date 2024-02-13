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

use super::tts::ttsable::{TtsProperty, Ttsable};
use crate::configs::tts::TtsConfig;
use crate::context::Context;
use crate::performer::tts::sapi5::Sapi5;
use crate::performer::tts::vvtts::Vvtts;
use std::fmt::{Debug, Formatter};
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::sync::Mutex;

type TtsAble = Arc<dyn Ttsable + Send + Sync + 'static>;

///  语音TTS的抽象实现
pub(crate) struct Tts {
    all_tts: Mutex<Vec<TtsAble>>,
    all_voices: Mutex<Vec<(usize, String)>>,
    tts_index: Mutex<i32>,
    voice_index: Mutex<usize>,
    tts: Mutex<TtsAble>,
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
            all_voices: vec![].into(),
            tts_index: 0.into(),
            voice_index: 0.into(),
            tts,
        };

        //  在这里添加所有可用的语音库
        _self
            .add_ttsable(context.clone(), Arc::new(Vvtts::default()))
            .await;

        _self.init_voices().await;
        _self.apply_config(&context.config_manager.get_config().tts_config);

        _self
    }

    pub(crate) async fn speak<S: Into<String>>(&self, text: S) {
        let text = text.into();
        self.tts.lock().await.speak(text.as_str()).await;
    }

    pub(crate) async fn apply_config(&self, config: &TtsConfig) {
        for tts in self.all_tts.lock().await.iter() {
            tts.set_value_by_prop(TtsProperty::Speed, config.speed);
            tts.set_value_by_prop(TtsProperty::Pitch, config.pitch);
            tts.set_value_by_prop(TtsProperty::Volume, config.volume);
            tts.set_value_by_prop(TtsProperty::Voice, 0);
        }
        *self.tts_index.lock().await.deref_mut() = config.voice_index;
    }

    // 添加语音库
    async fn add_ttsable(&self, context: Arc<Context>, tts: TtsAble) {
        tts.set_context(context.clone());
        self.all_tts.lock().await.push(tts);
    }

    // 初始化所有语音角色
    async fn init_voices(&self) {
        self.all_voices.lock().await.clear();

        for (i, tts) in self.all_tts.lock().await.iter().enumerate() {
            for v in tts.get_all_voices().await {
                self.all_voices.lock().await.push((i, v));
            }
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
