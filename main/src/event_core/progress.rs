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
use crate::performer::sound::SoundArgument::WithFreq;
use std::sync::Arc;
use win_wrap::msaa::object::ROLE_SYSTEM_PROGRESSBAR;

//noinspection SpellCheckingInspection
/**
 * 订阅进度通知事件。
 * `context` 读屏框架的上下文环境。
 * */
pub(crate) async fn subscribe_progress_events(context: Arc<Context>) {
    let main_handler = context.main_handler.clone();
    let performer = context.performer.clone();

    context
        .msaa
        .add_on_object_value_change_listener(move |src| {
            let Ok((obj, child)) = src.get_object() else {
                return;
            };
            if obj.get_role(child) != ROLE_SYSTEM_PROGRESSBAR {
                return;
            }
            let Ok(value) = obj
                .get_value(child)
                .trim_matches(|c| c == '%' || c == ' ')
                .parse::<u32>()
            else {
                return;
            };
            let performer = performer.clone();

            main_handler.spawn(async move {
                performer
                    .play_sound(WithFreq("progress.wav", 2000f32 + 460f32 * (value as f32)))
                    .await;
            });
        });
}
