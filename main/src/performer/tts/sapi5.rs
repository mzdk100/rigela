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
use tokio::sync::Mutex;
use win_wrap::tts::Sapi5TtsSynthesizer;

#[derive(Debug)]
pub(crate) struct Sapi5 {
    context: OnceLock<Arc<Context>>,
    name: String,
    synth: Sapi5TtsSynthesizer,
    voices: Mutex<Vec<String>>,
    voice_index: Mutex<i32>,
    speed: Mutex<i32>,
    pitch: Mutex<i32>,
    volume: Mutex<i32>,
}

impl Default for Sapi5 {
    fn default() -> Self {
        Self {
            context: OnceLock::new(),
            name: "Sapi_5".to_string().into(),
            synth: Sapi5TtsSynthesizer::new(),
            voices: vec![].into(),
            voice_index: 0.into(),
            speed: 50.into(),
            pitch: 50.into(),
            volume: 100.into(),
        }
    }
}

impl Sapi5 {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    async fn _speak(&self, text: &str) {
        let context = self.context.get().unwrap();
        let stream = context.performer.output_stream.clone();
        stream.stop();
        stream.start();

        let text = text.to_string();
        stream.put_data(&self.synth.synth(text.as_str()).await);
    }

    #[allow(unused)]
    pub(crate) async fn set_tts_properties_with_sapi5(&self, speed: i32, volume: i32, pitch: i32) {
        // self.sapi5_synth.set_properties(
        //     3.0 + (speed as f64 - 50.0) * 0.06,
        //     0.5 + (volume as f64 - 50.0) * 0.01,
        //     1.0 + (pitch as f64 - 50.0) * 0.01,
        // );
    }

    #[allow(unused)]
    pub fn get_tts_voices_with_sapi5(&self) -> Vec<(String, String)> {
        // self.sapi5_synth.get_voice_list()
        Default::default()
    }

    #[allow(unused)]
    pub(crate) async fn set_tts_voice_with_sapi5(&self, voice_id: String) {
        // self.sapi5_synth.set_voice(voice_id)
    }
}

#[async_trait::async_trait]

impl Ttsable for Sapi5 {
    fn set_context(&self, context: Arc<Context>) {
        self.context.set(context.clone()).unwrap();
    }

    async fn speak(&self, text: &str) {
        self._speak(text).await;
    }

    fn stop(&self) {
        todo!()
    }

    fn get_name(&self) -> String {
        self.name.clone()
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

unsafe impl Send for Sapi5 {}

unsafe impl Sync for Sapi5 {}
