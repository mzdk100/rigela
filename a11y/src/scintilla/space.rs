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
    SCTD_LONGARROW, SCTD_STRIKEOUT, SCWS_INVISIBLE, SCWS_VISIBLEAFTERINDENT, SCWS_VISIBLEALWAYS,
    SCWS_VISIBLEONLYININDENT,
};

#[derive(Debug, PartialEq)]
pub enum WhiteSpace {
    /// 空白显示为空背景色的正常显示模式。
    Invisible,
    /// 空白字符被绘制为点和箭头，
    VisibleAways,
    /// 用于缩进的空白通常显示，但在第一个可见字符之后，它显示为点和箭头。
    VisibleAfterIndent,
    /// 用于缩进的空白显示为点和箭头。
    VisibleOnlyInIndent,
}

impl From<u32> for WhiteSpace {
    fn from(value: u32) -> Self {
        match value {
            SCWS_INVISIBLE => Self::Invisible,
            SCWS_VISIBLEALWAYS => Self::VisibleAways,
            SCWS_VISIBLEAFTERINDENT => Self::VisibleAfterIndent,
            SCWS_VISIBLEONLYININDENT => Self::VisibleOnlyInIndent,
            _ => Self::Invisible,
        }
    }
}

impl Into<u32> for WhiteSpace {
    fn into(self) -> u32 {
        match self {
            Self::Invisible => SCWS_INVISIBLE,
            Self::VisibleAways => SCWS_VISIBLEALWAYS,
            Self::VisibleAfterIndent => SCWS_VISIBLEAFTERINDENT,
            Self::VisibleOnlyInIndent => SCWS_VISIBLEONLYININDENT,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TabDrawMode {
    /// 箭头拉伸到选项卡停止的默认模式。
    LongArrow,
    /// 一条水平线，一直延伸到凸舌止点。
    Strikeout,
}

impl From<u32> for TabDrawMode {
    fn from(value: u32) -> Self {
        match value {
            SCTD_LONGARROW => Self::LongArrow,
            SCTD_STRIKEOUT => Self::Strikeout,
            _ => Self::LongArrow,
        }
    }
}

impl Into<u32> for TabDrawMode {
    fn into(self) -> u32 {
        match self {
            Self::LongArrow => SCTD_LONGARROW,
            Self::Strikeout => SCTD_STRIKEOUT,
        }
    }
}
