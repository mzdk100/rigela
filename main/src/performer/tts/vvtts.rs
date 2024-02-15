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

use crate::{
    context::Context,
    performer::tts::{TtsEngine, TtsProperty},
};
use rigela_utils::bass::BassChannelOutputStream;
use std::{str::FromStr, sync::Arc};

//noinspection SpellCheckingInspection
/// VVTTS语音库封装
pub(crate) struct VvttsEngine {
    context: Arc<Context>,
    output_stream: BassChannelOutputStream,
}

impl VvttsEngine {
    pub(crate) fn new(context: Arc<Context>) -> Self {
        Self {
            context,
            output_stream: BassChannelOutputStream::new(11025, 1),
        }
    }

    async fn set_value_by_prop(&self, prop: TtsProperty) {
        let mut params = self
            .context
            .proxy32
            .as_ref()
            .await
            .eci_get_voice_params()
            .await;
        match prop {
            TtsProperty::Speed(v) => params.speed = (v as f32 * 2.5) as i32,
            TtsProperty::Volume(v) => params.volume = v,
            TtsProperty::Pitch(v) => {
                params.pitch_baseline = if v > 50 {
                    (69f32 + (v as f32 - 50f32) / 50f32 * 31f32) as i32
                } else {
                    ((v as f32) / 50f32 * 69f32) as i32
                }
            }
            _ => return,
        };

        self.context
            .proxy32
            .as_ref()
            .await
            .eci_set_voice_params(&params)
            .await;
    }
}

#[async_trait::async_trait]
impl TtsEngine for VvttsEngine {
    async fn speak(&self, text: &str) {
        self.output_stream.start();

        let text = text.to_string();
        let data = self
            .context
            .proxy32
            .as_ref()
            .await
            .eci_synth(text.as_str())
            .await;
        self.output_stream.put_data(&data);
    }

    async fn wait(&self) {
        self.output_stream.wait_until_stalled().await;
    }

    fn stop(&self) {
        self.output_stream.stop()
    }

    //noinspection SpellCheckingInspection
    fn get_name(&self) -> String {
        "Vvtts".to_string()
    }

    async fn get_all_voices(&self) -> Vec<(String, String)> {
        self.context
            .proxy32
            .as_ref()
            .await
            .eci_get_voices()
            .await
            .iter()
            .map(|i| (i.0.to_string(), i.1.clone()))
            .collect()
    }

    async fn set_speed(&self, value: i32) {
        self.set_value_by_prop(TtsProperty::Speed(value)).await
    }

    async fn set_volume(&self, value: i32) {
        self.set_value_by_prop(TtsProperty::Volume(value)).await
    }

    async fn set_pitch(&self, value: i32) {
        self.set_value_by_prop(TtsProperty::Pitch(value)).await
    }

    async fn set_voice(&self, id: String) {
        self.context
            .proxy32
            .as_ref()
            .await
            .eci_set_voice(u32::from_str(id.as_str()).unwrap_or(0))
            .await
    }
}

unsafe impl Send for VvttsEngine {}
unsafe impl Sync for VvttsEngine {}
