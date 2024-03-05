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
        version::AccessBridgeVersionInfo,
        role::AccessibleRole,
        key_binding::AccessibleKeyBinding,
        relation::AccessibleRelation,
    },
    JabLib::{
        packages::{
            AccessibleContextInfo,
            AccessibleContext as AC,
        },
        JabLib,
    },
};
use std::fmt::{Debug, Formatter};
use win_wrap::common::HWND;

pub struct AccessibleContext<'lib> {
    _lib: &'lib JabLib,
    _ac: AC,
    _vm_id: i32,
    _info: Option<AccessibleContextInfo>,
}

impl<'lib> AccessibleContext<'lib> {
    pub(crate) fn from_hwnd(lib: &'lib JabLib, h_wnd: HWND) -> Option<Self> {
        if let Some((vm_id, ac)) = lib.get_accessible_context_from_hwnd(h_wnd) {
            return Some(Self {
                _lib: lib,
                _ac: ac,
                _vm_id: vm_id,
                _info: lib.get_accessible_context_info(vm_id, ac),
            });
        }
        None
    }

    pub(crate) fn from_focus(lib: &'lib JabLib, h_wnd: HWND) -> Option<AccessibleContext<'lib>> {
        if let Some((vm_id, ac)) = lib.get_accessible_context_with_focus(h_wnd) {
            return Some(Self {
                _lib: lib,
                _ac: ac,
                _vm_id: vm_id,
                _info: lib.get_accessible_context_info(vm_id, ac),
            });
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
     * 查询窗口可访问上下文的对象或鼠标指针下的对象。
     * `x` X坐标。
     * `y` Y坐标。
     * */
    pub fn get_at(&self, x: i32, y: i32) -> Option<AccessibleContext<'lib>> {
        if let Some(ac) = self
            ._lib
            .get_accessible_context_at(self._vm_id, self._ac, x, y)
        {
            return Some(Self {
                _lib: self._lib,
                _ac: ac,
                _vm_id: self._vm_id,
                _info: self._lib.get_accessible_context_info(self._vm_id, ac),
            });
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
            return Some(Self {
                _lib: self._lib,
                _ac: child,
                _vm_id: self._vm_id,
                _info: self._lib.get_accessible_context_info(self._vm_id, child),
            });
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
            return Some(Self {
                _lib: self._lib,
                _ac: parent,
                _vm_id: self._vm_id,
                _info: self._lib.get_accessible_context_info(self._vm_id, parent),
            });
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
            return Some(Self {
                _lib: self._lib,
                _ac: parent,
                _vm_id: self._vm_id,
                _info: self._lib.get_accessible_context_info(self._vm_id, parent),
            });
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
            return Some(Self {
                _lib: self._lib,
                _ac: parent_or_root,
                _vm_id: self._vm_id,
                _info: self
                    ._lib
                    .get_accessible_context_info(self._vm_id, parent_or_root),
            });
        }
        None
    }

    /**
     * 返回Java窗口中顶级对象的AccessibleContext。这与从from_hwnd获得的AccessibleContext相同。出现错误时返回None。
     * */
    pub fn get_top_level(&self) -> Option<AccessibleContext<'lib>> {
        if let Some(top) = self._lib.get_top_level_object(self._vm_id, self._ac) {
            return Some(Self {
                _lib: self._lib,
                _ac: top,
                _vm_id: self._vm_id,
                _info: self._lib.get_accessible_context_info(self._vm_id, top),
            });
        }
        None
    }

    /**
     * 返回对象的当前ActiveDescendent的AccessibleContext。此方法假定ActiveDescendent是当前在容器对象中选择的组件。出现错误或没有选择时返回None。
     * */
    pub fn get_active_descendent(&self) -> Option<AccessibleContext<'lib>> {
        if let Some(descendent) = self._lib.get_active_descendent(self._vm_id, self._ac) {
            return Some(Self {
                _lib: self._lib,
                _ac: descendent,
                _vm_id: self._vm_id,
                _info: self
                    ._lib
                    .get_accessible_context_info(self._vm_id, descendent),
            });
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
        Some(
            String::from_utf16_lossy(&info.name)
                .trim_matches('\0')
                .to_string(),
        )
    }

    /**
     * 获取对象描述。
     * */
    pub fn get_description(&self) -> Option<String> {
        let Some(ref info) = self._info else {
            return None;
        };
        Some(
            String::from_utf16_lossy(&info.description)
                .trim_matches('\0')
                .to_string(),
        )
    }

    /**
     * 获取对象角色。
     * */
    pub fn get_role(&self) -> AccessibleRole {
        let Some(ref info) = self._info else {
            return AccessibleRole::Unknown;
        };
        AccessibleRole::from_str(
            String::from_utf16_lossy(&info.role)
                .trim_matches('\0')
        )
    }

    /**
     * 获取对象状态。
     * */
    pub fn get_states(&self) -> Option<String> {
        let Some(ref info) = self._info else {
            return None;
        };
        Some(
            String::from_utf16_lossy(&info.states)
                .trim_matches('\0')
                .to_string(),
        )
    }

    /**
     * 获取对象状态(英文描述）。
     * */
    pub fn get_states_en_us(&self) -> Option<String> {
        let Some(ref info) = self._info else {
            return None;
        };
        Some(
            String::from_utf16_lossy(&info.states_en_US)
                .trim_matches('\0')
                .to_string(),
        )
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
        self._lib.set_caret_position(self._vm_id, self._ac, position)
    }

    /**
     * 返回组件可以执行的操作列表。
     * */
    pub fn get_actions(&self) -> Vec<String> {
        if let Some(actions) = self._lib.get_accessible_actions(self._vm_id, self._ac) {
            let mut names = vec![];
            for i in 0..actions.actionsCount {
                names.push(String::from_utf16_lossy(&actions.actionInfo[i as usize].name).trim_matches('\0').to_string())
            }
            return names;
        }
        return vec![];
    }
    pub fn get_relations(&self) -> Vec<AccessibleRelation> {
        if let Some(r) = self._lib.get_accessible_relation_set(self._vm_id, self._ac) {
            let mut relations = vec![];
            for i in 0..r.relationCount {
                let item = &r.relations[i as usize];
                let mut targets = vec![];
                for j in 0..item.targetCount {
                    let c = AccessibleContext {
                        _lib: &self._lib,
                        _ac: item.targets[j as usize],
                        _vm_id: self._vm_id,
                        _info: self._lib.get_accessible_context_info(self._vm_id, item.targets[j as usize]),
                    };
                    targets.push(c)
                }
                relations.push(AccessibleRelation {
                    key: String::from_utf16_lossy(&item.key).trim_matches('\0').to_string(),
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
                ret.push((String::from_utf16_lossy(&item.description).trim_matches('\0').to_string(), item.width, item.height));
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
        let Some(name) = self._lib.get_virtual_accessible_name(self._vm_id, self._ac, len) else {
            return None;
        };
        Some(String::from_utf16_lossy(&name).trim_matches('\0').to_string())
    }

    /**
     * 获取当前值。
     * `len` 存放值的字符串长度。
     * */
    pub fn get_current_value(&self, len: i32) -> Option<String> {
        let Some(v) = self._lib.get_current_accessible_value_from_context(self._vm_id, self._ac, len as i16) else {
            return None;
        };
        let val = String::from_utf16_lossy(&v).trim_matches('\0').to_string();
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
        let Some(v) = self._lib.get_maximum_accessible_value_from_context(self._vm_id, self._ac, len as i16) else {
            return None;
        };
        let val = String::from_utf16_lossy(&v).trim_matches('\0').to_string();
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
        let Some(v) = self._lib.get_minimum_accessible_value_from_context(self._vm_id, self._ac, len as i16) else {
            return None;
        };
        let val = String::from_utf16_lossy(&v).trim_matches('\0').to_string();
        if val.is_empty() {
            return None;
        }
        Some(val)
    }
}

impl<'lib> PartialEq for AccessibleContext<'lib> {
    fn eq(&self, other: &Self) -> bool {
        self._lib.is_same_object(self._vm_id, self._ac, other._ac)
    }
}

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
