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
    jab::{
        hypertext::AccessibleHypertext, key_binding::AccessibleKeyBinding,
        relation::AccessibleRelation, role::AccessibleRole, table::AccessibleTable,
        text::AccessibleTextAttributes, version::AccessBridgeVersionInfo,
    },
    JabLib::{
        packages::{
            AccessibleActionInfo, AccessibleActions, AccessibleActionsToDo,
            AccessibleContext as AC, AccessibleContextInfo, JInt, MAX_ACTION_INFO, MAX_STRING_SIZE,
            SHORT_STRING_SIZE,
        },
        JabLib,
    },
};
use std::{
    cmp::min,
    ffi::{c_char, CStr},
    fmt::{Debug, Formatter},
};
use win_wrap::{common::HWND, ext::StringExt};

pub struct AccessibleContext<'lib> {
    _lib: &'lib JabLib,
    _ac: AC,
    _vm_id: i32,
    _info: Option<AccessibleContextInfo>,
}

impl<'lib> AccessibleContext<'lib> {
    /**
     * 创建一个实例。
     * `lib` 库引用。
     * `vm_id` 虚拟机ID。
     * `ac` 原始上下文对象。
     * */
    pub(crate) fn new(lib: &'lib JabLib, vm_id: i32, ac: AC) -> Self {
        Self {
            _lib: &lib,
            _vm_id: vm_id,
            _ac: ac,
            _info: lib.get_accessible_context_info(vm_id, ac),
        }
    }
    pub(crate) fn from_hwnd(lib: &'lib JabLib, h_wnd: HWND) -> Option<Self> {
        if let Some((vm_id, ac)) = lib.get_accessible_context_from_hwnd(h_wnd) {
            return Some(Self::new(&lib, vm_id, ac));
        }
        None
    }

    pub(crate) fn from_focus(lib: &'lib JabLib, h_wnd: HWND) -> Option<AccessibleContext<'lib>> {
        if let Some((vm_id, ac)) = lib.get_accessible_context_with_focus(h_wnd) {
            return Some(Self::new(&lib, vm_id, ac));
        }
        None
    }

    /**
     * 从顶级窗口的AccessibleContext返回HWND。
     * */
    pub fn get_hwnd(&self) -> HWND {
        self._lib
            .get_hwnd_from_accessible_context(self._vm_id, self._ac)
    }

    /**
     * 获取唯一ID。
     * */
    pub fn get_unique_id(&self) -> i32 {
        self._ac as i32
    }

    /**
     * 查询窗口可访问上下文的对象或鼠标指针下的对象。
     * `x` X坐标。
     * `y` Y坐标。
     * */
    pub fn get_at(&self, x: i32, y: i32) -> Option<AccessibleContext<'lib>> {
        if let Some(ac) = self
            ._lib
            .get_accessible_context_at(self._vm_id, self._ac, x, y)
        {
            return Some(Self::new(&self._lib, self._vm_id, ac));
        }
        None
    }

