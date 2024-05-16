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

use scintilla_sys::{SC_SEL_STREAM, SC_SEL_RECTANGLE, SC_SEL_LINES, SC_SEL_THIN};


#[derive(Debug, PartialEq)]
pub enum SelectionMode {
    /// 流模式
    Stream,
    /// 矩形模式
    Rectangle,
    /// 按行模式
    Lines,
    /// 细矩形模式
    Thin,
}

impl From<u32> for SelectionMode {
    fn from(value: u32) -> Self {
        match value {
            SC_SEL_STREAM => Self::Stream,
            SC_SEL_RECTANGLE => Self::Rectangle,
            SC_SEL_LINES => Self::Lines,
            SC_SEL_THIN => Self::Thin,
            _ => Self::Stream
        }
    }
}

impl Into<u32> for SelectionMode {
    fn into(self) -> u32 {
        match self {
            Self::Stream => SC_SEL_STREAM,
            Self::Rectangle => SC_SEL_RECTANGLE,
            Self::Lines => SC_SEL_LINES,
            Self::Thin => SC_SEL_THIN
        }
    }
}