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

pub mod context;
pub mod role;
pub mod version;

use crate::{jab::context::AccessibleContext, JabLib::JabLib};
use rigela_utils::library::get_library_path;
use win_wrap::common::HWND;

#[derive(Debug)]
pub struct Jab {
    _lib: JabLib,
}

impl Jab {
    /**
     * 创建一个新实例。
     * */
    pub fn new() -> Self {
        let path = get_library_path("windowsaccessbridge-64.dll");
        let lib = JabLib::new(Some(path)).unwrap();
        Self { _lib: lib }
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
}

#[cfg(all(test, target_arch = "x86_64"))]
mod test_jab {
    use crate::jab::role::AccessibleRole;
    use crate::jab::Jab;
    use crate::JabLib::{
        packages::{AccessibleActionsToDo, AccessibleContext},
        JabLib,
    };
    use win_wrap::common::{find_window, get_desktop_window};

    #[test]
    fn main() {
        let jab = Jab::new();
        assert!(!jab.is_java_window(get_desktop_window()));
        let h_wnd = find_window(Some("SunAwtFrame"), None);
        assert!(jab.is_java_window(h_wnd));
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
        dbg!(&jab);
    }

    fn test1(jab: &JabLib, vm_id: i32, ac: AccessibleContext) {
        dbg!(jab.set_caret_position(vm_id, ac, 2));
        dbg!(jab.get_accessible_actions(vm_id, ac));
        dbg!(jab.get_accessible_relation_set(vm_id, ac));
        dbg!(jab.get_accessible_key_bindings(vm_id, ac));
        dbg!(jab.get_accessible_icons(vm_id, ac));
        dbg!(jab.get_virtual_accessible_name(vm_id, ac, 10));
        dbg!(jab.get_current_accessible_value_from_context(vm_id, ac, 10));
        dbg!(jab.get_maximum_accessible_value_from_context(vm_id, ac, 10));
        dbg!(jab.get_minimum_accessible_value_from_context(vm_id, ac, 10));
    }

    fn test2(jab: &JabLib, vm_id: i32, ac: AccessibleContext) {
        dbg!(jab.get_accessible_table_column_description(vm_id, ac, 0));
        dbg!(jab.get_accessible_table_row_description(vm_id, ac, 0));
        dbg!(jab.get_accessible_table_row_header(vm_id, ac));
        dbg!(jab.get_accessible_table_column_header(vm_id, ac));
        if let Some(info) = jab.get_accessible_table_info(vm_id, ac) {
            dbg!(&info);
            dbg!(jab.is_accessible_table_row_selected(vm_id, info.accessibleTable, 0));
            dbg!(jab.is_accessible_table_column_selected(vm_id, info.accessibleTable, 0));
            dbg!(jab.get_accessible_table_cell_info(vm_id, info.accessibleTable, 0, 0));
            dbg!(jab.get_accessible_table_column(vm_id, info.accessibleTable, 0));
            dbg!(jab.get_accessible_table_row(vm_id, info.accessibleTable, 0));
            dbg!(jab.get_accessible_table_column_selection_count(vm_id, info.accessibleTable));
            dbg!(jab.get_accessible_table_row_selection_count(vm_id, info.accessibleTable));
            dbg!(jab.get_accessible_table_index(vm_id, info.accessibleTable, 0, 0));
            dbg!(jab.get_accessible_table_column_selections(vm_id, info.accessibleTable, 1));
            dbg!(jab.get_accessible_table_row_selections(vm_id, info.accessibleTable, 1));
        }
    }

    fn test3(jab: &JabLib, vm_id: i32, ac: AccessibleContext) {
        let info = jab.get_accessible_hypertext(vm_id, ac).unwrap();
        dbg!(&info);
        dbg!(jab.activate_accessible_hyperlink(vm_id, ac, info.links[0].accessibleHyperlink));
        dbg!(jab.get_accessible_hyperlink(vm_id, info.accessibleHypertext, 0));
        dbg!(jab.get_accessible_hyperlink_count(vm_id, info.accessibleHypertext));
        dbg!(jab.get_accessible_hypertext_link_index(vm_id, info.accessibleHypertext, 0));
        dbg!(jab.get_accessible_hypertext_ext(vm_id, ac, 0));
        let actions = jab.get_accessible_actions(vm_id, ac).unwrap();
        dbg!(jab.do_accessible_actions(vm_id, ac, &AccessibleActionsToDo::from_actions(&actions)));
        dbg!(jab.set_text_contents(vm_id, ac, [103u16, 104, 0].as_ptr()));
    }

    fn test4(jab: &JabLib, vm_id: i32, ac: AccessibleContext) {
        dbg!(jab.request_focus(vm_id, ac));
        dbg!(jab.get_visible_children_count(vm_id, ac));
        dbg!(jab.get_visible_children(vm_id, ac, 0));
        dbg!(jab.get_events_waiting());
        dbg!(jab.get_caret_location(vm_id, ac, 0));
        let info = jab.get_accessible_context_info(vm_id, ac).unwrap();
        dbg!(&info);
        jab.add_accessible_selection_from_context(vm_id, ac, 0);
        jab.remove_accessible_selection_from_context(vm_id, ac, 0);
        jab.clear_accessible_selection_from_context(vm_id, ac);
        jab.select_all_accessible_selection_from_context(vm_id, ac);
        dbg!(jab.get_accessible_selection_count_from_context(vm_id, ac));
        dbg!(jab.get_accessible_selection_from_context(vm_id, ac, 0));
        dbg!(jab.is_accessible_child_selected_from_context(vm_id, ac, 0));
    }

    fn test5(jab: &JabLib, vm_id: i32, ac: AccessibleContext) {
        dbg!(jab.select_text_range(vm_id, ac, 0, 8));
        dbg!(jab.get_text_attributes_in_range(vm_id, ac, 2, 5));
        dbg!(jab.get_accessible_text_selection_info(vm_id, ac));
        dbg!(jab.get_accessible_text_info(vm_id, ac, 100, 100));
        dbg!(jab.get_accessible_text_attributes(vm_id, ac, 0));
        dbg!(jab.get_accessible_text_items(vm_id, ac, 0));
        dbg!(jab.get_accessible_text_line_bounds(vm_id, ac, 0));
        dbg!(jab.get_accessible_text_range(vm_id, ac, 0, 20, 20));
        dbg!(jab.get_accessible_text_rect(vm_id, ac, 0));
    }
}
