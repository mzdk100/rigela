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

pub(crate) mod callbacks;
pub(crate) mod calls;
pub(crate) mod packages;

use crate::{
    jab,
    JabLib::{
        callbacks::{
            AccessBridgeCaretUpdateFp, AccessBridgeFocusGainedFp, AccessBridgeFocusLostFp,
            AccessBridgeJavaShutdownFp, AccessBridgeMenuCanceledFp, AccessBridgeMenuDeselectedFp,
            AccessBridgeMenuSelectedFp, AccessBridgeMouseClickedFp, AccessBridgeMouseEnteredFp,
            AccessBridgeMouseExitedFp, AccessBridgeMousePressedFp, AccessBridgeMouseReleasedFp,
            AccessBridgePopupMenuCanceledFp, AccessBridgePopupMenuWillBecomeInvisibleFp,
            AccessBridgePopupMenuWillBecomeVisibleFp, AccessBridgePropertyActiveDescendentChangeFp,
            AccessBridgePropertyCaretChangeFp, AccessBridgePropertyChangeFp,
            AccessBridgePropertyChildChangeFp, AccessBridgePropertyDescriptionChangeFp,
            AccessBridgePropertyNameChangeFp, AccessBridgePropertySelectionChangeFp,
            AccessBridgePropertyStateChangeFp, AccessBridgePropertyTableModelChangeFp,
            AccessBridgePropertyTextChangeFp, AccessBridgePropertyValueChangeFp,
            AccessBridgePropertyVisibleDataChangeFp,
        },
        packages::{
            AccessBridgeVersionInfo, AccessibleActions, AccessibleActionsToDo, AccessibleContext,
            AccessibleContextInfo, AccessibleHyperlink, AccessibleHypertext,
            AccessibleHypertextInfo, AccessibleIcons, AccessibleKeyBindings,
            AccessibleRelationSetInfo, AccessibleSelection, AccessibleTable,
            AccessibleTableCellInfo, AccessibleTableInfo, AccessibleText,
            AccessibleTextAttributesInfo, AccessibleTextInfo, AccessibleTextItemsInfo,
            AccessibleTextRectInfo, AccessibleTextSelectionInfo, AccessibleValue, JInt, JObject,
            JObject64, JavaObject, VisibleChildrenInfo, BOOL,
        },
    },
};
use rigela_utils::call_proc;
use std::{
    env::var,
    path::{Path, PathBuf},
};
use win_wrap::common::{
    free_library, get_proc_address, load_library, Result, FARPROC, HMODULE, HWND,
};
use windows::{
    core::{Error, HSTRING},
    Win32::Foundation::S_FALSE,
};

#[allow(unused)]
#[derive(Debug)]
pub(crate) struct JabLib {
    h_module: HMODULE,
}

#[allow(dead_code)]
impl JabLib {
    //noinspection RsUnresolvedPath
    //noinspection SpellCheckingInspection
    pub(crate) fn new(path: Option<PathBuf>) -> Result<Self> {
        #[cfg(target_arch = "x86_64")]
        const DLL_NAME: &str = "windowsaccessbridge-64.dll";
        #[cfg(target_arch = "x86")]
        const DLL_NAME: &str = "windowsaccessbridge-32.dll";
        let lib = match path {
            None => match var("JAVA_HOME") {
                Ok(s) => Path::new(&s).join("bin").join(DLL_NAME),
                Err(e) => {
                    return Err(Error::new(
                        S_FALSE,
                        &format!("Can't find the jab library. ({})", e),
                    ));
                }
            },
            Some(p) => p.to_path_buf(),
        };
        let h_module = match load_library(lib.to_str().unwrap()) {
            Ok(h) => h,
            Err(e) => return Err(e),
        };
        let res = jab!(h_module, windows_run);
        if res.is_none() {
            return Err(Error::new(S_FALSE, "Can't load the jab library."));
        }
        Ok(Self { h_module })
    }

    /**
     * 检查给定窗口是否实现了 Java 辅助功能 API。
     * */
    pub(crate) fn is_java_window(&self, h_wnd: HWND) -> bool {
        jab!(self.h_module, is_java_window, h_wnd).unwrap_or(0) != 0
    }

    /**
     * 获取给定窗口的 AccessibleContext和 vmID值。许多 Java Access Bridge 函数都需要 AccessibleContext和 vmID值。
     * `target` 目标窗口句柄。
     * */
    pub(crate) fn get_accessible_context_from_hwnd(
        &self,
        target: HWND,
    ) -> Option<(i32, AccessibleContext)> {
        let (mut context, mut vm_id) = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_accessible_context_from_hwnd,
            target,
            &mut vm_id,
            &mut context
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some((vm_id, context))
    }

    /**
     * 从顶级窗口的AccessibleContext返回HWND。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问的上下文。
     * */
    pub(crate) fn get_hwnd_from_accessible_context(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> HWND {
        jab!(self.h_module, get_hwnd__from_accessible_context, vm_id, ac).unwrap_or(HWND::default())
    }

    /**
     * 释放 Java 对象使用的内存，其中 object 是 Java Access Bridge 返回给您的对象。Java Access Bridge 会自动维护对它在 JVM 中返回给您的所有 Java 对象的引用，因此它们不会被垃圾回收。为了防止内存泄漏，请在完成 Java Access Bridge 返回的所有 Java 对象后调用它们。
     * `object` 一个java对象。
     * */
    pub(crate) fn release_java_object(&self, vm_id: i32, object: JavaObject) {
        jab!(self.h_module, release_java_object, vm_id, object);
    }

    /**
     * 获取应用程序正在使用的 Java Access Bridge 实例的版本信息。您可以使用此信息来确定您的 Java Access Bridge 版本的可用功能。
     * 注意：要确定 JVM 的版本，您需要传入一个有效的 vm_id;否则，返回的只是应用程序连接到WindowsAccessBridge.DLL的文件的版本。
     * `vm_id` 虚拟机ID。
     * */
    pub(crate) fn get_version_info(&self, vm_id: i32) -> Option<AccessBridgeVersionInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(self.h_module, get_version_info, vm_id, &mut info).unwrap_or(0) == 0 {
            return None;
        }
        Some(info)
    }

