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

use scintilla_sys::{
    SC_FOLDACTION_CONTRACT, SC_FOLDACTION_EXPAND, SC_FOLDACTION_TOGGLE, SC_FOLDDISPLAYTEXT_BOXED,
    SC_FOLDDISPLAYTEXT_HIDDEN, SC_FOLDDISPLAYTEXT_STANDARD,
};

const SC_FOLDACTION_CONTRACT_EVERY_LEVEL: u32 = 4;

#[derive(Debug, PartialEq)]
pub enum FoldDisplayText {
    /// 不显示文本标签。这是默认设置。
    Hidden,
    /// 显示文本标签。
    Standard,
    /// 显示文本标签并在其周围绘制一个框。
    Boxed,
}

impl From<u32> for FoldDisplayText {
    fn from(value: u32) -> Self {
        match value {
            SC_FOLDDISPLAYTEXT_HIDDEN => Self::Hidden,
            SC_FOLDDISPLAYTEXT_STANDARD => Self::Standard,
            SC_FOLDDISPLAYTEXT_BOXED => Self::Boxed,
            _ => Self::Hidden,
        }
    }
}

impl Into<u32> for FoldDisplayText {
    fn into(self) -> u32 {
        match self {
            Self::Hidden => SC_FOLDDISPLAYTEXT_HIDDEN,
            Self::Standard => SC_FOLDDISPLAYTEXT_STANDARD,
            Self::Boxed => SC_FOLDDISPLAYTEXT_BOXED,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FoldAction {
    /// 收缩
    Contract,
    /// 展开
    Expand,
    /// 在收缩和扩展之间切换。
    Toggle,
    /// 仅用于 SCI_FOLDALL，可以与 SC_FOLDACTION_CONTRACT 或 SC_FOLDACTION_TOGGLE 结合使用以收缩所有级别而不是仅仅收缩顶层。
    ContractEveryLevel,
}

impl From<u32> for FoldAction {
    fn from(value: u32) -> Self {
        match value {
            SC_FOLDACTION_CONTRACT => Self::Contract,
            SC_FOLDACTION_EXPAND => Self::Expand,
            SC_FOLDACTION_TOGGLE => Self::Toggle,
            SC_FOLDACTION_CONTRACT_EVERY_LEVEL => Self::ContractEveryLevel,
            _ => Self::Contract,
        }
    }
}

impl Into<u32> for FoldAction {
    fn into(self) -> u32 {
        match self {
            Self::Contract => SC_FOLDACTION_CONTRACT,
            Self::Expand => SC_FOLDACTION_EXPAND,
            Self::Toggle => SC_FOLDACTION_TOGGLE,
            Self::ContractEveryLevel => SC_FOLDACTION_CONTRACT_EVERY_LEVEL,
        }
    }
}
