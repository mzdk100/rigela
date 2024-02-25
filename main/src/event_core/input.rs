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


use std::sync::Arc;
use crate::context::Context;

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