    /**
     * 查询窗口可访问上下文的对象或鼠标指针下的对象。
     * `parent` 父对象。
     * `x` X坐标。
     * `y` Y坐标。
     * */
    pub(crate) fn get_accessible_context_at(
        &self,
        vm_id: i32,
        parent: AccessibleContext,
        x: JInt,
        y: JInt,
    ) -> Option<AccessibleContext> {
        let mut ac = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_accessible_context_at,
            vm_id,
            parent,
            x,
            y,
            &mut ac
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(ac)
    }

    //noinspection StructuralWrap
    /**
     * 查询窗口可访问上下文的对象或具有焦点的对象。
     * `window` 要查询的窗口句柄。
     * */
    pub(crate) fn get_accessible_context_with_focus(
        &self,
        window: HWND,
    ) -> Option<(i32, AccessibleContext)> {
        let (mut vm_id, mut ac) = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_accessible_context_with_focus,
            window,
            &mut vm_id,
            &mut ac
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some((vm_id, ac))
    }

    /**
     * 从AccessibleContext对象中查询对象AccessibleContextInfo。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_accessible_context_info(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleContextInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_accessible_context_info,
            vm_id,
            ac,
            &mut info
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
     * 返回一个对象，该对象表示该可访问上下文对象的第n个子对象，其中n由值索引指定。
     * `ac` 可访问上下文。
     * `vm_id` 虚拟机ID。
     * `index` 子对象索引。
     * */
    pub(crate) fn get_accessible_child_from_context(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        index: JInt,
    ) -> Option<AccessibleContext> {
        jab!(
            self.h_module,
            get_accessible_child_from_context,
            vm_id,
            ac,
            index
        )
    }

    /**
     * 返回一个表示可访问上下文对象的父级的对象。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_accessible_parent_from_context(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleContext> {
        jab!(self.h_module, get_accessible_parent_from_context, vm_id, ac)
    }

    /**
     * 返回两个对象引用是否引用同一个对象。
     * `vm_id` 虚拟机ID。
     * `obj1` 对象1.
     * `obj2` 对象2.
     * */
    pub(crate) fn is_same_object(&self, vm_id: i32, obj1: JObject64, obj2: JObject64) -> bool {
        jab!(self.h_module, is_same_object, vm_id, obj1, obj2).unwrap_or(0) != 0
    }

    /**
     * 返回具有指定角色的AccessibleContext，该角色是给定对象的祖先。角色是Java访问桥API数据结构中定义的角色字符串之一。如果没有具有指定角色的祖先对象，则返回(AccessibleContext)0。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * `role` 角色字符串。
     * */
    pub(crate) fn get_parent_with_role(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        role: &str,
    ) -> Option<AccessibleContext> {
        jab!(
            self.h_module,
            get_parent_with_role,
            vm_id,
            ac,
            HSTRING::from(role).as_ptr()
        )
    }

    /**
     * 返回具有指定角色的AccessibleContext，该角色是给定对象的祖先。角色是Java访问桥API数据结构中定义的角色字符串之一。如果具有指定角色的对象不存在，则返回Java窗口的顶级对象。出现错误时返回(AccessibleContext)0。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * `role` 角色字符串。
     * */
    pub(crate) fn get_parent_with_role_else_root(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        role: &str,
    ) -> Option<AccessibleContext> {
        jab!(
            self.h_module,
            get_parent_with_role_else_root,
            vm_id,
            ac,
            HSTRING::from(role).as_ptr()
        )
    }

    /**
     * 返回Java窗口中顶级对象的AccessibleContext。这与从该窗口的get_accessible_context_from_hwnd获得的AccessibleContext相同。出现错误时返回(AccessibleContext)0。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_top_level_object(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleContext> {
        jab!(self.h_module, get_top_level_object, vm_id, ac)
    }

    //noinspection StructuralWrap
    /**
     * 返回给定对象在对象层次结构中的深度。对象层次结构最顶端的对象的对象深度为0。出现错误时返回-1。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_object_depth(&self, vm_id: i32, ac: AccessibleContext) -> i32 {
        jab!(self.h_module, get_object_depth, vm_id, ac).unwrap_or(-1)
    }

    /**
     * 返回对象的当前ActiveDescendent的AccessibleContext。此方法假定ActiveDescendent是当前在容器对象中选择的组件。出现错误或没有选择时返回(AccessibleContext)0。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_active_descendent(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleContext> {
        jab!(self.h_module, get_active_descendent, vm_id, ac)
    }

    /**
     * 请求某个组件的焦点。返回是否成功。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn request_focus(&self, vm_id: i32, ac: AccessibleContext) -> bool {
        jab!(self.h_module, request_focus, vm_id, ac).unwrap_or(0) != 0
    }

    //noinspection StructuralWrap
    /**
     * 返回组件的可见子级数。出现错误时返回-1。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_visible_children_count(&self, vm_id: i32, ac: AccessibleContext) -> i32 {
        jab!(self.h_module, get_visible_children_count, vm_id, ac).unwrap_or(-1)
    }

    /**
     * 获取 AccessibleContext 的可见子级。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * `start_index` 起始索引。
     * */
    pub(crate) fn get_visible_children(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        start_index: i32,
    ) -> Option<VisibleChildrenInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_visible_children,
            vm_id,
            ac,
            start_index,
            &mut info
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
     * 获取等待触发的事件数。
     * */
    pub(crate) fn get_events_waiting(&self) -> i32 {
        jab!(self.h_module, get_events_waiting).unwrap_or(0)
    }

