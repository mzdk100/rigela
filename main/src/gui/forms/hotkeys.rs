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

use crate::gui::forms::settings_form::SettingsForm;
use crate::gui::utils::set_hook;
use crate::{
    commander::{keys::Keys, CommandType::Key},
    configs::config_operations::{get_hotkeys, save_hotkeys},
    talent::Talented,
};
use nwd::NwgPartial;
use nwg::{modal_message, InsertListViewItem, MessageParams};
use std::sync::Arc;

pub type Talent = Arc<dyn Talented + Send + Sync>;

#[derive(Default, NwgPartial)]
pub struct HotKeysUi {
    #[nwg_layout(max_size: [1200, 800], min_size: [650, 480], spacing: 20, max_column: Some(6), max_row: Some(10))]
    layout: nwg::GridLayout,

    #[nwg_layout(min_size: [600, 480], max_column: Some(4), max_row: Some(10))]
    layout2: nwg::GridLayout,

    #[nwg_control(list_style: nwg::ListViewStyle::Detailed, ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT)]
    #[nwg_layout_item(layout: layout, col: 0, col_span: 6, row: 0, row_span: 8)]
    pub(crate) data_view: nwg::ListView,

    #[nwg_control(text: & t ! ("hotkeys.lb_custom"))]
    #[nwg_layout_item(layout: layout, col: 0, row: 7)]
    lb_custom: nwg::Label,

    #[nwg_control(readonly: true, text: & t ! ("hotkeys.tb_keys_info"), flags: "DISABLED|VISIBLE")]
    #[nwg_layout_item(layout: layout, col: 1, row: 8, col_span: 3)]
    tb_keys_info: nwg::TextInput,

    #[nwg_control(text: & t ! ("hotkeys.btn_set"))]
    #[nwg_layout_item(layout: layout, col: 4, row: 8)]
    pub(crate) btn_set: nwg::Button,

    #[nwg_control(text: & t ! ("hotkeys.btn_clear"))]
    #[nwg_layout_item(layout: layout, col: 5, row: 8)]
    pub(crate) btn_clear: nwg::Button,

    #[nwg_control(text: & t ! ("hotkeys.btn_close"))]
    #[nwg_layout_item(layout: layout2, col: 3, row: 9)]
    pub(crate) btn_close: nwg::Button,

    #[nwg_control()]
    pub(crate) finish_custom: nwg::Notice,

