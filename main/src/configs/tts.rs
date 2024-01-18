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

use serde::{Deserialize, Serialize};

/// 语音TTS的配置项
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TtsConfig {
    pub(crate) speed: Option<i32>,
    pub(crate) volume: Option<i32>,
    pub(crate) pitch: Option<i32>,
}

impl Default for TtsConfig {
    fn default() -> Self {
        Self {
            speed: Some(50),
            volume: Some(100),
            pitch: Some(50),
        }
    }
}

#[derive(Clone)]
pub enum TtsProperty {
    Speed,
    Volume,
    Pitch,
}

impl TtsProperty {
    pub fn next(&self) -> Self {
        match self {
            TtsProperty::Speed => TtsProperty::Volume,
            TtsProperty::Volume => TtsProperty::Pitch,
            TtsProperty::Pitch => TtsProperty::Speed,
        }
    }
    pub fn prev(&self) -> Self {
        match self {
            TtsProperty::Speed => TtsProperty::Pitch,
            TtsProperty::Volume => TtsProperty::Speed,
            TtsProperty::Pitch => TtsProperty::Volume,
        }
    }
}
