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

use crate::commander::keyboard::keys::Keys;
use crate::{
    bring_window_front,
    commander::keyboard::combo_keys::ComboKey,
    configs::{
        config_operations::{
            get_auto_check_update, get_lang, get_mouse_read_state, get_run_on_startup,
        },
        general::Lang,
        tts::TtsPropertyItem,
    },
    context::Context,
    gui::{
        command::{
            add_desktop_shortcut_cmd, check_update_cmd, export_config_cmd,
            get_desktop_shortcut_path, import_config_cmd, reset_config_cmd,
            set_auto_check_update_cmd, set_auto_start_cmd, set_lang_cmd, set_mouse_read_cmd,
            set_pitch_cmd, set_speed_cmd, set_voice_cmd, set_volume_cmd,
        },
        forms::hotkeys::HotKeysUi,
        utils::set_hook_simple,
    },
    performer::tts::{TtsProperty, VoiceInfo},
};
use arc_swap::{ArcSwap, Guard};
use nwd::{NwgPartial, NwgUi};
use nwg::{
    modal_message,
    stretch::{
        geometry::Size,
        style::{Dimension as D, Style},
    },
    CheckBox, CheckBoxState, MessageParams, NoticeSender,
};
use rigela_macros::GuiFormImpl;
use rust_i18n::AtomicStr;
use std::sync::atomic::{AtomicI32, Ordering};
use std::{
    cell::RefCell,
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, OnceLock, Weak},
};
use win_wrap::hook::WindowsHook;

const FORM_SIZE: (u32, u32) = (800, 600);
const FRAME_SIZE: Size<D> = Size {
    width: D::Percent(1.0),
    height: D::Auto,
};

#[derive(Default, NwgUi, GuiFormImpl)]
pub struct SettingsForm {
    pub(crate) context: OnceLock<Weak<Context>>,

    pub(crate) talent_ids: RefCell<Vec<String>>,
    pub(crate) custom_combo_keys: RefCell<HashMap<String, ComboKey>>,
    pub(crate) hotkeys: Arc<ArcSwap<Option<ComboKey>>>,
    pub(crate) hook: RefCell<Option<WindowsHook>>,

    all_voices: Arc<ArcSwap<Vec<String>>>,
    voice: Arc<AStr>,
    speed: Arc<AtomicI32>,
    pitch: Arc<AtomicI32>,
    volume: Arc<AtomicI32>,

    #[nwg_control(size: (0, 0), position: (200, 200), title: & t ! ("settings.title"))]
    #[nwg_events(OnWindowClose: [SettingsForm::on_exit], OnInit: [SettingsForm::on_init, SettingsForm::load_data])]
    pub(crate) window: nwg::Window,

    #[nwg_layout(parent: window)]
    layout: nwg::FlexboxLayout,

