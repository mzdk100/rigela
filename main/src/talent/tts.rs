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

/* 使用talent macro必须导入的条目，便于IDE进行代码提示 */
use rigela_macros::talent;
#[allow(unused_imports)]
use std::sync::Arc;
/* 使用talent macro可选导入的条目 */
use crate::{
    configs::config_operations::{
        apply_tts_config, get_cur_tts_prop, next_tts_prop, prev_tts_prop,
    },
    configs::tts::TtsProperty,
    context::Context,
};
use async_trait::async_trait;
#[allow(unused_imports)]
use win_wrap::input::{
    VK_DOWN, VK_INSERT, VK_LCONTROL, VK_LEFT, VK_OEM_MINUS, VK_OEM_PLUS, VK_RCONTROL, VK_RIGHT,
    VK_UP,
};

//noinspection RsUnresolvedReference
#[talent(doc = "语音加速", key = ((VK_INSERT, false), (VK_LCONTROL, false), (VK_UP, true)))]
async fn increase(context: Arc<Context>) {
    apply_tts_config(context.clone(), 1).await;
    speak_tts_prop(context).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "语音加速", key = ((VK_INSERT, false), (VK_RCONTROL, true), (VK_UP, true)))]
async fn increase_r(context: Arc<Context>) {
    apply_tts_config(context.clone(), 1).await;
    speak_tts_prop(context).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "语音减速", key = ((VK_INSERT, false), (VK_LCONTROL, false), (VK_DOWN, true)))]
async fn reduce(context: Arc<Context>) {
    apply_tts_config(context.clone(), -1).await;
    speak_tts_prop(context).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "语音减速", key = ((VK_INSERT, false), (VK_RCONTROL, true), (VK_DOWN, true)))]
async fn reduce_r(context: Arc<Context>) {
    apply_tts_config(context.clone(), -1).await;
    speak_tts_prop(context).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "语音下一属性", key = ((VK_INSERT, false), (VK_LCONTROL, false), (VK_RIGHT, true)))]
async fn next_prop(context: Arc<Context>) {
    next_tts_prop();
    speak_tts_prop(context).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "语音下一属性", key = ((VK_INSERT, false), (VK_RCONTROL, true), (VK_RIGHT, true)))]
async fn next_prop_r(context: Arc<Context>) {
    next_tts_prop();
    speak_tts_prop(context).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "语音上一属性", key = ((VK_INSERT, false), (VK_LCONTROL, false), (VK_LEFT, true)))]
async fn prev_prop(context: Arc<Context>) {
    prev_tts_prop();
    speak_tts_prop(context).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "语音上一属性", key = ((VK_INSERT, false), (VK_RCONTROL, true), (VK_LEFT, true)))]
async fn prev_prop_r(context: Arc<Context>) {
    prev_tts_prop();
    speak_tts_prop(context).await;
}

async fn speak_tts_prop(context: Arc<Context>) {
    let cfg = context.config_manager.get_config().await.tts_config.clone();
    let info = match get_cur_tts_prop() {
        TtsProperty::Speed => format!("语速: {}", cfg.speed),
        TtsProperty::Volume => format!("音量: {}", cfg.volume),
        TtsProperty::Pitch => format!("语调: {}", cfg.pitch),
    };
    context.performer.speak_with_sapi5(&info).await;
}
