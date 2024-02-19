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
    hotkeys: OnceLock<(NoticeSender, NoticeSender)>,
}

// 防止重复初始化
fn already_init() -> &'static Mutex<bool> {
    static INSTANCE: OnceLock<Mutex<bool>> = OnceLock::new();
    INSTANCE.get_or_init(|| false.into())
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

            let welcome =
                WelcomeForm::build_ui(Default::default()).expect("could not build welcome form");
            welcome.set_context(context.clone());
            tx.send((
                welcome.get_show_notice_sender(),
                welcome.get_exit_notice_sender(),
            ))
            .unwrap();

            let system_tray =
                SystemTray::build_ui(Default::default()).expect("could not build tray form");
            system_tray.set_context(context.clone());
            tx.send((
                system_tray.get_show_notice_sender(),
                system_tray.get_exit_notice_sender(),
            ))
            .unwrap();

            let popup_menu = PopupMenuForm::build_ui(Default::default())
                .expect("could not build popupmenu form");
            popup_menu.set_context(context.clone());
            tx.send((
                popup_menu.get_show_notice_sender(),
                popup_menu.get_exit_notice_sender(),
            ))
            .unwrap();

            let hotkeys =
                HotKeysForm::build_ui(Default::default()).expect("could not build hotkeys form");
            hotkeys.set_context(context.clone());
            tx.send((
                hotkeys.get_show_notice_sender(),
                hotkeys.get_exit_notice_sender(),
            ))
            .unwrap();

            nwg::dispatch_thread_events()
        });

        let _ = self.welcome.set(rx.recv().unwrap());
        let _ = self.tray.set(rx.recv().unwrap());
        let _ = self.popup_menu.set(rx.recv().unwrap());
        let _ = self.hotkeys.set(rx.recv().unwrap());
    }

    pub(crate) fn show_hotkeys_form(&self) {
        self.hotkeys.get().unwrap().0.notice();
    }

    pub(crate) fn show_popup_menu(&self) {
        self.popup_menu.get().unwrap().0.notice();
    }
}

impl Debug for WinManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WinManager").finish()
    }
}
