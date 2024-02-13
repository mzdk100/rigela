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
use crate::performer::tts::ttsable::{TtsProperty, Ttsable};
use crate::performer::SAMPLE_RATE;
use rigela_utils::resample::resample_audio;
use std::str::FromStr;
use std::sync::{Arc, OnceLock};

/// VVTTS语音库封装
pub(crate) struct Vvtts {
    context: OnceLock<Arc<Context>>,
}

impl Default for Vvtts {
    fn default() -> Self {
        Self {
            context: OnceLock::new(),
        }
    }
}

impl Vvtts {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// 使用VVTTS语音朗读
    pub(crate) async fn _speak(&self, text: &str) {
        let context = self.context.get().unwrap().clone();
        let stream = context.performer.output_stream.clone();
        stream.stop();
        stream.start();

        let text = text.to_string();
        let data = context.proxy32.eci_synth(text.as_str()).await;
        let data = resample_audio(data, 11025, SAMPLE_RATE as usize).await;
        stream.put_data(&data);
    }

    /**
     * 设置vvtts语音合成器的参数。
     * `speed` 速度，0到100。
     * `volume` 音量，0到100。
     * `pitch` 音高，0到100。
     * */
    #[allow(unused)]
    pub(crate) fn set_tts_properties_with_vvtts(&self, speed: i32, volume: i32, pitch: i32) {
        let context = self.context.get().unwrap();
        let ctx = context.clone();

        context.main_handler.spawn(async move {
            let mut params = ctx.proxy32.eci_get_voice_params().await;
            params.volume = volume;
            params.speed = speed;
            params.pitch_baseline = pitch; // vvtts的默认音高是69
                                           // params.pitch_fluctuation = pitch;
            ctx.proxy32.eci_set_voice_params(&params).await;
        });
    }

    /**
     * 获取vvtts发音人列表。
     * 返回的每一个元祖中第一个成员是发音人id，第二个成员是发音人名称。
     * */
    #[allow(unused)]
    pub(crate) async fn get_tts_voices_with_vvtts(&self) -> Vec<(String, String)> {
        let ctx = self.context.get().unwrap();

        let voices = ctx.proxy32.eci_get_voices().await;
        voices
            .iter()
            .map(|i| (i.0.to_string(), i.1.clone()))
            .collect()
    }

    /**
     * 设置vvtts的当前发音人。
     * `voice_id` 发音人id。
     * */
    #[allow(unused)]
    pub(crate) fn set_tts_voice_with_vvtts(&self, voice_id: String) {
        let context = self.context.get().unwrap();
        let ctx = context.clone();

        context.main_handler.spawn(async move {
            ctx.proxy32
                .eci_set_voice(u32::from_str(&voice_id).unwrap())
                .await
        });
    }
}

#[async_trait::async_trait]

impl Ttsable for Vvtts {
    fn set_context(&self, context: Arc<Context>) {
        self.context.clone().set(context);
    }

    async fn speak(&self, text: &str) {
        self._speak(text).await;
    }

    fn stop(&self) {
        todo!()
    }

    fn get_name(&self) -> String {
        todo!()
    }

    async fn get_all_voices(&self) -> Vec<String> {
        vec![]
    }

    async fn get_value_by_prop(&self, prop: TtsProperty) -> i32 {
        todo!()
    }

    async fn set_value_by_prop(&self, prop: TtsProperty, value: i32) {
        todo!()
    }
}

unsafe impl Send for Vvtts {}
unsafe impl Sync for Vvtts {}
