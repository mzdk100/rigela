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

use nwd::NwgUi;
use nwg::{EventData, NativeUi};

const TITLE: &str = "Rigela";
const INFO: &str = "RigelA是一个开源读屏项目，使用 rust 语言构建，我们尊重开放和自由，并持续为无障碍基础设施建设贡献力量，让每一个人平等享受科技是我们共同的目标！";
const BUTTON_LABEL: &str = "我要捐献";

#[derive(Default, NwgUi)]
pub struct WelcomeForm {
    #[nwg_control( title: TITLE, size: (480, 320), position: (300,300), flags:"WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 5)]
    layout: nwg::GridLayout,

    #[nwg_control(text: INFO, readonly: true, flags: "TAB_STOP|VISIBLE", focus: true)]
    #[nwg_layout_item(layout: layout, row: 0, col: 0, row_span: 4)]
    #[nwg_events(OnKeyPress: [WelcomeForm::on_key_press(SELF, EVT_DATA)])]
    text_box: nwg::TextBox,

    #[nwg_control(text: BUTTON_LABEL, size: (100, 30), flags: "TAB_STOP|VISIBLE")]
    #[nwg_layout_item(layout: layout, row: 4, col: 0)]
    #[nwg_events(OnButtonClick: [WelcomeForm::on_btn_click])]
    btn: nwg::Button,
}

impl WelcomeForm {
    fn on_key_press(&self, data: &EventData) {
        if data.on_key() == nwg::keys::TAB {
            self.btn.set_focus();
        }
    }

    fn on_btn_click(&self) {
        // Todo: 捐献按钮点击事件，带实现

        nwg::modal_info_message(&self.window, "Rigela", &format!("感谢支持!"));
    }
}

/// 显示欢迎窗口
pub(crate) fn show() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let _app = WelcomeForm::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
