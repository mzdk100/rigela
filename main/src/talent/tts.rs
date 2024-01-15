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
#[allow(unused_imports)]
use crate::context::Context;
use rigela_macros::talent;
#[allow(unused_imports)]
use std::sync::Arc;

/* 使用talent macro可选导入的条目 */
#[allow(unused_imports)]
use win_wrap::input::{
    VK_DOWN, VK_INSERT, VK_LCONTROL, VK_LEFT, VK_OEM_MINUS, VK_OEM_PLUS, VK_RCONTROL, VK_RIGHT,
    VK_UP,
};

//noinspection RsUnresolvedReference
#[talent(doc = "语音加速", key = ((VK_INSERT, false),(VK_LCONTROL, false), (VK_UP, true)))]
async fn increase(context: Arc<Context>) {
    set_speed(context, 0.1).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "语音加速", key = ((VK_INSERT, false),(VK_RCONTROL, true), (VK_UP, true)))]
async fn increase_r(context: Arc<Context>) {
    set_speed(context, 0.1).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "语音减速", key = ((VK_INSERT, false), (VK_LCONTROL, false), (VK_DOWN, true)))]
async fn reduce(context: Arc<Context>) {
    set_speed(context, -0.1).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "语音减速", key = ((VK_INSERT, false), (VK_RCONTROL, true), (VK_DOWN, true)))]
async fn reduce_r(context: Arc<Context>) {
    set_speed(context, -0.1).await;
}

async fn set_speed(context: Arc<Context>, diff: f32) {
    context.performer.apply_config(context.clone(), move |c| {
        c.speed.replace(c.speed.unwrap() + diff);
    });

    speak_speed(context).await;
}

async fn speak_speed(context: Arc<Context>) {
    let cfg = context.config_manager.read().await;
    let speed = cfg.tts_config.unwrap().speed.unwrap();
    let speed = t!("tts.speed", value = speed);
    context.performer.speak_text(speed.as_str()).await;
}
