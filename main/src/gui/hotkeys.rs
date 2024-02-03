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

use crate::{bring_window_front, commander::CommandType::Key, context::Context};
use nwd::NwgUi;
use nwg::{InsertListViewItem, NativeUi};
use std::cell::RefCell;
use std::sync::Arc;

#[derive(Default, NwgUi)]
pub struct HotKeysForm {
    context: RefCell<Option<Arc<Context>>>,

    #[nwg_control(size: (600, 480), position: (300, 300), title: "热键自定义", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [nwg::stop_thread_dispatch()], OnInit: [HotKeysForm::load_data])]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 10)]
    layout: nwg::GridLayout,

    #[nwg_control(size: (580, 400), list_style: nwg::ListViewStyle::Detailed, focus: true,
    ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT)]
    #[nwg_layout_item(layout: layout, col: 0, col_span: 6, row: 0, row_span: 6)]
    data_view: nwg::ListView,

    #[nwg_control(text: "自定义: ")]
    #[nwg_layout_item(layout: layout, col: 0, row: 6)]
    label: nwg::Label,

    #[nwg_control(readonly: true, text: "")]
    #[nwg_layout_item(layout: layout, col: 1, row: 6, col_span: 3)]
    text_box: nwg::TextInput,

    #[nwg_control(text: "设置 (&S)")]
    #[nwg_layout_item(layout: layout, col: 4, row: 6)]
    #[nwg_events(OnButtonClick: [HotKeysForm::set_hotkey])]
    set_btn: nwg::Button,

    #[nwg_control(text: "清除 (&C)")]
    #[nwg_layout_item(layout: layout, col: 5, row: 6)]
    #[nwg_events(OnButtonClick: [HotKeysForm::clear_hotkey])]
    clear_btn: nwg::Button,
}

impl HotKeysForm {
    fn load_data(&self) {
        const COL_DATA: [(i32, &str); 3] =
            [(100, "技能名称"), (240, "初始热键"), (240, "自定义热键")];
        let dv = &self.data_view;

        for (i, (n, s)) in COL_DATA.into_iter().enumerate() {
            dv.insert_column(nwg::InsertListViewColumn {
                index: Some(i as i32),
                fmt: None,
                width: Some(n),
                text: Some(s.into()),
            });
        }

        dv.set_headers_enabled(true);
        self.update_list();
    }

    fn update_list(&self) {
        let dv = &self.data_view;
        dv.clear();

        if let Some(context) = self.context.borrow().clone() {
            let talents = context.talent_accessor.talents.clone();

            for (i, talent) in talents.iter().enumerate() {
                dv.insert_item(talent.get_doc());

                for cmd_type in talent.get_supported_cmd_list() {
                    if let Key(keys) = cmd_type {
                        let mut key_str = String::new();
                        for k in keys {
                            let k: &str = k.clone().into();
                            key_str.push_str(format!("{k} ").as_str());
                        }
                        dv.insert_item(InsertListViewItem {
                            index: Some(i as i32),
                            column_index: 1,
                            text: Some(key_str),
                            image: None,
                        });
                        break;
                    }
                }
            }
        }
    }

    fn set_hotkey(&self) {
        nwg::modal_info_message(&self.window, "Hotkey", "Press a hotkey");
    }

    fn clear_hotkey(&self) {
        nwg::modal_info_message(&self.window, "Hotkey", "Clear hotkey");
    }

    fn set_context(&self, context: Arc<Context>) {
        *self.context.borrow_mut() = Some(context);
    }
}

pub(crate) fn show(context: Arc<Context>) {
    nwg::init().expect("Failed to init Native Windows GUI");
    let ui = HotKeysForm::build_ui(Default::default()).expect("Failed to build UI");
    ui.set_context(context);
    bring_window_front!(ui.window);
    nwg::dispatch_thread_events();
}
