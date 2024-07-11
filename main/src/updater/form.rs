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

use crate::{utils, utils::download_and_replace_bin};
use log::error;
use native_windows_derive::NwgUi;
use native_windows_gui::{
    keys::TAB, stop_thread_dispatch, Button, EventData, GridLayout, Label, Notice, ProgressBar,
    TextBox, Window,
};
use rust_i18n::AtomicStr;
use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex, OnceLock},
};
use tokio::process::Command;

const TITLE: &str = "Rigela - 更新";

#[derive(Default, NwgUi)]
pub struct App {
    pub(crate) handler: OnceLock<tokio::runtime::Handle>,
    info: AStr,
    progress: Arc<Mutex<u32>>,

    #[nwg_control(title: TITLE, size: (480, 320), position: (300, 300))]
    #[nwg_events(OnWindowClose: [nwg::stop_thread_dispatch()   ], OnInit: [App::on_init])]
    window: Window,

    #[nwg_layout(parent: window, spacing: 5)]
    layout: GridLayout,

    #[nwg_control(text: "更新日志:")]
    #[nwg_layout_item(layout: layout, row: 0, col: 0)]
    label: Label,

    #[nwg_control(readonly: true, flags: "TAB_STOP|VISIBLE", focus: true)]
    #[nwg_layout_item(layout: layout, row: 1, col: 0, row_span: 6, col_span: 4)]
    #[nwg_events(OnKeyPress: [App::on_key_press(SELF, EVT_DATA)])]
    text_box: TextBox,

    #[nwg_control(text: "下载进度:")]
    #[nwg_layout_item(layout: layout, row: 7, col: 0)]
    progress_label: Label,

    #[nwg_control(range: 1..100)]
    #[nwg_layout_item(layout: layout, row: 7, col: 1, col_span: 3)]
    progress_bar: ProgressBar,

    #[nwg_control(text: "立即更新 (&U)", size: (100, 30))]
    #[nwg_layout_item(layout: layout, row: 8, col: 2)]
    #[nwg_events(OnButtonClick: [App::on_update])]
    update_btn: Button,

    #[nwg_control(text: "取消 (&C)", size: (100, 30))]
    #[nwg_layout_item(layout: layout, row: 8, col: 3)]
    #[nwg_events(OnButtonClick: [App::on_cancel])]
    cancel_btn: Button,

    #[nwg_control()]
    #[nwg_events(OnNotice: [App::on_get_info_notice])]
    get_info_notice: Notice,

    #[nwg_control()]
    #[nwg_events(OnNotice: [App::on_download_process])]
    download_process_notice: Notice,
}

impl App {
    // 界面初始化，下载更新升级日志
    fn on_init(&self) {
        self.text_box.set_focus();
        self.text_box.set_text("正在获取更新日志...");
        self.progress_label.set_enabled(false);
        self.progress_bar.set_enabled(false);

        let info = self.info.0.clone();
        let sender = self.get_info_notice.sender();
        self.handler.get().unwrap().spawn(async move {
            let text = utils::get_changelogs()
                .await
                .unwrap_or("网络异常".to_string());
            info.replace(text);
            sender.notice();
        });
    }

    fn on_key_press(&self, data: &EventData) {
        if data.on_key() == TAB {
            self.update_btn.set_focus();
        }
    }

    // 开始升级操作
    fn on_update(&self) {
        self.update_btn.set_enabled(false);
        self.progress_label.set_enabled(true);
        self.progress_bar.set_enabled(true);
        self.progress_bar.focus();

        let progress = self.progress.clone();
        let sender = self.download_process_notice.sender();

        self.handler.get().unwrap().spawn(async move {
            let cb = move |x: u32| {
                *progress.lock().unwrap().deref_mut() = x;
                sender.notice();
            };
            match download_and_replace_bin(&cb).await {
                Ok(target) => match Command::new(target).arg("--updated").spawn() {
                    Ok(_) => cb(101),
                    Err(e) => error!("Can't run rigela. {}", e),
                },
                Err(e) => error!("Can't update rigela. {}", e),
            }
        });
    }

    fn on_cancel(&self) {
        stop_thread_dispatch();
    }

    // 获取到新的日志更新到编辑框
    fn on_get_info_notice(&self) {
        let text = self.info.0.to_string().clone();
        self.text_box.set_text(text.as_str());
        self.text_box.set_selection(0..0);
    }

    // 响应更新进度通知
    fn on_download_process(&self) {
        let num = self.progress.lock().unwrap().deref().clone();

        if num > 100 {
            stop_thread_dispatch();
            return;
        }

        self.progress_bar.set_pos(num);
    }
}

struct AStr(Arc<AtomicStr>);

impl Default for AStr {
    fn default() -> Self {
        Self(Arc::new(AtomicStr::from("")))
    }
}
