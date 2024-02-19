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
    context::Context, event_core::ime::MS_IME_CLASS_NAME, ext::AccessibleObjectExt,
    performer::sound::SoundArgument::Single,
};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
use win_wrap::{
    msaa::object::{ROLE_SYSTEM_ALERT, ROLE_SYSTEM_DIALOG, ROLE_SYSTEM_LIST, ROLE_SYSTEM_LISTITEM},
    uia::element::ControlType,
};

//noinspection SpellCheckingInspection
/**
 * 订阅焦点改变事件。
 * `context` 读屏框架的上下文环境。
 * */
pub(crate) async fn subscribe_focus_events(context: Arc<Context>) {
    // 给UI Automation的焦点改变绑定处理事件
    let ctx = context.clone();
    context.ui_automation.add_focus_changed_listener(move |x| {
        let event_core = ctx.event_core.clone();
        let performer = ctx.performer.clone();

        // 异步执行元素朗读
        ctx.main_handler.spawn(async move {
            if let ControlType::ListItem = x.get_control_type() {
                // 列表项目的事件让MSAA处理，因为很多列表只有MSAA支持的完善
                if event_core
                    .should_ignore(x.get_name(), Duration::from_millis(100))
                    .await
                {
                    // 过滤重复的事件，因为同时订阅了UIA和MSAA的focus事件，就会有事件的重复
                    return;
                }
            }
            performer.speak(x).await;
        });
    });

    // 给MSAA的焦点改变绑定处理事件
    let ctx = context.clone();
    context.msaa.add_on_object_focus_listener(move |src| {
        let (obj, child) = match src.get_object() {
            Err(_) => return,
            Ok(o) => o,
        };

        let event_core = ctx.event_core.clone();
        let performer = ctx.performer.clone();

        ctx.main_handler.spawn(async move {
            match obj.get_role(child) {
                ROLE_SYSTEM_LISTITEM | ROLE_SYSTEM_LIST => (),
                ROLE_SYSTEM_ALERT | ROLE_SYSTEM_DIALOG => {
                    // 如果有对话框弹出，我们要延迟播报，因为很有可能被焦点元素打断
                    sleep(Duration::from_millis(500)).await;
                    performer.speak(obj.get_dialog_content()).await;
                    return;
                }
                _ => return,
            };
            if event_core
                .should_ignore(obj.get_name(child), Duration::from_millis(100))
                .await
            {
                // 过滤重复的事件，因为同时订阅了UIA和MSAA的focus事件，就会有事件的重复
                return;
            }
            performer.speak((obj, child)).await;
        });
    });

    // 监听容器控件中选择项改变（例如组合框）
    let ctx = context.clone();
    context.msaa.add_on_object_selection_listener(move |src| {
        if src.get_class_name() == MS_IME_CLASS_NAME {
            // 此类事件属于微软输入法候选项，处理逻辑在ime中已经实现
            return;
        }

        let event_core = ctx.event_core.clone();
        let performer = ctx.performer.clone();

        ctx.main_handler.spawn(async move {
            let Ok((obj, child)) = src.get_object() else {
                return;
            };
            if event_core
                .should_ignore(obj.get_name(child), Duration::from_millis(50))
                .await
            {
                // 过滤重复的事件，因为同时订阅了UIA和MSAA的focus事件，就会有事件的重复
                return;
            }

            performer.speak((obj, child)).await;
        });
    });

    // 监听工具提示信息
    let ctx = context.clone();
    context.msaa.add_on_object_show_listener(move |src| {
        if !src.get_class_name().to_lowercase().contains("tooltip") {
            return;
        }
        let Ok(obj) = src.get_object() else {
            return;
        };
        let performer = ctx.performer.clone();
        ctx.main_handler.spawn(async move {
            performer.play_sound(Single("tip.wav")).await;
            performer.speak(obj).await;
        });
    })
}
