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

use scintilla_sys::{SC_IME_INLINE, SC_IME_WINDOWED};

/// 当以中文、日语或韩语输入文本时，可能会显示输入法编辑器（IME）。IME可以是出现在闪烁体上方的额外窗口，也可以由闪烁体本身显示为文本。
/// 窗口行为可以用SCI_SETIMEINTERACTION（SC_IME_WINDOWED）来选择，内联行为可以用SCI_SETIMEINTERCTION（SC_IME_INLINE）来选择。闪烁体在某些情况下可能会忽略这一呼吁。例如，内联行为可能只支持某些语言。
/// 当内联输入法模式处于活动状态时，将在最终确定之前临时添加字符，并为每个字符发送SCN_CHARADED通知（其中characterSource设置为SC_CHARACTERSOURCE_TENTATIVE_INPUT）。
#[derive(Debug, PartialEq)]
pub enum Ime {
    /// 窗口IME SC_IME_WINDOWED（0）在外观和行为上可能与其他应用中的IME更相似。
    /// 输入法支持： Windows✓;GTK✓
    Windowed,
    /// 内联IME SC_IME_INLINE（1）可以更好地与一些闪烁体功能（如矩形和多选）以及IME交互（如检索周围或重新转换功能）配合使用。
    /// 输入法支持： Windows✓;GTK✓;Qt✓;macOS✓
    Inline,
}

impl From<u32> for Ime {
    fn from(value: u32) -> Self {
        match value {
            SC_IME_WINDOWED => Self::Windowed,
            SC_IME_INLINE => Self::Inline,
            _ => Self::Windowed,
        }
    }
}

impl Into<u32> for Ime {
    fn into(self) -> u32 {
        match self {
            Self::Windowed => SC_IME_WINDOWED,
            Self::Inline => SC_IME_INLINE,
        }
    }
}
