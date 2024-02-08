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

pub(crate) trait Ttsable {
    fn speak(&self, text: &str);
    fn stop(&self);
    fn get_all_voices(&self) -> Vec<String>;
    fn get_voice(&self) -> String;
    fn set_voice(&self, voice: &str);
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
    all_voices: Vec<(usize, String)>,
}

impl<T: Ttsable> Tts<T> {
    pub(crate) fn new() -> Self {
        Self {
            all_tts: vec![],
            all_voices: vec![],
        }
    }

    pub(crate) fn add_ttsable(&mut self, tts: T) {
        let index = self.all_tts.len();
        self.all_tts.push(tts);

        let voices = self.all_tts.last().unwrap().get_all_voices();
        for voice in voices {
            self.all_voices.push((index, voice));
        }
    }

    pub(crate) fn speak(&self, _text: &str) {}

    pub(crate) fn stop(&self) {}
}

#[derive(Debug, Clone)]
pub(crate) struct Sapi5();

impl Ttsable for Sapi5 {
    fn speak(&self, text: &str) {
        todo!()
    }

    fn stop(&self) {
        todo!()
    }

    fn get_all_voices(&self) -> Vec<String> {
        todo!()
    }

    fn get_voice(&self) -> String {
        todo!()
    }

    fn set_voice(&self, voice: &str) {
        todo!()
    }

    fn get_speed(&self) -> i32 {
        todo!()
    }

    fn set_speed(&self, speed: i32) {
        todo!()
    }

    fn get_pitch(&self) -> i32 {
        todo!()
    }

    fn set_pitch(&self, pitch: i32) {
        todo!()
    }

    fn get_volume(&self) -> i32 {
        todo!()
    }

    fn set_volume(&self, volume: i32) {
        todo!()
    }
}
