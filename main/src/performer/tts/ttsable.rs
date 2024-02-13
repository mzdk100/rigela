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
use std::sync::Arc;

/// 语音TTS的抽象接口
#[async_trait::async_trait]
pub(crate) trait Ttsable {
    fn set_context(&self, context: Arc<Context>);
    async fn speak(&self, text: &str);
    fn stop(&self);
    fn get_name(&self) -> String;
    async fn get_all_voices(&self) -> Vec<String>;
    async fn set_value_by_prop(&self, prop: TtsProperty, value: i32);
}

/// TTS的属性枚举
#[derive(Debug, Clone, Copy)]
pub(crate) enum TtsProperty {
    Speed,
    Voice,
    Pitch,
    Volume,
}

impl TtsProperty {
    pub(crate) fn next(&self) -> Self {
        match self {
            TtsProperty::Speed => TtsProperty::Voice,
            TtsProperty::Voice => TtsProperty::Pitch,
            TtsProperty::Pitch => TtsProperty::Volume,
            TtsProperty::Volume => TtsProperty::Speed,
        }
    }
    pub(crate) fn prev(&self) -> Self {
        match self {
            TtsProperty::Speed => TtsProperty::Volume,
            TtsProperty::Voice => TtsProperty::Speed,
            TtsProperty::Pitch => TtsProperty::Voice,
            TtsProperty::Volume => TtsProperty::Pitch,
        }
    }
}

/// 移动TTS属性的方向
#[derive(Debug, Clone, Copy)]
pub(crate) enum Direction {
    Next,
    Prev,
}

/// 增减TTS属性的值
#[derive(Debug, Clone, Copy)]
pub(crate) enum ValueChange {
    Increment,
    Decrement,
}
