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

#[async_trait::async_trait]
impl Ttsable for Vvtts {
    fn set_context(&self, context: Arc<Context>) {
        self.context.set(context.clone()).unwrap();
    }

    async fn speak(&self, text: &str) {
        let context = self.context.get().unwrap().clone();
        let stream = context.performer.output_stream.clone();
        stream.stop();
        stream.start();

        let text = text.to_string();
        let data = context.proxy32.eci_synth(text.as_str()).await;
        let data = resample_audio(data, 11025, SAMPLE_RATE as usize).await;
        stream.put_data(&data);
    }

    fn stop(&self) {
        todo!()
    }

    fn get_name(&self) -> String {
        "vvtts".to_string()
    }

    async fn get_all_voices(&self) -> Vec<String> {
        let ctx = self.context.get().unwrap();

        let voices = ctx.proxy32.eci_get_voices().await;
        voices.iter().map(|i| i.1.clone()).collect()
    }

    async fn set_value_by_prop(&self, prop: TtsProperty, value: i32) {
        let ctx = self.context.get().unwrap();
        // let list = ctx.proxy32.eci_get_voices().await;

        let mut params = ctx.proxy32.eci_get_voice_params().await;
        match prop {
            TtsProperty::Speed => params.speed = value,
            TtsProperty::Volume => params.volume = value,
            TtsProperty::Pitch => params.pitch_baseline = value,
            TtsProperty::Voice => {
                // let (_id, _) = list.get(value as usize).unwrap();
                // ctx.proxy32.eci_set_voice(_id.clone()).await;
                // return;
            }
        };

        ctx.proxy32.eci_set_voice_params(&params).await;
    }
}

unsafe impl Send for Vvtts {}
unsafe impl Sync for Vvtts {}
