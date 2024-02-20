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

use crate::context::Context;
use crate::gui::hotkeys::HotKeysForm;
use crate::gui::popup_menu::PopupMenuForm;
use crate::gui::settings_form::SettingsForm;
use crate::gui::system_tray::SystemTray;
use crate::gui::welcome::WelcomeForm;
use nwg::{NativeUi, NoticeSender};
use std::fmt::{Debug, Formatter};
use std::sync::{mpsc, Arc, Mutex, OnceLock};
use std::thread;

pub(crate) trait Formable {
    fn set_context(&self, context: Arc<Context>);
    fn get_show_notice_sender(&self) -> NoticeSender;
    fn get_exit_notice_sender(&self) -> NoticeSender;
}

#[derive(Clone, Default)]
pub(crate) struct WinManager {
    welcome: OnceLock<(NoticeSender, NoticeSender)>,
    tray: OnceLock<(NoticeSender, NoticeSender)>,
    popup_menu: OnceLock<(NoticeSender, NoticeSender)>,
    settings: OnceLock<(NoticeSender, NoticeSender)>,
    hotkeys: OnceLock<(NoticeSender, NoticeSender)>,
}

// 防止重复初始化
fn already_init() -> &'static Mutex<bool> {
    static INSTANCE: OnceLock<Mutex<bool>> = OnceLock::new();
    INSTANCE.get_or_init(|| false.into())
}

macro_rules! build_form {
    ($var:ident, $type_:ident, $context:expr, $sd:expr) => {
        let ctx = $context.clone();
        let sd = $sd.clone();
        let $var = $type_::build_ui(Default::default())
            .expect(format!("could not build {} form", stringify!($type_)).as_str());
        $var.set_context(ctx);
        sd.send(($var.get_show_notice_sender(), $var.get_exit_notice_sender()))
            .unwrap();
    };
}

impl WinManager {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub(crate) fn init(&self, context: Arc<Context>) {
        if already_init().lock().unwrap().clone() {
            return;
        }
        {
            *already_init().lock().unwrap() = true;
        }

        let (tx, rx) = mpsc::channel::<(NoticeSender, NoticeSender)>();

        thread::spawn(move || {
            nwg::init().expect("could not initialize nwg");

            build_form!(welcome, WelcomeForm, context, tx);
            build_form!(tray, SystemTray, context, tx);
            build_form!(popup_menu, PopupMenuForm, context, tx);
            build_form!(settings, SettingsForm, context, tx);
            build_form!(hotkeys, HotKeysForm, context, tx);

            nwg::dispatch_thread_events()
        });

        let _ = self.welcome.set(rx.recv().unwrap());
        let _ = self.tray.set(rx.recv().unwrap());
        let _ = self.popup_menu.set(rx.recv().unwrap());
        let _ = self.settings.set(rx.recv().unwrap());
        let _ = self.hotkeys.set(rx.recv().unwrap());

        self.welcome.get().unwrap().0.notice();
    }

    pub(crate) fn uninit(&self) {
        self.welcome.get().unwrap().1.notice();
        self.tray.get().unwrap().1.notice();
        self.popup_menu.get().unwrap().1.notice();
        self.settings.get().unwrap().1.notice();
        self.hotkeys.get().unwrap().1.notice();
    }

    pub(crate) fn show_settings_form(&self) {
        self.settings.get().unwrap().0.notice();
    }

    pub(crate) fn show_hotkeys_form(&self) {
        self.hotkeys.get().unwrap().0.notice();
    }

    pub(crate) fn show_popup_menu(&self) {
        self.popup_menu.get().unwrap().0.notice();
    }

    pub(crate) fn show_welcome_form(&self) {
        self.welcome.get().unwrap().0.notice();
    }
}

impl Debug for WinManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WinManager").finish()
    }
}

#[macro_export]
macro_rules! bring_window_front {
    ($win:expr) => {
        if let nwg::ControlHandle::Hwnd(h) = $win.handle {
            let current_thread_id = win_wrap::threading::get_current_thread_id();
            let h_foreground = win_wrap::common::get_foreground_window();
            let (remote_thread_id, _) =
                win_wrap::threading::get_window_thread_process_id(h_foreground);

            win_wrap::common::attach_thread_input(
                current_thread_id,
                remote_thread_id,
                win_wrap::common::TRUE,
            );

            win_wrap::common::show_window(
                win_wrap::common::HWND(h as isize),
                win_wrap::common::SW_HIDE,
            );
            win_wrap::common::show_window(
                win_wrap::common::HWND(h as isize),
                win_wrap::common::SW_SHOW,
            );
            win_wrap::common::set_foreground_window(win_wrap::common::HWND(h as isize));
            win_wrap::common::set_active_window(win_wrap::common::HWND(h as isize));

            win_wrap::common::attach_thread_input(
                current_thread_id,
                remote_thread_id,
                win_wrap::common::FALSE,
            );
        };
    };
}
