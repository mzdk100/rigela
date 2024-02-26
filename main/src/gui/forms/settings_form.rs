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
use crate::configs::config_operations::{
    get_auto_check_update, get_lang, get_mouse_read_state, get_run_on_startup,
};
use crate::configs::general::Lang;
use crate::configs::tts::TtsPropertyItem;
use crate::gui::command::{
    check_update_cmd, export_config_cmd, import_config_cmd, reset_config_cmd,
    set_auto_check_update_cmd, set_auto_start_cmd, set_lang_cmd, set_mouse_read_cmd, set_pitch_cmd,
    set_speed_cmd, set_voice_cmd, set_volume_cmd,
};
use crate::gui::forms::hotkeys::HotKeysUi;
use crate::performer::tts::{TtsProperty, VoiceInfo};
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
use std::ops::DerefMut;
use std::path::PathBuf;
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

    all_voices: Arc<Mutex<Vec<String>>>,
    voice: Arc<Mutex<String>>,
    speed: Arc<Mutex<i32>>,
    pitch: Arc<Mutex<i32>>,
    volume: Arc<Mutex<i32>>,

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
    (update_voice_notice, OnNotice): [SettingsForm::update_voice_notice],
    (btn_save, OnButtonClick): [SettingsForm::on_save],
    )]
    voice_ui: VoiceUi,

    #[nwg_partial(parent: hotkeys_frame)]
    #[nwg_events(
    (data_view, OnKeyRelease): [SettingsForm::on_dv_key_press(SELF, EVT_DATA)],
    (data_view, OnListViewItemChanged): [SettingsForm::on_dv_selection_changed],

    (btn_set, OnButtonClick): [SettingsForm::on_set_hotkey],
    (btn_set, OnKeyRelease): [SettingsForm::on_btn_key_release(SELF, EVT_DATA, HANDLE)],

    (btn_clear, OnButtonClick): [SettingsForm::on_clear_hotkey],
    (btn_clear, OnKeyRelease): [SettingsForm::on_btn_key_release(SELF, EVT_DATA, HANDLE)],

    (finish_custom, OnNotice): [SettingsForm::on_finish_custom],
    (cancel_custom, OnNotice): [SettingsForm::on_cancel_custom],

    (btn_close, OnButtonClick): [SettingsForm::on_save],
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
        let all_voice = self.all_voices.lock().unwrap().clone();
        let mut info = all_voice[index].split("_");
        set_voice_cmd(
            self.context.get().unwrap().clone(),
            info.next().unwrap().to_string(),
            info.next().unwrap().to_string(),
        );
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
        if self.advanced_ui.import_dialog.run(Some(&self.window)) {
            let path = self.advanced_ui.import_dialog.get_selected_item().unwrap();
            import_config_cmd(
                self.context.get().unwrap().clone(),
                PathBuf::from(path.as_os_str()),
            );
        }
    }

    fn on_export(&self) {
        if self.advanced_ui.export_dialog.run(Some(&self.window)) {
            let path = self.advanced_ui.export_dialog.get_selected_item().unwrap();
            export_config_cmd(
                self.context.get().unwrap().clone(),
                PathBuf::from(path.as_os_str()),
            );
        }
    }

    fn on_reset(&self) {
        reset_config_cmd(self.context.get().unwrap().clone());
    }

    fn on_show_notice(&self) {
        bring_window_front!(&self.window);

        // 更新开机自启显示
        let state = match get_run_on_startup(self.context.get().unwrap().clone()) {
            true => CheckBoxState::Checked,
            false => CheckBoxState::Unchecked,
        };
        self.general_ui.ck_run_on_startup.set_check_state(state);

        // 更新自动更新显示
        let state = match get_auto_check_update(self.context.get().unwrap().clone()) {
            true => CheckBoxState::Checked,
            false => CheckBoxState::Unchecked,
        };
        self.general_ui.ck_auot_update.set_check_state(state);

        // 更新语言显示
        let lang = get_lang(self.context.get().unwrap().clone());
        let index = match lang {
            Lang::Zh => 0,
            Lang::En => 1,
        };
        self.general_ui.cb_lang.set_selection(Some(index));

        // 更新语音角色框显示
        let format_voice_info = |v: &VoiceInfo| format!("{}_{}", v.engine, v.name);

        let ctx = self.context.get().unwrap().clone();
        let tts = ctx.performer.get_tts();
        let all_voice = self.all_voices.clone();
        let voice = self.voice.clone();
        let speed = self.speed.clone();
        let pitch = self.pitch.clone();
        let volume = self.volume.clone();
        let update_voice_sender = self.voice_ui.update_voice_notice.sender().clone();

        ctx.work_runtime.spawn(async move {
            *all_voice.lock().unwrap().deref_mut() = tts
                .get_all_voiceinfo()
                .await
                .iter()
                .map(|v| format_voice_info(v))
                .collect();
            let voiceinfo = tts.get_tts_prop_value(Some(TtsPropertyItem::Voice)).await;
            if let TtsProperty::Voice(v) = voiceinfo {
                *voice.lock().unwrap().deref_mut() = format_voice_info(&v);
            }
            let voiceinfo = tts.get_tts_prop_value(Some(TtsPropertyItem::Speed)).await;
            if let TtsProperty::Speed(v) = voiceinfo {
                *speed.lock().unwrap().deref_mut() = v;
            }
            let voiceinfo = tts.get_tts_prop_value(Some(TtsPropertyItem::Pitch)).await;
            if let TtsProperty::Pitch(v) = voiceinfo {
                *pitch.lock().unwrap().deref_mut() = v;
            }
            let voiceinfo = tts.get_tts_prop_value(Some(TtsPropertyItem::Volume)).await;
            if let TtsProperty::Volume(v) = voiceinfo {
                *volume.lock().unwrap().deref_mut() = v;
            }

            update_voice_sender.notice();
        });

        // 更新鼠标朗读显示
        let state = match get_mouse_read_state(self.context.get().unwrap().clone()) {
            true => CheckBoxState::Checked,
            false => CheckBoxState::Unchecked,
        };
        self.mouse_ui.ck_mouse_read.set_check_state(state);

        self.window.set_visible(true);
        self.menu.set_focus();
        self.menu.set_selection(Some(0));
    }

    fn on_exit_notice(&self) {
        nwg::stop_thread_dispatch()
    }

    fn on_show_hotkeys_notice(&self) {
        self.menu.set_selection(Some(2));
        self.change_interface();
        self.hotkeys_ui.data_view.set_focus();
        bring_window_front!(&self.window);
        self.window.set_visible(true);
    }

    fn update_voice_notice(&self) {
        let items = self.all_voices.lock().unwrap().clone();
        self.voice_ui.cb_role.set_collection(items.clone());

        let item_str = self.voice.lock().unwrap().clone();
        let index = items.iter().position(|v| v == &item_str).unwrap();
        self.voice_ui.cb_role.set_selection(Some(index));

        self.voice_ui
            .cb_speed
            .set_selection(Some((100 - self.speed.lock().unwrap().clone()) as usize));
        self.voice_ui
            .cb_pitch
            .set_selection(Some((100 - self.pitch.lock().unwrap().clone()) as usize));
        self.voice_ui
            .cb_volume
            .set_selection(Some((100 - self.volume.lock().unwrap().clone()) as usize));
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

    #[nwg_control(text: "关闭 (&C)")]
    #[nwg_layout_item(layout: layout2, col: 3, row: 9)]
    btn_save: nwg::Button,
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
    cb_role: nwg::ComboBox<String>,

    #[nwg_control(text: "朗读语速 (&E)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 2)]
    lb_speed: nwg::Label,

    #[nwg_control(collection: (1..101).map(|i| format!("{}", i)).rev().collect() )]
    #[nwg_layout_item(layout: layout, col: 2, row: 2)]
    cb_speed: nwg::ComboBox<String>,

    #[nwg_control(text: "朗读语调 (&P)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 3)]
    lb_pitch: nwg::Label,

    #[nwg_control(collection: (1..101).map(|i| format!("{}", i)).rev().collect() )]
    #[nwg_layout_item(layout: layout, col: 2, row: 3)]
    cb_pitch: nwg::ComboBox<String>,

    #[nwg_control(text: "朗读音量 (&O)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 4)]
    lb_volume: nwg::Label,

    #[nwg_control(collection: (1..101).map(|i| format!("{}", i)).rev().collect() )]
    #[nwg_layout_item(layout: layout, col: 2, row: 4)]
    cb_volume: nwg::ComboBox<String>,

    #[nwg_control]
    update_voice_notice: nwg::Notice,

    #[nwg_control(text: "关闭 (&C)")]
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

    #[nwg_control(text: "关闭 (&C)")]
    #[nwg_layout_item(layout: layout2, col: 3, row: 9)]
    btn_save: nwg::Button,
}

