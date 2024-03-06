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

pub mod callback;
pub mod context;
pub mod hyperlink;
pub mod hypertext;
pub mod key_binding;
pub mod relation;
pub mod role;
pub mod table;
pub mod text;
pub mod version;

use std::sync::{Arc, Mutex, OnceLock};
use crate::{
    add_event_fp,
    jab::{
        context::AccessibleContext,
        callback::{AccessibleCallback, AccessibleContextType},
    },
    JabLib::{
        JabLib,
        packages::JObject64,
    },
};
use rigela_utils::library::get_library_path;
use win_wrap::{
    common::HWND,
    ext::StringExt,
};

static mut LIB: OnceLock<JabLib> = OnceLock::new();
static FUNCS: OnceLock<Mutex<Vec<AccessibleCallback>>> = OnceLock::new();

#[derive(Debug)]
pub struct Jab {
    _lib: &'static JabLib,
}

impl Jab {
    /**
     * 创建一个新实例。
     * */
    pub fn new() -> Self {
        let lib = unsafe {
            LIB.get_or_init(|| {
                let path = get_library_path("windowsaccessbridge-64.dll");
                JabLib::new(Some(path)).unwrap()
            })
        };
        Self {
            _lib: lib
        }
    }

    /**
     * 从窗口上的焦点对象获取上下文。
     * `target` 目标窗口句柄。
     * */
    pub fn get_context_from_hwnd(&self, target: HWND) -> Option<AccessibleContext> {
        AccessibleContext::from_hwnd(&self._lib, target)
    }

    /**
     * 从窗口获取上下文。
     * `h_wnd` 父窗口句柄。
     * */
    pub fn get_context_with_focus(&self, h_wnd: HWND) -> Option<AccessibleContext> {
        AccessibleContext::from_focus(&self._lib, h_wnd)
    }

    /**
     * 判断窗口是否是java的窗口。
     * `h_wnd` 一个窗口句柄。
     * */
    pub fn is_java_window(&self, h_wnd: HWND) -> bool {
        self._lib.is_java_window(h_wnd)
    }

    /**
     * 获取等待触发的事件数。
     * */
    pub fn get_events_waiting(&self) -> i32 {
        self._lib.get_events_waiting()
    }

    /**
     * 移除所有监听器。
     * */
    pub fn remove_all_listeners(&self) {
        let Some(lock) = FUNCS.get() else {
            return;
        };
        let mut lock = lock.lock().unwrap();
        lock.clear();
    }
}

