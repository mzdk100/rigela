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

use crate::browser::{get_form_browser, Browseable};
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
    ctx.performer
        .speak(&ctx.ui_automation.get_root_element())
        .await;
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
        let fb = get_form_browser().lock().unwrap();
        if !fb.is_foreground_window_changed() {
            return;
        }

        let uia2 = Arc::clone(&ctx.ui_automation);
        get_form_browser().lock().unwrap().update_hwnd();
        get_form_browser().lock().unwrap().clear();

        let elements = uia2.get_foreground_window_elements();
        for ele in elements {
            get_form_browser().lock().unwrap().add(Box::new(ele));
        }
    });
}
