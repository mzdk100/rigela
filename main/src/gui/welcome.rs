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

use std::sync::Arc;
#[allow(unused_imports)]
use winit::platform::windows::EventLoopBuilderExtWindows;
use eframe::{App, EventLoopBuilderHook, Frame, NativeOptions, run_native};
use eframe::egui::{CentralPanel, Context as GuiContext, FontData, FontDefinitions, FontFamily, ViewportBuilder};
use tokio::io::AsyncReadExt;
use crate::context::Context;

pub(crate) fn show_welcome(context: Arc<Context>) {
    let event_loop_builder: Option<EventLoopBuilderHook> = Some(Box::new(|event_loop_builder| {
        event_loop_builder.with_any_thread(true);
    }));
    let options = NativeOptions {
        event_loop_builder,
        viewport: ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    struct RigelAApp;
    impl App for RigelAApp {
        #[allow(unused_must_use)]
        fn update(&mut self, ctx: &GuiContext, _frame: &mut Frame) {
            CentralPanel::default().show(ctx, |ui| {
                ui.heading("感谢您使用 RigelA");
                ui.label("RigelA是一个开源读屏项目，使用 rust 语言构建，我们尊重开放和自由，并持续为无障碍基础设施建设贡献力量，让每一个人平等享受科技是我们共同的目标！").request_focus();
                ui.button("我要捐献");
            });
        }
    }
    run_native("RigelA", options, Box::new(|cc| {
        let mut fonts = FontDefinitions::default();
        let main_handler = context.main_handler.clone();
        let ttf_file = main_handler.block_on(async move {
            let mut file = context
                .resource_accessor
                .open("bloom.ttf")
                .await
                .expect("Can't open the font resource.");
            let mut data = Vec::new();
            file.read_to_end(&mut data)
                .await
                .expect("Can't read the ttf resource.");
            data
        });
        println!("{}", ttf_file.len());
        fonts.font_data.insert(
            "bloom".to_owned(),
            FontData::from_owned(ttf_file),
        );
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "bloom".to_owned());
        fonts
            .families
            .entry(FontFamily::Monospace)
            .or_default()
            .push("bloom".to_owned());
        cc.egui_ctx.set_fonts(fonts);
        Box::new(RigelAApp)
    }))
        .expect("Can't initialize the app.");
}