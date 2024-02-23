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

use crate::commander::keys::Keys;
use crate::gui::command::{
    check_update_cmd, export_config_cmd, import_config_cmd, reset_config_cmd,
    set_auto_check_update_cmd, set_auto_start_cmd, set_lang_cmd, set_mouse_read_cmd, set_pitch_cmd,
    set_speed_cmd, set_voice_cmd, set_volume_cmd,
};
use crate::gui::forms::hotkeys::HotKeysUi;
use crate::{bring_window_front, context::Context};
use nwd::{NwgPartial, NwgUi};
use nwg::stretch::{
    geometry::Size,
    style::{Dimension as D, Style},
};
use nwg::{CheckBox, CheckBoxState, NoticeSender};
use rigela_macros::GuiFormImpl;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use win_wrap::hook::WindowsHook;

const MENUS: [&str; 5] = ["常规设置", "语音设置", "热键自定义", "鼠标设置", "高级设置"];
const FRAME_SIZE: Size<D> = Size {
    width: D::Percent(1.0),
    height: D::Auto,
};

#[derive(Default, NwgUi, GuiFormImpl)]
pub struct SettingsForm {
    pub(crate) context: OnceLock<Arc<Context>>,

    pub(crate) talents: RefCell<Arc<Vec<crate::gui::forms::hotkeys::Talent>>>,
    pub(crate) talent_keys: RefCell<HashMap<String, Vec<Keys>>>,
    pub(crate) hotkeys: Arc<Mutex<Vec<Keys>>>,
    pub(crate) hook: RefCell<Option<WindowsHook>>,

    #[nwg_control(size: (800, 600), position: (200, 200), title: "RigelA - 设置")]
    #[nwg_events(OnWindowClose: [SettingsForm::on_exit], OnInit: [SettingsForm::on_init, SettingsForm::load_data])]
    pub(crate) window: nwg::Window,

    #[nwg_layout(parent: window)]
    layout: nwg::FlexboxLayout,

    #[nwg_control(collection: MENUS.to_vec())]
    #[nwg_layout_item(layout: layout, size: Size{width: D::Points(150.0), height: D::Auto})]
    #[nwg_events(OnListBoxSelect: [SettingsForm::change_interface])]
    menu: nwg::ListBox<&'static str>,

    #[nwg_control]
    #[nwg_layout_item(layout: layout, size: FRAME_SIZE)]
    general_frame: nwg::Frame,

    #[nwg_control(flags: "BORDER")]
    voice_frame: nwg::Frame,

    #[nwg_control(flags: "BORDER")]
    hotkeys_frame: nwg::Frame,

    #[nwg_control(flags: "BORDER")]
    mouse_frame: nwg::Frame,

    #[nwg_control(flags: "BORDER")]
    advanced_frame: nwg::Frame,

    #[nwg_partial(parent: general_frame)]
    #[nwg_events(
    (ck_run_on_startup, OnButtonClick): [SettingsForm::on_run_on_startup(SELF, CTRL)],
    (ck_auot_update, OnButtonClick): [SettingsForm::on_auto_check_update(SELF, CTRL)],
    (btn_check_update, OnButtonClick): [SettingsForm::on_check_update],
    (cb_lang, OnComboxBoxSelection): [SettingsForm::on_lang_changed(SELF, CTRL)],
    (btn_save, OnButtonClick): [SettingsForm::on_save],
    )]
    general_ui: GeneralUi,

    #[nwg_partial(parent: voice_frame)]
    #[nwg_events(
    (cb_role, OnComboxBoxSelection): [SettingsForm::on_role_changed(SELF, CTRL)],
    (cb_speed, OnComboxBoxSelection): [SettingsForm::on_speed_changed(SELF, CTRL)],
    (cb_pitch, OnComboxBoxSelection): [SettingsForm::on_pitch_changed(SELF, CTRL)],
    (cb_volume, OnComboxBoxSelection): [SettingsForm::on_volume_changed(SELF, CTRL)],
    (btn_save, OnButtonClick): [SettingsForm::on_save],
    )]
    voice_ui: VoiceUi,

    #[nwg_partial(parent: hotkeys_frame)]
    #[nwg_events(
    (data_view, OnKeyRelease): [SettingsForm::on_dv_key_press(SELF, EVT_DATA)],
    (data_view, OnListViewItemChanged): [SettingsForm::on_dv_selection_changed],

    (set_btn, OnButtonClick): [SettingsForm::on_set_hotkey],
    (set_btn, OnKeyRelease): [SettingsForm::on_btn_key_release(SELF, EVT_DATA, HANDLE)],

    (clear_btn, OnButtonClick): [SettingsForm::on_clear_hotkey],
    (clear_btn, OnKeyRelease): [SettingsForm::on_btn_key_release(SELF, EVT_DATA, HANDLE)],

    (finish_custom, OnNotice): [SettingsForm::on_finish_custom],
    (cancel_custom, OnNotice): [SettingsForm::on_cancel_custom],

    (btn_save, OnButtonClick): [SettingsForm::on_save],
    )]
    pub(crate) hotkeys_ui: HotKeysUi,

    #[nwg_partial(parent: mouse_frame)]
    #[nwg_events(
    (ck_mouse_read, OnButtonClick): [SettingsForm::on_mouse_read(SELF, CTRL)],
    (btn_save, OnButtonClick): [SettingsForm::on_save],
    )]
    mouse_ui: MouseUi,

    #[nwg_partial(parent: advanced_frame)]
    #[nwg_events(
    (btn_import, OnButtonClick): [SettingsForm::on_import],
    (btn_export, OnButtonClick): [SettingsForm::on_export],
    (btn_reset, OnButtonClick): [SettingsForm::on_reset],
    (btn_save, OnButtonClick): [SettingsForm::on_save],
    )]
    advanced_ui: AdvancedUi,

    #[nwg_control()]
    #[nwg_events(OnNotice: [SettingsForm::on_show_notice])]
    show_notice: nwg::Notice,

    #[nwg_control()]
    #[nwg_events(OnNotice: [SettingsForm::on_exit_notice])]
    exit_notice: nwg::Notice,

    #[nwg_control()]
    #[nwg_events(OnNotice: [SettingsForm::on_show_hotkeys_notice])]
    pub(crate) show_hotkeys_notice: nwg::Notice,
}

