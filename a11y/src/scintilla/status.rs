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

use scintilla_sys::{SC_STATUS_BADALLOC, SC_STATUS_FAILURE, SC_STATUS_OK, SC_STATUS_WARN_REGEX};

/// 如果发生错误，Scintilla可能会设置一个内部错误编号，该编号可通过SCI_GETSTATUS检索。若要清除错误状态，请调用SCI_SETSTATUS（0）。从1到999的状态值为错误，状态SC_Status_WARN_START（1000）及以上为警告。
#[derive(Debug, PartialEq)]
pub enum Status {
    /// 无故障
    Ok,
    /// 一般故障
    Failure,
    /// 内存耗尽
    BadAlloc,
    /// 正规表达式无效
    WarnRegex,
}

impl From<u32> for Status {
    fn from(value: u32) -> Self {
        match value {
            SC_STATUS_BADALLOC => Self::BadAlloc,
            SC_STATUS_FAILURE => Self::Failure,
            SC_STATUS_WARN_REGEX => Self::WarnRegex,
            SC_STATUS_OK => Self::Ok,
            _ => Self::Failure,
        }
    }
}

impl Into<u32> for Status {
    fn into(self) -> u32 {
        match self {
            Self::Ok => SC_STATUS_OK,
            Self::Failure => SC_STATUS_FAILURE,
            Self::BadAlloc => SC_STATUS_BADALLOC,
            Self::WarnRegex => SC_STATUS_WARN_REGEX,
        }
    }
}
