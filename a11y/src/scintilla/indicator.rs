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
    INDIC_BOX, INDIC_COMPOSITIONTHICK, INDIC_COMPOSITIONTHIN, INDIC_DASH, INDIC_DIAGONAL,
    INDIC_DOTBOX, INDIC_DOTS, INDIC_FULLBOX, INDIC_HIDDEN, INDIC_PLAIN, INDIC_POINT,
    INDIC_POINTCHARACTER, INDIC_ROUNDBOX, INDIC_SQUIGGLE, INDIC_SQUIGGLELOW, INDIC_SQUIGGLEPIXMAP,
    INDIC_STRAIGHTBOX, INDIC_STRIKE, INDIC_TEXTFORE, INDIC_TT,
};

const INDIC_GRADIENT: u32 = 20;
const INDIC_GRADIENTCENTRE: u32 = 21;
const INDIC_POINT_TOP: u32 = 22;

/// 指示符用于在样式顶部显示其他信息。它们可以用来显示，例如，通过在文本周围的文本或框下绘制下文，例如语法错误，弃用的名称和不良凹痕。
/// 当鼠标越过鼠标或将其移入其中时，指示符可能具有不同的“悬停”颜色和样式。例如，可以使用这可以单击URL。
/// 指示符可以显示为简单的下划线，弯曲的下划线，一系列小的“T”形状，一线对角线孵化线，触球外或文本周围的矩形。当用于跟踪应用程序内容的内容时，它们也可能是看不见的。
/// SCI_INDIC*消息允许您获取并设置指示符的视觉外观。他们都使用指标0中的指示参数到INDICATOR_MAX（43）将指示符设置为样式。
/// 为了防止干扰，将指示符集分为征用器使用的范围（0..7）容器使用的范围（8 = INDICATOR_CONTAINER .. 31 = INDICATOR_IME-1）IME指示符的范围（32 = INDICATOR_IME .. 35 = INDICATOR_IME_MAX）和更改历史记录的范围（36 = INDICATOR_HISTORY_REVERTED_TO_ORIGIN_INSERTION .. 43 = INDICATOR_HISTORY_REVERTED_TO_MODIFIED_DELETION）。
/// 用于除法指示符的INDICATOR_*值以前是INDIC_CONTAINER，INDIC_IME，INDIC_IME_MAX和INDIC_MAX，但它们与指示符样式混淆，因此应使用新名称。
/// 指示符以类似于运行长度编码的格式存储，该格式在速度和存储方面都有高效，以获得稀疏信息。指示符可以存储每个范围的不同值，但通常所有值都相同。
/// SCI_INDICSETFLAGS API可用于显示不同值的不同颜色。最初，Scintilla对指示符使用了不同的技术，但已将其删除，API没有任何动作。尽管两种技术都得到了支持，但“现代指标”一词用于新的实施。
/// 默认指示器样式相当于：
/// SCI_INDICSETSTYLE(0, INDIC_SQUIGGLE);
/// SCI_INDICSETSTYLE(1, INDIC_TT);
/// SCI_INDICSETSTYLE(2, INDIC_PLAIN);
#[derive(Debug, PartialEq)]
pub enum Indicator {
    /// 用一条直线划下划线。
    Plain,
    /// 波浪下划线。需要 3 个像素的下行空间。
    Squiggle,
    /// 一排小 T 形。
    Tt,
    /// 对角线阴影。
    Diagonal,
    /// 出击。
    Strike,
    /// 没有视觉效果的指示符。
    Hidden,
    /// 文本周围的矩形。
    Box,
    /// 文本周围带有圆角的矩形，使用半透明绘图，内部通常比边框更透明。您可以使用 SCI_INDICSETALPHA 和 SCI_INDICSETOUTLINEALPHA 来控制 alpha 透明度值。填充颜色的默认 alpha 值为 30，轮廓颜色的默认 alpha 值为 50。
    RoundBox,
    /// 文本周围的矩形使用半透明绘图，内部通常比边框更透明。您可以使用 SCI_INDICSETALPHA 和 SCI_INDICSETOUTLINEALPHA 来控制
    /// alpha 透明度值。默认 alpha 值为填充颜色的 30 和轮廓颜色的 50。此指示符不会为线条的顶部像素着色，因此相邻线条上的指示符在视觉上是截然不同且不连贯的。
    StraightBox,
    /// 文本周围的矩形使用类似于 INDIC_STRAIGHTBOX 的半透明绘图但覆盖整个字符区域。
    FullBox,
    /// 虚线下划线。
    Dash,
    /// 虚线下划线。
    Dots,
    /// 与 INDIC_SQUIGGLE 类似，但仅使用 2 个垂直像素，因此适合小字体。
    SquiggleLow,
    /// 使用半透明绘图在文本周围绘制虚线矩形。半透明度在 alpha 和轮廓 alpha 设置之间交替，左上角像素使用 alpha 设置。SCI_INDICSETALPHA 和 SCI_INDICSETOUTLINEALPHA 控制 alpha 透明度值。alpha 的默认值为 30，轮廓 alpha 的默认值为 50。为避免分配过多内存，虚线框的最大宽度为 4000 像素。
    DotBox,
    /// 顶部颜色和 alpha 之间的垂直渐变到底部完全透明。
    Gradient,
    /// 具有指定颜色和 alpha 的垂直渐变，中间逐渐淡出到顶部和底部完全透明。
    GradientCentre,
    /// 使用PixMap而不是作为性能的一系列线段绘制的Indic_squiggle版本。测量的速度比GTK上的Indio_Squiggle快3到6倍。在HIDPI模式下，在MacOS上的MacOS上的外观将不如IndioD_Squiggle。
    SquigglePixmap,
    /// 位于行底的 2 像素粗下划线，尽量避免接触字符基部。每边插入 1 像素，以便此样式中覆盖一定范围的不同指示符看起来是独立的。这类似于亚洲语言输入组合中用于目标的外观。可以使用SCI_INDICSETOUTLINEALPHA 更改此指示符的半透明度。
    CompositionThick,
    /// 位于行底之前的 1 像素粗下划线。每边插入 1 像素，因此这种样式中覆盖某一个范围的不同指示符会孤立显示。这类似于亚洲语言输入排版中用于非目标范围的外观。
    CompositionThin,
    /// 将文本的颜色更改为指示符的前景色。
    TextFore,
    /// 在指示符范围的开始下方画一个三角形。
    Point,
    /// 在指示符范围第一个字符的中心下方画一个三角形。
    PointCharacter,
    /// 在指示符范围的起点上方绘制一个三角形。
    PointTop,
}

