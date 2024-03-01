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

use crate::commander::hooks::get_capital_state;
use crate::commander::keys::Keys::{VkCapital, VkNumlock};
use crate::context::Context;
use std::sync::Arc;
use win_wrap::input::{get_key_state, VK_NUMLOCK};

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
                performer.speak(c).await;
            });
        })
        .await;
}

/// 处理锁定键状态更改播报
pub(crate) async fn subscribe_lock_key_events(context: Arc<Context>) {
    let ctx = context.clone();

    context
        .commander
        .add_key_event_listener(&vec![VkCapital, VkNumlock], move |key, pressed| {
            let info = match key {
                VkCapital if !pressed => {
                    let (state, _, c) = get_capital_state().lock().unwrap().clone();
                    match state {
                        true if c => "大写",
                        false if c => "小写",
                        _ => "",
                    }
                }
                VkNumlock if pressed => {
                    let (_, state) = get_key_state(VK_NUMLOCK);
                    match state {
                        true => "热键",
                        false => "数字",
                    }
                }
                _ => "",
            };
            let pf = ctx.performer.clone();
            ctx.main_handler.spawn(async move {
                pf.speak(info.to_string()).await;
            });
        });
}
