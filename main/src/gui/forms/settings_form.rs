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
use nwd::{NwgPartial, NwgUi};
use nwg::NoticeSender;
use std::sync::{Arc, OnceLock};

const MENUS: [&str; 4] = ["常规设置", "语音设置", "鼠标设置", "高级设置"];

#[derive(Default, NwgUi)]
pub struct SettingsForm {
    context: OnceLock<Arc<Context>>,

    #[nwg_control(size: (800, 600), position: (200, 200), title: "RigelA - 设置")]
    #[nwg_events(OnWindowClose: [SettingsForm::on_exit], OnInit: [SettingsForm::on_init])]
    window: nwg::Window,

    #[nwg_layout(parent: window)]
    layout: nwg::FlexboxLayout,

    #[nwg_control(collection: MENUS.to_vec())]
    #[nwg_layout_item(layout: layout)]
    #[nwg_events(OnListBoxSelect: [SettingsForm::change_interface])]
    menu: nwg::ListBox<&'static str>,

    #[nwg_control]
    #[nwg_layout_item(layout: layout)]
    general_frame: nwg::Frame,

    #[nwg_control(flags: "BORDER")]
    voice_frame: nwg::Frame,

    #[nwg_control(flags: "BORDER")]
    mouse_frame: nwg::Frame,

    #[nwg_control(flags: "BORDER")]
    advanced_frame: nwg::Frame,

    #[nwg_partial(parent: general_frame)]
    #[nwg_events((save_btn, OnButtonClick): [SettingsForm::save])]
    general_ui: GeneralUi,

    #[nwg_partial(parent: voice_frame)]
    #[nwg_events((save_btn, OnButtonClick): [SettingsForm::save])]
    voice_ui: VoiceUi,

    #[nwg_partial(parent: mouse_frame)]
    #[nwg_events((save_btn, OnButtonClick): [SettingsForm::save])]
    mouse_ui: MouseUi,

    #[nwg_partial(parent: advanced_frame)]
    #[nwg_events((save_btn, OnButtonClick): [SettingsForm::save])]
    advanced_ui: AdvancedUi,

    #[nwg_control()]
    #[nwg_events(OnNotice: [SettingsForm::on_show_notice])]
    show_notice: nwg::Notice,

    #[nwg_control()]
    #[nwg_events(OnNotice: [SettingsForm::on_exit_notice])]
    exit_notice: nwg::Notice,
}

impl SettingsForm {
    fn change_interface(&self) {
        let frames = [
            &self.general_frame,
            &self.voice_frame,
            &self.mouse_frame,
            &self.advanced_frame,
        ];

        frames.map(|frame| {
            frame.set_visible(false);
            if self.layout.has_child(frame) {
                self.layout.remove_child(frame);
            }
        });

        use nwg::stretch::{
            geometry::Size,
            style::{Dimension as D, Style},
        };
        let mut style = Style::default();
        style.size = Size {
            width: D::Percent(1.0),
            height: D::Auto,
        };

        match self.menu.selection() {
            Some(n) => {
                self.layout.add_child(frames[n], style).unwrap();
                frames[n].set_visible(true);
            }
            _ => {
                self.layout.add_child(frames[0], style).unwrap();
                frames[0].set_visible(true);
            }
        }
    }

    fn save(&self) {
        self.window.set_visible(false);
    }

    fn on_init(&self) {
        self.window.set_visible(false);
    }

    fn on_exit(&self) {
        self.window.set_visible(false);
    }

