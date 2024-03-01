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

use std::fmt::{Debug, Formatter};
use crate::{
    JabLib::{
        JabLib,
        packages::AccessibleContext as AC,
    },
    jab::role::AccessibleRole,
};
use win_wrap::common::HWND;

pub struct AccessibleContext<'lib> {
    _lib: &'lib JabLib,
    _ac: AC,
    _vm_id: i32,
}

impl<'lib> AccessibleContext<'lib> {
    pub(crate) fn from_hwnd(lib: &'lib JabLib, h_wnd: HWND) -> Option<Self> {
        if let Some((vm_id, ac)) = lib.get_accessible_context_from_hwnd(h_wnd) {
            return Some(Self {
                _lib: lib,
                _ac: ac,
                _vm_id: vm_id,
            });
        }
        None
    }
    pub fn from_focus(lib: &'lib JabLib, h_wnd: HWND) -> Option<AccessibleContext<'lib>> {
        if let Some((vm_id, ac)) = lib.get_accessible_context_with_focus(h_wnd) {
            return Some(Self {
                _lib: lib,
                _ac: ac,
                _vm_id: vm_id,
            });
        }
        None
    }

    /**
     * 从顶级窗口的AccessibleContext返回HWND。
     * */
    pub fn get_hwnd(&self) -> HWND {
        self._lib.get_hwnd_from_accessible_context(self._vm_id, self._ac)
    }

    /**
     * 查询窗口可访问上下文的对象或鼠标指针下的对象。
     * `x` X坐标。
     * `y` Y坐标。
     * */
    pub fn get_at(&self, x: i32, y: i32) -> Option<AccessibleContext<'lib>> {
        if let Some(ac) = self._lib.get_accessible_context_at(self._vm_id, self._ac, x, y) {
            return Some(Self {
                _lib: self._lib,
                _ac: ac,
                _vm_id: self._vm_id,
            });
        }
        None
    }

    /**
     * 返回一个对象，该对象表示该可访问上下文对象的第n个子对象，其中n由值索引指定。
     * `index` 子对象索引。
     * */
    pub fn get_child(&self, index: i32) -> Option<AccessibleContext<'lib>> {
        if let Some(child) = self._lib.get_accessible_child_from_context(self._vm_id, self._ac, index) {
            return Some(Self {
                _lib: self._lib,
                _ac: child,
                _vm_id: self._vm_id,
            });
        }
        None
    }

    /**
     * 返回一个表示可访问上下文对象的父级的对象。
     * */
    pub fn get_parent(&self) -> Option<AccessibleContext<'lib>> {
        if let Some(parent) = self._lib.get_accessible_parent_from_context(self._vm_id, self._ac) {
            return Some(Self {
                _lib: self._lib,
                _ac: parent,
                _vm_id: self._vm_id,
            });
        }
        None
    }

    /**
     * 返回具有指定角色的AccessibleContext，该角色是给定对象的祖先。如果没有具有指定角色的祖先对象，则返回None。
     * `role` 角色枚举。
     * */
    pub fn get_parent_with_role(&self, role: &AccessibleRole) -> Option<AccessibleContext<'lib>> {
        if let Some(parent) = self._lib.get_parent_with_role(self._vm_id, self._ac, role.to_str()) {
            return Some(Self {
                _lib: self._lib,
                _ac: parent,
                _vm_id: self._vm_id,
            });
        }
        None
    }

    /**
     * 返回具有指定角色的AccessibleContext，该角色是给定对象的祖先。如果具有指定角色的对象不存在，则返回Java窗口的顶级对象。出现错误时返回None。
     * `role` 角色枚举。
     * */
    pub fn get_parent_with_role_else_root(&self, role: &AccessibleRole) -> Option<AccessibleContext<'lib>> {
        if let Some(parent_or_root) = self._lib.get_parent_with_role_else_root(self._vm_id, self._ac, role.to_str()) {
            return Some(Self {
                _lib: self._lib,
                _ac: parent_or_root,
                _vm_id: self._vm_id,
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
        write!(f, "AccessibleContext(vm_id:{}, ac:{})", self._vm_id, self._ac as isize)
    }
}