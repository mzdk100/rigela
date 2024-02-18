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
use crate::gui::utils::{update_docs, HELP_DIR};
use crate::talent::Talented;
use nwd::NwgUi;
use nwg::NativeUi;
use rigela_utils::get_program_directory;
use std::ops::DerefMut;
use std::process::Command;
use std::sync::{Arc, Mutex};

#[derive(Default, NwgUi)]
pub struct SystemTray {
    context: Mutex<Option<Arc<Context>>>,

    #[nwg_control]
    #[nwg_events (OnInit: [SystemTray::on_init] )]
    window: nwg::MessageWindow,

    #[nwg_resource(source_file: Some("./test_rc/cog.ico"))]
    icon: nwg::Icon,

    #[nwg_control(icon: Some(& data.icon), tip: Some("RigelA"))]
    #[nwg_events(MousePressLeftUp: [SystemTray::show_menu], OnContextMenu: [SystemTray::show_menu])]
    tray: nwg::TrayNotification,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: "设置 (&S)")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::on_setting])]
    setting_item: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "帮助 (&H)")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::on_help])]
    help_item: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "退出 (&X)")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::on_exit])]
    exit_item: nwg::MenuItem,

    #[nwg_control()]
    #[nwg_events(OnNotice: [SystemTray::on_exit_notice])]
    exit_notice: nwg::Notice,
}

impl SystemTray {
    fn on_init(&self) {
        let ctx = self.context.lock().unwrap().clone().unwrap();
        ctx.work_runtime.spawn(async move {
            update_docs().await;
        });
    }

    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn on_setting(&self) {
        // Todo:
        nwg::simple_message("RigelA", "Start Setting");
    }

    fn on_help(&self) {
        let help_path = get_program_directory().join(HELP_DIR);
        Command::new("notepad")
            .arg(help_path)
            .spawn()
            .expect("Failed to start notepad");
    }

    fn on_exit_notice(&self) {
        nwg::stop_thread_dispatch();
    }

    fn on_exit(&self) {
        let context = self.context.lock().unwrap().clone();
        if let Some(context) = context {
            let ctx = context.clone();
            context.main_handler.spawn(async move {
                ctx.talent_accessor
                    .get_exit_talent()
                    .perform(ctx.clone())
                    .await;
            });
        }
    }

    fn set_context(&self, context: Arc<Context>) {
        *self.context.lock().unwrap().deref_mut() = Some(context.clone());
    }
}

pub(crate) fn show(context: Arc<Context>) {
    nwg::init().expect("Failed to init Native Windows GUI");
    let ui = SystemTray::build_ui(Default::default()).expect("Failed to build UI");
    ui.set_context(context.clone());

    let exit_sender = ui.exit_notice.sender();
    let t = context.terminator.clone();
    context.main_handler.spawn(async move {
        t.add_exiting_listener(move || {
            exit_sender.notice();
        })
        .await;
    });

    nwg::dispatch_thread_events();
}