    #[nwg_control()]
    pub(crate) cancel_custom: nwg::Notice,
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
                .insert_column(nwg::InsertListViewColumn {
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
        let context = self.context.get().unwrap().clone();
        *self.talents.borrow_mut() = context.talent_provider.talents.clone();
        *self.talent_keys.borrow_mut() = get_hotkeys(context.clone());
    }

    // 更新列表项目
    fn update_list(&self) {
        let dv = &self.hotkeys_ui.data_view;
        dv.clear();

        let talents = self.talents.borrow().clone();
        for (i, talent) in talents.iter().enumerate() {
            dv.insert_item(talent.get_doc());

            let talent_keys = self.talent_keys.borrow().clone();
            let keys = talent_keys.get(&talent.get_id());

            // 获取默认的热键组合字符串
            let get_str = |t: &Talent| {
                for cmd_type in t.get_supported_cmd_list() {
                    if let Key(keys) = cmd_type {
                        return Self::keys_to_string(&keys);
                    }
                }
                "".to_string()
            };

            // 如果存在自定义热键，就仅显示自定义热键，否则显示默认热键
            let (keys_str, col) = match keys {
                Some(keys) => (Self::keys_to_string(keys), 2),
                None => (get_str(talent), 1),
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
    pub(crate) fn on_dv_key_press(&self, data: &nwg::EventData) {
        let index = self.get_list_sel_index();
        if data.on_key() == nwg::keys::RETURN && index != -1 {
            self.start_custom_hotkey();
        }
    }

    // 列表框选择变动， 根据选中项是否存在自定义热键，来启用清除按钮
    pub(crate) fn on_dv_selection_changed(&self) {
        self.hotkeys_ui.btn_clear.set_enabled(false);

        let index = self.get_list_sel_index();
        if index == -1 {
            return;
        }

        let id_ = self.talents.borrow().get(index as usize).unwrap().get_id();
        if self.talent_keys.borrow().get(&id_).is_some() {
            self.hotkeys_ui.btn_clear.set_enabled(true);
        }
    }

    // 编辑框键盘事件
    #[allow(unused)]
    pub(crate) fn on_tb_key_press(&self, data: &nwg::EventData) {
        if data.on_key() != nwg::keys::TAB {
            self.start_custom_hotkey();
        }
    }

    // 设置热键按钮事件
    pub(crate) fn on_set_hotkey(&self) {
        if self.get_list_sel_index() != -1 {
            self.start_custom_hotkey();
        }
    }

    // 屏蔽设置按钮的回车事件，使用空格激活，避免回车响应错误
    pub(crate) fn on_btn_key_release(&self, data: &nwg::EventData, _h: &nwg::ControlHandle) {
        if data.on_key() == nwg::keys::RETURN {
            //  Todo: 这里没有拦截住回车事件,需要使用句柄发送message拦截
            // return;;
        }
    }

    // 清除热键按钮事件
    pub(crate) fn on_clear_hotkey(&self) {
        let index = self.get_list_sel_index();
        if index == -1 {
            return;
        }

        {
            let talents = self.talents.borrow();
            let talent = talents.get(index as usize).unwrap();
            let doc = talent.get_doc();
            let id_ = talent.get_id();
            let info = t!("hotkeys.confirm_clear", value = doc).to_string();

            let msg_params = MessageParams {
                title: &t!("hotkeys.confirm_title"),
                content: &info,
                buttons: nwg::MessageButtons::OkCancel,
                icons: nwg::MessageIcons::Question,
            };
            if modal_message(&self.window, &msg_params) == nwg::MessageChoice::Cancel {
                return;
            }

            let context = self.context.get().unwrap().clone();
            let mut talent_keys = self.talent_keys.borrow_mut().clone();
            talent_keys.remove(&id_);
            save_hotkeys(context.clone(), talent_keys);
        }

        self.init_data();
        self.update_list();
    }

    // 产生新的热键
    pub(crate) fn on_finish_custom(&self) {
        self.hook.take().unwrap().unhook();

        let hotkeys: Vec<Keys> = self.hotkeys.lock().unwrap().clone();
        let key_str = Self::keys_to_string(&hotkeys);
        self.hotkeys_ui.tb_keys_info.set_text(&key_str);

        // 这里需要包裹，不然调用init_data会闪退
        {
            let talents = self.talents.borrow();
            let talents = talents.get(self.get_list_sel_index() as usize).unwrap();
            let doc = talents.get_doc();
            let id_ = talents.get_id();
            let info = t!("hotkeys.confirm_apply_keys", keys = key_str, value = doc).to_string();

            let msg_params = MessageParams {
                title: &t!("hotkeys.confirm_title"),
                content: &info,
                buttons: nwg::MessageButtons::OkCancel,
                icons: nwg::MessageIcons::Question,
            };
            if modal_message(&self.window, &msg_params) == nwg::MessageChoice::Cancel {
                return;
            }

            let context = self.context.get().unwrap().clone();
            let mut talent_keys = self.talent_keys.borrow_mut().clone();
            talent_keys.insert(id_.to_string(), hotkeys);
            save_hotkeys(context.clone(), talent_keys);
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
        let context = self.context.get().unwrap().clone();

        let pf = context.performer.clone();
        context.main_handler.spawn(async move {
            let info = t!("hotkeys.please_input_info").to_string();
            pf.speak(&info).await;
        });

        let senders = [
            self.hotkeys_ui.finish_custom.sender(),
            self.hotkeys_ui.cancel_custom.sender(),
        ];
        *self.hook.borrow_mut() = Some(set_hook(self.hotkeys.clone(), &senders));
    }

    // 获取当前列表项选中索引
    fn get_list_sel_index(&self) -> i32 {
        let items = self.hotkeys_ui.data_view.selected_items();
        match items.len() {
            0 => -1,
            _ => items[0] as i32,
        }
    }

    // 键码集合转字符串
    fn keys_to_string(keys: &[Keys]) -> String {
        keys.iter()
            .map(|x| -> &str { (*x).into() })
            .collect::<Vec<&str>>()
            .join("+")
    }
}
