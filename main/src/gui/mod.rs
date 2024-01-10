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

pub(crate) mod welcome;

use crate::context::Context;
use crate::gui::welcome::WelcomeFrameUi;
pub use eframe::egui::Context as GuiContext;
use eframe::{
    egui::{FontData, FontDefinitions, FontFamily, ViewportBuilder},
    run_native, App, CreationContext, EventLoopBuilderHook, NativeOptions,
};
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use winit::platform::windows::EventLoopBuilderExtWindows;

/**
 * 一个GUI页面的抽象接口。
 * */
pub trait FrameUi {
    /**
     * 把页面显示出来。
     * `context` 框架的上下文环境。
     * */
    fn show(&self, context: Arc<Context>);

    /**
     * 配置字体资源。
     * */
    fn apply_font(context: Arc<Context>, cc: &CreationContext)
    where
        Self: Sized,
    {
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
        fonts
            .font_data
            .insert("bloom".to_owned(), FontData::from_owned(ttf_file));
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
    }

    /**
     * 运行APP UI。
     * */
    fn run<A>(context: Arc<Context>, title: &str, app: A)
    where
        Self: Sized,
        A: App + 'static,
    {
        let event_loop_builder: Option<EventLoopBuilderHook> =
            Some(Box::new(|event_loop_builder| {
                event_loop_builder.with_any_thread(true);
            }));
        let options = NativeOptions {
            event_loop_builder,
            viewport: ViewportBuilder::default().with_inner_size([320.0, 240.0]),
            ..Default::default()
        };
        run_native(
            title,
            options,
            Box::new(|cc| {
                Self::apply_font(context, cc);
                Box::new(app)
            }),
        )
        .expect(format!("Can't initialize the `{}` app.", title).as_str());
    }
}

pub struct GuiAccessor {
    #[allow(dead_code)]
    pub(crate) frames: Arc<Vec<Box<dyn FrameUi + Sync + Send>>>,
}

impl GuiAccessor {
    /**
     * 创建一个UI访问器。
     * */
    pub(crate) fn new() -> Self {
        let frames: Vec<Box<dyn FrameUi + Sync + Send>> = vec![Box::new(WelcomeFrameUi)];
        Self {
            frames: frames.into(),
        }
    }
}
