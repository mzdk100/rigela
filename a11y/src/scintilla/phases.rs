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

use scintilla_sys::{SC_PHASES_MULTIPLE, SC_PHASES_ONE, SC_PHASES_TWO};

/// 有几种顺序可以绘制文本区域，从而在速度和允许看到文本的所有像素（即使它们与其他元素重叠）之间进行权衡。
#[derive(Debug, PartialEq)]
pub enum Phases {
    /// 在单相绘图（SC_PHASES_ONE）中，以一种样式绘制的每个字符序列都与其背景一起绘制。如果一个字符悬在游程的末尾，例如在“V_”中，“V”的样式与“_”不同，则这可能导致“V”右侧被“_”的背景透支，从而将其截断。
    /// 单相绘图已弃用，应用程序不应使用它。
    One,
    /// 两相绘制（SC_PHASES_TWO）通过先绘制一条线的所有背景，然后在透明模式下绘制文本来解决这一问题。线条是单独绘制的，任何线条都不会与另一条线条重叠，因此任何重叠到另一条线上的像素（如字符上的极端上升和下降）都将被切断。除非缓冲绘制在平台上或平台自然缓冲，否则两相绘制可能会比单相闪烁更多。默认情况下，绘制为两个阶段。
    Two,
    /// 多阶段绘制（SC_PHASES_MULTIPLE）多次绘制整个区域，每个特征绘制一次，从而在图层或阶段中构建外观。所有线条的彩色背景都绘制在任何文本之前，然后以透明模式在组合背景上绘制所有文本，而不将文本剪裁到线条边界。这允许极端的上升和下降溢出到相邻的行中。此模式与缓冲绘图不兼容，如果打开缓冲绘图，它将充当SC_PHASES_TWO。多阶段绘图比两阶段绘图慢。使用SCI_SETLAYOUTCACHE（SC_cache_PAGE）或更高版本设置布局缓存可以确保多阶段绘制不会明显变慢。
    Multiple,
}

impl From<u32> for Phases {
    fn from(value: u32) -> Self {
        match value {
            SC_PHASES_ONE => Self::One,
            SC_PHASES_TWO => Self::Two,
            SC_PHASES_MULTIPLE => Self::Multiple,
            _ => Self::One,
        }
    }
}

impl Into<u32> for Phases {
    fn into(self) -> u32 {
        match self {
            Self::One => SC_PHASES_ONE,
            Self::Two => SC_PHASES_TWO,
            Self::Multiple => SC_PHASES_MULTIPLE,
        }
    }
}
