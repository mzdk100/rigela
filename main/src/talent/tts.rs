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
use win_wrap::input::{VK_INSERT, VK_OEM_MINUS, VK_OEM_PLUS, VK_LCONTROL, VK_RCONTROL,VK_UP, VK_DOWN, VK_LEFT, VK_RIGHT};

//noinspection RsUnresolvedReference
#[talent(doc = "语音加速", key = ((VK_LCONTROL, false), (VK_UP, true)))]
async fn increase(context: Arc<Context>) {
    context.performer.apply_config(context.clone(), |c| {
        c.speed.replace(c.speed.unwrap() + 0.1);
    });

    let speed = context
        .config_manager
        .read()
        .await
        .tts_config
        .unwrap()
        .speed
        .unwrap();
    let pf = context.performer.clone();
    pf.speak_text(t!("tts.speed", value = speed).as_str()).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "语音减速", key = ((VK_INSERT, false), (VK_LCONTROL, false), (VK_DOWN, true)))]
async fn reduce(context: Arc<Context>) {
    context.performer.apply_config(context.clone(), |c| {
        c.speed.replace(c.speed.unwrap() - 0.1);
    });

    let speed = context
        .config_manager
        .read()
        .await
        .tts_config
        .unwrap()
        .speed
        .unwrap();
    let pf = context.performer.clone();
    pf.speak_text(t!("tts.speed", value = speed).as_str()).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "语音加速", key = ((VK_RCONTROL, true), (VK_UP, true)))]
async fn increase_r(context: Arc<Context>) {
    context.performer.apply_config(context.clone(), |c| {
        c.speed.replace(c.speed.unwrap() + 0.1);
    });

    let speed = context
        .config_manager
        .read()
        .await
        .tts_config
        .unwrap()
        .speed
        .unwrap();
    let pf = context.performer.clone();
    pf.speak_text(t!("tts.speed", value = speed).as_str()).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "语音减速", key = ((VK_INSERT, false), (VK_RCONTROL, true), (VK_DOWN, true)))]
async fn reduce_r(context: Arc<Context>) {
    context.performer.apply_config(context.clone(), |c| {
        c.speed.replace(c.speed.unwrap() - 0.1);
    });

    let speed = context
        .config_manager
        .read()
        .await
        .tts_config
        .unwrap()
        .speed
        .unwrap();
    let pf = context.performer.clone();
    pf.speak_text(t!("tts.speed", value = speed).as_str()).await;
}
