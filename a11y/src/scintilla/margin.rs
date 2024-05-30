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

use scintilla_sys::{SC_MARGINOPTION_NONE, SC_MARGINOPTION_SUBLINESELECT};

#[derive(Debug, PartialEq)]
pub enum MarginOptions {
    /// SC_MARGINOPTION_SUBLINESELECT=1，用于控制单击换行线前面的边距时如何选择换行线。如果设置了SC_MARGINOPTION_SUBLINESELECT，则只选择换行的子行，否则选择整个换行。
    SubLineSelect,
    /// 默认值，SC_MARGINOPTION_NONE=0
    None,
}

impl From<u32> for MarginOptions {
    fn from(value: u32) -> Self {
        match value {
            SC_MARGINOPTION_NONE => Self::None,
            SC_MARGINOPTION_SUBLINESELECT => Self::SubLineSelect,
            _ => Self::None,
        }
    }
}

impl Into<u32> for MarginOptions {
    fn into(self) -> u32 {
        match self {
            Self::None => SC_MARGINOPTION_NONE,
            Self::SubLineSelect => SC_MARGINOPTION_SUBLINESELECT,
        }
    }
}