impl SettingsForm {
    fn change_interface(&self) {
        let frames = [
            &self.general_frame,
            &self.voice_frame,
            &self.hotkeys_frame,
            &self.mouse_frame,
            &self.advanced_frame,
        ];

        frames.map(|frame| {
            frame.set_visible(false);
            if self.layout.has_child(frame) {
                self.layout.remove_child(frame);
            }
        });

        let style = Style {
            size: FRAME_SIZE,
            ..Default::default()
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

    fn on_save(&self) {
        self.window.set_visible(false);
    }

    fn on_init(&self) {
        self.window.set_visible(false);
    }

    fn on_exit(&self) {
        self.window.set_visible(false);
    }

    fn on_run_on_startup(&self, ctrl: &GeneralUi) {
        let toggle = ctrl.ck_run_on_startup.check_state() == CheckBoxState::Checked;
        set_auto_start_cmd(self.context.get().unwrap().clone(), toggle);
    }

    fn on_auto_check_update(&self, ctrl: &GeneralUi) {
        let toggle = ctrl.ck_auot_update.check_state() == CheckBoxState::Checked;
        set_auto_check_update_cmd(self.context.get().unwrap().clone(), toggle);
    }

    fn on_check_update(&self) {
        check_update_cmd(self.context.get().unwrap().clone(), false);
    }

    fn on_lang_changed(&self, ctrl: &GeneralUi) {
        let lang = ctrl.cb_lang.selection().unwrap();
        set_lang_cmd(self.context.get().unwrap().clone(), lang);
    }

    fn on_role_changed(&self, ctrl: &VoiceUi) {
        let index = ctrl.cb_role.selection().unwrap();
        set_voice_cmd(self.context.get().unwrap().clone(), index);
    }

    fn on_speed_changed(&self, ctrl: &VoiceUi) {
        let index = ctrl.cb_speed.selection().unwrap();
        set_speed_cmd(self.context.get().unwrap().clone(), index);
    }

    fn on_pitch_changed(&self, ctrl: &VoiceUi) {
        let index = ctrl.cb_pitch.selection().unwrap();
        set_pitch_cmd(self.context.get().unwrap().clone(), index);
    }

    fn on_volume_changed(&self, ctrl: &VoiceUi) {
        let index = ctrl.cb_volume.selection().unwrap();
        set_volume_cmd(self.context.get().unwrap().clone(), index);
    }

    fn on_mouse_read(&self, ctrl: &MouseUi) {
        let toggle = ctrl.ck_mouse_read.check_state() == CheckBoxState::Checked;
        set_mouse_read_cmd(self.context.get().unwrap().clone(), toggle);
    }

    fn on_import(&self) {
        import_config_cmd(self.context.get().unwrap().clone());
    }

    fn on_export(&self) {
        export_config_cmd(self.context.get().unwrap().clone());
    }

    fn on_reset(&self) {
        reset_config_cmd(self.context.get().unwrap().clone());
    }

    fn on_show_notice(&self) {
        bring_window_front!(&self.window);
        self.window.set_visible(true);
        self.menu.set_focus();
    }

    fn on_exit_notice(&self) {
        nwg::stop_thread_dispatch()
    }

    fn on_show_hotkeys_notice(&self) {
        self.menu.set_selection(Some(2));
        self.change_interface();
        bring_window_front!(&self.window);
        self.window.set_visible(true);
    }
}
#[derive(Default, NwgPartial)]
pub struct GeneralUi {
    #[nwg_layout(max_size: [1200, 800], min_size: [650, 480], spacing: 20, max_column: Some(3), max_row: Some(10))]
    layout: nwg::GridLayout,

    #[nwg_layout(min_size: [600, 480], max_column: Some(4), max_row: Some(10))]
    layout2: nwg::GridLayout,

    #[nwg_control(text: "开机启动 (&R)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 1)]
    ck_run_on_startup: CheckBox,

    #[nwg_control(text: "自动更新 (&A)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 2)]
    ck_auot_update: CheckBox,

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
    #[nwg_layout_item(layout: layout2, col: 3, row: 9)]
    btn_save: nwg::Button,
}

#[allow(unused)]
fn get_num_1_100() -> Vec<&'static str> {
    vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
        "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31",
        "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46",
        "47", "48", "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "60", "61",
        "62", "63", "64", "65", "66", "67", "68", "69", "70", "71", "72", "73", "74", "75", "76",
        "77", "78", "79", "80", "81", "82", "83", "84", "85", "86", "87", "88", "89", "90", "91",
        "92", "93", "94", "95", "96", "97", "98", "99", "100",
    ]
}

