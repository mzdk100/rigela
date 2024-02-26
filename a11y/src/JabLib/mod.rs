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

mod calls;
mod packages;

use crate::{
    JabLib::packages::{
        AccessibleActions,
        AccessibleContextInfo,
        JObject64,
        JInt,
        AccessBridgeVersionInfo,
        AccessibleContext,
        AccessibleTextRectInfo,
        AccessibleTextAttributesInfo,
        JavaObject,
        VisibleChildrenInfo,
        AccessibleIcons,
        AccessibleKeyBindings,
        AccessibleRelationSetInfo,
        AccessibleTableInfo,
        AccessibleHypertextInfo,
    },
    jab,
};
use rigela_utils::call_proc;
use std::{
    env::var,
    path::{Path, PathBuf},
};
use win_wrap::{
    common::{
        free_library, get_proc_address, load_library, Result, BOOL, FALSE, FARPROC, HMODULE, HWND,
    },
    message::pump_waiting_messages,
};
use windows::{
    core::{Error, HSTRING},
    Win32::Foundation::S_FALSE,
};

#[allow(unused)]
#[derive(Debug)]
pub struct JabLib {
    h_module: HMODULE,
}

#[allow(dead_code)]
impl JabLib {
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
                        HSTRING::from(format!("Can't find the jab library. ({})", e)),
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
            return Err(Error::new(
                S_FALSE,
                HSTRING::from("Can't load the jab library."),
            ));
        }
        Ok(Self { h_module })
    }

    /**
     * 检查给定窗口是否实现了 Java 辅助功能 API。
     * */
    pub(crate) fn is_java_window(&self, h_wnd: HWND) -> bool {
        pump_waiting_messages();
        jab!(self.h_module, is_java_window, h_wnd)
            .unwrap_or(BOOL::from(false))
            .as_bool()
    }

    /**
     * 获取给定窗口的 AccessibleContext和 vmID值。许多 Java Access Bridge 函数都需要 AccessibleContext和 vmID值。
     * `target` 目标窗口句柄。
     * */
    pub(crate) fn get_accessible_context_from_hwnd(
        &self,
        target: HWND,
    ) -> Option<(i32, AccessibleContext)> {
        pump_waiting_messages();
        let (mut context, mut vm_id) = unsafe { std::mem::zeroed() };
        if !jab!(
            self.h_module,
            get_accessible_context_from_hwnd,
            target,
            &mut vm_id,
            &mut context
        )
            .unwrap_or(FALSE)
            .as_bool()
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
        pump_waiting_messages();
        jab!(self.h_module, get_hwnd__from_accessible_context, vm_id, ac).unwrap_or(HWND::default())
    }

    /**
     * 释放 Java 对象使用的内存，其中 object 是 Java Access Bridge 返回给您的对象。Java Access Bridge 会自动维护对它在 JVM 中返回给您的所有 Java 对象的引用，因此它们不会被垃圾回收。为了防止内存泄漏，请在完成 Java Access Bridge 返回的所有 Java 对象后调用它们。
     * `object` 一个java对象。
     * */
    pub(crate) fn release_java_object(&self, vm_id: i32, object: JavaObject) {
        pump_waiting_messages();
        jab!(self.h_module, release_java_object, vm_id, object);
    }

    /**
     * 获取应用程序正在使用的 Java Access Bridge 实例的版本信息。您可以使用此信息来确定您的 Java Access Bridge 版本的可用功能。
     * 注意：要确定 JVM 的版本，您需要传入一个有效的 vm_id;否则，返回的只是应用程序连接到WindowsAccessBridge.DLL的文件的版本。
     * `vm_id` 虚拟机ID。
     * */
    pub(crate) fn get_version_info(&self, vm_id: i32) -> Option<AccessBridgeVersionInfo> {
        pump_waiting_messages();
        let mut info = unsafe { std::mem::zeroed() };
        if !jab!(self.h_module, get_version_info, vm_id, &mut info)
            .unwrap_or(FALSE)
            .as_bool()
        {
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
        pump_waiting_messages();
        let mut ac = unsafe { std::mem::zeroed() };
        if !jab!(
            self.h_module,
            get_accessible_context_at,
            vm_id,
            parent,
            x,
            y,
            &mut ac
        )
            .unwrap_or(FALSE)
            .as_bool()
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
        pump_waiting_messages();
        let (mut vm_id, mut ac) = unsafe { std::mem::zeroed() };
        if !jab!(
            self.h_module,
            get_accessible_context_with_focus,
            window,
            &mut vm_id,
            &mut ac
        )
            .unwrap_or(FALSE)
            .as_bool()
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
    pub(crate) fn getAccessibleContextInfo(
        &self,
        vm_id: i32,
        ac: AccessibleContext,
    ) -> Option<AccessibleContextInfo> {
        pump_waiting_messages();
        let mut info = unsafe { std::mem::zeroed() };
        if !jab!(
            self.h_module,
            get_accessible_context_info,
            vm_id,
            ac,
            &mut info
        )
            .unwrap_or(FALSE)
            .as_bool()
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
        pump_waiting_messages();
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
        pump_waiting_messages();
        jab!(self.h_module, get_accessible_parent_from_context, vm_id, ac)
    }

    /**
     * 返回两个对象引用是否引用同一个对象。
     * `vm_id` 虚拟机ID。
     * `obj1` 对象1.
     * `obj2` 对象2.
     * */
    pub(crate) fn is_same_object(&self, vm_id: i32, obj1: JObject64, obj2: JObject64) -> bool {
        pump_waiting_messages();
        jab!(self.h_module, is_same_object, vm_id, obj1, obj2)
            .unwrap_or(FALSE)
            .as_bool()
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
        pump_waiting_messages();
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
        pump_waiting_messages();
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
        pump_waiting_messages();
        jab!(self.h_module, get_top_level_object, vm_id, ac)
    }

    //noinspection StructuralWrap
    /**
     * 返回给定对象在对象层次结构中的深度。对象层次结构最顶端的对象的对象深度为0。出现错误时返回-1。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_object_depth(&self, vm_id: i32, ac: AccessibleContext) -> i32 {
        pump_waiting_messages();
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
        pump_waiting_messages();
        jab!(self.h_module, get_active_descendent, vm_id, ac)
    }

    /**
     * 请求某个组件的焦点。返回是否成功。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn request_focus(&self, vm_id: i32, ac: AccessibleContext) -> bool {
        pump_waiting_messages();
        jab!(self.h_module, request_focus, vm_id, ac)
            .unwrap_or(FALSE)
            .as_bool()
    }

    //noinspection StructuralWrap
    /**
     * 返回组件的可见子级数。出现错误时返回-1。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_visible_children_count(&self, vm_id: i32, ac: AccessibleContext) -> i32 {
        pump_waiting_messages();
        jab!(self.h_module, get_visible_children_count, vm_id, ac).unwrap_or(-1)
    }

    /**
     * 获取 AccessibleContext 的可见子级。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_visible_children(&self, vm_id: i32, ac: AccessibleContext, start_index: i32) -> Option<VisibleChildrenInfo> {
        pump_waiting_messages();
        let mut info = unsafe { std::mem::zeroed() };
        if !jab!(self.h_module, get_visible_children, vm_id, ac,start_index,&mut info).unwrap_or(FALSE).as_bool() {
            return None;
        }
        Some(info)
    }

    /**
     * 获取等待触发的事件数。
     * */
    pub(crate) fn get_events_waiting(&self) -> i32 {
        pump_waiting_messages();
        jab!(self.h_module,get_events_waiting).unwrap_or(0)
    }

    /**
     * 返回组件可以执行的操作列表。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_accessible_actions(&self, vm_id: i32, ac: AccessibleContext) -> Option<AccessibleActions> {
        pump_waiting_messages();
        let mut actions = unsafe { std::mem::zeroed() };
        if !jab!(self.h_module, get_accessible_actions, vm_id, ac,&mut actions).unwrap_or(FALSE).as_bool() {
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
    pub(crate) fn get_caret_location(&self, vm_id: i32, ac: AccessibleContext, index: JInt) -> Option<AccessibleTextRectInfo> {
        pump_waiting_messages();
        let mut info = unsafe { std::mem::zeroed() };
        if !jab!(self.h_module, get_caret_location, vm_id, ac,&mut info,index).unwrap_or(FALSE).as_bool() {
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
    pub(crate) fn set_caret_position(&self, vm_id: i32, ac: AccessibleContext, position: i32) -> bool {
        pump_waiting_messages();
        jab!(self.h_module, set_caret_position, vm_id, ac,position).unwrap_or(FALSE).as_bool()
    }

    /**
     * 获取两个索引之间的文本属性。属性列表包括起始索引处的文本和结束索引处的文字。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * `start_index` 开始位置。
     * `end_index` 结束位置。
     * */
    pub(crate) fn get_text_attributes_in_range(&self, vm_id: i32, ac: AccessibleContext, start_index: i32, end_index: i32) -> Option<(AccessibleTextAttributesInfo, i16)> {
        pump_waiting_messages();
        let (mut info, mut len) = unsafe { std::mem::zeroed() };
        if !jab!(self.h_module, get_text_attributes_in_range, vm_id, ac,start_index,end_index,&mut info,&mut len).unwrap_or(FALSE).as_bool() {
            return None;
        }
        Some((info, len))
    }

    /**
     * 返回有关对象的相关对象的信息。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_accessible_relation_set(&self, vm_id: i32, ac: AccessibleContext) -> Option<AccessibleRelationSetInfo> {
        pump_waiting_messages();
        let mut info = unsafe { std::mem::zeroed() };
        if !jab!(self.h_module, get_accessible_relation_set, vm_id, ac,&mut info).unwrap_or(FALSE).as_bool() {
            return None;
        }
        Some(info)
    }

    /**
     * 返回与组件关联的键绑定的列表。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_accessible_key_bindings(&self, vm_id: i32, ac: AccessibleContext) -> Option<AccessibleKeyBindings> {
        pump_waiting_messages();
        let mut info = unsafe { std::mem::zeroed() };
        if !jab!(self.h_module, get_accessible_key_bindings, vm_id, ac,&mut info).unwrap_or(FALSE).as_bool() {
            return None;
        }
        Some(info)
    }

    /**
     * 返回与组件关联的图标列表。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_accessible_icons(&self, vm_id: i32, ac: AccessibleContext) -> Option<AccessibleIcons> {
        pump_waiting_messages();
        let mut info = unsafe { std::mem::zeroed() };
        if !jab!(self.h_module, get_accessible_icons, vm_id, ac,&mut info).unwrap_or(FALSE).as_bool() {
            return None;
        }
        Some(info)
    }

    /**
     * 将指定表的表行标题作为表返回。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_accessible_table_row_header(&self, vm_id: i32, ac: AccessibleContext) -> Option<AccessibleTableInfo> {
        pump_waiting_messages();
        let mut info = unsafe { std::mem::zeroed() };
        if !jab!(self.h_module, get_accessible_table_row_header, vm_id, ac,&mut info).unwrap_or(FALSE).as_bool() {
            return None;
        }
        Some(info)
    }

    /**
     * 将指定表的表列标题作为表返回。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_accessible_table_column_header(&self, vm_id: i32, ac: AccessibleContext) -> Option<AccessibleTableInfo> {
        pump_waiting_messages();
        let mut info = unsafe { std::mem::zeroed() };
        if !jab!(self.h_module, get_accessible_table_column_header, vm_id, ac,&mut info).unwrap_or(FALSE).as_bool() {
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
    pub(crate) fn get_accessible_table_column_description(&self, vm_id: i32, ac: AccessibleContext, column: JInt) -> Option<AccessibleContext> {
        pump_waiting_messages();
        jab!(self.h_module, get_accessible_table_column_description, vm_id, ac,column)
    }

    /**
     * 返回指定表中指定行的描述。行说明符是从零开始的。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * `row` 行索引。
     * */
    pub(crate) fn get_accessible_table_row_description(&self, vm_id: i32, ac: AccessibleContext, row: JInt) -> Option<AccessibleContext> {
        pump_waiting_messages();
        jab!(self.h_module, get_accessible_table_row_description, vm_id, ac,row)
    }

    /**
     * 在两个索引之间选择文本。所选内容包括起始索引处的文本和结束索引处的文字。返回是否成功。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * `start_index` 开始索引。
     * `end_index` 结束索引。
     * */
    pub(crate) fn select_text_range(&self, vm_id: i32, ac: AccessibleContext, start_index: JInt, end_index: JInt) -> bool {
        pump_waiting_messages();
        jab!(self.h_module, select_text_range, vm_id, ac,start_index,end_index).unwrap_or(FALSE).as_bool()
    }

    /**
     * 返回有关表的信息，例如标题、摘要、行和列计数以及AccessibleTable。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_accessible_table_info(&self, vm_id: i32, ac: AccessibleContext) -> Option<AccessibleTableInfo> {
        pump_waiting_messages();
        let mut info = unsafe { std::mem::zeroed() };
        if !jab!(self.h_module, get_accessible_table_info, vm_id, ac,&mut info).unwrap_or(FALSE).as_bool() {
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
    pub(crate) fn get_virtual_accessible_name(&self, vm_id: i32, ac: AccessibleContext, len: i32) -> Option<Vec<u16>> {
        pump_waiting_messages();
        let mut name = Vec::new();
        for _ in 0..len {
            name.push(0);
        }
        if !jab!(self.h_module, get_virtual_accessible_name, vm_id, ac,name.as_mut_ptr(),len).unwrap_or(FALSE).as_bool() {
            return None;
        }
        Some(name)
    }

    /**
     * 返回与组件关联的超文本信息。
     * `vm_id` 虚拟机ID。
     * `ac` 可访问上下文。
     * */
    pub(crate) fn get_accessible_hypertext(&self, vm_id: i32, ac: AccessibleContext) -> Option<AccessibleHypertextInfo> {
        pump_waiting_messages();
        let mut info = unsafe { std::mem::zeroed() };
        if !jab!(self.h_module, get_accessible_hypertext, vm_id, ac,&mut info).unwrap_or(FALSE).as_bool() {
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
    pub(crate) fn get_accessible_hypertext_ext(&self, vm_id: i32, ac: AccessibleContext, start_index: JInt) -> Option<AccessibleHypertextInfo> {
        pump_waiting_messages();
        let mut info = unsafe { std::mem::zeroed() };
        if !jab!(self.h_module, get_accessible_hypertext_ext, vm_id, ac,start_index,&mut info).unwrap_or(FALSE).as_bool() {
            return None;
        }
        Some(info)
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

#[cfg(all(test, target_arch = "x86_64"))]
mod test_jab {
    use crate::JabLib::{packages::ACCESSIBLE_PANEL, JabLib};
    use win_wrap::common::{find_window, get_desktop_window};
    use crate::JabLib::packages::AccessibleContext;

    #[test]
    fn main() {
        let jab = JabLib::new(None).unwrap();
        assert!(!jab.is_java_window(get_desktop_window()));
        let h_wnd = find_window(Some("SunAwtFrame"), None);
        assert!(jab.is_java_window(h_wnd));
        let ac = jab.get_accessible_context_from_hwnd(h_wnd);
        dbg!(ac);
        let (vm_id, ac) = ac.unwrap();
        let h_wnd2 = jab.get_hwnd_from_accessible_context(vm_id, ac);
        assert_eq!(h_wnd, h_wnd2);
        if let Some(ac2) = jab.get_accessible_context_at(vm_id, ac, 100, 100) {
            dbg!(ac2);
            jab.release_java_object(vm_id, ac2);
        }
        jab.release_java_object(vm_id, ac);
        let ac = jab.get_accessible_context_with_focus(h_wnd);
        dbg!(ac);
        let (vm_id, ac) = ac.unwrap();
        if let Some(child) = jab.get_accessible_child_from_context(vm_id, ac, 0) {
            dbg!(child);
            jab.release_java_object(vm_id, child);
        }
        if let Some(parent) = jab.get_accessible_parent_from_context(vm_id, ac) {
            dbg!(parent);
            jab.release_java_object(vm_id, parent);
        }
        dbg!(jab.is_same_object(vm_id, ac, ac));
        if let Some(parent) = jab.get_parent_with_role(vm_id, ac, ACCESSIBLE_PANEL) {
            dbg!(parent);
            jab.release_java_object(vm_id, parent);
        }
        if let Some(parent_or_root) =
            jab.get_parent_with_role_else_root(vm_id, ac, ACCESSIBLE_PANEL)
        {
            dbg!(parent_or_root);
            jab.release_java_object(vm_id, parent_or_root);
        }
        if let Some(top) = jab.get_top_level_object(vm_id, ac) {
            dbg!(top);
            jab.release_java_object(vm_id, top);
        }
        dbg!(jab.get_object_depth(vm_id, ac));
        if let Some(descendent) = jab.get_active_descendent(vm_id, ac) {
            dbg!(descendent);
            jab.release_java_object(vm_id, descendent);
        }
        //test1(&jab, vm_id, ac);
        //test2(&jab, vm_id, ac);

        dbg!(jab.request_focus(vm_id, ac));
        dbg!(jab.get_visible_children_count(vm_id, ac));
        dbg!(jab.get_visible_children(vm_id, ac, 0));
        dbg!(jab.get_events_waiting());
        dbg!(jab.get_caret_location(vm_id, ac, 0));

        jab.release_java_object(vm_id, ac);

        dbg!(jab);
    }

    fn test1(jab: &JabLib, vm_id: i32, ac: AccessibleContext) {
        dbg!(jab.set_caret_position(vm_id, ac, 2));
        let version_info = jab.get_version_info(vm_id);
        dbg!(version_info);
        let info = jab.getAccessibleContextInfo(vm_id, ac);
        dbg!(info);
        dbg!(jab.get_accessible_actions(vm_id, ac));
        dbg!(jab.get_text_attributes_in_range(vm_id, ac, 2, 5));
        dbg!(jab.get_accessible_relation_set(vm_id, ac));
        dbg!(jab.get_accessible_key_bindings(vm_id, ac));
        dbg!(jab.get_accessible_icons(vm_id, ac));
        dbg!(jab.get_accessible_table_row_header(vm_id, ac));
        dbg!(jab.get_accessible_table_column_header(vm_id, ac));
        dbg!(jab.select_text_range(vm_id, ac, 0, 8));
    }

    fn test2(jab: &JabLib, vm_id: i32, ac: AccessibleContext) {
        dbg!(jab.get_accessible_table_column_description(vm_id, ac, 0));
        dbg!(jab.get_accessible_table_row_description(vm_id, ac, 0));
        dbg!(jab.get_accessible_table_info(vm_id, ac));
        dbg!(jab.get_virtual_accessible_name(vm_id, ac, 10));
        dbg!(jab.get_accessible_hypertext(vm_id, ac));
        dbg!(jab.get_accessible_hypertext_ext(vm_id, ac, 0));
    }
}
