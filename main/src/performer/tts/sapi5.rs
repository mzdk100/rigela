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
use std::sync::{Arc, OnceLock};
use win_wrap::tts::Sapi5TtsSynthesizer;

#[derive(Debug)]
pub(crate) struct Sapi5 {
    synth: Sapi5TtsSynthesizer,
    context: OnceLock<Arc<Context>>,
    name: String,
}

impl Default for Sapi5 {
    fn default() -> Self {
        Self {
            synth: Sapi5TtsSynthesizer::new(),
            context: OnceLock::new(),
            name: "Sapi_5".to_string().into(),
        }
    }
}

impl Sapi5 {
    #[allow(unused)]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    #[allow(unused)]
    pub(crate) async fn set_tts_voice_with_sapi5(&self, voice_id: String) {
        self.synth.set_voice(voice_id)
    }
}

#[async_trait::async_trait]

impl Ttsable for Sapi5 {
    fn set_context(&self, context: Arc<Context>) {
        self.context.set(context.clone()).unwrap();
    }

    async fn speak(&self, text: &str) {
        let context = self.context.get().unwrap();
        let stream = context.performer.output_stream.clone();
        stream.stop();
        stream.start();

        let text = text.to_string();
        stream.put_data(&self.synth.synth(text.as_str()).await);
    }

    fn stop(&self) {
        todo!()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    async fn get_all_voices(&self) -> Vec<String> {
        let list = self.synth.get_voice_list();
        list.iter().map(|x| x.1.clone()).collect()
    }

    async fn set_value_by_prop(&self, prop: TtsProperty, value: i32) {
        match prop {
            TtsProperty::Speed => self.synth.set_speed(3.0 + (value as f64 - 50.0) * 0.06),
            TtsProperty::Pitch => self.synth.set_pitch(1.0 + (value as f64 - 50.0) * 0.01),
            TtsProperty::Volume => self.synth.set_volume(0.5 + (value as f64 - 50.0) * 0.01),
            TtsProperty::Voice => todo!(),
        }
    }
}

unsafe impl Send for Sapi5 {}

unsafe impl Sync for Sapi5 {}