    /**
     * 返回一个对象，该对象表示该可访问上下文对象的第n个子对象，其中n由值索引指定。
     * `index` 子对象索引。
     * */
    pub fn get_child(&self, index: i32) -> Option<AccessibleContext<'lib>> {
        if let Some(child) =
            self._lib
                .get_accessible_child_from_context(self._vm_id, self._ac, index)
        {
            return Some(Self::new(&self._lib, self._vm_id, child));
        }
        None
    }

    /**
     * 返回一个表示可访问上下文对象的父级的对象。
     * */
    pub fn get_parent(&self) -> Option<AccessibleContext<'lib>> {
        if let Some(parent) = self
            ._lib
            .get_accessible_parent_from_context(self._vm_id, self._ac)
        {
            return Some(Self::new(&self._lib, self._vm_id, parent));
        }
        None
    }

    /**
     * 返回具有指定角色的AccessibleContext，该角色是给定对象的祖先。如果没有具有指定角色的祖先对象，则返回None。
     * `role` 角色枚举。
     * */
    pub fn get_parent_with_role(&self, role: &AccessibleRole) -> Option<AccessibleContext<'lib>> {
        if let Some(parent) = self
            ._lib
            .get_parent_with_role(self._vm_id, self._ac, role.to_str())
        {
            return Some(Self::new(&self._lib, self._vm_id, parent));
        }
        None
    }

    /**
     * 返回具有指定角色的AccessibleContext，该角色是给定对象的祖先。如果具有指定角色的对象不存在，则返回Java窗口的顶级对象。出现错误时返回None。
     * `role` 角色枚举。
     * */
    pub fn get_parent_with_role_else_root(
        &self,
        role: &AccessibleRole,
    ) -> Option<AccessibleContext<'lib>> {
        if let Some(parent_or_root) =
            self._lib
                .get_parent_with_role_else_root(self._vm_id, self._ac, role.to_str())
        {
            return Some(Self::new(&self._lib, self._vm_id, parent_or_root));
        }
        None
    }

    /**
     * 返回Java窗口中顶级对象的AccessibleContext。这与从from_hwnd获得的AccessibleContext相同。出现错误时返回None。
     * */
    pub fn get_top_level(&self) -> Option<AccessibleContext<'lib>> {
        if let Some(top) = self._lib.get_top_level_object(self._vm_id, self._ac) {
            return Some(Self::new(&self._lib, self._vm_id, top));
        }
        None
    }

    /**
     * 返回对象的当前ActiveDescendent的AccessibleContext。此方法假定ActiveDescendent是当前在容器对象中选择的组件。出现错误或没有选择时返回None。
     * */
    pub fn get_active_descendent(&self) -> Option<AccessibleContext<'lib>> {
        if let Some(descendent) = self._lib.get_active_descendent(self._vm_id, self._ac) {
            return Some(Self::new(&self._lib, self._vm_id, descendent));
        }
        None
    }

    /**
     * 返回当前对象在对象层次结构中的深度。对象层次结构最顶端的对象的对象深度为0。出现错误时返回-1。
     * */
    pub fn get_depth(&self) -> i32 {
        self._lib.get_object_depth(self._vm_id, self._ac)
    }

    /**
     * 获取应用程序正在使用的 Java Access Bridge 实例的版本信息。您可以使用此信息来确定您的 Java Access Bridge 版本的可用功能。
     * */
    pub fn get_version(&self) -> Option<AccessBridgeVersionInfo> {
        if let Some(version_info) = self._lib.get_version_info(self._vm_id) {
            return Some(AccessBridgeVersionInfo::from(&version_info));
        }
        None
    }

    /**
     * 获取对象名称。
     * */
    pub fn get_name(&self) -> Option<String> {
        let Some(ref info) = self._info else {
            return None;
        };
        Some(info.name.to_string_utf16())
    }

    /**
     * 获取对象描述。
     * */
    pub fn get_description(&self) -> Option<String> {
        let Some(ref info) = self._info else {
            return None;
        };
        Some(info.description.to_string_utf16())
    }

    /**
     * 获取对象角色。
     * */
    pub fn get_role(&self) -> AccessibleRole {
        let Some(ref info) = self._info else {
            return AccessibleRole::Unknown;
        };
        AccessibleRole::from_str(&info.role.to_string_utf16())
    }

    /**
     * 获取对象状态。
     * */
    pub fn get_states(&self) -> Option<String> {
        let Some(ref info) = self._info else {
            return None;
        };
        Some(info.states.to_string_utf16())
    }

    /**
     * 获取对象状态(英文描述）。
     * */
    pub fn get_states_en_us(&self) -> Option<String> {
        let Some(ref info) = self._info else {
            return None;
        };
        Some(info.states_en_US.to_string_utf16())
    }

    /**
     * 获取对象在父对象中的索引。
     * */
    pub fn get_index_in_parent(&self) -> i32 {
        let Some(ref info) = self._info else {
            return -1;
        };
        info.indexInParent
    }

    /**
     * 获取子对象数量。
     * */
    pub fn get_child_count(&self) -> i32 {
        let Some(ref info) = self._info else {
            return -1;
        };
        info.childrenCount
    }

    /**
     * 获取对象矩形边框（左边、顶部、宽度、高度）。
     * */
    pub fn get_bound_rectangle(&self) -> Option<(i32, i32, i32, i32)> {
        let Some(ref info) = self._info else {
            return None;
        };
        Some((info.x, info.y, info.width, info.height))
    }

    /**
     * 将插入符号设置为文本位置。返回是否成功。
     * `position` 文本位置。
     * */
    pub fn set_caret_position(&self, position: i32) -> bool {
        self._lib
            .set_caret_position(self._vm_id, self._ac, position)
    }

    /**
     * 获取文本插入符号的位置（左边、顶部、宽度和高度）。
     * `index` 字符索引。
     * */
    pub fn get_caret_location(&self, index: i32) -> Option<(i32, i32, i32, i32)> {
        if let Some(location) = self._lib.get_caret_location(self._vm_id, self._ac, index) {
            return Some((location.x, location.y, location.width, location.height));
        }
        None
    }

    /**
     * 返回组件可以执行的操作列表。
     * */
    pub fn get_actions(&self) -> Vec<String> {
        if let Some(actions) = self._lib.get_accessible_actions(self._vm_id, self._ac) {
            let mut names = vec![];
            for i in 0..actions.actionsCount {
                names.push(actions.actionInfo[i as usize].name.to_string_utf16())
            }
            return names;
        }
        return vec![];
    }

    /**
     * 请求组件执行可访问操作的列表。如果执行了所有操作，则返回TRUE。当第一个请求的操作失败时返回FALSE，在这种情况下，“failure”包含失败操作的索引。
     * `actions_to_do` 要执行的动作列表。
     * */
    pub fn do_actions(&self, actions_to_do: &[String]) -> (bool, i32) {
        let mut actions = unsafe { std::mem::zeroed::<AccessibleActions>() };
        actions.actionsCount = min(actions_to_do.len() as JInt, MAX_ACTION_INFO as JInt);
        for i in 0..actions.actionsCount {
            let mut name: [u16; SHORT_STRING_SIZE as usize] = [0; SHORT_STRING_SIZE as usize];
            for (i, x) in actions_to_do[i as usize].encode_utf16().enumerate() {
                name[i] = x;
            }
            actions.actionInfo[i as usize] = AccessibleActionInfo { name };
        }
        self._lib.do_accessible_actions(
            self._vm_id,
            self._ac,
            &AccessibleActionsToDo::from_actions(&actions),
        )
    }
    /**
     * 返回有关对象的关系对象的信息。
     * */
    pub fn get_relations(&self) -> Vec<AccessibleRelation> {
        if let Some(r) = self._lib.get_accessible_relation_set(self._vm_id, self._ac) {
            let mut relations = vec![];
            for i in 0..r.relationCount {
                let item = &r.relations[i as usize];
                let mut targets = vec![];
                for j in 0..item.targetCount {
                    targets.push(Self::new(&self._lib, self._vm_id, item.targets[j as usize]))
                }
                relations.push(AccessibleRelation {
                    key: item.key.to_string_utf16(),
                    targets,
                });
            }
            return relations;
        }
        vec![]
    }

    /**
     * 返回与组件关联的键绑定的列表。
     * */
    pub fn get_key_bindings(&self) -> Vec<AccessibleKeyBinding> {
        if let Some(key) = self._lib.get_accessible_key_bindings(self._vm_id, self._ac) {
            let mut keys = vec![];
            for i in 0..key.keyBindingsCount {
                keys.push(AccessibleKeyBinding::from(&key.keyBindingInfo[i as usize]));
            }
            return keys;
        }
        vec![]
    }

    /**
     * 返回与组件关联的图标列表。
     * 每一个图标包含描述、宽度和高度信息。
     * */
    pub fn get_icons(&self) -> Vec<(String, i32, i32)> {
        if let Some(icons) = self._lib.get_accessible_icons(self._vm_id, self._ac) {
            let mut ret = vec![];
            for i in 0..icons.iconsCount {
                let item = &icons.iconInfo[i as usize];
                ret.push((item.description.to_string_utf16(), item.width, item.height));
            }
            return ret;
        }
        vec![]
    }

    /**
     * 获取基于JAWS算法的组件的AccessibleName。返回是否成功。
     * Bug ID 4916682-实现JAWS AccessibleName策略
     * `len` 名称长度。
     * */
    pub fn get_virtual_name(&self, len: i32) -> Option<String> {
        let Some(name) = self
            ._lib
            .get_virtual_accessible_name(self._vm_id, self._ac, len)
            else {
                return None;
            };
        Some(name.to_string_utf16())
    }

    /**
     * 获取当前值。
     * `len` 存放值的字符串长度。
     * */
    pub fn get_current_value(&self, len: i32) -> Option<String> {
        let Some(v) =
            self._lib
                .get_current_accessible_value_from_context(self._vm_id, self._ac, len as i16)
            else {
                return None;
            };
        let val = v.to_string_utf16();
        if val.is_empty() {
            return None;
        }
        Some(val)
    }

    /**
     * 获取最大值。
     * `len` 存放值的字符串长度。
     * */
    pub fn get_maximum_value(&self, len: i32) -> Option<String> {
        let Some(v) =
            self._lib
                .get_maximum_accessible_value_from_context(self._vm_id, self._ac, len as i16)
            else {
                return None;
            };
        let val = v.to_string_utf16();
        if val.is_empty() {
            return None;
        }
        Some(val)
    }

    /**
     * 获取最小值。
     * `len` 存放值的字符串长度。
     * */
    pub fn get_minimum_value(&self, len: i32) -> Option<String> {
        let Some(v) =
            self._lib
                .get_minimum_accessible_value_from_context(self._vm_id, self._ac, len as i16)
            else {
                return None;
            };
        let val = v.to_string_utf16();
        if val.is_empty() {
            return None;
        }
        Some(val)
    }

    /**
     * 获取表格对象。
     * */
    pub fn get_table(&self) -> Option<AccessibleTable> {
        AccessibleTable::new(&self._lib, self._vm_id, self._ac)
    }

    /**
     * 获取超文本对象。
     * */
    pub fn get_hypertext(&self) -> Option<AccessibleHypertext> {
        let Some(info) = self._lib.get_accessible_hypertext(self._vm_id, self._ac) else {
            return None;
        };
        Some(AccessibleHypertext::new(
            self._lib,
            self._vm_id,
            self._ac,
            info,
        ))
    }

    /**
     * 遍历组件中的超链接。返回从超链接索引start_index开始的组件的超文本对象。出现错误时返回None。
     * `start_index` 开始索引。
     * */
    pub fn get_hypertext_ext(&self, start_index: i32) -> Option<AccessibleHypertext> {
        let Some(info) = self
            ._lib
            .get_accessible_hypertext_ext(self._vm_id, self._ac, start_index)
            else {
                return None;
            };
        Some(AccessibleHypertext::new(
            self._lib,
            self._vm_id,
            self._ac,
            info,
        ))
    }

    /**
     * 请求组件的焦点。返回是否成功。
     * */
    pub fn request_focus(&self) -> bool {
        self._lib.request_focus(self._vm_id, self._ac)
    }

    /**
     * 返回组件的可见子对象数。出现错误时返回-1。
     * */
    pub fn get_visible_child_count(&self) -> i32 {
        self._lib.get_visible_children_count(self._vm_id, self._ac)
    }

    /**
     * 获取可见子对象。
     * `start_index` 起始索引。
     * */
    pub fn get_visible_children(&self, start_index: i32) -> Option<Vec<Self>> {
        let Some(info) = self
            ._lib
            .get_visible_children(self._vm_id, self._ac, start_index)
            else {
                return None;
            };
        let mut v = vec![];
        for i in 0..info.returnedChildrenCount {
            v.push(Self::new(self._lib, self._vm_id, info.children[i as usize]));
        }
        Some(v)
    }

    /**
     * 把某个项目添加到选区。
     * `index` 索引。
     * */
    pub fn add_selection(&self, index: i32) {
        self._lib
            .add_accessible_selection_from_context(self._vm_id, self._ac, index)
    }

    /**
     * 把某个项目从选区中移除。
     * `index` 索引。
     * */
    pub fn remove_selection(&self, index: i32) {
        self._lib
            .remove_accessible_selection_from_context(self._vm_id, self._ac, index)
    }

    /**
     * 清除选区。
     * */
    pub fn clear_selection(&self) {
        self._lib
            .clear_accessible_selection_from_context(self._vm_id, self._ac)
    }

    /**
     * 选择所有。
     * */
    pub fn select_all(&self) {
        self._lib
            .select_all_accessible_selection_from_context(self._vm_id, self._ac)
    }

    /**
     * 获取选区中的对象数量。
     * */
    pub fn get_selection_count(&self) -> i32 {
        self._lib
            .get_accessible_selection_count_from_context(self._vm_id, self._ac)
    }

    /**
     * 获取选区中的对象。
     * `index` 索引。
     * */
    pub fn get_selection(&self, index: i32) -> Option<Self> {
        let Some(obj) =
            self._lib
                .get_accessible_selection_from_context(self._vm_id, self._ac, index)
            else {
                return None;
            };
        Some(Self::new(self._lib, self._vm_id, obj as AC))
    }

    /**
     * 判断子对象是否被选中。
     * `index` 子对象索引。
     * */
    pub fn is_child_selected(&self, index: i32) -> bool {
        self._lib
            .is_accessible_child_selected_from_context(self._vm_id, self._ac, index)
    }

    /**
     * 判断对象是否支持选择模式。
     * */
    pub fn is_supported_selection(&self) -> bool {
        let Some(ref info) = self._info else {
            return false;
        };
        info.accessibleSelection != 0
    }

    /**
     * 在两个索引之间选择文本。所选内容包括起始索引处的文本和结束索引处的文字。返回是否成功。
     * `start_index` 开始索引。
     * `end_index` 结束索引。
     * */
    pub fn select_text_range(&self, start_index: i32, end_index: i32) -> bool {
        self._lib
            .select_text_range(self._vm_id, self._ac, start_index, end_index)
    }

    /**
     * 获取两个索引之间的文本属性。属性列表包括起始索引处的文本和结束索引处的文字。
     * `start_index` 开始位置。
     * `end_index` 结束位置。
     * */
    pub fn get_text_attributes_in_range(
        &self,
        start_index: i32,
        end_index: i32,
    ) -> Option<(AccessibleTextAttributes, i16)> {
        let Some(info) =
            self._lib
                .get_text_attributes_in_range(self._vm_id, self._ac, start_index, end_index)
            else {
                return None;
            };
        Some((AccessibleTextAttributes::new(info.0), info.1))
    }

    /**
     * 获取文本选区信息（开始索引、结束索引和选中文字）。
     * */
    pub fn get_text_selection(&self) -> Option<(i32, i32, String)> {
        if self._info.is_none() || self._info.as_ref().unwrap().accessibleText == 0 {
            return None;
        }
        let Some(info) = self
            ._lib
            .get_accessible_text_selection_info(self._vm_id, self._ac)
            else {
                return None;
            };
        Some((
            info.selectionStartIndex,
            info.selectionEndIndex,
            info.selectedText.to_string_utf16(),
        ))
    }

    /**
     * 获取文本信息（字符数、插入点索引、坐标处索引）。
     * `x` X坐标。
     * `y` Y坐标。
     * */
    pub fn get_text_info(&self, x: i32, y: i32) -> Option<(i32, i32, i32)> {
        if self._info.is_none() || self._info.as_ref().unwrap().accessibleText == 0 {
            return None;
        }
        let Some(info) = self
            ._lib
            .get_accessible_text_info(self._vm_id, self._ac, x, y)
            else {
                return None;
            };
        Some((info.charCount, info.caretIndex, info.indexAtPoint))
    }

    /**
     * 获取文本属性。
     * `index` 索引。
     * */
    pub fn get_text_attributes(&self, index: i32) -> Option<(AccessibleTextAttributes, String)> {
        if self._info.is_none() || self._info.as_ref().unwrap().accessibleText == 0 {
            return None;
        }
        let (text, info) = self
            ._lib
            .get_accessible_text_attributes(self._vm_id, self._ac, index);
        unsafe {
            Some((
                AccessibleTextAttributes::new(info),
                CStr::from_ptr(text as *const c_char)
                    .to_string_lossy()
                    .to_string(),
            ))
        }
    }

    /**
     * 获取文本项目（字符、单词和句子）。
     * `index` 索引。
     * */
    pub fn get_text_items(&self, index: i32) -> Option<(u16, String, String)> {
        if self._info.is_none() || self._info.as_ref().unwrap().accessibleText == 0 {
            return None;
        }
        let Some(info) = self
            ._lib
            .get_accessible_text_items(self._vm_id, self._ac, index)
            else {
                return None;
            };
        let word = info.word.to_string_utf16();
        let sentence = info.sentence.to_string_utf16();
        Some((info.letter, word, sentence))
    }

    /**
     * 获取文本范围。
     * `start_index` 开始索引。
     * `end_index` 结束索引。
     * */
    pub fn get_text_range(&self, start_index: i32, end_index: i32) -> Option<String> {
        if self._info.is_none() || self._info.as_ref().unwrap().accessibleText == 0 {
            return None;
        }
        let Some(info) = self._lib.get_accessible_text_range(
            self._vm_id,
            self._ac,
            start_index,
            end_index,
            (end_index - start_index).abs() as i16,
        ) else {
            return None;
        };
        Some(info.to_string_utf16())
    }

    /**
     * 获取文本行边界。
     * `index` 索引。
     * */
    pub fn get_text_line_bounds(&self, index: i32) -> Option<(i32, i32)> {
        if self._info.is_none() || self._info.as_ref().unwrap().accessibleText == 0 {
            return None;
        }
        self._lib
            .get_accessible_text_line_bounds(self._vm_id, self._ac, index)
    }

    /**
     * 获取文本矩形范围（左边、顶部、宽度和高度）。
     * `index` 索引。
     * */
    pub fn get_text_rect(&self, index: i32) -> Option<(i32, i32, i32, i32)> {
        if self._info.is_none() || self._info.as_ref().unwrap().accessibleText == 0 {
            return None;
        }
        let Some(info) = self
            ._lib
            .get_accessible_text_rect(self._vm_id, self._ac, index)
            else {
                return None;
            };
        Some((info.x, info.y, info.width, info.height))
    }

    /**
     * 设置可编辑的文本内容。返回是否成功。
     * `text` 文字内容。
     * */
    pub fn set_text_contents(&self, text: &str) -> bool {
        let mut value: [u16; (MAX_STRING_SIZE - 1) as usize] = [0; (MAX_STRING_SIZE - 1) as usize];
        text.encode_utf16().enumerate().for_each(|(i, c)| {
            if i < (MAX_STRING_SIZE - 1) as usize {
                value[i] = c
            }
        });
        self._lib
            .set_text_contents(self._vm_id, self._ac, value.as_ptr())
    }
}

impl<'lib> PartialEq for AccessibleContext<'lib> {
    fn eq(&self, other: &Self) -> bool {
        self._lib.is_same_object(self._vm_id, self._ac, other._ac)
    }
}

impl<'lib> Eq for AccessibleContext<'lib> {}

impl<'lib> Drop for AccessibleContext<'lib> {
    fn drop(&mut self) {
        self._lib.release_java_object(self._vm_id, self._ac);
    }
}

impl<'lib> Debug for AccessibleContext<'lib> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut info = String::new();
        if let Some(name) = self.get_name() {
            info += format!("name:{},", name).as_str();
        }
        if let Some(description) = self.get_description() {
            info += format!("description:{},", description).as_str();
        }
        let role = self.get_role();
        info += format!("role:{},", role.to_str()).as_str();
        if let Some(states) = self.get_states() {
            info += format!("states:{},", states).as_str();
        }
        write!(f, "AccessibleContext({})", info)
    }
}

unsafe impl<'lib> Send for AccessibleContext<'lib> {}

unsafe impl<'lib> Sync for AccessibleContext<'lib> {}
