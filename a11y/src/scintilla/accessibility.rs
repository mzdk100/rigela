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

use scintilla_sys::{SC_ACCESSIBILITY_DISABLED, SC_ACCESSIBILITY_ENABLED};

/// 无障碍状态
#[derive(Debug, PartialEq)]
pub enum Accessibility {
    /// 辅助功能已禁用。
    Disabled,
    /// 辅助功能已启用。
    Enabled,
}

impl From<u32> for Accessibility {
    fn from(value: u32) -> Self {
        match value {
            SC_ACCESSIBILITY_DISABLED => Self::Disabled,
            SC_ACCESSIBILITY_ENABLED => Self::Enabled,
            _ => Self::Disabled,
        }
    }
}

impl Into<u32> for Accessibility {
    fn into(self) -> u32 {
        match self {
            Self::Disabled => SC_ACCESSIBILITY_DISABLED,
            Self::Enabled => SC_ACCESSIBILITY_ENABLED,
        }
    }
}
