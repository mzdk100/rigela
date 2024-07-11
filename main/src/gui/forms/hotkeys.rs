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

use crate::{
    commander::keyboard::combo_keys::ComboKey,
    configs::operations::{get_hotkeys, save_hotkeys},
    context::ContextAccessor,
    gui::{forms::settings_form::SettingsForm, utils::set_hook},
};
use arc_swap::access::{DynAccess, DynGuard};
use native_windows_derive::NwgPartial;
use native_windows_gui::{
    keys::{RETURN, TAB},
    modal_message, Button, ControlHandle, EventData, GridLayout, InsertListViewColumn,
    InsertListViewItem, Label, ListView, MessageButtons, MessageChoice, MessageIcons,
    MessageParams, Notice, TextInput,
};

#[derive(Default, NwgPartial)]
pub struct HotKeysUi {
    #[nwg_layout(max_size: [1200, 800], min_size: [650, 480], spacing: 20, max_column: Some(6), max_row: Some(10))]
    layout: GridLayout,

    #[nwg_layout(min_size: [600, 480], max_column: Some(4), max_row: Some(10))]
    layout2: GridLayout,

    #[nwg_control(list_style: nwg::ListViewStyle::Detailed, ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT)]
    #[nwg_layout_item(layout: layout, col: 0, col_span: 6, row: 0, row_span: 8)]
    pub(crate) data_view: ListView,

    #[nwg_control(text: & t ! ("hotkeys.lb_custom"))]
    #[nwg_layout_item(layout: layout, col: 0, row: 7)]
    lb_custom: Label,

    #[nwg_control(readonly: true, text: & t ! ("hotkeys.tb_keys_info"), flags: "DISABLED|VISIBLE")]
    #[nwg_layout_item(layout: layout, col: 1, row: 8, col_span: 3)]
    tb_keys_info: TextInput,

    #[nwg_control(text: & t ! ("hotkeys.btn_set"))]
    #[nwg_layout_item(layout: layout, col: 4, row: 8)]
    pub(crate) btn_set: Button,

    #[nwg_control(text: & t ! ("hotkeys.btn_clear"))]
    #[nwg_layout_item(layout: layout, col: 5, row: 8)]
    pub(crate) btn_clear: Button,

    #[nwg_control(text: & t ! ("hotkeys.btn_close"))]
    #[nwg_layout_item(layout: layout2, col: 3, row: 9)]
    pub(crate) btn_close: Button,

    #[nwg_control()]
    pub(crate) finish_custom: Notice,

    #[nwg_control()]
    pub(crate) cancel_custom: Notice,
}

impl SettingsForm {
    // 窗口初始化
    pub(crate) fn load_data(&self) {
        self.init_data();
        self.init_list_cols();
        self.update_list();
        self.hotkeys_ui.btn_clear.set_enabled(false);
    }

    // 初始化列表表头
    fn init_list_cols(&self) {
        let col_data = [
            (100, t!("hotkeys.col_talent_name")),
            (240, t!("hotkeys.col_init_key")),
            (240, t!("hotkeys.col_custom_key")),
        ];

        for (i, (n, s)) in col_data.into_iter().enumerate() {
            self.hotkeys_ui
                .data_view
                .insert_column(InsertListViewColumn {
                    index: Some(i as i32),
                    fmt: None,
                    width: Some(n),
                    text: Some(s.into()),
                });
        }

        self.hotkeys_ui.data_view.set_headers_enabled(true);
    }

    // 初始化数据
    fn init_data(&self) {
        let Some(context) = self.context.get() else {
            return;
        };

        *self.talent_ids.borrow_mut() = context.get_talent_provider().get_talent_ids().into();
        *self.custom_combo_keys.borrow_mut() = get_hotkeys(context.clone());
    }

    // 更新列表项目
    fn update_list(&self) {
        let dv = &self.hotkeys_ui.data_view;
        dv.clear();

        let Some(context) = self.context.get() else {
            return;
        };

        let ids = self.talent_ids.borrow().clone();
        for (i, id) in ids.iter().enumerate() {
            let talent = context.get_talent_provider().get_talent_by_id(id).unwrap();
            dv.insert_item(talent.get_doc());

            let custom_talents = self.custom_combo_keys.borrow().clone();
            let custom_talent = custom_talents.get(id);

            // 如果存在自定义热键，就仅显示自定义热键，否则显示默认热键
            let (keys_str, col) = match custom_talent {
                Some(combo_key) => (combo_key.to_string(), 2),
                None => (
                    talent
                        .get_combo_key()
                        .unwrap_or(ComboKey::default())
                        .to_string(),
                    1,
                ),
            };

            dv.insert_item(InsertListViewItem {
                index: Some(i as i32),
                column_index: col,
                text: Some(keys_str),
                image: None,
            });
        }
    }

    // 列表框键盘事件，当列表框有选中项按下回车，启动自定义热键配置
    pub(crate) fn on_dv_key_press(&self, data: &EventData) {
        let index = self.get_list_sel_index();
        if data.on_key() == RETURN && index.is_some() {
            self.start_custom_hotkey();
        }
    }

