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

use std::{str::FromStr, sync::Weak};

use rigela_utils::bass::BassChannelOutputStream;
#[cfg(target_arch = "x86")]
use rigela_utils::ibmeci::Ibmeci;

#[cfg(target_arch = "x86_64")]
use crate::context::ContextAccessor;
#[cfg(target_arch = "x86_64")]
use crate::performer::tts::TtsProperty;
use crate::{context::Context, performer::tts::TtsEngine};

//noinspection SpellCheckingInspection
/// VVTTS语音库封装
pub(crate) struct VvttsEngine {
    #[cfg(target_arch = "x86_64")]
    context: Weak<Context>,
    #[cfg(target_arch = "x86")]
    eci: &'static Ibmeci,
    output_stream: BassChannelOutputStream,
}

impl VvttsEngine {
    #[allow(unused_variables)]
    pub(crate) async fn new(context: Weak<Context>) -> Self {
        #[cfg(target_arch = "x86")]
        let eci = Ibmeci::get().await.unwrap();
        Self {
            #[cfg(target_arch = "x86_64")]
            context,
            #[cfg(target_arch = "x86")]
            eci,
            output_stream: BassChannelOutputStream::new(11025, 1),
        }
    }

    #[cfg(target_arch = "x86_64")]
    async fn set_value_by_prop(&self, prop: TtsProperty) {
        let proxy32 = self.context.get_proxy32process().await;
        let mut params = proxy32.eci_get_voice_params().await;

        match prop {
            TtsProperty::Speed(v) => params.speed = Self::convert_speed_param(v),
            TtsProperty::Volume(v) => params.volume = v,
            TtsProperty::Pitch(v) => {
                let pitch = Self::convert_pitch_param(v);
                params.pitch_baseline = pitch;
                params.pitch_fluctuation = pitch;
            }
            _ => return,
        };

        proxy32.eci_set_voice_params(&params).await;
    }

    fn convert_speed_param(value: i32) -> i32 {
        (value as f32 * 2.5) as i32
    }

    fn convert_pitch_param(value: i32) -> i32 {
        if value > 50 {
            (69f32 + (value as f32 - 50f32) / 50f32 * 31f32) as i32
        } else {
            ((value as f32) / 50f32 * 69f32) as i32
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[async_trait::async_trait]
impl TtsEngine for VvttsEngine {
    async fn speak(&self, text: &str) {
        self.output_stream.start();
        let data = self
            .context
            .get_proxy32process()
            .await
            .eci_synth(text)
            .await;
        self.output_stream.put_data(&data);
    }

    async fn wait(&self) {
        self.output_stream.wait_until_stopped_or_stalled().await;
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
            .get_proxy32process()
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
            .get_proxy32process()
            .await
            .eci_set_voice(u32::from_str(id.as_str()).unwrap_or(0))
            .await
    }
}

#[cfg(target_arch = "x86")]
#[allow(unused)]
#[async_trait::async_trait]
impl TtsEngine for VvttsEngine {
    async fn speak(&self, text: &str) {
        self.output_stream.start();
        let data = self.eci.synth(text).await;
        self.output_stream.put_data(&data);
    }

    async fn wait(&self) {
        self.output_stream.wait_until_stopped_or_stalled().await;
    }

    fn stop(&self) {
        self.output_stream.stop()
    }

    //noinspection SpellCheckingInspection
    fn get_name(&self) -> String {
        "Vvtts".to_string()
    }

    async fn get_all_voices(&self) -> Vec<(String, String)> {
        self.eci
            .get_voices()
            .iter()
            .map(|i| (i.0.to_string(), i.1.clone()))
            .collect()
    }

    async fn set_speed(&self, value: i32) {
        use rigela_utils::ibmeci::VP_SPEED;
        self.eci
            .set_voice_param(VP_SPEED, Self::convert_speed_param(value));
    }

    async fn set_volume(&self, value: i32) {
        use rigela_utils::ibmeci::VP_VOLUME;
        self.eci.set_voice_param(VP_VOLUME, value);
    }

    async fn set_pitch(&self, value: i32) {
        use rigela_utils::ibmeci::{VP_PITCH_BASELINE, VP_PITCH_FLUCTUATION};
        let pitch = Self::convert_pitch_param(value);
        self.eci.set_voice_param(VP_PITCH_BASELINE, pitch);
        self.eci.set_voice_param(VP_PITCH_FLUCTUATION, pitch);
    }

    async fn set_voice(&self, id: String) {
        self.eci.set_voice(u32::from_str(id.as_str()).unwrap_or(0))
    }
}

unsafe impl Send for VvttsEngine {}

unsafe impl Sync for VvttsEngine {}
