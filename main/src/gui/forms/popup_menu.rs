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
    bring_window_front,
    context::Context,
    gui::command::{
        about_form_cmd, check_update_cmd, custom_hotkeys_cmd, donate_cmd, exit_cmd, help_cmd,
        settings_cmd, visit_host_website_cmd, welcome_form_cmd,
    },
};
use nwd::NwgUi;
use nwg::NoticeSender;
use rigela_macros::GuiFormImpl;
use std::sync::{Arc, OnceLock};

#[derive(Default, NwgUi, GuiFormImpl)]
pub struct PopupMenuForm {
    context: OnceLock<Arc<Context>>,

    #[nwg_control]
    window: nwg::MessageWindow,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: &t!("popupmenu.setting_item"))]
    #[nwg_events(OnMenuItemSelected: [PopupMenuForm::on_setting])]
    setting_item: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: & t ! ("popupmenu.donate_item"))]
    #[nwg_events(OnMenuItemSelected: [PopupMenuForm::on_donate])]
    donate_item: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: &t!("popupmenu.welcome_item"))]
    #[nwg_events(OnMenuItemSelected: [PopupMenuForm::on_welcome_form])]
    welcome_form_item: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: &t!("popupmenu.custom_hotkeys_item"))]
    #[nwg_events(OnMenuItemSelected: [PopupMenuForm::on_custom_hotkeys])]
    costom_hotkeys_item: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: &t!("popupmenu.help_item"))]
    out_help_item: nwg::Menu,

    #[nwg_control(parent: out_help_item, text: &t!("popupmenu.visit_host_item"))]
    #[nwg_events(OnMenuItemSelected: [PopupMenuForm::on_visit_host])]
    visit_host_item: nwg::MenuItem,

    #[nwg_control(parent: out_help_item, text: &t!("popupmenu.help_item"))]
    #[nwg_events(OnMenuItemSelected: [PopupMenuForm::on_help])]
    help_item: nwg::MenuItem,

    #[nwg_control(parent: out_help_item, text: &t!("popupmenu.check_update_item"))]
    #[nwg_events(OnMenuItemSelected: [PopupMenuForm::on_check_update])]
    check_update_item: nwg::MenuItem,

    #[nwg_control(parent: out_help_item, text: &t!("popupmenu.about_item"))]
    #[nwg_events(OnMenuItemSelected: [PopupMenuForm::on_about])]
    about_item: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: &t!("popupmenu.exit_item"))]
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

    fn on_about(&self) {
        about_form_cmd(self.context.get().unwrap().clone());
    }

    fn on_visit_host(&self) {
        visit_host_website_cmd(self.context.get().unwrap().clone());
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
