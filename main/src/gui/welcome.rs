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
use crate::gui::utils::HELP_DIR;
use nwd::NwgUi;
use nwg::{EventData, NativeUi};
use rigela_utils::get_program_directory;
use std::process::Command;

const INFO: &str = "RigelA是一个开源读屏项目，使用 rust 语言构建，我们尊重开放和自由，并持续为无障碍基础设施建设贡献力量，让每一个人平等享受科技是我们共同的目标！";

#[derive(Default, NwgUi)]
pub struct WelcomeForm {
    #[nwg_control( title: &t!("welcome.title"), size: (480, 320), position: (300,300), flags:"WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()] )]
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
}

impl WelcomeForm {
    fn on_key_press(&self, data: &EventData) {
        if data.on_key() == nwg::keys::TAB {
            self.btn_donate.set_focus();
        }
    }

    fn on_btn_donate(&self) {
        // Todo: 捐献按钮点击事件，带实现

        nwg::modal_info_message(&self.window, "Rigela", "感谢支持!");
    }

    fn on_btn_setting(&self) {
        // Todo: 设置按钮点击事件，带实现
        nwg::modal_info_message(&self.window, "Rigela", "设置");
    }

    fn on_btn_help(&self) {
        let help_path = get_program_directory().join(HELP_DIR);
        Command::new("notepad")
            .arg(help_path)
            .spawn()
            .expect("Failed to start notepad");
    }

    fn on_btn_close(&self) {
        nwg::stop_thread_dispatch();
    }
}

/// 显示欢迎窗口
pub(crate) fn show() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let ui = WelcomeForm::build_ui(Default::default()).expect("Failed to build UI");
    bring_window_front!(ui.window);
    nwg::dispatch_thread_events();
}