#[derive(Default, NwgPartial)]
pub struct VoiceUi {
    #[nwg_layout(max_size: [1200, 800], min_size: [650, 480], spacing: 20, max_column: Some(4), max_row: Some(10))]
    layout: nwg::GridLayout,

    #[nwg_layout(min_size: [600, 480], max_column: Some(4), max_row: Some(10))]
    layout2: nwg::GridLayout,

    #[nwg_control(text: "朗读角色 (&R)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 1)]
    lb_role: nwg::Label,

    #[nwg_control(collection: vec![] )]
    #[nwg_layout_item(layout: layout, col: 2, row: 1)]
    cb_role: nwg::ComboBox<&'static str>,

    #[nwg_control(text: "朗读语速 (&E)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 2)]
    lb_speed: nwg::Label,

    #[nwg_control(collection: get_num_1_100() )]
    #[nwg_layout_item(layout: layout, col: 2, row: 2)]
    cb_speed: nwg::ComboBox<&'static str>,

    #[nwg_control(text: "朗读语调 (&P)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 3)]
    lb_pitch: nwg::Label,

    #[nwg_control(collection: get_num_1_100() )]
    #[nwg_layout_item(layout: layout, col: 2, row: 3)]
    cb_pitch: nwg::ComboBox<&'static str>,

    #[nwg_control(text: "朗读音量 (&O)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 4)]
    lb_volume: nwg::Label,

    #[nwg_control(collection: get_num_1_100() )]
    #[nwg_layout_item(layout: layout, col: 2, row: 4)]
    cb_volume: nwg::ComboBox<&'static str>,

    #[nwg_control(text: "保存 (&S)")]
    #[nwg_layout_item(layout: layout2, col: 3, row: 9)]
    btn_save: nwg::Button,
}

#[derive(Default, NwgPartial)]
pub struct MouseUi {
    #[nwg_layout(max_size: [1200, 800], min_size: [650, 480], spacing: 20, max_column: Some(3), max_row: Some(10))]
    layout: nwg::GridLayout,

    #[nwg_layout(min_size: [600, 480], max_column: Some(4), max_row: Some(10))]
    layout2: nwg::GridLayout,

    #[nwg_control(text: "朗读鼠标 (&R)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 1)]
    ck_mouse_read: CheckBox,

    #[nwg_control(text: "保存 (&S)")]
    #[nwg_layout_item(layout: layout2, col: 3, row: 9)]
    btn_save: nwg::Button,
}

#[derive(Default, NwgPartial)]
pub struct AdvancedUi {
    #[nwg_layout(max_size: [1200, 800], min_size: [650, 480], spacing: 20, max_column: Some(3), max_row: Some(10))]
    layout: nwg::GridLayout,

    #[nwg_layout(min_size: [600, 480], max_column: Some(4), max_row: Some(10))]
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
    #[nwg_layout_item(layout: layout2, col: 3, row: 9)]
    btn_save: nwg::Button,
}