    fn on_show_notice(&self) {
        bring_window_front!(&self.window);
        self.window.set_visible(true);
        self.menu.set_focus();
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

#[derive(Default, NwgPartial)]
pub struct GeneralUi {
    #[nwg_layout(max_size: [1200, 800], min_size: [680, 480], spacing: 20, max_column: Some(3), max_row: Some(8))]
    layout: nwg::GridLayout,

    #[nwg_layout(min_size: [600, 480], max_column: Some(2), max_row: Some(8))]
    layout2: nwg::GridLayout,

    #[nwg_control(text: "开机启动 (&R)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 1)]
    ck_run_on_startup: nwg::CheckBox,

    #[nwg_control(text: "自动更新 (&A)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 2)]
    ck_auot_update: nwg::CheckBox,

    #[nwg_control(text: "检查更新 (&C)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 3)]
    btn_check_update: nwg::Button,

    #[nwg_control(text: "语言 (&L)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 5)]
    lb_lang: nwg::Label,

    #[nwg_control(collection: vec ! ["中文", "English"])]
    #[nwg_layout_item(layout: layout, col: 2, row: 5)]
    cb_lang: nwg::ComboBox<&'static str>,

    #[nwg_control(text: "保存 (&S)")]
    #[nwg_layout_item(layout: layout2, col: 1, row: 7)]
    save_btn: nwg::Button,
}

#[derive(Default, NwgPartial)]
pub struct VoiceUi {
    #[nwg_layout(max_size: [1200, 800], min_size: [680, 480], spacing: 20, max_column: Some(4), max_row: Some(6))]
    layout: nwg::GridLayout,

    #[nwg_layout(min_size: [600, 480], max_column: Some(2), max_row: Some(6))]
    layout2: nwg::GridLayout,

    #[nwg_control(text: "朗读角色 (&R)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 1)]
    lb_role: nwg::Label,

    #[nwg_control(collection: vec ! [])]
    #[nwg_layout_item(layout: layout, col: 2, row: 1)]
    cb_role: nwg::ComboBox<&'static str>,

    #[nwg_control(text: "朗读语速 (&E)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 2)]
    lb_speed: nwg::Label,

    #[nwg_control(collection: vec ! [])]
    #[nwg_layout_item(layout: layout, col: 2, row: 2)]
    cb_speed: nwg::ComboBox<&'static str>,

    #[nwg_control(text: "朗读语调 (&P)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 3)]
    lb_pitch: nwg::Label,

    #[nwg_control(collection: vec ! [])]
    #[nwg_layout_item(layout: layout, col: 2, row: 3)]
    cb_pitch: nwg::ComboBox<&'static str>,

    #[nwg_control(text: "朗读音量 (&O)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 4)]
    lb_volume: nwg::Label,

    #[nwg_control(collection: vec ! [])]
    #[nwg_layout_item(layout: layout, col: 2, row: 4)]
    cb_volume: nwg::ComboBox<&'static str>,

    #[nwg_control(text: "保存 (&S)")]
    #[nwg_layout_item(layout: layout2, col: 1, row: 5)]
    save_btn: nwg::Button,
}

#[derive(Default, NwgPartial)]
pub struct MouseUi {
    #[nwg_layout(max_size: [1200, 800], min_size: [680, 480], spacing: 20, max_column: Some(3), max_row: Some(6))]
    layout: nwg::GridLayout,

    #[nwg_layout(min_size: [600, 480], max_column: Some(2), max_row: Some(6))]
    layout2: nwg::GridLayout,

    #[nwg_control(text: "朗读鼠标 (&R)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 1)]
    ck_mouse_read: nwg::CheckBox,

    #[nwg_control(text: "保存 (&S)")]
    #[nwg_layout_item(layout: layout2, col: 1, row: 5)]
    save_btn: nwg::Button,
}

#[derive(Default, NwgPartial)]
pub struct AdvancedUi {
    #[nwg_layout(max_size: [1200, 800], min_size: [680, 480], spacing: 20, max_column: Some(3), max_row: Some(6))]
    layout: nwg::GridLayout,

    #[nwg_layout(min_size: [600, 480], max_column: Some(2), max_row: Some(6))]
    layout2: nwg::GridLayout,

    #[nwg_control(text: "导入配置... (&I)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 1)]
    btn_import: nwg::Button,

    #[nwg_control(text: "导出配置... (&E)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 2)]
    btn_export: nwg::Button,

    #[nwg_control(text: "恢复默认配置 (&R)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 3)]
    btn_reset: nwg::Button,

    #[nwg_control(text: "保存 (&S)")]
    #[nwg_layout_item(layout: layout2, col: 1, row: 5)]
    save_btn: nwg::Button,
}
