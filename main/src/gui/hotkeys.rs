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

use crate::configs::config_operations::{get_hotkeys, save_hotkeys};
use crate::{
    bring_window_front,
    commander::{keys::Keys, CommandType::Key},
    context::Context,
    talent::Talented,
};
use nwd::NwgUi;
use nwg::{modal_message, InsertListViewItem, MessageParams, NativeUi};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use win_wrap::common::LRESULT;
use win_wrap::{
    ext::LParamExt,
    hook::{KbdLlHookStruct, WindowsHook, HOOK_TYPE_KEYBOARD_LL, LLKHF_EXTENDED},
    input::{WM_KEYDOWN, WM_SYSKEYDOWN},
};

type Talent = Arc<dyn Talented + Send + Sync>;

#[derive(Default, NwgUi)]
pub struct HotKeysForm {
    context: RefCell<Option<Arc<Context>>>,
    talents: RefCell<Arc<Vec<Talent>>>,
    talent_keys: RefCell<HashMap<String, Vec<Keys>>>,
    hotkeys: Arc<Mutex<Vec<Keys>>>,
    hook: RefCell<Option<WindowsHook>>,

    #[nwg_control(size: (600, 480), position: (300, 300), title: "热键自定义", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [HotKeysForm::on_exit], OnInit: [HotKeysForm::load_data])]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 10)]
    layout: nwg::GridLayout,

    #[nwg_control(size: (580, 400), list_style: nwg::ListViewStyle::Detailed, focus: true,
    ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT)]
    #[nwg_layout_item(layout: layout, col: 0, col_span: 6, row: 0, row_span: 6)]
    #[nwg_events(OnKeyRelease: [HotKeysForm::on_dv_key_press(SELF, EVT_DATA)],
    OnListViewItemChanged: [HotKeysForm::on_dv_selection_changed])]
    data_view: nwg::ListView,

    #[nwg_control(text: "自定义: ")]
    #[nwg_layout_item(layout: layout, col: 0, row: 6)]
    label: nwg::Label,

    #[nwg_control(readonly: true, text: "请输入新的热键!", flags: "DISABLED|VISIBLE")]
    #[nwg_layout_item(layout: layout, col: 1, row: 6, col_span: 3)]
    text_box: nwg::TextInput,

    #[nwg_control(text: "设置 (&S)")]
    #[nwg_layout_item(layout: layout, col: 4, row: 6)]
    #[nwg_events(OnButtonClick: [HotKeysForm::on_set_hotkey],
    OnKeyRelease: [HotKeysForm::on_btn_key_release(SELF, EVT_DATA, HANDLE)])]
    set_btn: nwg::Button,

    #[nwg_control(text: "清除 (&C)")]
    #[nwg_layout_item(layout: layout, col: 5, row: 6)]
    #[nwg_events(OnButtonClick: [HotKeysForm::on_clear_hotkey],
    OnKeyRelease: [HotKeysForm::on_btn_key_release(SELF, EVT_DATA, HANDLE)])]
    clear_btn: nwg::Button,

    #[nwg_control()]
    #[nwg_events(OnNotice: [HotKeysForm::on_finish_custom])]
    finish_custom: nwg::Notice,

    #[nwg_control()]
    #[nwg_events(OnNotice: [HotKeysForm::on_cancel_custom])]
    cancel_custom: nwg::Notice,
}

impl HotKeysForm {
    // 窗口初始化
    fn load_data(&self) {
        self.init_data();
        self.init_list_cols();
        self.update_list();
    }

    // 初始化列表表头
    fn init_list_cols(&self) {
        const COL_DATA: [(i32, &str); 3] =
            [(100, "技能名称"), (240, "初始热键"), (240, "自定义热键")];

        for (i, (n, s)) in COL_DATA.into_iter().enumerate() {
            self.data_view.insert_column(nwg::InsertListViewColumn {
                index: Some(i as i32),
                fmt: None,
                width: Some(n),
                text: Some(s.into()),
            });
        }

        self.data_view.set_headers_enabled(true);
    }

    // 初始化数据
    fn init_data(&self) {
        let context = self.context.borrow().clone().unwrap();
        *self.talents.borrow_mut() = context.talent_accessor.talents.clone();
        *self.talent_keys.borrow_mut() = get_hotkeys(context.clone());
    }

