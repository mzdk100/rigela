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

use crate::context::Context;
use std::sync::Arc;
use win_wrap::input::{get_key_state, VK_CAPITAL, VK_NUMLOCK};

//noinspection SpellCheckingInspection
/**
 * 订阅键盘输入事件。
 * `context` 读屏框架的上下文环境。
 * */
pub(crate) async fn subscribe_input_events(context: Arc<Context>) {
    let ctx = context.clone();

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
pub(crate) async fn handle_lockkey(context: Arc<Context>, vk: u16) {
    let mut info: &str = "";
    if vk == VK_CAPITAL.0 {
        let (_, state) = get_key_state(VK_CAPITAL);
        info = match state {
            true => "小写",
            false => "大写",
        }
    } else if vk == VK_NUMLOCK.0 {
        let (_, state) = get_key_state(VK_NUMLOCK);
        info = match state {
            true => "热键",
            false => "数字",
        }
    }
    context.performer.speak(info.to_string()).await;
}
