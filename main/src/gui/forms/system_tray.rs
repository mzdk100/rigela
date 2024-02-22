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
use crate::gui::command::{check_update_cmd, exit_cmd, help_cmd, settings_cmd};
use crate::gui::window_manager::Formable;
use nwd::NwgUi;
use nwg::NoticeSender;
use std::sync::{Arc, OnceLock};

#[derive(Default, NwgUi)]
pub struct SystemTray {
    context: OnceLock<Arc<Context>>,

    #[nwg_control]
    #[nwg_events (OnInit: [SystemTray::on_init] )]
    window: nwg::MessageWindow,

    #[nwg_resource(source_file: Some("./test_rc/cog.ico"))]
    icon: nwg::Icon,

    #[nwg_control(icon: Some(& data.icon), tip: Some("RigelA"))]
    #[nwg_events(MousePressLeftUp: [SystemTray::show_menu], OnContextMenu: [SystemTray::show_menu])]
    tray: nwg::TrayNotification,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: "设置 (&S)")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::on_setting])]
    setting_item: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "帮助 (&H)")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::on_help])]
    help_item: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "退出 (&X)")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::on_exit])]
    exit_item: nwg::MenuItem,

    #[nwg_control()]
    #[nwg_events(OnNotice: [SystemTray::on_show_notice])]
    show_notice: nwg::Notice,

    #[nwg_control()]
    #[nwg_events(OnNotice: [SystemTray::on_exit_notice])]
    exit_notice: nwg::Notice,
}

impl SystemTray {
    fn on_init(&self) {
        // 启动程序自动检查更新
        check_update_cmd(self.context.get().unwrap().clone(), true);
    }

    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
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
        nwg::stop_thread_dispatch();
    }
}

impl Formable for SystemTray {
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