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
use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct PopupMenu {
    #[nwg_control]
    #[nwg_events(OnInit: [PopupMenu::on_init])]
    window: nwg::MessageWindow,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: "设置 (&S)")]
    #[nwg_events(OnMenuItemSelected: [PopupMenu::on_setting])]
    setting_item: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "帮助 (&H)")]
    #[nwg_events(OnMenuItemSelected: [PopupMenu::on_help])]
    help_item: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "退出 (&X)")]
    #[nwg_events(OnMenuItemSelected: [PopupMenu::on_exit])]
    exit_item: nwg::MenuItem,
}

impl PopupMenu {
    fn on_init(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn on_setting(&self) {
        // Todo:
        nwg::simple_message("RigelA", "Start Setting");

        nwg::stop_thread_dispatch();
    }

    fn on_help(&self) {
        // Todo:
        nwg::simple_message("RigelA", "Start Help");

        nwg::stop_thread_dispatch();
    }

    fn on_exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

pub(crate) fn show() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let ui = PopupMenu::build_ui(Default::default()).expect("Failed to build UI");
    bring_window_front!(&ui.window);
    nwg::dispatch_thread_events();
}
