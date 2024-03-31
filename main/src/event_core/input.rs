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
    commander::keyboard::keys::Keys::{VkNumlock, VkScroll},
    context::{Context, ContextAccessor},
};
use std::sync::Weak;
use win_wrap::input::{get_key_state, VK_NUMLOCK, VK_SCROLL};

//noinspection SpellCheckingInspection
/**
 * 订阅键盘输入事件。
 * `context` 读屏框架的上下文环境。
 * */
pub(crate) async fn subscribe_input_events(context: Weak<Context>) {
    let ctx = context.clone();

    subscribe_lock_key_events(context.clone()).await;

    context
        .get_peeper_server()
        .add_on_input_char_listener(move |c| {
            let ctx2 = ctx.clone();
            ctx.get_task_manager().abort("ime");

            ctx.get_work_runtime().spawn(async move {
                ctx2.get_performer().speak(&c).await;
            });
        })
        .await;
}

/// 处理锁定键状态更改播报
pub(crate) async fn subscribe_lock_key_events(context: Weak<Context>) {
    let ctx = context.clone();
    let handle = move |key, pressed: bool| {
        if !pressed {
            return;
        }

        let info = match key {
            VkScroll => {
                let (_, state) = get_key_state(VK_SCROLL);
                match state {
                    true => t!("input.scroll_off"),
                    false => t!("input.scroll_on"),
                }
            }
            VkNumlock => {
                let (_, state) = get_key_state(VK_NUMLOCK);
                match state {
                    true => t!("input.numlock_off"),
                    false => t!("input.numlock_on"),
                }
            }
            _ => unreachable!(),
        };
        let ctx2 = ctx.clone();
        ctx.get_work_runtime().spawn(async move {
            ctx2.get_performer().speak(&info.to_string()).await;
        });
    };

    context
        .get_commander()
        .get_keyboard_manager()
        .add_key_event_listener(&vec![VkScroll, VkNumlock], handle);
}
