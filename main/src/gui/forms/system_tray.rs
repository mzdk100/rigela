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
    configs::operations::get_auto_check_update,
    context::Context,
    gui::command::{check_update_cmd, exit_cmd, help_cmd, settings_cmd},
};
use native_windows_derive::NwgUi;
use native_windows_gui::{
    stop_thread_dispatch, GlobalCursor, Icon, Menu, MenuItem, MessageWindow, Notice, NoticeSender,
    TrayNotification,
};
use rigela_macros::GuiFormImpl;
use std::sync::{OnceLock, Weak};

#[derive(Default, NwgUi, GuiFormImpl)]
pub struct SystemTray {
    context: OnceLock<Weak<Context>>,

    #[nwg_control]
    #[nwg_events(OnInit: [SystemTray::on_init])]
    window: MessageWindow,

    #[nwg_resource(source_file: Some("./test_rc/cog.ico"))]
    icon: Icon,

    #[nwg_control(icon: Some(& data.icon), tip: Some(& t ! ("tray.tip")))]
    #[nwg_events(MousePressLeftUp: [SystemTray::show_menu], OnContextMenu: [SystemTray::show_menu])]
    tray: TrayNotification,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: Menu,

    #[nwg_control(parent: tray_menu, text: & t ! ("tray.setting_item"))]
    #[nwg_events(OnMenuItemSelected: [SystemTray::on_setting])]
    setting_item: MenuItem,

    #[nwg_control(parent: tray_menu, text: & t ! ("tray.help_item"))]
    #[nwg_events(OnMenuItemSelected: [SystemTray::on_help])]
    help_item: MenuItem,

    #[nwg_control(parent: tray_menu, text: & t ! ("tray.exit_item"))]
    #[nwg_events(OnMenuItemSelected: [SystemTray::on_exit])]
    exit_item: MenuItem,

    #[nwg_control()]
    #[nwg_events(OnNotice: [SystemTray::on_show_notice])]
    show_notice: Notice,

    #[nwg_control()]
    #[nwg_events(OnNotice: [SystemTray::on_exit_notice])]
    exit_notice: Notice,
}

impl SystemTray {
    fn on_init(&self) {
        // 启动程序自动检查更新
        if get_auto_check_update(self.context.get().unwrap().clone()) {
            check_update_cmd(self.context.get().unwrap().clone(), true);
        }
    }

    fn show_menu(&self) {
        let (x, y) = GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn on_setting(&self) {
        settings_cmd(self.context.get().unwrap().clone());
    }

    fn on_help(&self) {
        help_cmd(self.context.get().unwrap().clone());
    }

    fn on_exit(&self) {
        exit_cmd(self.context.get().unwrap().clone());
    }

    fn on_show_notice(&self) {}

    fn on_exit_notice(&self) {
        stop_thread_dispatch();
    }
}
