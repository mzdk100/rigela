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
    cancel_edge_handle, combo_key,
    commander::keyboard::{
        combo_keys::{ComboKey, State},
        keys::Keys::*,
        modify_keys::ModifierKeys,
    },
    context::{Context, ContextAccessor},
    performer::{
        cache::Direction as CacheDirection,
        tts::{Direction, TtsProperty, ValueChange},
    },
};
#[allow(unused_imports)]
use async_trait::async_trait;
use rigela_macros::talent;
use rigela_utils::clip::set_clipboard_text;
use std::sync::{atomic::Ordering, Weak};

//noinspection RsUnresolvedPath
#[talent(doc = "语音属性值增加", key = combo_key!("RigelA_Ctrl", VkUp))]
async fn increase(context: Weak<Context>) {
    cancel_edge_handle!(context);

    let tts = context.get_performer().get_tts();
    tts.set_tts_prop_value(ValueChange::Increment).await;
    speak_tts_prop(context).await;
}

//noinspection RsUnresolvedPath
#[talent(doc = "语音属性值降低", key = combo_key!("RigelA_Ctrl", VkDown))]
async fn reduce(context: Weak<Context>) {
    cancel_edge_handle!(context);

    let tts = context.get_performer().get_tts();
    tts.set_tts_prop_value(ValueChange::Decrement).await;
    speak_tts_prop(context).await;
}

//noinspection RsUnresolvedPath
#[talent(doc = "语音下一属性", key = combo_key!("RigelA_Ctrl", VkRight))]
async fn next_prop(context: Weak<Context>) {
    cancel_edge_handle!(context);

    let tts = context.get_performer().get_tts();
    tts.move_tts_prop(Direction::Next).await;
    speak_tts_prop(context).await;
}

//noinspection RsUnresolvedPath
#[talent(doc = "语音上一属性", key = combo_key!("RigelA_Ctrl", VkLeft))]
async fn prev_prop(context: Weak<Context>) {
    cancel_edge_handle!(context);

    let tts = context.get_performer().get_tts();
    tts.move_tts_prop(Direction::Prev).await;
    speak_tts_prop(context).await;
}

#[talent(doc = "缓冲区上一字符", key = combo_key!("RigelA", VkLeft))]
async fn prev_cache_char(context: Weak<Context>) {
    cancel_edge_handle!(context);

    let Some(cache) = context.get_performer().get_cache() else {
        return;
    };
    let text = unsafe { &*cache.as_ptr() }.get(CacheDirection::Backward);
    let tts = context.get_performer().get_tts();
    tts.stop().await;
    tts.speak(text).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "缓冲区下一字符", key = combo_key!("RigelA", VkRight))]
async fn next_cache_char(context: Weak<Context>) {
    cancel_edge_handle!(context);

    let Some(cache) = context.get_performer().get_cache() else {
        return;
    };
    let text = unsafe { &*cache.as_ptr() }.get(CacheDirection::Forward);
    let tts = context.get_performer().get_tts();
    tts.stop().await;
    tts.speak(text).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "解释缓冲区当前字符", key = combo_key!("RigelA", VkUp))]
async fn trans_cache_char(context: Weak<Context>) {
    cancel_edge_handle!(context);

    let Some(cache) = context.get_performer().get_cache() else {
        return;
    };
    let text = unsafe { &*cache.as_ptr() }.get(CacheDirection::Current);
    // Todo: 查字典

    let tts = context.get_performer().get_tts();
    tts.stop().await;
    tts.speak(text).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "缓冲区当前字符组词", key = combo_key!("RigelA", VkDown))]
async fn make_word_cache_char(context: Weak<Context>) {
    cancel_edge_handle!(context);

    let Some(cache) = context.get_performer().get_cache() else {
        return;
    };
    let tts = context.get_performer().get_tts();
    tts.stop().await;
    let words = unsafe { &*cache.as_ptr() }.get_current_char_words();
    tts.speak(words).await;
}

//noinspection RsUnresolvedPath
#[talent(doc = "拷贝缓冲区", key = combo_key!("RigelA", VkC))]
async fn cache_to_clipboard(context: Weak<Context>) {
    let Some(cache) = context.get_performer().get_cache() else {
        return;
    };
    let text = unsafe { &*cache.as_ptr() }.get_data();
    set_clipboard_text(text);
    // context.performer.play_sound("boundary.wav").await;
}

async fn speak_tts_prop(context: Weak<Context>) {
    let tts = context.get_performer().get_tts();

    let info = match tts.get_tts_prop_value(None).await {
        TtsProperty::Speed(v) => t!("tts.speed_info", value = v),
        TtsProperty::Pitch(v) => t!("tts.pitch_info", value = v),
        TtsProperty::Volume(v) => t!("tts.volume_info", value = v),
        TtsProperty::Voice(v) => t!("tts.role", value = format!("{}_{}", v.engine, v.name)),
    };
    context.get_performer().speak(&info).await;
}
