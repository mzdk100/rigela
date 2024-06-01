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

use scintilla_sys::{SC_BIDIRECTIONAL_DISABLED, SC_BIDIRECTIONAL_L2R, SC_BIDIRECTIONAL_R2L};

/// 有些语言，如阿拉伯语和希伯来语，是从右到左书写的，而不是像英语那样从左到右书写。使用多种语言的文档可能包含两个方向，这被称为“双向”。默认的文本方向可以是从右向左或从左向右。闪烁体仅在某些平台上正确显示双向文本。目前，在使用DirectWrite的Win32和使用Cocoa的macOS上都有双向文本的实验支持。只有UTF-8文档将显示双向行为，并且仅在SC_BIDIRECTIONAL_L2R模式下显示。某些功能，如虚拟空间，可能无法使用双向文本，或者可能仅在某些情况下有效。SC_BIDIRECTIONAL_R2L可以在未来实现。
/// 双向文本有额外的处理和存储成本。由于某些应用程序可能不想支付成本，因此必须通过调用SCI_SETBIDIRECTIONAL（SC_BIDIRECTIONAL_L2R）（1）（选择从左到右作为默认方向）或SCI_SETBIDIRECTIONAL（SC_BIDIRECTIONAL_R2L）（2）（选择默认从右到左）来显式启用双向支持。在Win32上，应在将技术设置为SC_TECHNOLOGY_DIRECTWRITE、SC_TECHNOLOGY_DIRECTWRITERETAIN或SC_TECHNOLOGY_DIRECTWRITEDC之后执行此操作。
/// 如果设置成功，SCI_GETBIDIRECTIONAL将返回相同的值，否则返回SC_BIDIRECTIONAL_DISABLED（0）。
/// 双向模式下不支持不透明选择图形（SCI_SETSELECTIONLAYER（SC_LAYER_BASE））。请改用SC_LAYER_UNDER_TEXT或SC_LAYER_OVER_TEXT。
#[derive(Debug, PartialEq)]
pub enum Bidirectional {
    /// 禁用
    Disabled,
    /// 从左到右
    L2R,
    /// 从右到左
    R2L,
}

impl From<u32> for Bidirectional {
    fn from(value: u32) -> Self {
        match value {
            SC_BIDIRECTIONAL_DISABLED => Self::Disabled,
            SC_BIDIRECTIONAL_L2R => Self::L2R,
            SC_BIDIRECTIONAL_R2L => Self::R2L,
            _ => Self::Disabled,
        }
    }
}

impl Into<u32> for Bidirectional {
    fn into(self) -> u32 {
        match self {
            Self::Disabled => SC_BIDIRECTIONAL_DISABLED,
            Self::L2R => SC_BIDIRECTIONAL_L2R,
            Self::R2L => SC_BIDIRECTIONAL_R2L,
        }
    }
}
