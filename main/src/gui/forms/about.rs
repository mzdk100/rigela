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

use crate::{bring_window_front, context::Context};
use nwd::NwgUi;
use nwg::{EventData, NoticeSender};
use rigela_macros::GuiFormImpl;
use std::sync::{OnceLock, Weak};

const SIZE: (u32, u32) = (320, 240);

#[derive(Default, NwgUi, GuiFormImpl)]
pub struct AboutForm {
    context: OnceLock<Weak<Context>>,

    #[nwg_control(title: & t ! ("about.title"), size: (0, 0), position: (300, 300), flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [AboutForm::on_exit], OnInit: [AboutForm::on_init])]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 5)]
    layout: nwg::GridLayout,

    #[nwg_control(text: & t ! ("about.info"), readonly: true, flags: "TAB_STOP|VISIBLE", focus: true)]
    #[nwg_layout_item(layout: layout, row: 0, col: 0, row_span: 4, col_span: 6)]
    #[nwg_events(OnKeyPress: [AboutForm::on_key_press(SELF, EVT_DATA)])]
    text_box: nwg::TextBox,

    #[nwg_control(text: & t ! ("about.btn_ok"), size: (100, 30), flags: "TAB_STOP|VISIBLE")]
    #[nwg_layout_item(layout: layout, row: 4, col: 2, col_span: 2)]
    #[nwg_events(OnButtonClick: [AboutForm::on_btn_ok])]
    btn_ok: nwg::Button,

    #[nwg_control()]
    #[nwg_events(OnNotice: [AboutForm::on_show_notice])]
    show_notice: nwg::Notice,

    #[nwg_control()]
    #[nwg_events(OnNotice: [AboutForm::on_exit_notice])]
    exit_notice: nwg::Notice,
}

impl AboutForm {
    fn on_init(&self) {
        self.window.set_visible(false);
    }

    fn on_exit(&self) {
        self.window.set_visible(false);
    }

    fn on_key_press(&self, data: &EventData) {
        if data.on_key() == nwg::keys::TAB {
            self.btn_ok.set_focus();
        }
    }

    fn on_btn_ok(&self) {
        self.window.set_visible(false);
    }

    fn on_show_notice(&self) {
        bring_window_front!(&self.window);
        self.window.set_size(SIZE.0, SIZE.1);
        self.window.set_visible(true)
    }

    fn on_exit_notice(&self) {
        nwg::stop_thread_dispatch()
    }
}
