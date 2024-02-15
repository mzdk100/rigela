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

use crate::{
    commander::keys::Keys::*,
    context::Context,
    performer::tts::{Direction, TtsProperty, ValueChange},
};
#[allow(unused_imports)]
use async_trait::async_trait;
use rigela_macros::talent;
#[allow(unused_imports)]
use std::sync::Arc;

//noinspection RsUnresolvedReference
#[talent(doc = "语音属性值增加", key = (VkRigelA, VkCtrl, VkUp))]
async fn increase(context: Arc<Context>) {
    let tts = context.performer.get_tts();
    tts.set_tts_prop_value(ValueChange::Increment).await;
    speak_tts_prop(context).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "语音属性值降低", key = (VkRigelA, VkCtrl, VkDown))]
async fn reduce(context: Arc<Context>) {
    let tts = context.performer.get_tts();
    tts.set_tts_prop_value(ValueChange::Decrement).await;
    speak_tts_prop(context).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "语音下一属性", key = (VkRigelA, VkCtrl, VkRight))]
async fn next_prop(context: Arc<Context>) {
    let tts = context.performer.get_tts();
    tts.move_tts_prop(Direction::Next).await;
    speak_tts_prop(context).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "语音上一属性", key = (VkRigelA, VkCtrl, VkLeft))]
async fn prev_prop(context: Arc<Context>) {
    let tts = context.performer.get_tts();
    tts.move_tts_prop(Direction::Prev).await;
    speak_tts_prop(context).await;
}

async fn speak_tts_prop(context: Arc<Context>) {
    let tts = context.performer.get_tts();

    let info = match tts.get_tts_prop_value().await {
        TtsProperty::Speed(v) => format!("语速: {}", v),
        TtsProperty::Volume(v) => format!("音量: {}", v),
        TtsProperty::Pitch(v) => format!("语调: {}", v),
        TtsProperty::Voice(v) => format!("角色: {}:{}", v.engine, v.name),
    };
    context.performer.speak(info).await;
}
