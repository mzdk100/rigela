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

use nwd::NwgUi;
use nwg::{EventData, NativeUi};

const TITLE: &str = "Rigela - 更新";

#[derive(Default, NwgUi)]
pub struct App {
    #[nwg_control( title: TITLE, size: (480, 320), position: (300,300))]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()   ] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 5)]
    layout: nwg::GridLayout,

    #[nwg_control(text: "更新日志:")]
    #[nwg_layout_item(layout: layout, row: 0, col: 0)]
    #[nwg_events(OnKeyPress: [App::on_key_press(SELF, EVT_DATA)])]
    label: nwg::Label,

    #[nwg_control(readonly: true)]
    #[nwg_layout_item(layout: layout, row: 1, col: 0, row_span: 6, col_span: 4)]
    #[nwg_events(OnKeyPress: [App::on_key_press(SELF, EVT_DATA)])]
    text_box: nwg::TextBox,

    #[nwg_control(text: "立即更新 (&U)", size: (100, 30))]
    #[nwg_layout_item(layout: layout, row: 7, col: 2)]
    #[nwg_events(OnButtonClick: [App::on_update])]
    update_btn: nwg::Button,

    #[nwg_control(text: "取消 (&C)", size: (100, 30))]
    #[nwg_layout_item(layout: layout, row: 7, col: 3)]
    #[nwg_events(OnButtonClick: [nwg::stop_thread_dispatch()])]
    cancel_btn: nwg::Button,
}

impl App {
    fn on_key_press(&self, data: &EventData) {
        if data.on_key() == nwg::keys::TAB {
            self.update_btn.set_focus();
        }
    }

    fn set_info(&self, info: &str) {
        self.text_box.set_text(info);
    }

    fn on_update(&self) {
        // Todo: 添加程序替换代码

        nwg::modal_info_message(&self.window, "提示:", &format!("更新完成!"));
    }
}

pub fn show(log_text: &str) {
    nwg::init().expect("Failed to init Native Windows GUI");
    let app = App::build_ui(Default::default()).expect("Failed to build UI");
    app.set_info(log_text);
    nwg::dispatch_thread_events();
}
