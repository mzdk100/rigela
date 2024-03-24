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
    context::Context, ext::dialog::AccessibleDialogExt, performer::sound::SoundArgument::Single,
};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
use win_wrap::msaa::{event::WinEventSource, object::AccessibleObject};

pub(crate) fn handle_dialog_events(context: Arc<Context>, src: WinEventSource) {
    let performer = context.performer.clone();
    context.main_handler.spawn(async move {
        // 延迟朗读，防止被焦点元素打断。
        sleep(Duration::from_millis(500)).await;

        let obj = match src.get_object() {
            Err(_) => match AccessibleObject::from_window(src.h_wnd) {
                Ok(o) => o,
                Err(_) => return,
            },
            Ok(o) => o.0,
        };
        performer.play_sound(Single("dialog.wav")).await;
        performer.speak(&obj.get_dialog_content()).await;
    });
}

//noinspection SpellCheckingInspection
/**
 * 订阅对话框事件。
 * `context` 读屏框架的上下文环境。
 * */
pub(crate) async fn subscribe_dialog_events(context: Arc<Context>) {
    let ctx = context.clone();
    context
        .msaa
        .add_on_system_alert_listener(move |src| handle_dialog_events(ctx.clone(), src));

    let ctx = context.clone();
    context
        .msaa
        .add_on_system_dialog_start_listener(move |src| handle_dialog_events(ctx.clone(), src));
    let ctx = context.clone();

    context.msaa.add_on_system_foreground_listener(move |src| {
        if src.get_class_name() == "#32770" {
            handle_dialog_events(ctx.clone(), src)
        }
    });
}
