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

use crate::configs::items::navigation::NavigationMode;
use crate::{
    combo_key,
    commander::keyboard::{
        combo_keys::{ComboKey, State},
        keys::Keys::*,
        modify_keys::ModifierKeys,
    },
    context::Context,
    navigator::linear::LinearNavigator,
    performer::sound::SoundArgument::Single,
};
use async_trait::async_trait;
use rigela_macros::talent;
#[allow(unused_imports)]
use std::sync::Weak;

const WAVE: &str = "boundary.wav";

//noinspection RsUnresolvedPath
#[talent(doc = "上一个控件", key = combo_key!(VkNumPad7))]
async fn prev_element(context: Weak<Context>) {
    let context = unsafe { &*context.as_ptr() };

    match context.ui_navigator.prev().await.current().await {
        Some(element) => {
            context.performer.speak(element.as_ref()).await;
        }
        None => {
            context.performer.play_sound(Single(WAVE)).await;
        }
    };
}

//noinspection RsUnresolvedPath
#[talent(doc = "下一个控件", key = combo_key!(VkNumPad9))]
async fn next_element(context: Weak<Context>) {
    let context = unsafe { &*context.as_ptr() };

    match context.ui_navigator.next().await.current().await {
        Some(element) => {
            context.performer.speak(element.as_ref()).await;
        }
        None => context.performer.play_sound(Single(WAVE)).await,
    };
}

//noinspection RsUnresolvedPath
#[talent(doc = "当前控件", key = combo_key!(VkNumPad8))]
async fn curr_element(context: Weak<Context>) {
    let context = unsafe { &*context.as_ptr() };

    match context.ui_navigator.current().await {
        Some(element) => {
            context.performer.speak(element.as_ref()).await;
        }
        None => {
            context.performer.play_sound(Single(WAVE)).await;
        }
    };
}

//noinspection RsUnresolvedPath
#[talent(doc = "上一个子控件", key = combo_key!(VkNumPad4))]
async fn prev_child_element(context: Weak<Context>) {
    let context = unsafe { &*context.as_ptr() };

    context.performer.play_sound(Single(WAVE)).await;
}

//noinspection RsUnresolvedPath
#[talent(doc = "下一个子控件", key = combo_key!(VkNumPad6))]
async fn next_child_element(context: Weak<Context>) {
    let context = unsafe { &*context.as_ptr() };

    context.performer.play_sound(Single(WAVE)).await;
}

//noinspection RsUnresolvedPath
#[talent(doc = "当前子控件", key = combo_key!(VkNumPad5))]
async fn curr_child_element(context: Weak<Context>) {
    let context = unsafe { &*context.as_ptr() };

    context.performer.play_sound(Single(WAVE)).await;
}

//noinspection RsUnresolvedPath
#[talent(doc = "下一个模式", key = combo_key!(VkAdd))]
async fn mode_next(context: Weak<Context>) {
    let context = unsafe { &*context.as_ptr() };

    let mut config = context.config_manager.get_config();
    config.navigation_config.mode = match config.navigation_config.mode {
        NavigationMode::Linear => NavigationMode::Plane,
        NavigationMode::Plane => NavigationMode::Tree,
        NavigationMode::Tree => NavigationMode::Linear,
    };
    context.config_manager.set_config(&config);
    let text = match config.navigation_config.mode {
        NavigationMode::Linear => {
            let performer = context.performer.clone();
            context.work_runtime.spawn(async move {
                performer.play_sound(Single(WAVE)).await;
            });
            t!("navigator.linear")
        }
        NavigationMode::Plane => t!("navigator.plane"),
        NavigationMode::Tree => t!("navigator.tree"),
    }
    .to_string();
    context.performer.speak(&text).await;
}

//noinspection RsUnresolvedPath
#[talent(doc = "上一个模式", key = combo_key!(VkSubtract))]
async fn mode_prev(context: Weak<Context>) {
    let context = unsafe { &*context.as_ptr() };

    let mut config = context.config_manager.get_config();
    config.navigation_config.mode = match config.navigation_config.mode {
        NavigationMode::Linear => NavigationMode::Tree,
        NavigationMode::Plane => NavigationMode::Linear,
        NavigationMode::Tree => NavigationMode::Plane,
    };
    context.config_manager.set_config(&config);
    let text = match config.navigation_config.mode {
        NavigationMode::Linear => t!("navigator.linear"),
        NavigationMode::Plane => t!("navigator.plane"),
        NavigationMode::Tree => {
            let performer = context.performer.clone();
            context.work_runtime.spawn(async move {
                performer.play_sound(Single(WAVE)).await;
            });
            t!("navigator.tree")
        }
    }
    .to_string();
    context.performer.speak(&text).await;
}
