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

#[macro_export]
macro_rules! jab {
    ($module:expr,windows_run) => {
        call_proc!($module, Windows_run, extern "cdecl" fn() -> BOOL,)
    };
    ($module:expr,is_java_window,$h_wnd:expr) => {
        call_proc!(
            $module,
            isJavaWindow,
            extern "cdecl" fn(HWND) -> BOOL,
            $h_wnd
        )
    };
    ($module:expr,get_accessible_context_from_hwnd,$target:expr,$vm_id:expr,$ac:expr) => {
        call_proc!(
            $module,
            getAccessibleContextFromHWND,
            extern "cdecl" fn(HWND, *mut i32, *mut AccessibleContext) -> BOOL,
            $target,
            $vm_id,
            $ac
        )
    };
    ($module:expr,get_hwnd__from_accessible_context,$vm_id:expr,$ac:expr) => {
        call_proc!(
            $module,
            getHWNDFromAccessibleContext,
            extern "cdecl" fn(i32, AccessibleContext) -> HWND,
            $vm_id,
            $ac
        )
    };
    ($module:expr,release_java_object,$vm_id:expr,$object:expr) => {
        call_proc!(
            $module,
            releaseJavaObject,
            extern "cdecl" fn(i32, JavaObject),
            $vm_id,
            $object
        )
    };
    ($module:expr,get_version_info,$vm_id:expr,$info:expr) => {
        call_proc!(
            $module,
            getVersionInfo,
            extern "cdecl" fn(i32, *mut AccessBridgeVersionInfo) -> BOOL,
            $vm_id,
            $info
        )
    };
    ($module:expr,get_accessible_context_at,$vm_id:expr,$parent:expr,$x:expr,$y:expr,$ac:expr) => {
        call_proc!(
            $module,
            getAccessibleContextAt,
            extern "cdecl" fn(i32, AccessibleContext, JInt, JInt, *mut AccessibleContext) -> BOOL,
            $vm_id,
            $parent,
            $x,
            $y,
            $ac
        )
    };
    ($module:expr,get_accessible_context_with_focus,$window:expr,$vm_id:expr,$ac:expr) => {
        call_proc!(
            $module,
            getAccessibleContextWithFocus,
            extern "cdecl" fn(HWND, *mut i32, *mut AccessibleContext) -> BOOL,
            $window,
            $vm_id,
            $ac
        )
    };
    ($module:expr,get_accessible_context_info,$vm_id:expr,$ac:expr,$info:expr) => {
        call_proc!(
            $module,
            getAccessibleContextInfo,
            extern "cdecl" fn(i32, AccessibleContext, *mut AccessibleContextInfo) -> BOOL,
            $vm_id,
            $ac,
            $info
        )
    };
    ($module:expr,get_accessible_child_from_context,$vm_id:expr,$ac:expr,$index:expr) => {
        call_proc!(
            $module,
            getAccessibleChildFromContext,
            extern "system" fn(i32, AccessibleContext, JInt) -> AccessibleContext,
            $vm_id,
            $ac,
            $index
        )
    };
    ($module:expr,get_accessible_parent_from_context,$vm_id:expr,$ac:expr) => {
        call_proc!(
            $module,
            getAccessibleParentFromContext,
            extern "cdecl" fn(i32, AccessibleContext) -> AccessibleContext,
            $vm_id,
            $ac
        )
    };
    ($module:expr,is_same_object,$vm_id:expr,$obj1:expr,$obj2:expr) => {
        call_proc!(
            $module,
            isSameObject,
            extern "cdecl" fn(i32, JObject64, JObject64) -> BOOL,
            $vm_id,
            $obj1,
            $obj2
        )
    };
    ($module:expr,get_parent_with_role,$vm_id:expr,$ac:expr,$role:expr) => {
        call_proc!(
            $module,
            getParentWithRole,
            extern "cdecl" fn(i32, AccessibleContext, *const u16) -> AccessibleContext,
            $vm_id,
            $ac,
            $role
        )
    };
    ($module:expr,get_parent_with_role_else_root,$vm_id:expr,$ac:expr,$role:expr) => {
        call_proc!(
            $module,
            getParentWithRoleElseRoot,
            extern "cdecl" fn(i32, AccessibleContext, *const u16) -> AccessibleContext,
            $vm_id,
            $ac,
            $role
        )
    };
    ($module:expr,get_top_level_object,$vm_id:expr,$ac:expr) => {
        call_proc!(
            $module,
            getTopLevelObject,
            extern "cdecl" fn(i32, AccessibleContext) -> AccessibleContext,
            $vm_id,
            $ac
        )
    };
    ($module:expr,get_object_depth,$vm_id:expr,$ac:expr) => {
        call_proc!(
            $module,
            getObjectDepth,
            extern "cdecl" fn(i32, AccessibleContext) -> i32,
            $vm_id,
            $ac
        )
    };
    ($module:expr,get_active_descendent,$vm_id:expr,$ac:expr) => {
        call_proc!(
            $module,
            getActiveDescendent,
            extern "cdecl" fn(i32, AccessibleContext) -> AccessibleContext,
            $vm_id,
            $ac
        )
    };
    ($module:expr,request_focus,$vm_id:expr,$ac:expr) => {
        call_proc!(
            $module,
            requestFocus,
            extern "cdecl" fn(i32, AccessibleContext) -> BOOL,
            $vm_id,
            $ac
        )
    };
    ($module:expr,get_visible_children_count,$vm_id:expr,$ac:expr) => {
        call_proc!(
            $module,
            getVisibleChildrenCount,
            extern "cdecl" fn(i32, AccessibleContext) -> i32,
            $vm_id,
            $ac
        )
    };
}
