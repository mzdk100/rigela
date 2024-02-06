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

use crate::commander::keys::Keys;
use crate::talent::Talented;
use crate::{bring_window_front, commander::CommandType::Key, context::Context};
use nwd::NwgUi;
use nwg::{modal_info_message, InsertListViewItem, NativeUi};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock, RwLock};
use win_wrap::{
    ext::LParamExt,
    hook::{KbdLlHookStruct, WindowsHook, HOOK_TYPE_KEYBOARD_LL, LLKHF_EXTENDED},
    input::{WM_KEYDOWN, WM_SYSKEYDOWN},
};

type Talent = Arc<dyn Talented + Send + Sync>;

#[derive(Default, NwgUi)]
pub struct HotKeysForm {
    context: RefCell<Option<Arc<Context>>>,
    talents: RefCell<Option<Arc<Vec<Talent>>>>,
    talent_keys: RefCell<Option<HashMap<String, Vec<Keys>>>>,
    hook: RefCell<Option<WindowsHook>>,

    #[nwg_control(size: (600, 480), position: (300, 300), title: "热键自定义", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [HotKeysForm::exit], OnInit: [HotKeysForm::load_data])]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 10)]
    layout: nwg::GridLayout,

    #[nwg_control(size: (580, 400), list_style: nwg::ListViewStyle::Detailed, focus: true,
    ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT)]
    #[nwg_layout_item(layout: layout, col: 0, col_span: 6, row: 0, row_span: 6)]
    #[nwg_events(OnKeyRelease: [HotKeysForm::dv_key_press(SELF, EVT_DATA)])]
    data_view: nwg::ListView,

    #[nwg_control(text: "自定义: ")]
    #[nwg_layout_item(layout: layout, col: 0, row: 6)]
    label: nwg::Label,

    #[nwg_control(readonly: true, text: "请输入新的热键!", flags: "DISABLED|VISIBLE")]
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
        *self.talents.borrow_mut() = Some(context.talent_accessor.talents.clone());
        *self.talent_keys.borrow_mut() = Some(
            context
                .config_manager
                .get_config()
                .hotkeys_config
                .clone()
                .talent_keys,
        );
    }

    // 更新列表项目
    fn update_list(&self) {
        let dv = &self.data_view;
        dv.clear();

        let talents = self.talents.borrow().clone().unwrap();
        for (i, talent) in talents.iter().enumerate() {
            dv.insert_item(talent.get_doc());

            for cmd_type in talent.get_supported_cmd_list() {
                if let Key(keys) = cmd_type {
                    let key_str = keys
                        .iter()
                        .map(|k| -> &str { (*k).into() })
                        .collect::<Vec<&str>>()
                        .join(" + ");

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

    // 设置热键按钮事件
    fn set_hotkey(&self) {
        self.start_custom_hotkey();
    }

    // 清除热键按钮事件
    fn clear_hotkey(&self) {
        modal_info_message(&self.window, "Hotkey", "Clear hotkey");
    }

    // 退出程序事件
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    // 设置钩子
    fn set_hook(&self) -> WindowsHook {
        let key_track: RwLock<HashMap<Keys, bool>> = RwLock::new(HashMap::new());
        let finish_custom_sender = self.finish_custom.sender();
        let cancel_custom_sender = self.cancel_custom.sender();

        WindowsHook::new(HOOK_TYPE_KEYBOARD_LL, move |w_param, l_param, next| {
            let info: &KbdLlHookStruct = l_param.to();
            let is_extended = info.flags.contains(LLKHF_EXTENDED);
            let pressed = w_param.0 == WM_KEYDOWN as usize || w_param.0 == WM_SYSKEYDOWN as usize;

            let mut map = key_track.write().unwrap();
            let cur_key = (info.vkCode, is_extended).into();
            map.insert(cur_key, pressed);

            // 当前已经按下的键位
            let keys = map
                .iter()
                .filter(|(k, p)| **k == cur_key || **p)
                .map(|(x, _)| *x)
                .collect::<Vec<Keys>>();
            let len = keys.len();

            // 读取已经按下键位到存储缓冲
            hotkeys().lock().unwrap().clear();
            hotkeys().lock().unwrap().extend(keys);

            // 有一个键位松开，完成读取
            if !pressed {
                // 仅按下Esc, 取消当前读取
                if len == 1 && cur_key == Keys::VkEscape {
                    cancel_custom_sender.notice();
                } else {
                    finish_custom_sender.notice();
                }
            }

            drop(map); // 必须先释放锁再next()，否则可能会死锁
            next()
        })
    }

    // 产生新的热键
    fn on_finish_custom(&self) {
        self.hook.take().unwrap().unhook();

        let key_str = hotkeys()
            .lock()
            .unwrap()
            .iter()
            .map(|x| -> &str { (*x).into() })
            .collect::<Vec<&str>>()
            .join("+");
        self.text_box.set_text(&key_str);

        let info = format!("您按下了\n{}", key_str);
        modal_info_message(&self.window, "info", info.as_str());
    }

    // 编辑框切换到下一个控件
    fn on_cancel_custom(&self) {
        self.hook.take().unwrap().unhook();
        // *self.enable_hook.lock().unwrap() = false;
        self.data_view.set_focus();
    }

    // 编辑框键盘事件
    #[allow(unused)]
    fn tb_key_press(&self, data: &nwg::EventData) {
        if data.on_key() != nwg::keys::TAB {
            self.start_custom_hotkey();
        }
    }

    // 列表框键盘事件
    fn dv_key_press(&self, data: &nwg::EventData) {
        if data.on_key() == nwg::keys::RETURN {
            self.start_custom_hotkey();
        }
    }

    // 开始自定义热键
    fn start_custom_hotkey(&self) {
        const INFO: &str = "请在键盘上按下您喜欢的热键，ESC取消";

        let context = self.context.borrow().clone().unwrap();
        let pf = context.performer.clone();
        context.main_handler.spawn(async move {
            pf.speak_with_sapi5(INFO.to_string()).await;
        });

        *self.hook.borrow_mut() = Some(self.set_hook());
        // *self.enable_hook.lock().unwrap() = true;
    }

    // 传入程序上下文对象
    fn set_context(&self, context: Arc<Context>) {
        *self.context.borrow_mut() = Some(context);
    }
}

fn hotkeys() -> &'static Mutex<Vec<Keys>> {
    static INSTANCE: OnceLock<Mutex<Vec<Keys>>> = OnceLock::new();
    INSTANCE.get_or_init(|| Mutex::new(vec![]))
}

pub(crate) fn show(context: Arc<Context>) {
    nwg::init().expect("Failed to init Native Windows GUI");
    let ui = HotKeysForm::build_ui(Default::default()).expect("Failed to build UI");
    ui.set_context(context);
    bring_window_front!(ui.window);
    nwg::dispatch_thread_events();
}