    #[nwg_control(collection: vec ! [
    t ! ("settings.menu_general_item").to_string(),
    t ! ("settings.menu_voice_item").to_string(),
    t ! ("settings.menu_hotkeys_item").to_string(),
    t ! ("settings.menu_mouse_item").to_string(),
    t ! ("settings.menu_advanced_item").to_string(),
    ])]
    #[nwg_layout_item(layout: layout, size: Size{width: D::Points(150.0), height: D::Auto})]
    #[nwg_events(OnListBoxSelect: [SettingsForm::change_interface])]
    menu: nwg::ListBox<String>,

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
    (ck_add_desktop_shortcut, OnButtonClick): [SettingsForm::on_add_desktop_shortcut(SELF, CTRL)],
    (ck_run_on_startup, OnButtonClick): [SettingsForm::on_run_on_startup(SELF, CTRL)],
    (ck_auot_update, OnButtonClick): [SettingsForm::on_auto_check_update(SELF, CTRL)],
    (btn_check_update, OnButtonClick): [SettingsForm::on_check_update],
    (cb_lang, OnComboxBoxSelection): [SettingsForm::on_lang_changed(SELF, CTRL)],
    (btn_close, OnButtonClick): [SettingsForm::on_save],
    (finish_program_hotkeys_notice, OnNotice): [SettingsForm::on_finish_program_hotkeys_hook],
    (cancel_program_hotkeys_notice, OnNotice): [SettingsForm::on_cancel_program_hotkeys_hook],
    )]
    general_ui: GeneralUi,

    #[nwg_partial(parent: voice_frame)]
    #[nwg_events(
    (cb_role, OnComboxBoxSelection): [SettingsForm::on_role_changed(SELF, CTRL)],
    (cb_speed, OnComboxBoxSelection): [SettingsForm::on_speed_changed(SELF, CTRL)],
    (cb_pitch, OnComboxBoxSelection): [SettingsForm::on_pitch_changed(SELF, CTRL)],
    (cb_volume, OnComboxBoxSelection): [SettingsForm::on_volume_changed(SELF, CTRL)],
    (update_voice_notice, OnNotice): [SettingsForm::update_voice_notice],
    (btn_close, OnButtonClick): [SettingsForm::on_save],
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
    (btn_close, OnButtonClick): [SettingsForm::on_save],
    )]
    mouse_ui: MouseUi,

    #[nwg_partial(parent: advanced_frame)]
    #[nwg_events(
    (btn_import, OnButtonClick): [SettingsForm::on_import],
    (btn_export, OnButtonClick): [SettingsForm::on_export],
    (btn_reset, OnButtonClick): [SettingsForm::on_reset],
    (btn_close, OnButtonClick): [SettingsForm::on_save],
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

    fn on_add_desktop_shortcut(&self, ctrl: &GeneralUi) {
        let context = self.context.get().unwrap().clone();

        match ctrl.ck_add_desktop_shortcut.check_state() {
            // 先取消掉选择，等热键确定好在勾选上
            CheckBoxState::Checked => {
                self.general_ui
                    .ck_add_desktop_shortcut
                    .set_check_state(CheckBoxState::Unchecked);

                let keys = self.general_ui.program_hotkeys.clone();
                let senders = [
                    self.general_ui.finish_program_hotkeys_notice.sender(),
                    self.general_ui.cancel_program_hotkeys_notice.sender(),
                ];

                let pf = unsafe { &*context.as_ptr() }.performer.clone();
                unsafe { &*context.as_ptr() }
                    .work_runtime
                    .spawn(async move {
                        pf.speak(&t!("settings.def_shortcut_hotkey")).await;
                    });

                let mut hook = self.general_ui.hook.borrow_mut();
                *hook = Some(set_hook_simple(keys, &senders));
            }
            CheckBoxState::Unchecked => {
                add_desktop_shortcut_cmd(self.context.get().unwrap().clone(), false, &vec![])
            }
            _ => {}
        };
    }

    fn on_finish_program_hotkeys_hook(&self) {
        self.general_ui.hook.borrow_mut().take().unwrap().unhook();
        let keys: Guard<Arc<ComboKey>> = self.general_ui.program_hotkeys.load();
        let keys: Arc<ComboKey> = keys.clone();

        let keys_str = format!("{keys}");
        let info = format!("您确定要将{keys_str}用作程序启动的热键吗？");

        let msg_params = MessageParams {
            title: "确认",
            content: &info,
            buttons: nwg::MessageButtons::OkCancel,
            icons: nwg::MessageIcons::Question,
        };
        if modal_message(&self.window, &msg_params) == nwg::MessageChoice::Cancel {
            return;
        }

        let keys: Vec<Keys> = (*keys).clone().into();
        add_desktop_shortcut_cmd(self.context.get().unwrap().clone(), true, &keys);

        self.general_ui
            .ck_add_desktop_shortcut
            .set_check_state(CheckBoxState::Checked);
    }

    fn on_cancel_program_hotkeys_hook(&self) {
        self.general_ui.hook.borrow_mut().take().unwrap().unhook();
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
        let all_voice = self.all_voices.load();
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
        // 更新桌面快捷显示
        let state = match get_desktop_shortcut_path().exists() {
            true => CheckBoxState::Checked,
            false => CheckBoxState::Unchecked,
        };
        self.general_ui
            .ck_add_desktop_shortcut
            .set_check_state(state);

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
            Lang::FollowSystem => 0,
            Lang::En => 1,
            Lang::Zh => 2,
        };
        self.general_ui.cb_lang.set_selection(Some(index));

        // 更新语音角色框显示
        let format_voice_info = |v: &VoiceInfo| format!("{}_{}", v.engine, v.name);

        let ctx = self.context.get().unwrap().clone();
        let tts = unsafe { &*ctx.as_ptr() }.performer.get_tts();
        let all_voice = self.all_voices.clone();
        let voice = self.voice.clone();
        let speed = self.speed.clone();
        let pitch = self.pitch.clone();
        let volume = self.volume.clone();
        let update_voice_sender = self.voice_ui.update_voice_notice.sender().clone();

        unsafe { &*ctx.as_ptr() }.work_runtime.spawn(async move {
            let voices = tts
                .get_all_voiceinfo()
                .await
                .iter()
                .map(|v| format_voice_info(v))
                .collect();
            all_voice.store(Arc::new(voices));
            let voiceinfo = tts.get_tts_prop_value(Some(TtsPropertyItem::Voice)).await;
            if let TtsProperty::Voice(v) = voiceinfo {
                voice.0.replace(format_voice_info(&v));
            }
            let voiceinfo = tts.get_tts_prop_value(Some(TtsPropertyItem::Speed)).await;
            if let TtsProperty::Speed(v) = voiceinfo {
                speed.store(v, Ordering::Release);
            }
            let voiceinfo = tts.get_tts_prop_value(Some(TtsPropertyItem::Pitch)).await;
            if let TtsProperty::Pitch(v) = voiceinfo {
                pitch.store(v, Ordering::Release);
            }
            let voiceinfo = tts.get_tts_prop_value(Some(TtsPropertyItem::Volume)).await;
            if let TtsProperty::Volume(v) = voiceinfo {
                volume.store(v, Ordering::Release);
            }

            update_voice_sender.notice();
        });

        // 更新鼠标朗读显示
        let state = match get_mouse_read_state(self.context.get().unwrap().clone()) {
            true => CheckBoxState::Checked,
            false => CheckBoxState::Unchecked,
        };
        self.mouse_ui.ck_mouse_read.set_check_state(state);

        bring_window_front!(&self.window);
        self.window.set_size(FORM_SIZE.0, FORM_SIZE.1);
        self.window.set_visible(true);
        self.menu.set_focus();
        self.menu.set_selection(Some(0));
    }

    fn on_exit_notice(&self) {
        nwg::stop_thread_dispatch()
    }

    fn on_show_hotkeys_notice(&self) {
        self.on_show_notice();

        self.menu.set_selection(Some(2));
        self.change_interface();
        self.hotkeys_ui.data_view.set_focus();
    }

    fn update_voice_notice(&self) {
        let items = self.all_voices.load();
        self.voice_ui.cb_role.set_collection((**items).clone());

        let item_str = self.voice.0.to_string();
        let index = items.iter().position(|v| v == &item_str).unwrap();
        self.voice_ui.cb_role.set_selection(Some(index));

        self.voice_ui
            .cb_speed
            .set_selection(Some((100 - self.speed.load(Ordering::Acquire)) as usize));
        self.voice_ui
            .cb_pitch
            .set_selection(Some((100 - self.pitch.load(Ordering::Acquire)) as usize));
        self.voice_ui
            .cb_volume
            .set_selection(Some((100 - self.volume.load(Ordering::Acquire)) as usize));
    }
}

