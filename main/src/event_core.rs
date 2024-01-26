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
use std::ops::DerefMut;
use std::sync::{Arc, Mutex, OnceLock};
use win_wrap::common::{get_foreground_window, HWND};

/// 事件处理中心
#[derive(Clone, Debug)]
pub struct EventCore;

impl EventCore {
    pub fn new() -> Self {
        Self
    }

    /// 启动事件监听
    pub async fn run(&self, context: Arc<Context>) {
        // 订阅UIA的焦点元素改变事件
        speak_focus_item(context.clone()).await;

        // 监听前台窗口变动
        watch_foreground_window(context.clone()).await;

        // 订阅输入事件
        let ctx = context.clone();
        context
            .peeper_server
            .add_on_input_char_listener(move |c| {
                let performer = ctx.performer.clone();
                ctx.main_handler.spawn(async move {
                    performer.speak_with_sapi5(&c).await;

                    // 这里是测试代码
                    performer.speak_with_vvtts(&c).await;
                });
            })
            .await;

        // 订阅输入法候选事件
        let ctx = context.clone();
        context
            .peeper_server
            .add_on_ime_candidate_list_listener(move |candidate_list| {
                let performer = ctx.performer.clone();
                ctx.main_handler.spawn(async move {
                    performer.speak_with_sapi5(&candidate_list).await;
                });
            })
            .await;
    }
}

/// 朗读焦点元素
async fn speak_focus_item(context: Arc<Context>) {
    let uia = context.ui_automation.clone();
    let ctx = context.clone();

    // 给UI Automation的焦点改变绑定处理事件
    uia.add_focus_changed_listener(move |x| {
        let performer = ctx.performer.clone();

        // 异步执行元素朗读
        ctx.main_handler.spawn(async move {
            performer.speak_with_sapi5(&x).await;
        });
    });
}

// 存储前台窗口句柄
fn get_old_foreground_window_hwnd() -> &'static Mutex<HWND> {
    static INSTANCE: OnceLock<Mutex<HWND>> = OnceLock::new();
    INSTANCE.get_or_init(|| Mutex::new(HWND::default()))
}

/// 监测前台窗口变动，发送控件元素到form_browser
async fn watch_foreground_window(context: Arc<Context>) {
    let ctx = context.clone();

    // 给UI Automation的焦点改变绑定处理事件
    context.ui_automation.add_focus_changed_listener(move |_| {
        // 如果前台窗口没有变动直接返回
        let handle = get_foreground_window();
        if handle == *get_old_foreground_window_hwnd().lock().unwrap() {
            return;
        }

        // 保存新的前台窗口句柄
        *get_old_foreground_window_hwnd().lock().unwrap().deref_mut() = handle;

        // form_browser需要异步操作
        let main_handler = ctx.main_handler.clone();
        if let Some(root) = ctx.ui_automation.element_from_handle(handle) {
            let form_browser = ctx.form_browser.clone();
            main_handler.spawn(async move { form_browser.render(Arc::new(root)).await });
        }
    });
}
