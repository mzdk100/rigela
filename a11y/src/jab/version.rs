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

use crate::JabLib::packages::AccessBridgeVersionInfo as ABVI;

#[derive(Debug)]
pub struct AccessBridgeVersionInfo {
    /// "java -version"的输出
    pub vm_version: String,
    /// AccessBridge.class的版本
    pub bridge_java_class_version: String,
    /// JavaAccessBridge.dll的版本
    pub bridge_java_dll_version: String,
    /// WindowsAccessBridge.dll的版本
    pub bridge_win_dll_version: String,
}

impl AccessBridgeVersionInfo {
    pub(crate) fn from(info: &ABVI) -> Self {
        Self {
            vm_version: String::from_utf16_lossy(&info.VMversion).trim_matches('\0').to_string(),
            bridge_java_class_version: String::from_utf16_lossy(&info.bridgeJavaClassVersion).trim_matches('\0').to_string(),
            bridge_java_dll_version: String::from_utf16_lossy(&info.bridgeJavaDLLVersion).trim_matches('\0').to_string(),
            bridge_win_dll_version: String::from_utf16_lossy(&info.bridgeWinDLLVersion).trim_matches('\0').to_string(),
        }
    }
}