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

use crate::jab::jab_lib::packages::JObject64;

/**
 * Header file defining callback typedefs for Windows routines
 * which are called from Java (responding to events, etc.).
 * */

pub(crate) type AccessBridgePropertyChangeFp = extern "cdecl" fn(
    i32,        /*vmID*/
    JObject64,  /*event*/
    JObject64,  /*source*/
    *const u16, /*property*/
    *const u16, /*oldValue*/
    *const u16, /*newValue*/
);

pub(crate) type AccessBridgeJavaShutdownFp = extern "cdecl" fn(i32 /*vm_id*/);

pub(crate) type AccessBridgeFocusGainedFp =
    extern "cdecl" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgeFocusLostFp =
    extern "cdecl" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);

pub(crate) type AccessBridgeCaretUpdateFp =
    extern "cdecl" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);

pub(crate) type AccessBridgeMouseClickedFp =
    extern "cdecl" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgeMouseEnteredFp =
    extern "cdecl" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgeMouseExitedFp =
    extern "cdecl" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgeMousePressedFp =
    extern "cdecl" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgeMouseReleasedFp =
    extern "cdecl" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);

pub(crate) type AccessBridgeMenuCanceledFp =
    extern "cdecl" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgeMenuDeselectedFp =
    extern "cdecl" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgeMenuSelectedFp =
    extern "cdecl" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgePopupMenuCanceledFp =
    extern "cdecl" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgePopupMenuWillBecomeInvisibleFp =
    extern "cdecl" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgePopupMenuWillBecomeVisibleFp =
    extern "cdecl" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);

pub(crate) type AccessBridgePropertyNameChangeFp = extern "cdecl" fn(
    i32,        /*vm_id*/
    JObject64,  /*event*/
    JObject64,  /*source*/
    *const u16, /*oldName*/
    *const u16, /*newName*/
);
pub(crate) type AccessBridgePropertyDescriptionChangeFp = extern "cdecl" fn(
    i32,        /*vm_id*/
    JObject64,  /*event*/
    JObject64,  /*source*/
    *const u16, /*oldDescription*/
    *const u16, /*newDescription*/
);
pub(crate) type AccessBridgePropertyStateChangeFp = extern "cdecl" fn(
    i32,        /*vm_id*/
    JObject64,  /*event*/
    JObject64,  /*source*/
    *const u16, /*oldState*/
    *const u16, /*newState*/
);
pub(crate) type AccessBridgePropertyValueChangeFp = extern "cdecl" fn(
    i32,        /*vm_id*/
    JObject64,  /*event*/
    JObject64,  /*source*/
    *const u16, /*oldValue*/
    *const u16, /*newValue*/
);
pub(crate) type AccessBridgePropertySelectionChangeFp =
    extern "cdecl" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgePropertyTextChangeFp =
    extern "cdecl" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgePropertyCaretChangeFp = extern "cdecl" fn(
    i32,       /*vm_id*/
    JObject64, /*event*/
    JObject64, /*source*/
    i32,       /*oldPosition*/
    i32,       /*newPosition*/
);
pub(crate) type AccessBridgePropertyVisibleDataChangeFp =
    extern "cdecl" fn(i32 /*vm_id*/, JObject64 /*event*/, JObject64 /*source*/);
pub(crate) type AccessBridgePropertyChildChangeFp = extern "cdecl" fn(
    i32,       /*vm_id*/
    JObject64, /*event*/
    JObject64, /*source*/
    JObject64, /*oldChild*/
    JObject64, /*newChild*/
);
pub(crate) type AccessBridgePropertyActiveDescendentChangeFp = extern "cdecl" fn(
    i32,       /*vm_id*/
    JObject64, /*event*/
    JObject64, /*source*/
    JObject64, /*oldActiveDescendent*/
    JObject64, /*newActiveDescendent*/
);

pub(crate) type AccessBridgePropertyTableModelChangeFp = extern "cdecl" fn(
    i32,        /*vm_id*/
    JObject64,  /*event*/
    JObject64,  /*src*/
    *const u16, /*oldValue*/
    *const u16, /*newValue*/
);