#[derive(Default, NwgPartial)]
pub struct GeneralUi {
    program_hotkeys: Arc<ArcSwap<ComboKey>>,
    hook: RefCell<Option<WindowsHook>>,

    #[nwg_layout(max_size: [1200, 800], min_size: [650, 480], spacing: 20, max_column: Some(3), max_row: Some(10))]
    layout: nwg::GridLayout,

    #[nwg_layout(min_size: [600, 480], max_column: Some(4), max_row: Some(10))]
    layout2: nwg::GridLayout,

    #[nwg_control(text: & t ! ("settings.ck_add_desktop_shortcut"))]
    #[nwg_layout_item(layout: layout, col: 1, row: 1)]
    ck_add_desktop_shortcut: CheckBox,

    #[nwg_control(text: & t ! ("settings.run_on_startup"))]
    #[nwg_layout_item(layout: layout, col: 1, row: 2)]
    ck_run_on_startup: CheckBox,

    #[nwg_control(text: & t ! ("settings.auto_check_update"))]
    #[nwg_layout_item(layout: layout, col: 1, row: 3)]
    ck_auot_update: CheckBox,

    #[nwg_control(text: & t ! ("settings.check_update"))]
    #[nwg_layout_item(layout: layout, col: 1, row: 4)]
    btn_check_update: nwg::Button,

    #[nwg_control(text: & t ! ("settings.lang"))]
    #[nwg_layout_item(layout: layout, col: 1, row: 6)]
    lb_lang: nwg::Label,

    #[nwg_control(collection: vec ! [
    t ! ("settings.lang_follow_system_item").to_string(),
    t ! ("settings.lang_en_item").to_string(),
    t ! ("settings.lang_zh_item").to_string(),
    ])]
    #[nwg_layout_item(layout: layout, col: 2, row: 6)]
    cb_lang: nwg::ComboBox<String>,

    #[nwg_control(text: & t ! ("settings.btn_close"))]
    #[nwg_layout_item(layout: layout2, col: 3, row: 9)]
    btn_close: nwg::Button,

    #[nwg_control]
    finish_program_hotkeys_notice: nwg::Notice,

    #[nwg_control]
    cancel_program_hotkeys_notice: nwg::Notice,
}

