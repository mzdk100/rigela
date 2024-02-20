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
}
