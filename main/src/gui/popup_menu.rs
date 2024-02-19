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

use crate::bring_window_front;
use crate::context::Context;
use crate::gui::window_manager::Formable;
use crate::talent::Talented;
use nwd::NwgUi;
use nwg::{NativeUi, NoticeSender};
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

#[derive(Default, NwgUi)]
pub struct PopupMenuForm {
    context: Mutex<Option<Arc<Context>>>,

    #[nwg_control]
    #[nwg_events(OnInit: [PopupMenuForm::on_init])]
    window: nwg::MessageWindow,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: "设置 (&S)")]
    #[nwg_events(OnMenuItemSelected: [PopupMenuForm::on_setting])]
    setting_item: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "帮助 (&H)")]
    #[nwg_events(OnMenuItemSelected: [PopupMenuForm::on_help])]
    help_item: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "退出 (&X)")]
    #[nwg_events(OnMenuItemSelected: [PopupMenuForm::on_exit])]
    exit_item: nwg::MenuItem,

    #[nwg_control()]
    #[nwg_events(OnNotice: [PopupMenuForm::on_show_notice])]
    show_notice: nwg::Notice,

    #[nwg_control()]
    #[nwg_events(OnNotice: [PopupMenuForm::on_exit_notice])]
    exit_notice: nwg::Notice,
}

impl PopupMenuForm {
    fn on_init(&self) {}

    fn on_setting(&self) {
        // Todo:
        nwg::simple_message("RigelA", "Start Setting");
    }

    fn on_help(&self) {
        // Todo:
        nwg::simple_message("RigelA", "Start Help");
    }

    fn on_exit(&self) {
        let context = self.context.lock().unwrap().clone();
        if let Some(context) = context {
            let ctx = context.clone();
            context.main_handler.spawn(async move {
                ctx.talent_accessor
                    .get_exit_talent()
                    .perform(ctx.clone())
                    .await;
            });
        }
        nwg::stop_thread_dispatch();
    }

    fn on_show_notice(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn on_exit_notice(&self) {
        nwg::stop_thread_dispatch()
    }
}

impl Formable for PopupMenuForm {
    fn set_context(&self, context: Arc<Context>) {
        *self.context.lock().unwrap().deref_mut() = Some(context.clone());
    }

    fn get_show_notice_sender(&self) -> NoticeSender {
        self.show_notice.sender().clone()
    }

    fn get_exit_notice_sender(&self) -> NoticeSender {
        self.exit_notice.sender().clone()
    }
}

#[allow(unused)]
fn show(context: Arc<Context>) {
    nwg::init().expect("Failed to init Native Windows GUI");
    let ui = PopupMenuForm::build_ui(Default::default()).expect("Failed to build UI");
    ui.set_context(context.clone());
    bring_window_front!(&ui.window);
    nwg::dispatch_thread_events();
}
