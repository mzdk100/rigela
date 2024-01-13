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

use crate::browser::form_browser;
use crate::context::Context;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex, OnceLock};
use win_wrap::common::{get_foreground_window, HWND};

/// 事件处理中心
#[derive(Clone)]
pub struct EventCore;

impl EventCore {
    pub fn new() -> Self {
        Self
    }

    /// 启动事件监听
    pub async fn run(&self, context: Arc<Context>) {
        // 订阅UIA的焦点元素改变事件
        speak_focus_item(Arc::clone(&context.clone())).await;

        // 监听前台窗口变动
        watch_foreground_window(Arc::clone(&context.clone())).await;
    }
}

/// 朗读焦点元素
async fn speak_focus_item(context: Arc<Context>) {
    let uia = Arc::clone(&context.ui_automation);
    let ctx = Arc::clone(&context);

    uia.add_focus_changed_listener(move |x| {
        let handle = Arc::clone(&ctx.main_handler);
        let performer = Arc::clone(&ctx.performer);

        handle.spawn(async move { performer.speak(&x).await });
    });
}

// 存储前台窗口句柄
fn get_old_foreground_window_hwnd() -> &'static Mutex<HWND> {
    static INSTANCE: OnceLock<Mutex<HWND>> = OnceLock::new();
    INSTANCE.get_or_init(|| Mutex::new(HWND::default()))
}

/// 监测前台窗口变动，发送控件元素到form_browser
async fn watch_foreground_window(context: Arc<Context>) {
    let uia = Arc::clone(&context.ui_automation);
    let ctx = Arc::clone(&context);

    uia.add_focus_changed_listener(move |_| {
        if get_foreground_window() == *get_old_foreground_window_hwnd().lock().unwrap() {
            return;
        }

        *get_old_foreground_window_hwnd().lock().unwrap().deref_mut() = get_foreground_window();
        form_browser::clear();

        let uia2 = Arc::clone(&ctx.ui_automation);
        let elements = uia2.get_foreground_window_elements();
        for ele in elements {
            form_browser::add(Arc::new(ele));
        }
    });
}
