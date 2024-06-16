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

/**
 * 换行视觉标志
 * 符号 | 值 | 效果
 * SC_WRAPVISUALFLAG_NONE | 0 | 无视觉标志
 * SC_WRAPVISUALFLAG_END | 1 | 换行子行末尾的视觉标志。
 * SC_WRAPVISUALFLAG_START | 2 | 换行子行开头的视觉标志。子行至少缩进 1 以给标志留出空间。
 * SC_WRAPVISUALFLAG_MARGIN | 4 | 行号边距中的视觉标志。
 * */
pub use scintilla_sys::{
    SC_WRAPVISUALFLAG_END, SC_WRAPVISUALFLAG_MARGIN, SC_WRAPVISUALFLAG_NONE,
    SC_WRAPVISUALFLAG_START,
};

/**
 * 换行视觉标志位置
 * 符号 | 值 | 效果
 * SC_WRAPVISUALFLAGLOC_DEFAULT | 0 | 在边框附近绘制的视觉标志
 * SC_WRAPVISUALFLAGLOC_END_BY_TEXT | 1 | 在文本附近绘制的子线末尾的视觉标志
 * SC_WRAPVISUALFLAGLOC_START_BY_TEXT | 2 | 在文本附近绘制的子线开头的视觉标志
 * */
pub use scintilla_sys::{
    SC_WRAPVISUALFLAGLOC_DEFAULT, SC_WRAPVISUALFLAGLOC_END_BY_TEXT,
    SC_WRAPVISUALFLAGLOC_START_BY_TEXT,
};

use scintilla_sys::{
    SC_WRAPINDENT_FIXED, SC_WRAPINDENT_INDENT, SC_WRAPINDENT_SAME, SC_WRAP_CHAR, SC_WRAP_NONE,
    SC_WRAP_WHITESPACE, SC_WRAP_WORD,
};

const SC_WRAPINDENT_DEEPINDENT: u32 = 3;

/// 换行模式
#[derive(Debug, PartialEq)]
pub enum WrapMode {
    /// SC_WRAP_NONE(0)禁用换行。
    /// SC_WRAP_NONE (0)，每行文本都会生成一行输出，如果行太长而无法放入打印区域，则会截断该行。
    None,
    /// SC_WRAP_WORD(1)默认值，以启用单词或样式边界的换行。
    /// 它将打印输出换行，以便所有字符都适合打印矩形。尝试仅在单词之间换行，如空格或样式更改所示，但如果单词长于一行，则会在行尾之前换行。
    Word,
    /// SC_WRAP_CHAR(2)启用任何字符之间的换行。
    /// 适用于单词之间没有空格的亚洲语言。
    /// 不支持打印。
    Char,
    /// SC_WRAP_WHITESPACE (3) 启用空格换行。
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

/// 换行缩进模式
#[derive(Debug, PartialEq)]
pub enum WrapIndent {
    /// 换行子行与窗口左侧对齐，加上 SCI_SETWRAPSTARTINDENT 设置的量
    Fixed,
    /// 换行的子行与第一个子行缩进对齐
    Same,
    /// 换行的子行与第一个子行缩进对齐，再加上一级缩进
    Indent,
    /// 换行的子行与第一个子行缩进加上另外两级缩进对齐
    DeepIndent,
}

impl From<u32> for WrapIndent {
    fn from(value: u32) -> Self {
        match value {
            SC_WRAPINDENT_FIXED => Self::Fixed,
            SC_WRAPINDENT_SAME => Self::Same,
            SC_WRAPINDENT_INDENT => Self::Indent,
            SC_WRAPINDENT_DEEPINDENT => Self::DeepIndent,
            _ => Self::DeepIndent,
        }
    }
}

impl Into<u32> for WrapIndent {
    fn into(self) -> u32 {
        match self {
            Self::Fixed => SC_WRAPINDENT_FIXED,
            Self::Same => SC_WRAPINDENT_SAME,
            Self::Indent => SC_WRAPINDENT_INDENT,
            Self::DeepIndent => SC_WRAPINDENT_DEEPINDENT,
        }
    }
}