    /**
     * 返回组件可以执行的操作列表。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_accessible_actions(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleActions> {
        let mut actions = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_accessible_actions,
            vm_id,
            ac,
            &mut actions
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(actions)
    }

    /**
     * 获取文本插入符号的位置。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * `index`
     * */
    pub(crate) fn get_caret_location(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        index: JInt,
    ) -> Option<AccessibleTextRectInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_caret_location,
            vm_id,
            ac,
            &mut info,
            index
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
     * 将插入符号设置为文本位置。返回是否成功。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * `position` 文本位置。
     * */
    pub(crate) fn set_caret_position(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        position: i32,
    ) -> bool {
        jab!(self.h_module, set_caret_position, vm_id, ac, position).unwrap_or(0) != 0
    }

    /**
     * 获取两个索引之间的文本属性。属性列表包括起始索引处的文本和结束索引处的文字。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * `start_index` 开始位置。
     * `end_index` 结束位置。
     * */
    pub(crate) fn get_text_attributes_in_range(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        start_index: i32,
        end_index: i32,
    ) -> Option<(AccessibleTextAttributesInfo, i16)> {
        let (mut info, mut len) = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_text_attributes_in_range,
            vm_id,
            ac,
            start_index,
            end_index,
            &mut info,
            &mut len
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some((info, len))
    }

    /**
     * 返回有关对象的关系对象的信息。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_accessible_relation_set(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleRelationSetInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_accessible_relation_set,
            vm_id,
            ac,
            &mut info
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
     * 返回与组件关联的键绑定的列表。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_accessible_key_bindings(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleKeyBindings> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_accessible_key_bindings,
            vm_id,
            ac,
            &mut info
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
     * 返回与组件关联的图标列表。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_accessible_icons(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleIcons> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(self.h_module, get_accessible_icons, vm_id, ac, &mut info).unwrap_or(0) == 0 {
            return None;
        }
        Some(info)
    }

    /**
     * 将指定表的表行标题作为表返回。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_accessible_table_row_header(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleTableInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_accessible_table_row_header,
            vm_id,
            ac,
            &mut info
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
     * 将指定表的表列标题作为表返回。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_accessible_table_column_header(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleTableInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_accessible_table_column_header,
            vm_id,
            ac,
            &mut info
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
     * 返回指定表中指定列的说明。列说明符是从零开始的。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * `column` 列索引。
     * */
    pub(crate) fn get_accessible_table_column_description(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        column: JInt,
    ) -> Option<AccessibleContext> {
        jab!(
            self.h_module,
            get_accessible_table_column_description,
            vm_id,
            ac,
            column
        )
    }

    /**
     * 返回指定表中指定行的描述。行说明符是从零开始的。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * `row` 行索引。
     * */
    pub(crate) fn get_accessible_table_row_description(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        row: JInt,
    ) -> Option<AccessibleContext> {
        jab!(
            self.h_module,
            get_accessible_table_row_description,
            vm_id,
            ac,
            row
        )
    }

    /**
     * 在两个索引之间选择文本。所选内容包括起始索引处的文本和结束索引处的文字。返回是否成功。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * `start_index` 开始索引。
     * `end_index` 结束索引。
     * */
    pub(crate) fn select_text_range(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        start_index: JInt,
        end_index: JInt,
    ) -> bool {
        jab!(
            self.h_module,
            select_text_range,
            vm_id,
            ac,
            start_index,
            end_index
        )
            .unwrap_or(0)
            != 0
    }

