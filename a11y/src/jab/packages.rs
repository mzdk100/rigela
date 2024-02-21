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

#[allow(unused)]
const MAX_STRING_SIZE: u32 = 1024;
#[allow(unused)]
const SHORT_STRING_SIZE: u32 = 256;

#[allow(unused)]
pub(crate) type JInt = i32;
#[allow(unused)]
pub(crate) type JLong = i64;
#[allow(unused)]
pub(crate) type JObject = *const ();

#[cfg(target_arch = "x86")]
#[allow(unused)]
pub(crate) type JObject64 = JObject;

#[cfg(target_arch = "x86_64")]
#[allow(unused)]
pub(crate) type JObject64 = JLong;

// object types
pub(crate) type AccessibleContext = JObject64;
#[allow(unused)]
pub(crate) type AccessibleText = JObject64;
#[allow(unused)]
pub(crate) type AccessibleValue = JObject64;
#[allow(unused)]
pub(crate) type AccessibleSelection = JObject64;
#[allow(unused)]
pub(crate) type JavaObject = JObject64;
#[allow(unused)]
pub(crate) type PropertyChangeEvent = JObject64;
#[allow(unused)]
pub(crate) type FocusEvent = JObject64;
#[allow(unused)]
pub(crate) type CaretEvent = JObject64;
#[allow(unused)]
pub(crate) type MouseEvent = JObject64;
#[allow(unused)]
pub(crate) type MenuEvent = JObject64;
#[allow(unused)]
pub(crate) type AccessibleTable = JObject64;
#[allow(unused)]
pub(crate) type AccessibleHyperlink = JObject64;
#[allow(unused)]
pub(crate) type AccessibleHypertext = JObject64;

#[derive(Debug)]
#[repr(C)]
pub(crate) struct AccessBridgeVersionInfo {
    // output of "java -version"
    vm_version: [u16; SHORT_STRING_SIZE as usize],
    // version of the AccessBridge.class
    bridge_java_class_version: [u16; SHORT_STRING_SIZE as usize],
    // version of JavaAccessBridge.dll
    bridge_java_dll_version: [u16; SHORT_STRING_SIZE as usize],
    // version of WindowsAccessBridge.dll
    bridge_win_dll_version: [u16; SHORT_STRING_SIZE as usize],
}
