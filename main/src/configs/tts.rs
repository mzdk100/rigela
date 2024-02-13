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
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct TtsConfig {
    pub(crate) speed: i32,
    pub(crate) voice_index: i32,
    pub(crate) pitch: i32,
    pub(crate) volume: i32,
}

impl Default for TtsConfig {
    fn default() -> Self {
        Self {
            speed: 50,
            voice_index: 0,
            pitch: 50,
            volume: 100,
        }
    }
}
