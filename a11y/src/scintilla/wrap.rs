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

use scintilla_sys::{SC_WRAP_CHAR, SC_WRAP_NONE, SC_WRAP_WHITESPACE, SC_WRAP_WORD};

/// 换行模式
#[derive(Debug, PartialEq)]
pub enum WrapMode {
    /// SC_WRAP_NONE (0)，每行文本都会生成一行输出，如果行太长而无法放入打印区域，则会截断该行。
    None,
    /// SC_WRAP_WORD (1)，默认值，它将打印输出换行，以便所有字符都适合打印矩形。尝试仅在单词之间换行，如空格或样式更改所示，但如果单词长于一行，则会在行尾之前换行。
    Word,
    /// SC_WRAP_CHAR 不支持打印。
    Char,
    Whitespace,
}

impl From<u32> for WrapMode {
    fn from(value: u32) -> Self {
        match value {
            SC_WRAP_NONE => Self::None,
            SC_WRAP_WORD => Self::Word,
            SC_WRAP_CHAR => Self::Char,
            SC_WRAP_WHITESPACE => Self::Whitespace,
            _ => Self::None,
        }
    }
}

impl Into<u32> for WrapMode {
    fn into(self) -> u32 {
        match self {
            Self::None => SC_WRAP_NONE,
            Self::Word => SC_WRAP_WORD,
            Self::Char => SC_WRAP_CHAR,
            Self::Whitespace => SC_WRAP_WHITESPACE,
        }
    }
}
