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

use scintilla_sys::{SC_TYPE_BOOLEAN, SC_TYPE_INTEGER, SC_TYPE_STRING};

/// 属性类型
#[derive(Debug, PartialEq)]
pub enum PropertyType {
    /// 布尔值 (SC_TYPE_BOOLEAN)
    Boolean,
    /// 整数 (SC_TYPE_INTEGER)
    Integer,
    /// 字符串 (SC_TYPE_STRING)
    String,
}

impl From<u32> for PropertyType {
    fn from(value: u32) -> Self {
        match value {
            SC_TYPE_BOOLEAN => Self::Boolean,
            SC_TYPE_INTEGER => Self::Integer,
            SC_TYPE_STRING => Self::String,
            _ => Self::Boolean,
        }
    }
}

impl Into<u32> for PropertyType {
    fn into(self) -> u32 {
        match self {
            Self::Boolean => SC_TYPE_BOOLEAN,
            Self::Integer => SC_TYPE_INTEGER,
            Self::String => SC_TYPE_STRING,
        }
    }
}
