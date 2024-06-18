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

use scintilla_sys::{EDGE_BACKGROUND, EDGE_LINE, EDGE_MULTILINE, EDGE_NONE};

/// 长的 行 样式
#[derive(Debug, PartialEq)]
pub enum EdgeMode {
    /// 长的 行 不标记。这是默认状态。
    None,
    /// 在 SCI_SETEDGECOLUMN 设置的列号处绘制一条垂直线。这适用于等宽字体。该线绘制在基于 STYLE_DEFAULT 中空格字符宽度的位置，因此如果您的样式使用比例字体，或者您的样式具有不同的字体大小，或者您混合使用粗体、斜体和普通文本，则它可能不太适用。
    Line,
    /// 列限制后的字符背景颜色将更改为 SCI_SETEDGECOLOUR 设置的颜色。建议用于比例字体。
    Background,
    /// 这与 EDGE_LINE 类似，但与仅显示一条线相反，可以同时显示一组可配置的垂直线。此 edgeMode 使用完全独立的数据集，只能使用 SCI_MULTIEDGE* 消息进行配置。
    MultiLine,
}

impl From<u32> for EdgeMode {
    fn from(value: u32) -> Self {
        match value {
            EDGE_NONE => Self::None,
            EDGE_LINE => Self::Line,
            EDGE_BACKGROUND => Self::Background,
            EDGE_MULTILINE => Self::MultiLine,
            _ => Self::None,
        }
    }
}

impl Into<u32> for EdgeMode {
    fn into(self) -> u32 {
        match self {
            Self::None => EDGE_NONE,
            Self::Line => EDGE_LINE,
            Self::Background => EDGE_BACKGROUND,
            Self::MultiLine => EDGE_MULTILINE,
        }
    }
}
