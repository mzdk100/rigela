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

use win_wrap::tts::Sapi5TtsSynthesizer;

pub(crate) trait Ttsable: Default {
    fn speak(&self, text: &str);
    fn stop(&self);
    fn get_name(&self) -> String;
    fn get_all_voices(&self) -> Vec<String>;
    fn get_voice(&self) -> i32;
    fn set_voice(&self, voice: i32);
    fn get_speed(&self) -> i32;
    fn set_speed(&self, speed: i32);
    fn get_pitch(&self) -> i32;
    fn set_pitch(&self, pitch: i32);
    fn get_volume(&self) -> i32;
    fn set_volume(&self, volume: i32);
}

#[derive(Debug, Clone)]
pub(crate) struct Tts<T = Sapi5> {
    all_tts: Vec<T>,
    all_voices: Vec<(String, String)>,
    voice_index: usize,
    tts: T,
}

impl<T: Ttsable> Tts<T> {
    pub(crate) fn new() -> Self {
        Self {
            all_tts: vec![],
            all_voices: vec![],
            voice_index: 0,
            tts: Default::default(),
        }
    }

    pub(crate) fn add_ttsable(&mut self, tts: T) {
        self.all_tts.push(tts);

        let name = tts.get_name();
        let voices = self.all_tts.last().unwrap().get_all_voices();
        for voice in voices {
            self.all_voices.push((name.clone(), voice));
        }
    }

    pub(crate) fn speak(&self, _text: &str) {
        self.tts.speak(_text);
    }

    pub(crate) fn stop(&self) {
        self.tts.stop();
    }

    pub(crate) fn get_all_voices(&self) -> Vec<String> {
        todo!()
    }
    pub(crate) fn get_voice(&self) -> String {
        todo!()
    }
    pub(crate) fn set_voice(&self, _voice: &str) {}
    pub(crate) fn get_speed(&self) -> i32 {
        self.get_speed()
    }
    pub(crate) fn set_speed(&self, speed: i32) {
        self.set_speed(speed);
    }
    pub(crate) fn get_pitch(&self) -> i32 {
        self.get_pitch()
    }
    pub(crate) fn set_pitch(&self, pitch: i32) {
        self.set_pitch(pitch);
    }
    pub(crate) fn get_volume(&self) -> i32 {
        self.get_volume()
    }
    pub(crate) fn set_volume(&self, volume: i32) {
        self.set_volume(volume);
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Sapi5 {
    name: String,
    synth: Sapi5TtsSynthesizer,
    voices: Vec<String>,
    voice_index: i32,
    speed: i32,
    pitch: i32,
    volume: i32,
}

impl Default for Sapi5 {
    fn default() -> Self {
        Self {
            name: "Sapi_5".to_string(),
            synth: Sapi5TtsSynthesizer::new(),
            voices: vec![],
            voice_index: 0,
            speed: 50,
            pitch: 50,
            volume: 100,
        }
    }
}

impl Sapi5 {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl Ttsable for Sapi5 {
    fn speak(&self, text: &str) {
        self.synth.speak(text);
    }

    fn stop(&self) {
        self.synth.stop();
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_all_voices(&self) -> Vec<String> {
        todo!()
    }

    fn get_voice(&self) -> i32 {
        self.voice_index
    }

    fn set_voice(&self, voice: i32) {
        // self.voice_index = voice;
    }

    fn get_speed(&self) -> i32 {
        self.speed
    }

    fn set_speed(&self, speed: i32) {
        todo!()
    }

    fn get_pitch(&self) -> i32 {
        self.pitch
    }

    fn set_pitch(&self, pitch: i32) {
        todo!()
    }

    fn get_volume(&self) -> i32 {
        self.volume
    }

    fn set_volume(&self, volume: i32) {
        todo!()
    }
}
