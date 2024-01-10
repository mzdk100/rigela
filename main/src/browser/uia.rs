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
use crate::browser::Browseable;
use crate::context::Context;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use win_wrap::uia::UiAutomationElement;

impl Browseable for UiAutomationElement {
    fn get_name(&self) -> String {
        self.get_name()
    }

    fn get_role(&self) -> String {
        self.get_localized_control_type()
    }
}

pub(crate) async fn speak_desktop(context: Arc<Context>) {
    let ctx = Arc::clone(&context);
    let root = ctx.ui_automation.get_root_element();
    ctx.performer.speak(&root).await;

    sleep(Duration::from_millis(1000)).await;
}

pub(crate) async fn speak_focus_item(context: Arc<Context>) {
    let uia = Arc::clone(&context.ui_automation);
    let ctx = Arc::clone(&context);

    uia.add_focus_changed_listener(move |x| {
        let handle = Arc::clone(&ctx.main_handler);
        let performer = Arc::clone(&ctx.performer);

        handle.spawn(async move { performer.speak(&x).await });
    });
}

pub(crate) async fn watch_foreground_window(context: Arc<Context>) {
    let uia = Arc::clone(&context.ui_automation);
    let ctx = Arc::clone(&context);

    uia.add_focus_changed_listener(move |_| {
        if !form_browser::is_foreground_window_changed() {
            return;
        }

        form_browser::update_form_browser_hwnd();
        form_browser::clear_browseable();

        let uia2 = Arc::clone(&ctx.ui_automation);
        let elements = uia2.get_foreground_window_elements();
        for ele in elements {
            form_browser::add_browseable(Arc::new(ele));
        }
    });
}
