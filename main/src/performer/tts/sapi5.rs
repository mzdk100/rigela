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

use crate::performer::tts::TtsEngine;
use rigela_utils::bass::BassChannelOutputStream;
use win_wrap::tts::Sapi5TtsSynthesizer;

#[derive(Debug)]
pub(crate) struct Sapi5Engine {
    output_stream: BassChannelOutputStream,
    synth: Sapi5TtsSynthesizer,
}

impl Sapi5Engine {
    pub(crate) fn new() -> Self {
        Self {
            output_stream: BassChannelOutputStream::new(16000, 1),
            synth: Sapi5TtsSynthesizer::new(),
        }
    }
}

#[async_trait::async_trait]
impl TtsEngine for Sapi5Engine {
    async fn speak(&self, text: &str) {
        self.output_stream.start();

        let text = text.to_string();
        let data = self.synth.synth(text.as_str()).await;
        if data.len() < 320 {
            // sapi5语音的采样率是每秒16000个样本，320个字节等于160个样本（0.01秒）
            return;
        }
        // 跳过开头的0.01秒，因为基本上他是静音的
        self.output_stream.put_data(&data[320..]);
    }

    async fn wait(&self) {
        self.output_stream.wait_until_stalled().await;
    }

    fn stop(&self) {
        self.output_stream.stop();
    }

    fn get_name(&self) -> String {
        String::from("Sapi5")
    }

    async fn get_all_voices(&self) -> Vec<(String, String)> {
        self.synth.get_voice_list()
    }

    async fn set_speed(&self, value: i32) {
        self.synth.set_speed(3.0 + (value as f64 - 50.0) * 0.06)
    }

    async fn set_volume(&self, value: i32) {
        self.synth.set_volume(0.5 + (value as f64 - 50.0) * 0.01)
    }

    async fn set_pitch(&self, value: i32) {
        self.synth.set_pitch(1.0 + (value as f64 - 50.0) * 0.01)
    }

    async fn set_voice(&self, id: String) {
        self.synth.set_voice(id)
    }
}

unsafe impl Send for Sapi5Engine {}

unsafe impl Sync for Sapi5Engine {}
