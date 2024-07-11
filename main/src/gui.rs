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

mod command;
mod forms;
mod utils;

use crate::{
    context::Context,
    gui::forms::{
        about::AboutForm, popup_menu::PopupMenuForm, settings_form::SettingsForm,
        system_tray::SystemTray, welcome::WelcomeForm,
    },
};
use log::error;
use native_windows_gui::{dispatch_thread_events, init, NativeUi, NoticeSender};
use std::{
    fmt::{Debug, Formatter},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, OnceLock, Weak,
    },
    thread::spawn,
};
use win_wrap::com::co_uninitialize;

/**
 * 一个UI界面的抽象接口。
 * 可以使用rigela-macros中的GuiFormImpl派生宏标记在struct上自动实现。
 * */
pub(crate) trait GuiForm {
    fn set_context(&self, context: Weak<Context>);
    fn get_show_notice_sender(&self) -> NoticeSender;
    fn get_exit_notice_sender(&self) -> NoticeSender;
}

/// GUI提供者
#[derive(Clone, Default)]
pub(crate) struct GuiProvider {
    welcome: OnceLock<(NoticeSender, NoticeSender)>,
    tray: OnceLock<(NoticeSender, NoticeSender)>,
    popup_menu: OnceLock<(NoticeSender, NoticeSender)>,
    settings: OnceLock<(NoticeSender, NoticeSender)>,
    about: OnceLock<(NoticeSender, NoticeSender)>,
    hotkeys: OnceLock<NoticeSender>,
}

// 防止重复初始化
static ALREADY_INIT: AtomicBool = AtomicBool::new(false);

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

impl GuiProvider {
    /**
     * 创建一个实例。
     * */
    pub(crate) fn new() -> Self {
        Default::default()
    }

    /**
     * 初始化。
     * `context` 读屏的上下文环境。
     * */
    pub(crate) fn apply(&self, context: Weak<Context>) {
        if ALREADY_INIT.load(Ordering::SeqCst) {
            return;
        }
        ALREADY_INIT.store(true, Ordering::Release);

        let (tx, rx) = mpsc::channel::<(NoticeSender, NoticeSender)>();

        spawn(move || {
            init().unwrap_or_else(|e| error!("Can't initialize the native window gui. ({})", e));
            // nwg的init中使用CoInitialize初始化com为单线程模型(STA)，和读屏使用的多线程模型(MTA)有冲突，因此我们恢复STA模型到MTA。
            // 例如IA2的调用在STA模型下有可能无法成功。
            co_uninitialize();

            build_form!(welcome, WelcomeForm, context, tx);
            build_form!(tray, SystemTray, context, tx);
            build_form!(popup_menu, PopupMenuForm, context, tx);
            build_form!(settings, SettingsForm, context, tx);
            build_form!(about, AboutForm, context, tx);

            let s = settings.show_hotkeys_notice.sender().clone();
            tx.send((s.clone(), s.clone())).unwrap();

            dispatch_thread_events()
        });

        let _ = self.welcome.set(rx.recv().unwrap());
        let _ = self.tray.set(rx.recv().unwrap());
        let _ = self.popup_menu.set(rx.recv().unwrap());
        let _ = self.settings.set(rx.recv().unwrap());
        let _ = self.about.set(rx.recv().unwrap());
        let _ = self.hotkeys.set(rx.recv().unwrap().0);

        self.welcome.get().unwrap().0.notice();
    }

    /**
     * 清理GUI占用的资源。
     * */
    pub(crate) fn dispose(&self) {
        self.welcome.get().unwrap().1.notice();
        self.tray.get().unwrap().1.notice();
        self.popup_menu.get().unwrap().1.notice();
        self.settings.get().unwrap().1.notice();
    }

    pub(crate) fn show_settings_form(&self) {
        self.settings.get().unwrap().0.notice();
    }

    pub(crate) fn show_hotkeys_form(&self) {
        self.hotkeys.get().unwrap().notice();
    }

    pub(crate) fn show_popup_menu(&self) {
        self.popup_menu.get().unwrap().0.notice();
    }

    pub(crate) fn show_welcome_form(&self) {
        self.welcome.get().unwrap().0.notice();
    }

    pub(crate) fn show_about_form(&self) {
        self.about.get().unwrap().0.notice();
    }
}

impl Debug for GuiProvider {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WinManager").finish()
    }
}

#[macro_export]
macro_rules! bring_window_front {
    ($win:expr) => {
        if let native_windows_gui::ControlHandle::Hwnd(h) = $win.handle {
            let current_thread_id = win_wrap::threading::get_current_thread_id();
            let h_foreground = win_wrap::common::get_foreground_window();
            let (remote_thread_id, _) =
                win_wrap::threading::get_window_thread_process_id(h_foreground);

            win_wrap::common::attach_thread_input(current_thread_id, remote_thread_id, true);

            win_wrap::common::show_window(
                win_wrap::common::HWND(h as _),
                win_wrap::common::SW_HIDE,
            );
            win_wrap::common::show_window(
                win_wrap::common::HWND(h as _),
                win_wrap::common::SW_SHOW,
            );
            win_wrap::common::set_foreground_window(win_wrap::common::HWND(h as _));
            win_wrap::input::set_active_window(win_wrap::common::HWND(h as _));

            win_wrap::common::attach_thread_input(current_thread_id, remote_thread_id, false);
        };
    };
}
