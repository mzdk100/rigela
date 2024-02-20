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
use crate::gui::utils::HELP_DIR;
use crate::talent::Talented;
use rigela_utils::get_program_directory;
use std::process::Command;
use std::sync::Arc;

pub(crate) fn exit_cmd(context: Arc<Context>) {
    let ctx = context.clone();
    context.main_handler.spawn(async move {
        let talent = ctx.talent_accessor.get_exit_talent();
        talent.perform(ctx.clone()).await;
    });
}

pub(crate) fn help_cmd(_context: Arc<Context>) {
    let help_path = get_program_directory().join(HELP_DIR);
    Command::new("notepad")
        .arg(help_path)
        .spawn()
        .expect("Failed to start notepad");
}

pub(crate) fn settings_cmd(context: Arc<Context>) {
    context.window_manager.show_settings_form();
}

pub(crate) fn check_update_cmd(_context: Arc<Context>) {}

pub(crate) fn custom_hotkeys_cmd(context: Arc<Context>) {
    context.window_manager.show_hotkeys_form();
}

pub(crate) fn welcome_form_cmd(context: Arc<Context>) {
    context.window_manager.show_welcome_form();
}

pub(crate) fn donate_cmd(_context: Arc<Context>) {
    // Todo: 捐献按钮点击事件，带实现
}