impl From<u32> for Indicator {
    fn from(value: u32) -> Self {
        match value {
            INDIC_PLAIN => Self::Plain,
            INDIC_SQUIGGLE => Self::Squiggle,
            INDIC_TT => Self::Tt,
            INDIC_DIAGONAL => Self::Diagonal,
            INDIC_STRIKE => Self::Strike,
            INDIC_HIDDEN => Self::Hidden,
            INDIC_BOX => Self::Box,
            INDIC_ROUNDBOX => Self::RoundBox,
            INDIC_STRAIGHTBOX => Self::StraightBox,
            INDIC_FULLBOX => Self::FullBox,
            INDIC_DASH => Self::Dash,
            INDIC_DOTS => Self::Dots,
            INDIC_SQUIGGLELOW => Self::SquiggleLow,
            INDIC_DOTBOX => Self::DotBox,
            INDIC_SQUIGGLEPIXMAP => Self::SquigglePixmap,
            INDIC_COMPOSITIONTHICK => Self::CompositionThick,
            INDIC_COMPOSITIONTHIN => Self::CompositionThin,
            INDIC_TEXTFORE => Self::TextFore,
            INDIC_POINT => Self::Point,
            INDIC_POINTCHARACTER => Self::PointCharacter,
            INDIC_GRADIENT => Self::Gradient,
            INDIC_GRADIENTCENTRE => Self::GradientCentre,
            INDIC_POINT_TOP => Self::PointTop,
            _ => Self::Plain,
        }
    }
}

impl Into<u32> for Indicator {
    fn into(self) -> u32 {
        match self {
            Self::Plain => INDIC_PLAIN,
            Self::Squiggle => INDIC_SQUIGGLE,
            Self::Tt => INDIC_TT,
            Self::Diagonal => INDIC_DIAGONAL,
            Self::Strike => INDIC_STRIKE,
            Self::Hidden => INDIC_HIDDEN,
            Self::Box => INDIC_BOX,
            Self::RoundBox => INDIC_ROUNDBOX,
            Self::StraightBox => INDIC_STRAIGHTBOX,
            Self::FullBox => INDIC_FULLBOX,
            Self::Dash => INDIC_DASH,
            Self::Dots => INDIC_DOTS,
            Self::SquiggleLow => INDIC_SQUIGGLELOW,
            Self::DotBox => INDIC_DOTBOX,
            Self::Gradient => INDIC_GRADIENT,
            Self::GradientCentre => INDIC_GRADIENTCENTRE,
            Self::SquigglePixmap => INDIC_SQUIGGLEPIXMAP,
            Self::CompositionThick => INDIC_COMPOSITIONTHICK,
            Self::CompositionThin => INDIC_COMPOSITIONTHIN,
            Self::TextFore => INDIC_TEXTFORE,
            Self::Point => INDIC_POINT,
            Self::PointCharacter => INDIC_POINTCHARACTER,
            Self::PointTop => INDIC_POINT_TOP,
        }
    }
}
