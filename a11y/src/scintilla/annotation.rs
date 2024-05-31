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
    ANNOTATION_BOXED, ANNOTATION_HIDDEN, ANNOTATION_INDENTED, ANNOTATION_STANDARD,
};

/// 批注可以在视图中显示，并且可以选择显示时的显示样式。
#[derive(Debug, PartialEq)]
pub enum Annotation {
    /// 不显示批注。
    Hidden,
    /// 批注以左对齐方式绘制，没有任何修饰。
    Standard,
    /// 批注缩进以匹配文本并且被框包围。
    Boxed,
    /// 批注缩进以匹配文本。
    Indented,
}

impl From<u32> for Annotation {
    fn from(value: u32) -> Self {
        match value {
            ANNOTATION_HIDDEN => Self::Hidden,
            ANNOTATION_STANDARD => Self::Standard,
            ANNOTATION_BOXED => Self::Boxed,
            ANNOTATION_INDENTED => Self::Indented,
            _ => Self::Hidden,
        }
    }
}

impl Into<u32> for Annotation {
    fn into(self) -> u32 {
        match self {
            Self::Hidden => ANNOTATION_HIDDEN,
            Self::Standard => ANNOTATION_STANDARD,
            Self::Boxed => ANNOTATION_BOXED,
            Self::Indented => ANNOTATION_INDENTED,
        }
    }
}
