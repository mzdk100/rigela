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
use crate::gui::window_manager::Formable;
use nwd::NwgUi;
use nwg::NoticeSender;
use std::sync::{Arc, OnceLock};

#[derive(Default, NwgUi)]
pub struct SettingsForm {
    context: OnceLock<Arc<Context>>,

    #[nwg_control( title: "RigelA 设置", size: (640, 480), position: (200,200), flags:"WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [SettingsForm::on_exit], OnInit: [SettingsForm::on_init] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 5)]
    layout: nwg::GridLayout,

    #[nwg_control()]
    #[nwg_events(OnNotice: [SettingsForm::on_show_notice])]
    show_notice: nwg::Notice,

    #[nwg_control()]
    #[nwg_events(OnNotice: [SettingsForm::on_exit_notice])]
    exit_notice: nwg::Notice,
}

impl SettingsForm {
    fn on_init(&self) {
        self.window.set_visible(false);
    }

    fn on_exit(&self) {
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

impl Formable for SettingsForm {
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