#[derive(Default, NwgPartial)]
pub struct VoiceUi {
    #[nwg_layout(max_size: [1200, 800], min_size: [650, 480], spacing: 20, max_column: Some(4), max_row: Some(10))]
    layout: nwg::GridLayout,

    #[nwg_layout(min_size: [600, 480], max_column: Some(4), max_row: Some(10))]
    layout2: nwg::GridLayout,

    #[nwg_control(text: & t ! ("settings.lb_role"))]
    #[nwg_layout_item(layout: layout, col: 1, row: 1)]
    lb_role: nwg::Label,

    #[nwg_control(collection: vec ! [])]
    #[nwg_layout_item(layout: layout, col: 2, row: 1)]
    cb_role: nwg::ComboBox<String>,

    #[nwg_control(text: & t ! ("settings.lb_speed"))]
    #[nwg_layout_item(layout: layout, col: 1, row: 2)]
    lb_speed: nwg::Label,

    #[nwg_control(collection: (1..101).map(| i | format ! ("{}", i)).rev().collect())]
    #[nwg_layout_item(layout: layout, col: 2, row: 2)]
    cb_speed: nwg::ComboBox<String>,

    #[nwg_control(text: & t ! ("settings.lb_pitch"))]
    #[nwg_layout_item(layout: layout, col: 1, row: 3)]
    lb_pitch: nwg::Label,

    #[nwg_control(collection: (1..101).map(| i | format ! ("{}", i)).rev().collect())]
    #[nwg_layout_item(layout: layout, col: 2, row: 3)]
    cb_pitch: nwg::ComboBox<String>,

    #[nwg_control(text: & t ! ("settings.lb_volume"))]
    #[nwg_layout_item(layout: layout, col: 1, row: 4)]
    lb_volume: nwg::Label,

    #[nwg_control(collection: (1..101).map(| i | format ! ("{}", i)).rev().collect())]
    #[nwg_layout_item(layout: layout, col: 2, row: 4)]
    cb_volume: nwg::ComboBox<String>,

    #[nwg_control]
    update_voice_notice: nwg::Notice,

    #[nwg_control(text: & t ! ("settings.btn_close"))]
    #[nwg_layout_item(layout: layout2, col: 3, row: 9)]
    btn_close: nwg::Button,
}

#[derive(Default, NwgPartial)]
pub struct MouseUi {
    #[nwg_layout(max_size: [1200, 800], min_size: [650, 480], spacing: 20, max_column: Some(3), max_row: Some(10))]
    layout: nwg::GridLayout,

    #[nwg_layout(min_size: [600, 480], max_column: Some(4), max_row: Some(10))]
    layout2: nwg::GridLayout,

    #[nwg_control(text: & t ! ("settings.ck_mouse_read"))]
    #[nwg_layout_item(layout: layout, col: 1, row: 1)]
    ck_mouse_read: CheckBox,

    #[nwg_control(text: & t ! ("settings.btn_close"))]
    #[nwg_layout_item(layout: layout2, col: 3, row: 9)]
    btn_close: nwg::Button,
}

#[derive(Default, NwgPartial)]
pub struct AdvancedUi {
    #[nwg_resource(title: t ! ("settings.export_title").to_string(), action: nwg::FileDialogAction::Save, filters: "Zip(*.zip)")]
    export_dialog: nwg::FileDialog,

    #[nwg_resource(title: t ! ("settings.import_title").to_string(), action: nwg::FileDialogAction::Open, filters: "Zip(*.zip)")]
    import_dialog: nwg::FileDialog,

    #[nwg_layout(max_size: [1200, 800], min_size: [650, 480], spacing: 20, max_column: Some(3), max_row: Some(10))]
    layout: nwg::GridLayout,

    #[nwg_layout(min_size: [600, 480], max_column: Some(4), max_row: Some(10))]
    layout2: nwg::GridLayout,

    #[nwg_control(text: & t ! ("settings.btn_export"))]
    #[nwg_layout_item(layout: layout, col: 1, row: 2)]
    btn_export: nwg::Button,

    #[nwg_control(text: & t ! ("settings.btn_import"))]
    #[nwg_layout_item(layout: layout, col: 1, row: 1)]
    btn_import: nwg::Button,

    #[nwg_control(text: & t ! ("settings.btn_reset"))]
    #[nwg_layout_item(layout: layout, col: 1, row: 3)]
    btn_reset: nwg::Button,

    #[nwg_control(text: & t ! ("settings.btn_close"))]
    #[nwg_layout_item(layout: layout2, col: 3, row: 9)]
    btn_close: nwg::Button,
}

struct AStr(AtomicStr);
impl Default for AStr {
    fn default() -> Self {
        Self(AtomicStr::new(""))
    }
}
