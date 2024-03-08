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

use crate::commander::keys::Keys::{VkNumlock, VkScroll};
use crate::context::Context;
use std::sync::Arc;
use win_wrap::input::{get_key_state, VK_NUMLOCK, VK_SCROLL};

//noinspection SpellCheckingInspection
/**
 * 订阅键盘输入事件。
 * `context` 读屏框架的上下文环境。
 * */
pub(crate) async fn subscribe_input_events(context: Arc<Context>) {
    let ctx = context.clone();

    subscribe_lock_key_events(context.clone()).await;

    context
        .peeper_server
        .add_on_input_char_listener(move |c| {
            ctx.task_manager.abort("ime");
            let performer = ctx.performer.clone();

            ctx.main_handler.spawn(async move {
                performer.speak(&c).await;
            });
        })
        .await;
}

/// 处理锁定键状态更改播报
pub(crate) async fn subscribe_lock_key_events(context: Arc<Context>) {
    let ctx = context.clone();
    let handle = move |key, pressed: bool| {
        if !pressed {
            return;
        }

        let info = match key {
            VkScroll => {
                let (_, state) = get_key_state(VK_SCROLL);
                match state {
                    true => "滚动",
                    false => "滚动锁定",
                }
            }
            VkNumlock => {
                let (_, state) = get_key_state(VK_NUMLOCK);
                match state {
                    true => "热键",
                    false => "数字",
                }
            }
            _ => unreachable!(),
        };
        let pf = ctx.performer.clone();
        ctx.main_handler.spawn(async move {
            pf.speak(&info.to_string()).await;
        });
    };
    context
        .commander
        .add_key_event_listener(&vec![VkScroll, VkNumlock], handle);
}
