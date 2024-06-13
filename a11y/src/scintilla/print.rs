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
    SC_PRINT_BLACKONWHITE, SC_PRINT_COLOURONWHITE, SC_PRINT_COLOURONWHITEDEFAULTBG,
    SC_PRINT_INVERTLIGHT, SC_PRINT_NORMAL,
};

const SC_PRINT_SCREENCOLOURS: u32 = 5;

#[derive(Debug, PartialEq)]
pub enum PrintMode {
    /// 使用当前屏幕颜色打印，但行号边距除外，行号边距打印在白色背景上。这是默认设置。
    Normal,
    /// 如果使用深色屏幕背景，则可以通过反转所有颜色的光值并在白色背景上打印来节省墨水。
    InvertLight,
    /// 将所有文本打印为白色背景上的黑色。
    BlackOnWhite,
    /// 所有内容都以自己的颜色打印在白色背景上。
    ColourOnWhite,
    /// 所有内容都以其自己的前景色打印，但包括 STYLE_LINENUMBER 在内的所有样式都将打印在白色背景上。
    ColourOnWhiteDefaultBg,
    /// 使用当前屏幕颜色作为前景和背景进行打印。这是唯一不将行号边距的背景颜色设置为白色的模式。
    ScreenColours,
}

impl From<u32> for PrintMode {
    fn from(value: u32) -> Self {
        match value {
            SC_PRINT_NORMAL => Self::Normal,
            SC_PRINT_INVERTLIGHT => Self::InvertLight,
            SC_PRINT_BLACKONWHITE => Self::BlackOnWhite,
            SC_PRINT_COLOURONWHITE => Self::ColourOnWhite,
            SC_PRINT_COLOURONWHITEDEFAULTBG => Self::ColourOnWhiteDefaultBg,
            SC_PRINT_SCREENCOLOURS => Self::ScreenColours,
            _ => Self::Normal,
        }
    }
}

impl Into<u32> for PrintMode {
    fn into(self) -> u32 {
        match self {
            Self::Normal => SC_PRINT_NORMAL,
            Self::InvertLight => SC_PRINT_INVERTLIGHT,
            Self::BlackOnWhite => SC_PRINT_BLACKONWHITE,
            Self::ColourOnWhite => SC_PRINT_COLOURONWHITE,
            Self::ColourOnWhiteDefaultBg => SC_PRINT_COLOURONWHITEDEFAULTBG,
            Self::ScreenColours => SC_PRINT_SCREENCOLOURS,
        }
    }
}
