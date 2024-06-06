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

use scintilla_sys::{SC_MULTIAUTOC_EACH, SC_MULTIAUTOC_ONCE};

#[derive(Debug, PartialEq)]
pub enum MultiAutoc {
    /// 当自动完成多个选择时，自动完成的文本仅进入主选择（SC_MULTIAUTOC_ONCE (0)）
    Once,
    /// 当自动完成多个选择时，自动完成的文本仅进入每个选择（SC_MULTIAUTOC_EACH (1)。
    Each,
}

impl From<u32> for MultiAutoc {
    fn from(value: u32) -> Self {
        match value {
            SC_MULTIAUTOC_EACH => Self::Each,
            SC_MULTIAUTOC_ONCE => Self::Once,
            _ => Self::Once,
        }
    }
}

impl Into<u32> for MultiAutoc {
    fn into(self) -> u32 {
        match self {
            Self::Each => SC_MULTIAUTOC_EACH,
            Self::Once => SC_MULTIAUTOC_ONCE,
        }
    }
}
