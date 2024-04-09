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
    combo_key,
    commander::keyboard::{
        combo_keys::{ComboKey, State},
        keys::Keys::*,
        modify_keys::ModifierKeys,
    },
    configs::items::navigation::NavigationMode,
    context::{Context, ContextAccessor},
    navigator::linear::LinearNavigator,
    performer::sound::SoundArgument::Single,
};
use async_trait::async_trait;
use rigela_macros::talent;
#[allow(unused_imports)]
use std::sync::Weak;

const WAVE: &str = "boundary.wav";

//noinspection RsUnresolvedPath
#[talent(doc = t ! ("navigator.element_prev_doc").to_string(), key = combo_key ! (VkNumPad7))]
async fn element_prev(context: Weak<Context>) {
    match context.get_ui_navigator().prev().await.current().await {
        Some(element) => {
            context.get_performer().speak(element.as_ref()).await;
        }
        None => {
            context.get_performer().play_sound(Single(WAVE)).await;
        }
    };
}

//noinspection RsUnresolvedPath
#[talent(doc = t ! ("navigator.element_next_doc").to_string(), key = combo_key ! (VkNumPad9))]
async fn element_next(context: Weak<Context>) {
    match context.get_ui_navigator().next().await.current().await {
        Some(element) => {
            context.get_performer().speak(element.as_ref()).await;
        }
        None => context.get_performer().play_sound(Single(WAVE)).await,
    };
}

//noinspection RsUnresolvedPath
#[talent(doc = t ! ("navigator.element_current_doc").to_string(), key = combo_key ! (VkNumPad8))]
async fn element_current(context: Weak<Context>) {
    match context.get_ui_navigator().current().await {
        Some(element) => {
            context.get_performer().speak(element.as_ref()).await;
        }
        None => {
            context.get_performer().play_sound(Single(WAVE)).await;
        }
    };
}

//noinspection RsUnresolvedPath
#[talent(doc = t ! ("navigator.element_prev_line_doc").to_string(), key = combo_key ! (VkNumPad4))]
async fn element_prev_line(context: Weak<Context>) {
    context.get_performer().play_sound(Single(WAVE)).await;
}

//noinspection RsUnresolvedPath
#[talent(doc = t ! ("navigator.element_next_line_doc").to_string(), key = combo_key ! (VkNumPad6))]
async fn element_next_line(context: Weak<Context>) {
    context.get_performer().play_sound(Single(WAVE)).await;
}

//noinspection RsUnresolvedPath
#[talent(doc = t ! ("navigator.element_current_line_doc").to_string(), key = combo_key ! (VkNumPad5))]
async fn element_current_line(context: Weak<Context>) {
    context.get_performer().play_sound(Single(WAVE)).await;
}

//noinspection RsUnresolvedPath
#[talent(doc = t ! ("navigator.mode_next_doc").to_string(), key = combo_key ! (VkAdd))]
async fn mode_next(context: Weak<Context>) {
    let mut config = context.get_config_manager().get_config();
    config.navigation_config.mode = match config.navigation_config.mode {
        NavigationMode::Linear => NavigationMode::Plane,
        NavigationMode::Plane => NavigationMode::Tree,
        NavigationMode::Tree => NavigationMode::Linear,
    };
    context.get_config_manager().set_config(&config);
    let text = match config.navigation_config.mode {
        NavigationMode::Linear => {
            let ctx = context.clone();
            context.get_work_runtime().spawn(async move {
                ctx.get_performer().play_sound(Single(WAVE)).await;
            });
            t!("navigator.linear")
        }
        NavigationMode::Plane => t!("navigator.plane"),
        NavigationMode::Tree => t!("navigator.tree"),
    }
        .to_string();
    context.get_performer().speak(&text).await;
}

//noinspection RsUnresolvedPath
#[talent(doc = t ! ("navigator.mode_prev_doc").to_string(), key = combo_key ! (VkSubtract))]
async fn mode_prev(context: Weak<Context>) {
    let mut config = context.get_config_manager().get_config();
    config.navigation_config.mode = match config.navigation_config.mode {
        NavigationMode::Linear => NavigationMode::Tree,
        NavigationMode::Plane => NavigationMode::Linear,
        NavigationMode::Tree => NavigationMode::Plane,
    };
    context.get_config_manager().set_config(&config);
    let text = match config.navigation_config.mode {
        NavigationMode::Linear => t!("navigator.linear"),
        NavigationMode::Plane => t!("navigator.plane"),
        NavigationMode::Tree => {
            let ctx = context.clone();
            context.get_work_runtime().spawn(async move {
                ctx.get_performer().play_sound(Single(WAVE)).await;
            });
            t!("navigator.tree")
        }
    }
        .to_string();
    context.get_performer().speak(&text).await;
}

//noinspection RsUnresolvedPath
#[talent(doc = t ! ("navigator.element_color_set_doc").to_string(), key = combo_key ! (VkNumPad8, double))]
async fn element_color_set(context: Weak<Context>) {
    let Some(element) = context.get_ui_navigator().current().await else {
        return;
    };
    let Some(color_set) = element.get_color_set() else {
        return;
    };
    let info = color_set.iter().map(|i| i.name.as_str()).collect::<Vec<_>>().join(",");
    context.get_performer().speak(&t!("navigator.element_color_set", count=color_set.len(), list=info)).await;
}
