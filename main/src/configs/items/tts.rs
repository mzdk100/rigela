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

/// 属性条目
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) enum TtsPropertyItem {
    Speed,
    Pitch,
    Volume,
    Voice,
}

/// 语音TTS的配置项
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct TtsConfig {
    pub(crate) speed: i32,
    pub(crate) volume: i32,
    pub(crate) pitch: i32,
    pub(crate) voice: (String, String),
    pub(crate) item: TtsPropertyItem,
}

impl Default for TtsConfig {
    //noinspection SpellCheckingInspection
    fn default() -> Self {
        Self {
            speed: 50,
            volume: 100,
            pitch: 50,
            voice: ("Sapi5".to_string(), "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech_OneCore\\Voices\\Tokens\\MSTTS_V110_zhCN_HuihuiM".to_string()),
            item: TtsPropertyItem::Speed,
        }
    }
}
