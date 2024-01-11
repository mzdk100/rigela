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

#[allow(unused_imports)]
use crate::{context::Context, gui::GuiContext};
use eframe::egui::CentralPanel;
/** 必选条目。 */
use rigela_macros::gui;
#[allow(unused_imports)]
use std::sync::Arc;

const INFO: &str = "RigelA是一个开源读屏项目，使用 rust 语言构建，我们尊重开放和自由，并持续为无障碍基础设施建设贡献力量，让每一个人平等享受科技是我们共同的目标！";
const TITLE: &str = "感谢您使用 RigelA";
const BUTTON_LABEL: &str = "我要捐献";

#[gui(doc = "欢迎页面", title = "欢迎")]
fn welcome(_context: Arc<Context>, gui_context: &GuiContext) {
    CentralPanel::default().show(gui_context, |ui| {
        ui.heading(TITLE);
        ui.text_edit_multiline(&mut String::from(INFO));

        if ui.button(BUTTON_LABEL).clicked() {
            // speak 需要 await
            // context.performer.speak_text("开始捐献。");
        }
    });
}