add_event_fp!(general,LIB, FUNCS, cb_caret_update,add_on_caret_update_listener,AccessibleCallback::CaretUpdate,set_caret_update_fp, "插入点改变");
add_event_fp!(general,LIB, FUNCS, cb_focus_gained,add_on_focus_gained_listener,AccessibleCallback::FocusGained,set_focus_gained_fp, "得到焦点");
add_event_fp!(general,LIB, FUNCS, cb_mouse_clicked,add_on_mouse_clicked_listener,AccessibleCallback::MouseClicked,set_mouse_clicked_fp, "鼠标点击");
add_event_fp!(general,LIB, FUNCS, cb_mouse_entered,add_on_mouse_entered_listener,AccessibleCallback::MouseEntered,set_mouse_entered_fp, "鼠标进入");
add_event_fp!(general,LIB, FUNCS, cb_mouse_exited,add_on_mouse_exited_listener,AccessibleCallback::MouseExited,set_mouse_exited_fp, "鼠标离开");
add_event_fp!(general,LIB, FUNCS, cb_mouse_pressed,add_on_mouse_pressed_listener,AccessibleCallback::MousePressed,set_mouse_pressed_fp, "鼠标被按下");
add_event_fp!(general,LIB, FUNCS, cb_mouse_released,add_on_mouse_released_listener,AccessibleCallback::MouseReleased,set_mouse_released_fp, "鼠标被释放");
add_event_fp!(general,LIB, FUNCS, cb_menu_canceled,add_on_menu_canceled_listener,AccessibleCallback::MenuCanceled,set_menu_canceled_fp, "菜单被取消");
add_event_fp!(general,LIB, FUNCS, cb_menu_deselected,add_on_menu_deselected_listener,AccessibleCallback::MenuDeselected,set_menu_deselected_fp, "菜单被取消选择");
add_event_fp!(general,LIB, FUNCS, cb_menu_selected,add_on_menu_selected_listener,AccessibleCallback::MenuSelected,set_menu_selected_fp, "菜单被选择");
add_event_fp!(general,LIB, FUNCS, cb_popup_menu_canceled,add_on_popup_menu_canceled_listener,AccessibleCallback::PopupMenuCanceled,set_popup_menu_canceled_fp, "弹出菜单被取消");
add_event_fp!(general,LIB, FUNCS, cb_popup_menu_will_become_invisible,add_on_popup_menu_will_become_invisible_listener,AccessibleCallback::PopupMenuWillBecomeInvisible,set_popup_menu_will_become_invisible_fp, "弹出菜单即将隐藏");
add_event_fp!(general,LIB, FUNCS, cb_popup_menu_will_become_visible,add_on_popup_menu_will_become_visible_listener,AccessibleCallback::PopupMenuWillBecomeVisible,set_popup_menu_will_become_visible_fp, "弹出菜单即将显示");
add_event_fp!(general,LIB, FUNCS, cb_property_selection_change,add_on_property_selection_change_listener,AccessibleCallback::PropertySelectionChange,set_property_selection_change_fp, "属性选择改变");
add_event_fp!(general,LIB, FUNCS, cb_property_text_change,add_on_property_text_change_listener,AccessibleCallback::PropertyTextChange,set_property_text_change_fp, "属性文本改变");
add_event_fp!(general,LIB, FUNCS, cb_property_visible_data_change,add_on_property_visible_data_change_listener,AccessibleCallback::PropertyVisibleDataChange,set_property_visible_data_change_fp, "属性可见数据改变");
add_event_fp!(property_change,LIB, FUNCS, "属性改变");
add_event_fp!(property_x_change,LIB, FUNCS, cb_property_name_change,add_on_property_name_change_listener,AccessibleCallback::PropertyNameChange,set_property_name_change_fp, "属性名称改变");
add_event_fp!(property_x_change,LIB, FUNCS, cb_property_description_change,add_on_property_description_change_listener,AccessibleCallback::PropertyDescriptionChange,set_property_description_change_fp, "属性描述改变");
add_event_fp!(property_x_change,LIB, FUNCS, cb_property_state_change,add_on_property_state_change_listener,AccessibleCallback::PropertyStateChange,set_property_state_change_fp, "属性状态改变");
add_event_fp!(property_x_change,LIB, FUNCS, cb_property_value_change,add_on_property_value_change_listener,AccessibleCallback::PropertyValueChange,set_property_value_change_fp, "属性值改变");
add_event_fp!(property_caret_change,LIB, FUNCS, "属性插入点改变");
add_event_fp!(property_y_change,LIB, FUNCS, cb_property_child_change,add_on_property_child_change_listener,AccessibleCallback::PropertyChildChange,set_property_child_change_fp, "属性子对象改变");
add_event_fp!(property_y_change,LIB, FUNCS, cb_property_active_descendent_change,add_on_property_active_descendent_change_listener,AccessibleCallback::PropertyActiveDescendentChange,set_property_active_descendent_change_fp, "属性激活、取消激活");
add_event_fp!(property_x_change,LIB, FUNCS, cb_property_table_model_change,add_on_property_table_model_change_listener,AccessibleCallback::PropertyTableModelChange,set_property_table_model_change_fp, "属性表格模式改变");
add_event_fp!(java_shutdown,LIB, FUNCS, "java关闭");


impl Drop for Jab {
    fn drop(&mut self) {
        unsafe { LIB.take(); }
    }
}


#[cfg(all(test, target_arch = "x86_64"))]
mod test_jab {
    use crate::jab::{role::AccessibleRole, Jab};
    use win_wrap::{
        common::{find_window, get_desktop_window},
        message::message_loop,
    };