    // 更新列表项目
    fn update_list(&self) {
        let dv = &self.data_view;
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

    // 退出程序事件
    fn on_exit(&self) {
        nwg::stop_thread_dispatch();
    }

    // 列表框键盘事件，当列表框有选中项按下回车，启动自定义热键配置
    fn on_dv_key_press(&self, data: &nwg::EventData) {
        let index = self.get_list_sel_index();
        if data.on_key() == nwg::keys::RETURN && index != -1 {
            self.start_custom_hotkey();
        }
    }

    // 列表框选择变动， 根据选中项是否存在自定义热键，来启用清除按钮
    fn on_dv_selection_changed(&self) {
        self.clear_btn.set_enabled(false);

        let index = self.get_list_sel_index();
        if index == -1 {
            return;
        }

        let id_ = self.talents.borrow().get(index as usize).unwrap().get_id();
        if self.talent_keys.borrow().get(&id_).is_some() {
            self.clear_btn.set_enabled(true);
        }
    }

    // 编辑框键盘事件
    #[allow(unused)]
    fn on_tb_key_press(&self, data: &nwg::EventData) {
        if data.on_key() != nwg::keys::TAB {
            self.start_custom_hotkey();
        }
    }

    // 设置热键按钮事件
    fn on_set_hotkey(&self) {
        if self.get_list_sel_index() != -1 {
            self.start_custom_hotkey();
        }
    }

    // 屏蔽设置按钮的回车事件，使用空格激活，避免回车响应错误
    fn on_btn_key_release(&self, data: &nwg::EventData, _h: &nwg::ControlHandle) {
        if data.on_key() == nwg::keys::RETURN {
            //  Todo: 这里没有拦截住回车事件,需要使用句柄发送message拦截
            // return;;
        }
    }

    // 清除热键按钮事件
    fn on_clear_hotkey(&self) {
        let index = self.get_list_sel_index();
        if index == -1 {
            return;
        }

        {
            let talents = self.talents.borrow();
            let talent = talents.get(index as usize).unwrap();
            let doc = talent.get_doc();
            let id_ = talent.get_id();
            let info = format!("您确定要删除 {} 的自定义热键?", doc);

            let msg_params = MessageParams {
                title: "确认",
                content: &info,
                buttons: nwg::MessageButtons::OkCancel,
                icons: nwg::MessageIcons::Question,
            };
            if modal_message(&self.window, &msg_params) == nwg::MessageChoice::Cancel {
                return;
            }

            let context = self.context.borrow().clone().unwrap();
            let mut talent_keys = self.talent_keys.borrow_mut().clone();
            talent_keys.remove(&id_);
            save_hotkeys(context.clone(), talent_keys);
        }

        self.init_data();
        self.update_list();
    }

    // 产生新的热键
    fn on_finish_custom(&self) {
        self.hook.take().unwrap().unhook();

        let hotkeys: Vec<Keys> = self.hotkeys.lock().unwrap().clone();
        let key_str = Self::keys_to_string(&hotkeys);
        self.text_box.set_text(&key_str);

        // 这里需要包裹，不然调用init_data会闪退
        {
            let talents = self.talents.borrow();
            let talents = talents.get(self.get_list_sel_index() as usize).unwrap();
            let doc = talents.get_doc();
            let id_ = talents.get_id();
            let info = format!("您确定要将\n{}\n应用到 {} 吗?", key_str, doc);

            let msg_params = MessageParams {
                title: "确认",
                content: &info,
                buttons: nwg::MessageButtons::OkCancel,
                icons: nwg::MessageIcons::Question,
            };
            if modal_message(&self.window, &msg_params) == nwg::MessageChoice::Cancel {
                return;
            }

            let context = self.context.borrow().clone().unwrap();
            let mut talent_keys = self.talent_keys.borrow_mut().clone();
            talent_keys.insert(id_.to_string(), hotkeys);
            save_hotkeys(context.clone(), talent_keys);
        }

        self.init_data();
        self.update_list();
    }

    // 取消热键自定义
    fn on_cancel_custom(&self) {
        self.hook.take().unwrap().unhook();
    }

    // 开始自定义热键
    fn start_custom_hotkey(&self) {
        const INFO: &str = "请在键盘上按下您喜欢的热键，ESC取消";

        let context = self.context.borrow().clone().unwrap();
        let pf = context.performer.clone();
        context.main_handler.spawn(async move {
            pf.speak(INFO.to_string());
        });

        *self.hook.borrow_mut() = Some(self.set_hook());
    }

    // 设置钩子
    fn set_hook(&self) -> WindowsHook {
        let key_track = Arc::new(Mutex::new(HashMap::<Keys, bool>::new()));
        let hotkeys = Arc::clone(&self.hotkeys);
        let finish_custom_sender = self.finish_custom.sender();
        let cancel_custom_sender = self.cancel_custom.sender();

        WindowsHook::new(HOOK_TYPE_KEYBOARD_LL, move |w_param, l_param, _next| {
            let info: &KbdLlHookStruct = l_param.to();
            let is_extended = info.flags.contains(LLKHF_EXTENDED);
            let cur_key = (info.vkCode, is_extended).into();
            let pressed = w_param.0 == WM_KEYDOWN as usize || w_param.0 == WM_SYSKEYDOWN as usize;
            {
                key_track.lock().unwrap().insert(cur_key, pressed);
            }

            // 当前已经按下的键位
            let keys: Vec<_> = key_track
                .lock()
                .unwrap()
                .iter()
                .filter(|(k, p)| **k == cur_key || **p)
                .map(|(x, _)| *x)
                .collect();

            // 有一个键位松开，完成读取
            if !pressed {
                match keys.len() {
                    1 if cur_key == Keys::VkEscape || cur_key == Keys::VkReturn => {
                        cancel_custom_sender.notice()
                    }
                    _ => {
                        // 读取已经按下键位到存储缓冲
                        let mut hotkeys = hotkeys.lock().unwrap();
                        hotkeys.clear();
                        hotkeys.extend(keys);

                        finish_custom_sender.notice();
                    }
                }
            }

            LRESULT(1)
        })
    }

    // 获取当前列表项选中索引
    fn get_list_sel_index(&self) -> i32 {
        let items = self.data_view.selected_items();
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

    // 传入程序上下文对象
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
