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
use crate::commander::keys::Keys::*;
#[allow(unused_imports)]
use crate::context::Context;
use rigela_macros::talent;
#[allow(unused_imports)]
use std::sync::Arc;
/* 使用talent macro可选导入的条目 */
use async_trait::async_trait;

const WAVE: &str = "boundary.wav";

//noinspection RsUnresolvedReference
#[talent(doc = "上一个控件", key = (VkNumPad7))]
async fn prev_element(context: Arc<Context>) {
    match context.form_browser.prev().await.current().await {
        Some(element) => context.performer.speak(element),
        None => context.performer.play_sound(WAVE).await,
    };
}

//noinspection RsUnresolvedReference
#[talent(doc = "下一个控件", key = (VkNumPad9))]
async fn next_element(context: Arc<Context>) {
    match context.form_browser.next().await.current().await {
        Some(element) => context.performer.speak(element),
        None => context.performer.play_sound(WAVE).await,
    };
}

//noinspection RsUnresolvedReference
#[talent(doc = "当前控件", key = (VkNumPad8))]
async fn curr_element(context: Arc<Context>) {
    match context.form_browser.current().await {
        Some(element) => context.performer.speak(element),
        None => context.performer.play_sound(WAVE).await,
    };
}

//noinspection RsUnresolvedReference
#[talent(doc = "上一个子控件", key = (VkNumPad4))]
async fn prev_child_element(context: Arc<Context>) {
    match context
        .form_browser
        .prev_child()
        .await
        .current_child()
        .await
    {
        Some(element) => context.performer.speak(element),
        None => context.performer.play_sound(WAVE).await,
    };
}

//noinspection RsUnresolvedReference
#[talent(doc = "下一个子控件", key = (VkNumPad6))]
async fn next_child_element(context: Arc<Context>) {
    match context
        .form_browser
        .next_child()
        .await
        .current_child()
        .await
    {
        Some(element) => context.performer.speak(element),
        None => context.performer.play_sound(WAVE).await,
    };
}

//noinspection RsUnresolvedReference
#[talent(doc = "当前子控件", key = (VkNumPad5))]
async fn curr_child_element(context: Arc<Context>) {
    match context.form_browser.current_child().await {
        Some(element) => context.performer.speak(element),
        None => context.performer.play_sound(WAVE).await,
    };
}

//noinspection RsUnresolvedReference
#[talent(doc = "下一个模式", key = (VkAdd))]
async fn mode_next(context: Arc<Context>) {
    context.performer.play_sound(WAVE).await;
}