    #[test]
    fn main() {
        let jab = Jab::new();
        assert!(!jab.is_java_window(get_desktop_window()));
        let h_wnd = find_window(Some("SunAwtFrame"), None);
        assert!(jab.is_java_window(h_wnd));
        dbg!(jab.get_events_waiting());
        jab.add_on_property_caret_change_listener(|src, old_value, new_value| {
            dbg!(src);
            println!("{},{}", old_value, new_value);
        });
        let context = jab.get_context_from_hwnd(h_wnd).unwrap();
        dbg!(&context);
        assert_eq!(h_wnd, context.get_hwnd());
        dbg!(context.get_at(100, 100));
        let context = jab.get_context_with_focus(h_wnd).unwrap();
        dbg!(&context);
        dbg!(context.get_child(0));
        dbg!(context.get_parent());
        dbg!(context.get_parent_with_role(&AccessibleRole::ToolBar));
        dbg!(context.get_parent_with_role_else_root(&AccessibleRole::ToolBar));
        dbg!(context.get_top_level());
        dbg!(context.get_active_descendent());
        dbg!(context.get_depth());
        dbg!(context.get_version());
        dbg!(context.get_states_en_us());
        dbg!(context.get_bound_rectangle());
        dbg!(context.get_index_in_parent());
        dbg!(context.get_child_count());
        let actions = context.get_actions();
        dbg!(&actions);
        dbg!(context.do_actions(&actions));
        dbg!(context.get_relations());
        dbg!(context.get_key_bindings());
        dbg!(context.get_icons());
        dbg!(context.get_virtual_name(10));
        dbg!(context.get_current_value(10));
        dbg!(context.get_maximum_value(10));
        dbg!(context.get_minimum_value(10));
        dbg!(context.request_focus());
        dbg!(context.get_visible_child_count());
        dbg!(context.get_visible_children(0));
        dbg!(context.set_caret_position(2));
        dbg!(context.get_caret_location(2));
        if let Some(table) = context.get_table() {
            dbg!(&table);
            dbg!(table.get_column_description(0));
            dbg!(table.get_row_description(0));
            dbg!(table.get_row_header());
            dbg!(table.get_column_header());
            dbg!(table.is_row_selected(0));
            dbg!(table.is_column_selected(0));
            dbg!(table.get_cell(0, 0));
            dbg!(table.get_column(0));
            dbg!(table.get_row(0));
            dbg!(table.get_column_selection_count());
            dbg!(table.get_row_selection_count());
            dbg!(table.get_index(0, 0));
            dbg!(table.get_column_selections(1));
            dbg!(table.get_row_selections(1));
        }
        if let Some(hypertext) = context.get_hypertext() {
            dbg!(&hypertext);
            let links = hypertext.get_links();
            dbg!(&links);
            if !links.is_empty() {
                dbg!(links[0].activate());
            }
            dbg!(hypertext.get_hyperlink(0));
            dbg!(hypertext.get_hyperlink_count());
            dbg!(hypertext.get_link_index(0));
        }
        dbg!(context.get_hypertext_ext(0));
        context.add_selection(0);
        context.remove_selection(0);
        context.clear_selection();
        context.select_all();
        dbg!(context.get_selection_count());
        dbg!(context.get_selection(0));
        dbg!(context.is_child_selected(0));
        dbg!(context.is_supported_selection());
        dbg!(context.select_text_range(0, 8));
        dbg!(context.get_text_attributes_in_range(0, 8));
        dbg!(context.get_text_selection());
        dbg!(context.get_text_info(100, 100));
        dbg!(context.get_text_attributes(0));
        dbg!(context.get_text_items(0));
        dbg!(context.get_text_range(0, 8));
        dbg!(context.get_text_line_bounds(0));
        dbg!(context.get_text_rect(0));
        dbg!(context.set_text_contents("测试"));
        dbg!(&jab);
        message_loop(|_| ());
    }
}