    /**
     * 返回有关表的信息，例如标题、摘要、行和列计数以及AccessibleTable。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_accessible_table_info(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleTableInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_accessible_table_info,
            vm_id,
            ac,
            &mut info
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
     * 获取基于JAWS算法的组件的AccessibleName。返回是否成功。
     * Bug ID 4916682-实现JAWS AccessibleName策略
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * `len` 名称长度。
     * */
    pub(crate) fn get_virtual_accessible_name(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        len: i32,
    ) -> Option<Vec<u16>> {
        let mut name = Vec::new();
        for _ in 0..len {
            name.push(0);
        }
        if jab!(
            self.h_module,
            get_virtual_accessible_name,
            vm_id,
            ac,
            name.as_mut_ptr(),
            len
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(name)
    }

    /**
     * 返回与组件关联的超文本信息。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_accessible_hypertext(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleHypertextInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_accessible_hypertext,
            vm_id,
            ac,
            &mut info
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
     * 遍历组件中的超链接。返回从超链接索引start_index开始的组件的超文本信息。对于此方法的每次调用，返回的AccessibleHypertextInfo对象不超过MAX_HYPERLINKS个。出现错误时返回None。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * `start_index` 开始索引。
     * */
    pub(crate) fn get_accessible_hypertext_ext(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        start_index: JInt,
    ) -> Option<AccessibleHypertextInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_accessible_hypertext_ext,
            vm_id,
            ac,
            start_index,
            &mut info
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
     * 请求组件执行可访问操作的列表。如果执行了所有操作，则返回true。当第一个请求的操作失败时返回false，在这种情况下，“failure”包含失败操作的索引。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * `actions_to_do` 要执行的动作列表。
     * */
    pub(crate) fn do_accessible_actions(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        actions_to_do: *const AccessibleActionsToDo,
    ) -> (bool, JInt) {
        let mut failure = unsafe { std::mem::zeroed() };
        (
            jab!(
                self.h_module,
                do_accessible_actions,
                vm_id,
                ac,
                actions_to_do,
                &mut failure
            )
                .unwrap_or(0)
                != 0,
            failure,
        )
    }

    /**
     * 设置可编辑的文本内容。AccessibleContext必须实现AccessibleEditableText并可编辑。可以设置的最大文本长度为MAX_STRING_SIZE-1。返回是否成功。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * `text` 文字内容。
     * */
    pub(crate) fn set_text_contents(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        text: *const u16,
    ) -> bool {
        jab!(self.h_module, set_text_contents, vm_id, ac, text).unwrap_or(0) != 0
    }

    /**
     * 设置插入点更新时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_caret_update_fp(&self, cb: AccessBridgeCaretUpdateFp) {
        jab!(self.h_module, set_caret_update_fp, cb).unwrap_or(())
    }

    /**
     * 设置得到焦点时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_focus_gained_fp(&self, cb: AccessBridgeFocusGainedFp) {
        jab!(self.h_module, set_focus_gained_fp, cb).unwrap_or(())
    }

    /**
     * 设置失去焦点时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_focus_lost_fp(&self, cb: AccessBridgeFocusLostFp) {
        jab!(self.h_module, set_focus_lost_fp, cb).unwrap_or(())
    }

    /**
     * 设置jvm关闭时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_java_shutdown_fp(&self, cb: AccessBridgeJavaShutdownFp) {
        jab!(self.h_module, set_java_shutdown_fp, cb).unwrap_or(())
    }

    /**
     * 设置菜单被取消时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_menu_canceled_fp(&self, cb: AccessBridgeMenuCanceledFp) {
        jab!(self.h_module, set_menu_canceled_fp, cb).unwrap_or(())
    }

    /**
     * 设置菜单被取消选择时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_menu_deselected_fp(&self, cb: AccessBridgeMenuDeselectedFp) {
        jab!(self.h_module, set_menu_deselected_fp, cb).unwrap_or(())
    }

    /**
     * 设置菜单被选择时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_menu_selected_fp(&self, cb: AccessBridgeMenuSelectedFp) {
        jab!(self.h_module, set_menu_selected_fp, cb).unwrap_or(())
    }

    /**
     * 设置鼠标点击时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_mouse_clicked_fp(&self, cb: AccessBridgeMouseClickedFp) {
        jab!(self.h_module, set_mouse_clicked_fp, cb).unwrap_or(())
    }

    /**
     * 设置鼠标进入时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_mouse_entered_fp(&self, cb: AccessBridgeMouseEnteredFp) {
        jab!(self.h_module, set_mouse_entered_fp, cb).unwrap_or(())
    }

    /**
     * 设置鼠标离开时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_mouse_exited_fp(&self, cb: AccessBridgeMouseExitedFp) {
        jab!(self.h_module, set_mouse_exited_fp, cb).unwrap_or(())
    }

    /**
     * 设置鼠标被按下时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_mouse_pressed_fp(&self, cb: AccessBridgeMousePressedFp) {
        jab!(self.h_module, set_mouse_pressed_fp, cb).unwrap_or(())
    }

    /**
     * 设置鼠标被释放时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_mouse_released_fp(&self, cb: AccessBridgeMouseReleasedFp) {
        jab!(self.h_module, set_mouse_released_fp, cb).unwrap_or(())
    }

    /**
     * 设置弹出菜单被取消时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_popup_menu_canceled_fp(&self, cb: AccessBridgePopupMenuCanceledFp) {
        jab!(self.h_module, set_popup_menu_canceled_fp, cb).unwrap_or(())
    }

    /**
     * 设置弹出菜单将要隐藏时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_popup_menu_will_become_invisible_fp(
        &self,
        cb: AccessBridgePopupMenuWillBecomeInvisibleFp,
    ) {
        jab!(self.h_module, set_popup_menu_will_become_invisible_fp, cb).unwrap_or(())
    }

    /**
     * 设置弹出菜单将要显示时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_popup_menu_will_become_visible_fp(
        &self,
        cb: AccessBridgePopupMenuWillBecomeVisibleFp,
    ) {
        jab!(self.h_module, set_popup_menu_will_become_visible_fp, cb).unwrap_or(())
    }

    /**
     * 设置属性激活、取消激活时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_property_active_descendent_change_fp(
        &self,
        cb: AccessBridgePropertyActiveDescendentChangeFp,
    ) {
        jab!(self.h_module, set_property_active_descendent_change_fp, cb).unwrap_or(())
    }

    /**
     * 设置属性插入点改变时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_property_caret_change_fp(&self, cb: AccessBridgePropertyCaretChangeFp) {
        jab!(self.h_module, set_property_caret_change_fp, cb).unwrap_or(())
    }

    /**
     * 设置属性改变时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_property_change_fp(&self, cb: AccessBridgePropertyChangeFp) {
        jab!(self.h_module, set_property_change_fp, cb).unwrap_or(())
    }

    /**
     * 设置属性孩子改变时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_property_child_change_fp(&self, cb: AccessBridgePropertyChildChangeFp) {
        jab!(self.h_module, set_property_child_change_fp, cb).unwrap_or(())
    }

    /**
     * 设置属性描述改变时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_property_description_change_fp(
        &self,
        cb: AccessBridgePropertyDescriptionChangeFp,
    ) {
        jab!(self.h_module, set_property_description_change_fp, cb).unwrap_or(())
    }

    /**
     * 设置属性名称改变时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_property_name_change_fp(&self, cb: AccessBridgePropertyNameChangeFp) {
        jab!(self.h_module, set_property_name_change_fp, cb).unwrap_or(())
    }

    /**
     * 设置属性选择改变时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_property_selection_change_fp(
        &self,
        cb: AccessBridgePropertySelectionChangeFp,
    ) {
        jab!(self.h_module, set_property_selection_change_fp, cb).unwrap_or(())
    }

    /**
     * 设置属性状态改变时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_property_state_change_fp(&self, cb: AccessBridgePropertyStateChangeFp) {
        jab!(self.h_module, set_property_state_change_fp, cb).unwrap_or(())
    }

    /**
     * 设置属性表格模式改变时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_property_table_model_change_fp(
        &self,
        cb: AccessBridgePropertyTableModelChangeFp,
    ) {
        jab!(self.h_module, set_property_table_model_change_fp, cb).unwrap_or(())
    }

    /**
     * 设置属性文字改变时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_property_text_change_fp(&self, cb: AccessBridgePropertyTextChangeFp) {
        jab!(self.h_module, set_property_text_change_fp, cb).unwrap_or(())
    }

    /**
     * 设置属性值改变时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_property_value_change_fp(&self, cb: AccessBridgePropertyValueChangeFp) {
        jab!(self.h_module, set_property_value_change_fp, cb).unwrap_or(())
    }

    /**
     * 设置属性可见数据改变时的处理函数。
     * `cb` 接收事件的函数。
     * */
    pub(crate) fn set_property_visible_data_change_fp(
        &self,
        cb: AccessBridgePropertyVisibleDataChangeFp,
    ) {
        jab!(self.h_module, set_property_visible_data_change_fp, cb).unwrap_or(())
    }

    /**
     * 请求激活超链接。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * `link` 超链接对象。
     * */
    pub(crate) fn activate_accessible_hyperlink(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
        link: AccessibleHyperlink,
    ) -> bool {
        jab!(
            self.h_module,
            activate_accessible_hyperlink,
            vm_id,
            ac,
            link
        )
            .unwrap_or(0)
            != 0
    }

    /**
     * 把某个项目添加到选区。
     * 如果AccessibleContextInfo数据结构中的标志AccessibleSelection设置为TRUE，则AccessibleContext中包含AccessibleSelection信息。
     * AccessibleSelection支持是第一个可以通过添加和删除选择中的项目来操作用户界面（而不是查询）的地方。
     * 有些函数使用子坐标中的索引，而其他函数则使用选择坐标。
     * 例如，通过传递子索引从选择中添加到删除（例如，将第四个子项添加到选择中）。
     * 另一方面，枚举选定的子对象是在选择坐标中完成的（例如，获取选定的第一个对象的AccessibleContext）。
     * `vm_id` 虚拟机ID。
     * `as` 可访问上下文。
     * `index` 索引。
     * */
    pub(crate) fn add_accessible_selection_from_context(
        &self,
        vm_id: i32,
        r#as: AccessibleSelection,
        index: i32,
    ) {
        jab!(
            self.h_module,
            add_accessible_selection_from_context,
            vm_id,
            r#as,
            index
        )
            .unwrap_or(())
    }

    /**
     * 把某个项目从选区中移除。
     * 如果AccessibleContextInfo数据结构中的标志AccessibleSelection设置为TRUE，则AccessibleContext中包含AccessibleSelection信息。
     * AccessibleSelection支持是第一个可以通过添加和删除选择中的项目来操作用户界面（而不是查询）的地方。
     * 有些函数使用子坐标中的索引，而其他函数则使用选择坐标。
     * 例如，通过传递子索引从选择中添加到删除（例如，将第四个子项添加到选择中）。
     * 另一方面，枚举选定的子对象是在选择坐标中完成的（例如，获取选定的第一个对象的AccessibleContext）。
     * `vm_id` 虚拟机ID。
     * `as` 可访问上下文。
     * `index` 索引。
     * */
    pub(crate) fn remove_accessible_selection_from_context(
        &self,
        vm_id: i32,
        r#as: AccessibleSelection,
        index: i32,
    ) {
        jab!(
            self.h_module,
            remove_accessible_selection_from_context,
            vm_id,
            r#as,
            index
        )
            .unwrap_or(())
    }

    /**
     * 清除选区。
     * 如果AccessibleContextInfo数据结构中的标志AccessibleSelection设置为TRUE，则AccessibleContext中包含AccessibleSelection信息。
     * AccessibleSelection支持是第一个可以通过添加和删除选择中的项目来操作用户界面（而不是查询）的地方。
     * 有些函数使用子坐标中的索引，而其他函数则使用选择坐标。
     * 例如，通过传递子索引从选择中添加到删除（例如，将第四个子项添加到选择中）。
     * 另一方面，枚举选定的子对象是在选择坐标中完成的（例如，获取选定的第一个对象的AccessibleContext）。
     * `vm_id` 虚拟机ID。
     * `as` 可访问上下文。
     * */
    pub(crate) fn clear_accessible_selection_from_context(
        &self,
        vm_id: i32,
        r#as: AccessibleSelection,
    ) {
        jab!(
            self.h_module,
            clear_accessible_selection_from_context,
            vm_id,
            r#as
        )
            .unwrap_or(())
    }

    /**
     * 选择所有。
     * 如果AccessibleContextInfo数据结构中的标志AccessibleSelection设置为TRUE，则AccessibleContext中包含AccessibleSelection信息。
     * AccessibleSelection支持是第一个可以通过添加和删除选择中的项目来操作用户界面（而不是查询）的地方。
     * 有些函数使用子坐标中的索引，而其他函数则使用选择坐标。
     * 例如，通过传递子索引从选择中添加到删除（例如，将第四个子项添加到选择中）。
     * 另一方面，枚举选定的子对象是在选择坐标中完成的（例如，获取选定的第一个对象的AccessibleContext）。
     * `vm_id` 虚拟机ID。
     * `as` 可访问上下文。
     * */
    pub(crate) fn select_all_accessible_selection_from_context(
        &self,
        vm_id: i32,
        r#as: AccessibleSelection,
    ) {
        jab!(
            self.h_module,
            select_all_accessible_selection_from_context,
            vm_id,
            r#as
        )
            .unwrap_or(())
    }

    /**
     * 返回文档中的第n个超链接。映射到AccessibleHypertext.getLink。出现错误时返回None。
     * `vm_id` 虚拟机ID。
     * `ah` 可访问超文本。
     * `index` 索引。
     * */
    pub(crate) fn get_accessible_hyperlink(
        &self,
        vm_id: i32,
        ah: AccessibleContext,
        index: JInt,
    ) -> Option<AccessibleHypertextInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_accessible_hyperlink,
            vm_id,
            ah,
            index,
            &mut info
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
     * 返回组件中的超链接数。映射到AccessibleHypertext.getLinkCount。出现错误时返回-1。
     * `vm_id` 虚拟机ID。
     * `ah` 可访问超文本。
     * */
    pub(crate) fn get_accessible_hyperlink_count(
        &self,
        vm_id: i32,
        ah: AccessibleHypertext,
    ) -> JInt {
        jab!(self.h_module, get_accessible_hyperlink_count, vm_id, ah).unwrap_or(-1)
    }

    /**
     * 将索引返回到与文档中的字符索引关联的超链接数组中。映射到AccessibleHypertext.getLinkIndex。出现错误时返回-1。
     * `vm_id` 虚拟机ID。
     * `ah` 可访问超文本。
     * `index` 索引。
     * */
    pub(crate) fn get_accessible_hypertext_link_index(
        &self,
        vm_id: i32,
        ah: AccessibleHypertext,
        index: JInt,
    ) -> JInt {
        jab!(
            self.h_module,
            get_accessible_hypertext_link_index,
            vm_id,
            ah,
            index
        )
            .unwrap_or(-1)
    }

    /**
     * 获取选区中的对象数量。
     * `vm_id` 虚拟机ID。
     * `as` 可访问选择上下文。
     * */
    pub(crate) fn get_accessible_selection_count_from_context(
        &self,
        vm_id: i32,
        r#as: AccessibleSelection,
    ) -> i32 {
        jab!(
            self.h_module,
            get_accessible_selection_count_from_context,
            vm_id,
            r#as
        )
            .unwrap_or(-1)
    }

    /**
     * 获取选区中的对象。
     * `vm_id` 虚拟机ID。
     * `as` 可访问选择上下文。
     * `index` 索引。
     * */
    pub(crate) fn get_accessible_selection_from_context(
        &self,
        vm_id: i32,
        r#as: AccessibleSelection,
        index: i32,
    ) -> Option<JObject> {
        jab!(
            self.h_module,
            get_accessible_selection_from_context,
            vm_id,
            r#as,
            index
        )
    }

    /**
     * 判断对象是否被选中。
     * `vm_id` 虚拟机ID。
     * `as` 可访问选择上下文。
     * `index` 子对象索引。
     * */
    pub(crate) fn is_accessible_child_selected_from_context(
        &self,
        vm_id: i32,
        r#as: AccessibleSelection,
        index: i32,
    ) -> bool {
        jab!(
            self.h_module,
            is_accessible_child_selected_from_context,
            vm_id,
            r#as,
            index
        )
            .unwrap_or(0)
            != 0
    }

    /**
     * 判断表格某行是否被选中。如果选择了指定的从零开始的行，则返回true。
     * `vm_id` 虚拟机ID。
     * `at` 可访问表格上下文。
     * `row` 行索引。
     * */
    pub(crate) fn is_accessible_table_row_selected(
        &self,
        vm_id: i32,
        at: AccessibleTable,
        row: JInt,
    ) -> bool {
        jab!(
            self.h_module,
            is_accessible_table_row_selected,
            vm_id,
            at,
            row
        )
            .unwrap_or(0)
            != 0
    }

    /**
     * 判断表格某列是否被选中。如果选择了指定的从零开始的列，则返回true。
     * `vm_id` 虚拟机ID。
     * `at` 可访问表格上下文。
     * `column` 列索引。
     * */
    pub(crate) fn is_accessible_table_column_selected(
        &self,
        vm_id: i32,
        at: AccessibleTable,
        column: JInt,
    ) -> bool {
        jab!(
            self.h_module,
            is_accessible_table_column_selected,
            vm_id,
            at,
            column
        )
            .unwrap_or(0)
            != 0
    }

    /**
     * 返回有关指定表单元格的信息。行和列说明符是从零开始的。
     * `vm_id` 虚拟机ID。
     * `at` 可访问表格上下文。
     * `row` 行索引。
     * `column` 列索引。
     * */
    pub(crate) fn get_accessible_table_cell_info(
        &self,
        vm_id: i32,
        at: AccessibleTable,
        row: JInt,
        column: JInt,
    ) -> Option<AccessibleTableCellInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_accessible_table_cell_info,
            vm_id,
            at,
            row,
            column,
            &mut info
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
     * 返回指定单元格索引处单元格的列编号。这些值以零为基础。
     * `vm_id` 虚拟机ID。
     * `at` 可访问表格上下文。
     * `index` 索引。
     * */
    pub(crate) fn get_accessible_table_column(
        &self,
        vm_id: i32,
        at: AccessibleTable,
        index: JInt,
    ) -> JInt {
        jab!(self.h_module, get_accessible_table_column, vm_id, at, index).unwrap_or(0)
    }

    /**
     * 返回指定单元格索引处单元格的行号。这些值以零为基础。
     * `vm_id` 虚拟机ID。
     * `at` 可访问表格上下文。
     * `index` 索引。
     * */
    pub(crate) fn get_accessible_table_row(
        &self,
        vm_id: i32,
        at: AccessibleTable,
        index: JInt,
    ) -> JInt {
        jab!(self.h_module, get_accessible_table_row, vm_id, at, index).unwrap_or(0)
    }

    /**
     * 返回表中选定的列数。
     * `vm_id` 虚拟机ID。
     * `at` 可访问表格上下文。
     * */
    pub(crate) fn get_accessible_table_column_selection_count(
        &self,
        vm_id: i32,
        at: AccessibleTable,
    ) -> JInt {
        jab!(
            self.h_module,
            get_accessible_table_column_selection_count,
            vm_id,
            at
        )
            .unwrap_or(0)
    }

    /**
     * 返回表中选定的行数。
     * `vm_id` 虚拟机ID。
     * `at` 可访问表格上下文。
     * */
    pub(crate) fn get_accessible_table_row_selection_count(
        &self,
        vm_id: i32,
        at: AccessibleTable,
    ) -> JInt {
        jab!(
            self.h_module,
            get_accessible_table_row_selection_count,
            vm_id,
            at
        )
            .unwrap_or(0)
    }

    /**
     * 返回表中指定行和列偏移量的索引。这些值以零为基础。
     * `vm_id` 虚拟机ID。
     * `at` 可访问表格上下文。
     * `row` 行索引。
     * `column` 列索引。
     * */
    pub(crate) fn get_accessible_table_index(
        &self,
        vm_id: i32,
        at: AccessibleTable,
        row: JInt,
        column: JInt,
    ) -> JInt {
        jab!(
            self.h_module,
            get_accessible_table_index,
            vm_id,
            at,
            row,
            column
        )
            .unwrap_or(0)
    }

    /**
     * 返回所选列的从零开始的索引数组。
     * `vm_id` 虚拟机ID。
     * `at` 可访问表格上下文。
     * `count` 数组长度。
     * */
    pub(crate) fn get_accessible_table_column_selections(
        &self,
        vm_id: i32,
        at: AccessibleTable,
        count: JInt,
    ) -> Option<Vec<JInt>> {
        let mut arr = Vec::new();
        for _ in 0..count {
            arr.push(0);
        }
        if jab!(
            self.h_module,
            get_accessible_table_column_selections,
            vm_id,
            at,
            count,
            arr.as_mut_ptr()
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        return Some(arr);
    }

    /**
     * 返回所选行的从零开始的索引数组。
     * `vm_id` 虚拟机ID。
     * `at` 可访问表格上下文。
     * `count` 数组长度。
     * */
    pub(crate) fn get_accessible_table_row_selections(
        &self,
        vm_id: i32,
        at: AccessibleTable,
        count: JInt,
    ) -> Option<Vec<JInt>> {
        let mut arr = Vec::new();
        for _ in 0..count {
            arr.push(0);
        }
        if jab!(
            self.h_module,
            get_accessible_table_row_selections,
            vm_id,
            at,
            count,
            arr.as_mut_ptr()
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        return Some(arr);
    }

    /**
     * 获取文本选区信息。
     * 如果将AccessibleContextInfo数据结构中的标志AccessibleText设置为TRUE，则AccessibleContext中包含AccessibleText信息。
     * 文件AccessBridgePackages.h定义了这些函数中使用的结构值。
     * Java Access Bridge API回调对它们进行了描述。
     * `vm_id` 虚拟机ID。
     * `at` 可访问文字上下文。
     * */
    pub(crate) fn get_accessible_text_selection_info(
        &self,
        vm_id: i32,
        at: AccessibleText,
    ) -> Option<AccessibleTextSelectionInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_accessible_text_selection_info,
            vm_id,
            at,
            &mut info
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
     * 获取文本信息。
     * 如果将AccessibleContextInfo数据结构中的标志AccessibleText设置为TRUE，则AccessibleContext中包含AccessibleText信息。
     * 文件AccessBridgePackages.h定义了这些函数中使用的结构值。
     * Java Access Bridge API回调对它们进行了描述。
     * `vm_id` 虚拟机ID。
     * `at` 可访问文字上下文。
     * `x` X坐标。
     * `y` Y坐标。
     * */
    pub(crate) fn get_accessible_text_info(
        &self,
        vm_id: i32,
        at: AccessibleText,
        x: JInt,
        y: JInt,
    ) -> Option<AccessibleTextInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_accessible_text_info,
            vm_id,
            at,
            &mut info,
            x,
            y
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
     * 获取文本属性。
     * 如果将AccessibleContextInfo数据结构中的标志AccessibleText设置为TRUE，则AccessibleContext中包含AccessibleText信息。
     * 文件AccessBridgePackages.h定义了这些函数中使用的结构值。
     * Java Access Bridge API回调对它们进行了描述。
     * `vm_id` 虚拟机ID。
     * `at` 可访问文字上下文。
     * `index` 索引。
     * */
    pub(crate) fn get_accessible_text_attributes(
        &self,
        vm_id: i32,
        at: AccessibleText,
        index: JInt,
    ) -> (*const u8, AccessibleTextAttributesInfo) {
        let mut info = unsafe { std::mem::zeroed() };
        let char = jab!(
            self.h_module,
            get_accessible_text_attributes,
            vm_id,
            at,
            index,
            &mut info
        )
            .unwrap_or(std::ptr::null());
        (char, info)
    }

    /**
     * 获取文本项目。
     * 如果将AccessibleContextInfo数据结构中的标志AccessibleText设置为TRUE，则AccessibleContext中包含AccessibleText信息。
     * 文件AccessBridgePackages.h定义了这些函数中使用的结构值。
     * Java Access Bridge API回调对它们进行了描述。
     * `vm_id` 虚拟机ID。
     * `at` 可访问文字上下文。
     * `index` 索引。
     * */
    pub(crate) fn get_accessible_text_items(
        &self,
        vm_id: i32,
        at: AccessibleText,
        index: JInt,
    ) -> Option<AccessibleTextItemsInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_accessible_text_items,
            vm_id,
            at,
            &mut info,
            index
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
     * 获取文本行边界。
     * 如果将AccessibleContextInfo数据结构中的标志AccessibleText设置为TRUE，则AccessibleContext中包含AccessibleText信息。
     * 文件AccessBridgePackages.h定义了这些函数中使用的结构值。
     * Java Access Bridge API回调对它们进行了描述。
     * `vm_id` 虚拟机ID。
     * `at` 可访问文字上下文。
     * `index` 索引。
     * */
    pub(crate) fn get_accessible_text_line_bounds(
        &self,
        vm_id: i32,
        at: AccessibleText,
        index: JInt,
    ) -> Option<(JInt, JInt)> {
        let (mut start, mut end) = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_accessible_text_line_bounds,
            vm_id,
            at,
            index,
            &mut start,
            &mut end
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some((start, end))
    }

    /**
     * 获取文本范围。
     * 如果将AccessibleContextInfo数据结构中的标志AccessibleText设置为TRUE，则AccessibleContext中包含AccessibleText信息。
     * 文件AccessBridgePackages.h定义了这些函数中使用的结构值。
     * Java Access Bridge API回调对它们进行了描述。
     * `vm_id` 虚拟机ID。
     * `at` 可访问文字上下文。
     * `start_index` 开始索引。
     * `end_index` 结束索引。
     * `len` 长度。
     * */
    pub(crate) fn get_accessible_text_range(
        &self,
        vm_id: i32,
        at: AccessibleText,
        start_index: JInt,
        end_index: JInt,
        len: i16,
    ) -> Option<Vec<u16>> {
        let mut text = Vec::new();
        for _ in 0..len {
            text.push(0);
        }
        if jab!(
            self.h_module,
            get_accessible_text_range,
            vm_id,
            at,
            start_index,
            end_index,
            text.as_mut_ptr(),
            len
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(text)
    }

    /**
     * 获取文本矩形范围。
     * 如果将AccessibleContextInfo数据结构中的标志AccessibleText设置为TRUE，则AccessibleContext中包含AccessibleText信息。
     * 文件AccessBridgePackages.h定义了这些函数中使用的结构值。
     * Java Access Bridge API回调对它们进行了描述。
     * `vm_id` 虚拟机ID。
     * `at` 可访问文字上下文。
     * `index` 索引。
     * */
    pub(crate) fn get_accessible_text_rect(
        &self,
        vm_id: i32,
        at: AccessibleText,
        index: JInt,
    ) -> Option<AccessibleTextRectInfo> {
        let mut info = unsafe { std::mem::zeroed() };
        if jab!(
            self.h_module,
            get_accessible_text_rect,
            vm_id,
            at,
            &mut info,
            index
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(info)
    }

    /**
     * 获取当前值。
     * 如果AccessibleContextInfo数据结构中的标志AccessibleValue设置为TRUE，则AccessibleContext对象中包含AccessibleValue信息。
     * 返回的值是字符串（char*value），因为无法提前判断该值是整数、浮点值还是Java语言构造java.lang.Number的子类的其他对象。
     * `vm_id` 虚拟机ID。
     * `av` 可访问值上下文。
     * `len` 接收文本的长度。
     * */
    pub(crate) fn get_current_accessible_value_from_context(
        &self,
        vm_id: i32,
        av: AccessibleValue,
        len: i16,
    ) -> Option<Vec<u16>> {
        let mut value = Vec::new();
        for _ in 0..len {
            value.push(0);
        }
        if jab!(
            self.h_module,
            get_current_accessible_value_from_context,
            vm_id,
            av,
            value.as_mut_ptr(),
            len
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(value)
    }

    /**
     * 获取最大值。
     * 如果AccessibleContextInfo数据结构中的标志AccessibleValue设置为TRUE，则AccessibleContext对象中包含AccessibleValue信息。
     * 返回的值是字符串（char*value），因为无法提前判断该值是整数、浮点值还是Java语言构造java.lang.Number的子类的其他对象。
     * `vm_id` 虚拟机ID。
     * `av` 可访问值上下文。
     * `len` 接收文本的长度。
     * */
    pub(crate) fn get_maximum_accessible_value_from_context(
        &self,
        vm_id: i32,
        av: AccessibleValue,
        len: i16,
    ) -> Option<Vec<u16>> {
        let mut value = Vec::new();
        for _ in 0..len {
            value.push(0);
        }
        if jab!(
            self.h_module,
            get_maximum_accessible_value_from_context,
            vm_id,
            av,
            value.as_mut_ptr(),
            len
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(value)
    }

    /**
     * 获取最小值。
     * 如果AccessibleContextInfo数据结构中的标志AccessibleValue设置为TRUE，则AccessibleContext对象中包含AccessibleValue信息。
     * 返回的值是字符串（char*value），因为无法提前判断该值是整数、浮点值还是Java语言构造java.lang.Number的子类的其他对象。
     * `vm_id` 虚拟机ID。
     * `av` 可访问值上下文。
     * `len` 接收文本的长度。
     * */
    pub(crate) fn get_minimum_accessible_value_from_context(
        &self,
        vm_id: i32,
        av: AccessibleValue,
        len: i16,
    ) -> Option<Vec<u16>> {
        let mut value = Vec::new();
        for _ in 0..len {
            value.push(0);
        }
        if jab!(
            self.h_module,
            get_minimum_accessible_value_from_context,
            vm_id,
            av,
            value.as_mut_ptr(),
            len
        )
            .unwrap_or(0)
            == 0
        {
            return None;
        }
        Some(value)
    }
}

impl Drop for JabLib {
    fn drop(&mut self) {
        if self.h_module.is_invalid() {
            return;
        }
        free_library(self.h_module);
    }
}
