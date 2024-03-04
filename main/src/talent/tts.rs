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
    performer::cache::Direction as CacheDirection,
    performer::tts::{Direction, TtsProperty, ValueChange},
};
#[allow(unused_imports)]
use async_trait::async_trait;
use rigela_macros::talent;
use rigela_utils::clip::set_clipboard_text;
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

//noinspection RsUnresolvedReference
#[talent(doc = "缓冲区上一字符", key = (VkRigelA, VkLeft))]
async fn prev_cache_char(context: Arc<Context>) {
    {
        *crate::event_core::editor::editor_key_handle()
            .lock()
            .unwrap() = true;
    }

    let cache = context.performer.get_cache();
    let text = cache.get(CacheDirection::Backward).await;
    let tts = context.performer.get_tts();
    tts.stop().await;
    tts.speak(text).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "缓冲区下一字符", key = (VkRigelA, VkRight))]
async fn next_cache_char(context: Arc<Context>) {
    {
        *crate::event_core::editor::editor_key_handle()
            .lock()
            .unwrap() = true;
    }

    let cache = context.performer.get_cache();
    let text = cache.get(CacheDirection::Forward).await;
    let tts = context.performer.get_tts();
    tts.stop().await;
    tts.speak(text).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "解释缓冲区当前字符", key = (VkRigelA, VkUp))]
async fn trans_cache_char(context: Arc<Context>) {
    let cache = context.performer.get_cache();
    let text = cache.get(CacheDirection::Current).await;
    // Todo: 查字典

    let tts = context.performer.get_tts();
    tts.stop().await;
    tts.speak(text).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "缓冲区当前字符组词", key = (VkRigelA, VkDown))]
async fn make_word_cache_char(context: Arc<Context>) {
    {
        *crate::event_core::editor::editor_key_handle()
            .lock()
            .unwrap() = true;
    }

    let cache = context.performer.get_cache();
    let words = context.performer.get_cache().get_words();
    let text = cache.get(CacheDirection::Current).await;
    let word = words.get(&text).unwrap_or(&text);

    let tts = context.performer.get_tts();
    tts.stop().await;
    tts.speak(word.clone()).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "拷贝缓冲区", key = (VkRigelA, VkC))]
async fn cache_to_clipboard(context: Arc<Context>) {
    let cache = context.performer.get_cache();
    let text = cache.get_data().await;
    set_clipboard_text(text);
    // context.performer.play_sound("boundary.wav").await;
}

async fn speak_tts_prop(context: Arc<Context>) {
    let tts = context.performer.get_tts();

    let info = match tts.get_tts_prop_value(None).await {
        TtsProperty::Speed(v) => t!("tts.speed_info", value = v),
        TtsProperty::Pitch(v) => t!("tts.pitch_info", value = v),
        TtsProperty::Volume(v) => t!("tts.volume_info", value = v),
        TtsProperty::Voice(v) => t!("tts.role", value = format!("{}_{}", v.engine, v.name)),
    };
    context.performer.speak(info).await;
}
