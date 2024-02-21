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

use win_wrap::common::BOOL;

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
    /// output of "java -version"
    pub(crate) VMversion: [u16; SHORT_STRING_SIZE as usize],
    /// version of the AccessBridge.class
    pub(crate) bridgeJavaClassVersion: [u16; SHORT_STRING_SIZE as usize],
    /// version of JavaAccessBridge.dll
    pub(crate) bridgeJavaDLLVersion: [u16; SHORT_STRING_SIZE as usize],
    /// version of WindowsAccessBridge.dll
    pub(crate) bridgeWinDLLVersion: [u16; SHORT_STRING_SIZE as usize],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct AccessibleContextInfo {
    /// the AccessibleName of the object
    pub(crate) name: [u16; MAX_STRING_SIZE as usize],
    /// the AccessibleDescription of the object
    pub(crate) description: [u16; MAX_STRING_SIZE as usize],
    /// localized AccesibleRole string
    pub(crate) role: [u16; SHORT_STRING_SIZE as usize],
    /// AccesibleRole string in the en_US locale
    pub(crate) role_en_US: [u16; SHORT_STRING_SIZE as usize],
    /// localized AccesibleStateSet string (comma separated)
    pub(crate) states: [u16; SHORT_STRING_SIZE as usize],
    /// AccesibleStateSet string in the en_US locale (comma separated)
    pub(crate) states_en_US: [u16; SHORT_STRING_SIZE as usize],
    /// index of an object in parent
    pub(crate) indexInParent: JInt,
    /// # of children, if any
    pub(crate) childrenCount: JInt,
    // screen coords in pixels
    pub(crate) x: JInt,
    pub(crate) y: JInt,
    /// pixel width of an object
    pub(crate) width: JInt,
    /// pixel height of an object
    pub(crate) height: JInt,
    /// flags for various additional
    pub(crate) accessibleComponent: BOOL,
    ///  Java Accessibility interfaces
    pub(crate) accessibleAction: BOOL,
    ///  FALSE if this object doesn't
    pub(crate) accessibleSelection: BOOL,
    ///  implement the additional interface
    pub(crate) accessibleText: BOOL,
    //  in question
    /// new bitfield containing additional interface flags
    pub(crate) accessibleInterfaces: BOOL,
}
