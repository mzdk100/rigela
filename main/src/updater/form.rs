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
use nwd::NwgUi;
use nwg::EventData;
use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex, OnceLock},
};

const TITLE: &str = "Rigela - 更新";

#[derive(Default, NwgUi)]
pub struct App {
    pub(crate) handler: OnceLock<tokio::runtime::Handle>,
    info: Arc<Mutex<String>>,
    process: Arc<Mutex<u32>>,

    #[nwg_control( title: TITLE, size: (480, 320), position: (300,300))]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()   ], OnInit: [App::on_init] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 5)]
    layout: nwg::GridLayout,

    #[nwg_control(text: "更新日志:")]
    #[nwg_layout_item(layout: layout, row: 0, col: 0)]
    label: nwg::Label,

    #[nwg_control(readonly: true, flags: "TAB_STOP|VISIBLE", focus: true)]
    #[nwg_layout_item(layout: layout, row: 1, col: 0, row_span: 6, col_span: 4)]
    #[nwg_events(OnKeyPress: [App::on_key_press(SELF, EVT_DATA)])]
    text_box: nwg::TextBox,

    #[nwg_control(text: "下载进度:")]
    #[nwg_layout_item(layout: layout, row: 7, col: 0)]
    progress_label: nwg::Label,

    #[nwg_control(range: 1..100)]
    #[nwg_layout_item(layout: layout, row: 7, col: 1, col_span: 3)]
    progress_bar: nwg::ProgressBar,

    #[nwg_control(text: "立即更新 (&U)", size: (100, 30))]
    #[nwg_layout_item(layout: layout, row: 8, col: 2)]
    #[nwg_events(OnButtonClick: [App::on_update])]
    update_btn: nwg::Button,

    #[nwg_control(text: "取消 (&C)", size: (100, 30))]
    #[nwg_layout_item(layout: layout, row: 8, col: 3)]
    #[nwg_events(OnButtonClick: [App::on_cancel])]
    cancel_btn: nwg::Button,

    #[nwg_control()]
    #[nwg_events(OnNotice: [App::on_get_info_notice])]
    get_info_notice: nwg::Notice,

    #[nwg_control()]
    #[nwg_events(OnNotice: [App::on_download_process])]
    download_process_notice: nwg::Notice,
}

impl App {
    // 界面初始化，下载更新升级日志
    fn on_init(&self) {
        self.text_box.set_focus();
        self.text_box.set_text("正在获取更新日志...");
        self.progress_label.set_enabled(false);
        self.progress_bar.set_enabled(false);

        let info = self.info.clone();
        let sender = self.get_info_notice.sender();
        self.handler.get().unwrap().spawn(async move {
            *info.lock().unwrap().deref_mut() = utils::get_update_log()
                .await
                .unwrap_or("网络异常".to_string());
            sender.notice();
        });
    }

    fn on_key_press(&self, data: &EventData) {
        if data.on_key() == nwg::keys::TAB {
            self.update_btn.set_focus();
        }
    }

    // 开始升级操作
    fn on_update(&self) {
        self.update_btn.set_enabled(false);
        self.progress_label.set_enabled(true);
        self.progress_bar.set_enabled(true);

        let process = self.process.clone();
        let sender = self.download_process_notice.sender();

        self.handler.get().unwrap().spawn(async move {
            let cb = move |x: u32| {
                *process.lock().unwrap().deref_mut() = x;
                sender.notice();
            };
            match download_and_replace_bin(&cb).await {
                Ok(_) => {
                    cb(101);
                }
                Err(_) => {}
            }
        });
    }

    fn on_cancel(&self) {
        nwg::stop_thread_dispatch();
    }

    // 获取到新的日志更新到编辑框
    fn on_get_info_notice(&self) {
        let text = self.info.lock().unwrap().deref().clone();
        self.text_box.set_text(text.as_str());
        self.text_box.set_selection(0..0);
    }

    // 响应更新进度通知
    fn on_download_process(&self) {
        let num = self.process.lock().unwrap().deref().clone();

        if num > 100 {
            nwg::modal_info_message(&self.window, "提示:", "更新完成！");
            nwg::stop_thread_dispatch();
            return;
        }

        self.progress_bar.set_pos(num);
    }
}