    // 列表框选择变动， 根据选中项是否存在自定义热键，来启用清除按钮
    pub(crate) fn on_dv_selection_changed(&self) {
        self.hotkeys_ui.btn_clear.set_enabled(false);

        let index = self.get_list_sel_index();
        if index.is_none() {
            return;
        }

        let ids = self.talent_ids.borrow().clone();
        let id = ids.get(index.unwrap()).unwrap();
        if self.custom_combo_keys.borrow().contains_key(id) {
            self.hotkeys_ui.btn_clear.set_enabled(true);
        }
    }

    // 编辑框键盘事件
    #[allow(unused)]
    pub(crate) fn on_tb_key_press(&self, data: &EventData) {
        if data.on_key() != TAB {
            self.start_custom_hotkey();
        }
    }

    // 设置热键按钮事件
    pub(crate) fn on_set_hotkey(&self) {
        if self.get_list_sel_index().is_some() {
            self.start_custom_hotkey();
        }
    }

    // 屏蔽设置按钮的回车事件，使用空格激活，避免回车响应错误
    pub(crate) fn on_btn_key_release(&self, data: &EventData, _h: &ControlHandle) {
        if data.on_key() == RETURN {
            //  Todo: 这里没有拦截住回车事件,需要使用句柄发送message拦截
            // return;;
        }
    }

    // 清除热键按钮事件
    pub(crate) fn on_clear_hotkey(&self) {
        let index = self.get_list_sel_index();
        if index.is_none() {
            return;
        }

        let Some(context) = self.context.get() else {
            return;
        };
        {
            let ids = self.talent_ids.borrow().clone();
            let id = ids.get(index.unwrap()).unwrap();

            let talent = context.get_talent_provider().get_talent_by_id(id).unwrap();
            let doc = talent.get_doc();
            let info = t!("hotkeys.confirm_clear", value = doc).to_string();

            let msg_params = MessageParams {
                title: &t!("hotkeys.confirm_title"),
                content: &info,
                buttons: MessageButtons::OkCancel,
                icons: MessageIcons::Question,
            };
            if modal_message(&self.window, &msg_params) == MessageChoice::Cancel {
                return;
            }

            self.custom_combo_keys.borrow_mut().remove(id);
            save_hotkeys(context.clone(), self.custom_combo_keys.borrow().clone());

            context
                .get_talent_provider()
                .update_custom_combo_key_map(context.clone());
        }

        self.init_data();
        self.update_list();
    }

    // 产生新的热键
    pub(crate) fn on_finish_custom(&self) {
        self.hook.take().unwrap().unhook();

        let combo_key: DynGuard<Option<ComboKey>> = self.hotkeys.load();
        if combo_key.is_none() {
            return;
        }
        let combo_key = combo_key.unwrap();

        let key_str = combo_key.to_string();
        self.hotkeys_ui.tb_keys_info.set_text(&key_str);

        let Some(context) = self.context.get() else {
            return;
        };
        // 这里需要包裹，不然调用init_data会闪退
        {
            let ids = self.talent_ids.borrow().clone();
            let id = ids.get(self.get_list_sel_index().unwrap()).unwrap();

            let talent = context.get_talent_provider().get_talent_by_id(id).unwrap();
            let doc = talent.get_doc();
            let info = t!("hotkeys.confirm_apply_keys", keys = key_str, value = doc).to_string();

            let msg_params = MessageParams {
                title: &t!("hotkeys.confirm_title"),
                content: &info,
                buttons: MessageButtons::OkCancel,
                icons: MessageIcons::Question,
            };
            if modal_message(&self.window, &msg_params) == MessageChoice::Cancel {
                return;
            }

            self.custom_combo_keys
                .borrow_mut()
                .insert(id.clone(), combo_key);
            save_hotkeys(context.clone(), self.custom_combo_keys.borrow().clone());

            context
                .get_talent_provider()
                .update_custom_combo_key_map(context.clone());
        }

        self.init_data();
        self.update_list();
    }

    // 取消热键自定义
    pub(crate) fn on_cancel_custom(&self) {
        self.hook.take().unwrap().unhook();
    }

    // 开始自定义热键
    fn start_custom_hotkey(&self) {
        let Some(context) = self.context.get() else {
            return;
        };

        let ctx = context.clone();
        context.get_work_runtime().spawn(async move {
            let info = t!("hotkeys.please_input_info").to_string();
            ctx.get_performer().speak(&info).await;
        });

        let senders = [
            self.hotkeys_ui.finish_custom.sender(),
            self.hotkeys_ui.cancel_custom.sender(),
        ];
        *self.hook.borrow_mut() = Some(set_hook(context.clone(), self.hotkeys.clone(), &senders));
    }

    // 获取当前列表项选中索引
    fn get_list_sel_index(&self) -> Option<usize> {
        let items = self.hotkeys_ui.data_view.selected_items();
        if items.len() == 0 {
            None
        } else {
            Some(items[0])
        }
    }
}