#[derive(Default, NwgPartial)]
pub struct AdvancedUi {
    #[nwg_resource(title: "请选择需要导出的文件夹:", action: nwg::FileDialogAction::Save, filters: "Zip(*.zip)")]
    export_dialog: nwg::FileDialog,

    #[nwg_resource(title: "请选择需要导入的文件:", action: nwg::FileDialogAction::Open, filters: "Zip(*.zip)")]
    import_dialog: nwg::FileDialog,

    #[nwg_layout(max_size: [1200, 800], min_size: [650, 480], spacing: 20, max_column: Some(3), max_row: Some(10))]
    layout: nwg::GridLayout,

    #[nwg_layout(min_size: [600, 480], max_column: Some(4), max_row: Some(10))]
    layout2: nwg::GridLayout,

    #[nwg_control(text: "导出配置... (&E)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 2)]
    btn_export: nwg::Button,

    #[nwg_control(text: "导入配置... (&I)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 1)]
    btn_import: nwg::Button,

    #[nwg_control(text: "恢复默认配置 (&R)")]
    #[nwg_layout_item(layout: layout, col: 1, row: 3)]
    btn_reset: nwg::Button,

    #[nwg_control(text: "关闭 (&C)")]
    #[nwg_layout_item(layout: layout2, col: 3, row: 9)]
    btn_save: nwg::Button,
}
