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

mod editor;

use crate::{context::Context, event_core::editor::subscribe_events, ext::AccessibleObjectExt};
use std::{
    sync::{Arc, OnceLock},
    time::Duration,
};
use tokio::time::sleep;
use win_wrap::{
    msaa::object::{
        AccessibleObject, ROLE_SYSTEM_ALERT, ROLE_SYSTEM_DIALOG, ROLE_SYSTEM_LIST,
        ROLE_SYSTEM_LISTITEM,
    },
    uia::element::ControlType,
};

/// 事件处理中心
#[derive(Clone, Debug)]
pub(crate) struct EventCore {
    context: OnceLock<Arc<Context>>,
}

impl EventCore {
    pub(crate) fn new() -> Self {
        Self {
            context: OnceLock::new(),
        }
    }

    /// 启动事件监听
    pub(crate) async fn run(&self, context: Arc<Context>) {
        self.context.set(context.clone()).unwrap_or(());
        // 订阅UIA的焦点元素改变事件
        speak_focus_item(context.clone()).await;

        // 监听前台窗口变动
        subscribe_foreground_window_events(context.clone()).await;

        // 订阅输入事件
        speak_input(context.clone()).await;

        // 订阅输入法候选事件
        speak_candidate(context.clone()).await;

        // 处理编辑框朗读
        let ctx = context.clone();
        context
            .main_handler
            .spawn(subscribe_events(ctx))
            .await
            .unwrap();
    }

    /**
     * 停止所有事件处理。
     * */
    pub(crate) fn shutdown(&self) {}
}

//noinspection SpellCheckingInspection
/// 朗读焦点元素
async fn speak_focus_item(context: Arc<Context>) {
    // 给UI Automation的焦点改变绑定处理事件
    let ctx = context.clone();
    context.ui_automation.add_focus_changed_listener(move |x| {
        let performer = ctx.performer.clone();

        // 异步执行元素朗读
        ctx.main_handler.spawn(async move {
            if let ControlType::ListItem = x.get_control_type() {
                // 列表项目的事件让MSAA处理，因为很多列表只有MSAA支持的完善
                //return;
            }
            performer.speak(x);
        });
    });

    // 给MSAA的焦点改变绑定处理事件
    let ctx = context.clone();
    context.msaa.add_on_object_focus_listener(move |src| {
        let performer = ctx.performer.clone();
        let (obj, child) = match src.get_object() {
            Err(_) => return,
            Ok(o) => o,
        };
        ctx.main_handler.spawn(async move {
            match obj.get_role(child) {
                ROLE_SYSTEM_LISTITEM | ROLE_SYSTEM_LIST => (),
                ROLE_SYSTEM_ALERT | ROLE_SYSTEM_DIALOG => {
                    // 如果有对话框弹出，我们要延迟播报，因为很有可能被焦点元素打断
                    sleep(Duration::from_millis(500)).await;
                    performer.speak(obj.get_dialog_content());
                    return;
                }
                _ => return,
            };
            performer.speak_with_sapi5((obj, child)).await;
        });
    });

    // 监听容器控件中选择项改变（例如组合框）
    let ctx = context.clone();
    context.msaa.add_on_object_selection_listener(move |src| {
        let performer = ctx.performer.clone();
        ctx.main_handler.spawn(async move {
            performer.speak(src.get_object().unwrap());
        });
    });
}

/// 监测前台窗口变动，发送控件元素到form_browser
async fn subscribe_foreground_window_events(context: Arc<Context>) {
    // 给MSAA前台窗口改变绑定处理事件
    let ctx = context.clone();
    context.msaa.add_on_system_foreground_listener(move |src| {
        let form_browser = ctx.form_browser.clone();
        let ui_automation = ctx.ui_automation.clone();

        // form_browser需要异步操作
        ctx.main_handler.spawn(async move {
            if let Some(root) = ui_automation.element_from_handle(src.h_wnd) {
                form_browser.render(Arc::new(root)).await
            }
        });
    });

    // 订阅对话框事件
    let ctx = context.clone();
    context.msaa.add_on_system_alert_listener(move |src| {
        let performer = ctx.performer.clone();
        ctx.main_handler.spawn(async move {
            // 延迟朗读，防止被焦点元素打断。
            sleep(Duration::from_millis(500)).await;

            let obj = match src.get_object() {
                Err(_) => match AccessibleObject::from_window(src.h_wnd) {
                    Ok(o) => o,
                    Err(_) => return,
                },
                Ok(o) => o.0,
            };
            performer.speak(obj.get_dialog_content());
        });
    });
}

// 朗读输入
async fn speak_input(context: Arc<Context>) {
    let ctx = context.clone();

    context
        .peeper_server
        .add_on_input_char_listener(move |c| {
            let performer = ctx.performer.clone();

            ctx.main_handler.spawn(async move {
                performer.speak(c);
            });
        })
        .await;
}

// 朗读输入法切换
async fn speak_candidate(context: Arc<Context>) {
    let ctx = context.clone();

    context
        .peeper_server
        .add_on_ime_candidate_list_listener(move |candidate_list| {
            let performer = ctx.performer.clone();

            ctx.main_handler.spawn(async move {
                performer.speak(candidate_list);
            });
        })
        .await;
}
