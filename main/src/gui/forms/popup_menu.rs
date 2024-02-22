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
use crate::gui::command::{
    check_update_cmd, custom_hotkeys_cmd, donate_cmd, exit_cmd, help_cmd, settings_cmd,
    welcome_form_cmd,
};
use crate::gui::window_manager::Formable;
use nwd::NwgUi;
use nwg::NoticeSender;
use std::sync::{Arc, OnceLock};

#[derive(Default, NwgUi)]
pub struct PopupMenuForm {
    context: OnceLock<Arc<Context>>,

    #[nwg_control]
    window: nwg::MessageWindow,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: "设置 (&S)")]
    #[nwg_events(OnMenuItemSelected: [PopupMenuForm::on_setting])]
    setting_item: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: &t!("welcome.btn_donate"))]
    #[nwg_events(OnMenuItemSelected: [PopupMenuForm::on_donate])]
    donate_item: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "欢迎窗口 (&W)")]
    #[nwg_events(OnMenuItemSelected: [PopupMenuForm::on_welcome_form])]
    welcome_form_item: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "自定义快捷键 (&K)")]
    #[nwg_events(OnMenuItemSelected: [PopupMenuForm::on_custom_hotkeys])]
    costom_hotkeys_item: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "帮助 (&H)")]
    #[nwg_events(OnMenuItemSelected: [PopupMenuForm::on_help])]
    help_item: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "检测升级 (&U)")]
    #[nwg_events(OnMenuItemSelected: [PopupMenuForm::on_check_update])]
    check_update_item: nwg::MenuItem,

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
    fn on_setting(&self) {
        settings_cmd(self.context.get().unwrap().clone());
    }

    fn on_donate(&self) {
        donate_cmd(self.context.get().unwrap().clone());
    }

    fn on_welcome_form(&self) {
        welcome_form_cmd(self.context.get().unwrap().clone());
    }

    fn on_custom_hotkeys(&self) {
        custom_hotkeys_cmd(self.context.get().unwrap().clone());
    }

    fn on_help(&self) {
        help_cmd(self.context.get().unwrap().clone());
    }

    fn on_check_update(&self) {
        check_update_cmd(self.context.get().unwrap().clone(), false);
    }

    fn on_exit(&self) {
        exit_cmd(self.context.get().unwrap().clone());
    }

    fn on_show_notice(&self) {
        bring_window_front!(&self.window);
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn on_exit_notice(&self) {
        nwg::stop_thread_dispatch()
    }
}

impl Formable for PopupMenuForm {
    fn set_context(&self, context: Arc<Context>) {
        self.context.set(context.clone()).unwrap();
    }

    fn get_show_notice_sender(&self) -> NoticeSender {
        self.show_notice.sender().clone()
    }

    fn get_exit_notice_sender(&self) -> NoticeSender {
        self.exit_notice.sender().clone()
    }
}
