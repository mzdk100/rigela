/*
 * Copyright (c) 2023. The RigelA open source project team and
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
use crate::gui::command::{donate_cmd, help_cmd, settings_cmd};
use crate::gui::window_manager::Formable;
use nwd::NwgUi;
use nwg::{EventData, NoticeSender};
use std::sync::{Arc, OnceLock};

const INFO: &str = "RigelA是一个开源读屏项目，使用 rust 语言构建，我们尊重开放和自由，并持续为无障碍基础设施建设贡献力量，让每一个人平等享受科技是我们共同的目标！";

#[derive(Default, NwgUi)]
pub struct WelcomeForm {
    context: OnceLock<Arc<Context>>,

    #[nwg_control( title: &t!("welcome.title"), size: (480, 320), position: (300,300), flags:"WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [WelcomeForm::on_exit] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 5)]
    layout: nwg::GridLayout,

    #[nwg_control(text: INFO, readonly: true, flags: "TAB_STOP|VISIBLE", focus: true)]
    #[nwg_layout_item(layout: layout, row: 0, col: 0, row_span: 4, col_span: 6)]
    #[nwg_events(OnKeyPress: [WelcomeForm::on_key_press(SELF, EVT_DATA)])]
    text_box: nwg::TextBox,

    #[nwg_control(text: &t!("welcome.btn_donate"), size: (100, 30), flags: "TAB_STOP|VISIBLE")]
    #[nwg_layout_item(layout: layout, row: 4, col: 1, col_span: 4)]
    #[nwg_events(OnButtonClick: [WelcomeForm::on_btn_donate])]
    btn_donate: nwg::Button,

    #[nwg_control(text: &t!("welcome.btn_setting"), size: (100, 30), flags: "TAB_STOP|VISIBLE")]
    #[nwg_layout_item(layout: layout, row: 5, col: 3)]
    #[nwg_events(OnButtonClick: [WelcomeForm::on_btn_setting])]
    btn_setting: nwg::Button,

    #[nwg_control(text: &t!("welcome.btn_help"), size: (100, 30), flags: "TAB_STOP|VISIBLE")]
    #[nwg_layout_item(layout: layout, row: 5, col: 4)]
    #[nwg_events(OnButtonClick: [WelcomeForm::on_btn_help])]
    btn_help: nwg::Button,

    #[nwg_control(text: &t!("welcome.btn_close"), size: (100, 30), flags: "TAB_STOP|VISIBLE")]
    #[nwg_layout_item(layout: layout, row: 5, col: 5)]
    #[nwg_events(OnButtonClick: [WelcomeForm::on_btn_close])]
    btn_close: nwg::Button,

    #[nwg_control()]
    #[nwg_events(OnNotice: [WelcomeForm::on_show_notice])]
    show_notice: nwg::Notice,

    #[nwg_control()]
    #[nwg_events(OnNotice: [WelcomeForm::on_exit_notice])]
    exit_notice: nwg::Notice,
}

impl WelcomeForm {
    // 屏蔽关闭窗口事件退出UI线程
    fn on_exit(&self) {
        self.window.set_visible(false);
    }

    // 修复编辑框Tab切换到按钮
    fn on_key_press(&self, data: &EventData) {
        if data.on_key() == nwg::keys::TAB {
            self.btn_donate.set_focus();
        }
    }

    fn on_btn_donate(&self) {
        donate_cmd(self.context.get().unwrap().clone());
    }

    fn on_btn_setting(&self) {
        settings_cmd(self.context.get().unwrap().clone());
    }

    fn on_btn_help(&self) {
        help_cmd(self.context.get().unwrap().clone());
    }

    fn on_btn_close(&self) {
        self.window.set_visible(false);
    }

    fn on_show_notice(&self) {
        bring_window_front!(&self.window);
        self.window.set_visible(true)
    }

    fn on_exit_notice(&self) {
        nwg::stop_thread_dispatch()
    }
}

impl Formable for WelcomeForm {
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
