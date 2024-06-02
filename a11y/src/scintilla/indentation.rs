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

use scintilla_sys::{SC_IV_LOOKBOTH, SC_IV_LOOKFORWARD, SC_IV_NONE, SC_IV_REAL};

/// 缩进指南是每缩进大小列在缩进空白处出现的垂直虚线。它们可以很容易地看到哪些结构对齐，特别是当它们延伸到多页时。样式 STYLE_INDENTGUIDE (37) 用于指定缩进指南的前景色和背景色。有 4 个缩进指南视图。SC_IV_NONE 关闭该功能，但其他 3 个状态决定指南在空行上出现多远。
#[derive(Debug, PartialEq)]
pub enum IndentView {
    /// 不显示缩进指南。
    None,
    /// 缩进指南显示在实际缩进空白处内。
    Real,
    /// 缩进指南显示在实际缩进之外，直到下一个非空行的级别。如果前一个非空行是折叠标题，则缩进指南会比该行多显示一个缩进级别。此设置适用于 Python。
    LookForward,
    /// 缩进指南显示在实际缩进之外，直到下一个非空行或上一个非空行（以较大者为准）的级别。此设置适用于大多数语言。
    LookBoth,
}

impl From<u32> for IndentView {
    fn from(value: u32) -> Self {
        match value {
            SC_IV_NONE => Self::None,
            SC_IV_REAL => Self::Real,
            SC_IV_LOOKFORWARD => Self::LookForward,
            SC_IV_LOOKBOTH => Self::LookBoth,
            _ => Self::None,
        }
    }
}

impl Into<u32> for IndentView {
    fn into(self) -> u32 {
        match self {
            Self::None => SC_IV_NONE,
            Self::Real => SC_IV_REAL,
            Self::LookForward => SC_IV_LOOKFORWARD,
            Self::LookBoth => SC_IV_LOOKBOTH,
        }
    }
}
