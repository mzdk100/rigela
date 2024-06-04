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

//! 有 32 个标记，编号为 0 到 MARKER_MAX (31)，您可以将它们的任意组合分配给文档中的每一行。
//! 标记出现在文本左侧的选择边距中。如果将选择边距设置为零宽度，则将更改整行的背景颜色。
//! Scintilla 在折叠边距中使用标记编号 25 到 31，其符号名称形式为 SC_MARKNUM_*，例如 SC_MARKNUM_FOLDEROPEN。
//! 标记编号 21 到 24 用于更改历史记录（如果已启用），但其他情况下可自由用于应用程序。
//! 标记编号 0 到 20 没有预定义功能；您可以使用它们来标记语法错误或当前执行点、断点或任何您需要标记的内容。
//! 如果您不需要折叠，您可以将所有 32 个标记用于任何您想要的目的。每个标记编号都有一个与之关联的符号。
//! 您还可以为每个标记编号设置前景色和背景色，这样您就可以多次使用相同的符号，并使用不同的颜色来满足不同的用途。
//! Scintilla 有一组您可以分配的符号 (SC_MARK_*)，或者您可以使用字符。默认情况下，所有 32 个标记都设置为 SC_MARK_CIRCLE，前景为黑色，背景为白色。
//! 标记按其编号的顺序绘制（SC_MARK_BAR 除外），因此编号较高的标记出现在编号较低的标记之上。SC_MARK_BAR 标记首先绘制，因此它们位于下方，因为它们通常覆盖多行以记录更改历史记录，而其他标记则标记单独地行。
//! 标记会尝试通过跟踪行首移动的位置来随文本一起移动。删除一行时，其标记将通过 OR 运算与下一行的标记合并。

use scintilla_sys::{
    SC_MARKNUM_FOLDER, SC_MARKNUM_FOLDEREND, SC_MARKNUM_FOLDERMIDTAIL, SC_MARKNUM_FOLDEROPEN,
    SC_MARKNUM_FOLDEROPENMID, SC_MARKNUM_FOLDERSUB, SC_MARKNUM_FOLDERTAIL, SC_MARK_ARROW,
    SC_MARK_ARROWDOWN, SC_MARK_ARROWS, SC_MARK_AVAILABLE, SC_MARK_BACKGROUND, SC_MARK_BOOKMARK,
    SC_MARK_BOXMINUS, SC_MARK_BOXMINUSCONNECTED, SC_MARK_BOXPLUS, SC_MARK_BOXPLUSCONNECTED,
    SC_MARK_CHARACTER, SC_MARK_CIRCLE, SC_MARK_CIRCLEMINUS, SC_MARK_CIRCLEMINUSCONNECTED,
    SC_MARK_CIRCLEPLUS, SC_MARK_CIRCLEPLUSCONNECTED, SC_MARK_DOTDOTDOT, SC_MARK_EMPTY,
    SC_MARK_FULLRECT, SC_MARK_LCORNER, SC_MARK_LCORNERCURVE, SC_MARK_LEFTRECT, SC_MARK_MINUS,
    SC_MARK_PIXMAP, SC_MARK_PLUS, SC_MARK_RGBAIMAGE, SC_MARK_ROUNDRECT, SC_MARK_SHORTARROW,
    SC_MARK_SMALLRECT, SC_MARK_TCORNER, SC_MARK_TCORNERCURVE, SC_MARK_UNDERLINE, SC_MARK_VLINE,
};

/// 还有更多的扁平树样式：SC_MARKNUM_FOLDEREND、SC_MARKNUM_FOLDERMIDTAIL、SC_MARKNUM_FOLDEROPENMID、SC_MARKNUM_FOLDERSUB 和 SC_MARKNUM_FOLDERTAIL。用于折叠的位由 SC_MASK_FOLDERS 指定，在定义用于折叠的边距时，它通常用作 SCI_SETMARGINMASKN 的参数。
pub trait MarkerNumber {
    /// 箭头 EMPTY, 加减号 EMPTY, 圆形树 CIRCLEPLUSCONNECTED, 矩形树 BOXPLUSCONNECTED
    const FOLDER_END: u32 = SC_MARKNUM_FOLDEREND;
    /// 箭头 EMPTY, 加减号 EMPTY, 圆形树 CIRCLEMINUSCONNECTED, 矩形树 BOXMINUSCONNECTED
    const FOLDER_OPEN_MID: u32 = SC_MARKNUM_FOLDEROPENMID;
    /// 箭头 EMPTY, 加减号 EMPTY, 圆形树 TCORNERCURVE, 矩形树 TCORNER
    const FOLDER_MID_TAIL: u32 = SC_MARKNUM_FOLDERMIDTAIL;
    /// 箭头 EMPTY, 加减号 EMPTY, 圆形树 LCORNERCURVE, 矩形树 LCORNER
    const FOLDER_TAIL: u32 = SC_MARKNUM_FOLDERTAIL;
    /// 箭头 EMPTY, 加减号 EMPTY, 圆形树 VLINE, 矩形树 VLINE
    const FOLDER_SUB: u32 = SC_MARKNUM_FOLDERSUB;
    /// 标记号 SC_MARKNUM_FOLDER 用于显示当前折叠。可以为此目的分配任何符号，尽管 (SC_MARK_PLUS, SC_MARK_MINUS) 对或 (SC_MARK_ARROW, SC_MARK_ARROWDOWN) 对都是不错的选择。
    /// 箭头 ARROW, 加减号 PLUS, 圆形树 CIRCLEPLUS, 矩形树 BOXPLUS
    const FOLDER: u32 = SC_MARKNUM_FOLDER;
    /// 标记号 SC_MARKNUM_FOLDER 用于显示当前折叠打开。可以为此目的分配任何符号，尽管 (SC_MARK_PLUS, SC_MARK_MINUS) 对或 (SC_MARK_ARROW, SC_MARK_ARROWDOWN) 对都是不错的选择。
    /// 箭头 ARROWDOWN, 加减号 MINUS, 圆形树 CIRCLEMINUS, 矩形树 BOXMINUS
    const FOLDER_OPEN: u32 = SC_MARKNUM_FOLDEROPEN;
}

impl MarkerNumber for u32 {}

/// 大多数标记符号设计用于扁平树样式的折叠边距。 SC_MARK_BOXMINUS、SC_MARK_BOXMINUSCONNECTED、SC_MARK_BOXPLUS、SC_MARK_BOXPLUSCONNECTED、SC_MARK_CIRCLEMINUS、SC_MARK_CIRCLEMINUSCONNECTED、SC_MARK_CIRCLEPLUS、SC_MARK_CIRCLEPLUSCONNECTED、SC_MARK_LCORNER、SC_MARK_LCORNERCURVE、SC_MARK_TCORNER、SC_MARK_TCORNERCURVE 和 SC_MARK_VLINE。
#[derive(Debug, PartialEq)]
pub enum Mark {
    // 小圆圈
    Circle,
    /// 圆角矩形
    RoundRect,
    /// 箭头（模仿 Macintosh）
    Arrow,
    /// 小矩形
    SmallRect,
    /// 短的箭头
    ShortArrow,
    /// SC_MARK_EMPTY 符号不可见，允许客户端代码跟踪线条的移动。如果您更改了折叠样式并希望一个或多个 SC_FOLDERNUM_* 标记没有关联符号，也可以使用它。
    Empty,
    /// 下箭头
    ArrowDown,
    /// 减号，将打开的折叠显示为“-”
    Minus,
    /// 加号（将折叠线显示为“+”）
    Plus,
    /// 垂直行
    VLine,
    LCorner,
    TCorner,
    BoxPlus,
    BoxPlusConnected,
    BoxMinus,
    BoxMinusConnected,
    LCornerCurve,
    TCornerCurve,
    CirclePlus,
    CirclePlusConnected,
    CircleMinus,
    CircleMinusConnected,
    /// SC_MARK_BACKGROUND 标记仅更改线条的背景颜色。
    Background,
    /// 省略号
    DotDotDot,
    /// 箭头
    Arrows,
    Pixmap,
    /// SC_MARK_FULLRECT 仅更改边距背景颜色。
    FullRect,
    LeftRect,
    /// 应用程序可以使用标记符号 SC_MARK_AVAILABLE 来指示插件可以分配该标记编号。
    Available,
    /// SC_MARK_UNDERLINE 在文本上绘制下划线。
    Underline,
    RgbaImage,
    /// 书签
    Bookmark,
    /// 通过将字符的 Unicode 代码点添加到 SC_MARK_CHARACTER (10000)，可以将字符用作标记。例如，要使用“▥”SQUARE WITH VERTICAL FILL（Unicode 代码点 9637）作为标记编号 1，请使用： SCI_MARKERDEFINE(1, SC_MARK_CHARACTER+9637)。
    Character(i16),
}

impl From<u32> for Mark {
    fn from(value: u32) -> Self {
        match value {
            SC_MARK_CIRCLE => Self::Circle,
            SC_MARK_ROUNDRECT => Self::RoundRect,
            SC_MARK_ARROW => Self::Arrow,
            SC_MARK_SMALLRECT => Self::SmallRect,
            SC_MARK_SHORTARROW => Self::ShortArrow,
            SC_MARK_EMPTY => Self::Empty,
            SC_MARK_ARROWDOWN => Self::ArrowDown,
            SC_MARK_MINUS => Self::Minus,
            SC_MARK_PLUS => Self::Plus,
            SC_MARK_VLINE => Self::VLine,
            SC_MARK_LCORNER => Self::LCorner,
            SC_MARK_TCORNER => Self::TCorner,
            SC_MARK_BOXPLUS => Self::BoxPlus,
            SC_MARK_BOXPLUSCONNECTED => Self::BoxPlusConnected,
            SC_MARK_BOXMINUS => Self::BoxMinus,
            SC_MARK_BOXMINUSCONNECTED => Self::BoxMinusConnected,
            SC_MARK_LCORNERCURVE => Self::LCornerCurve,
            SC_MARK_TCORNERCURVE => Self::TCornerCurve,
            SC_MARK_CIRCLEPLUS => Self::CirclePlus,
            SC_MARK_CIRCLEPLUSCONNECTED => Self::CirclePlusConnected,
            SC_MARK_CIRCLEMINUS => Self::CircleMinus,
            SC_MARK_CIRCLEMINUSCONNECTED => Self::CircleMinusConnected,
            SC_MARK_BACKGROUND => Self::Background,
            SC_MARK_DOTDOTDOT => Self::DotDotDot,
            SC_MARK_ARROWS => Self::Arrows,
            SC_MARK_PIXMAP => Self::Pixmap,
            SC_MARK_FULLRECT => Self::FullRect,
            SC_MARK_LEFTRECT => Self::LeftRect,
            SC_MARK_AVAILABLE => Self::Available,
            SC_MARK_UNDERLINE => Self::Underline,
            SC_MARK_RGBAIMAGE => Self::RgbaImage,
            SC_MARK_BOOKMARK => Self::Bookmark,
            _ => Self::Character((SC_MARK_CHARACTER + value) as i16),
        }
    }
}

impl Into<u32> for Mark {
    fn into(self) -> u32 {
        match self {
            Self::Arrow => SC_MARK_ARROW,
            Self::ArrowDown => SC_MARK_ARROWDOWN,
            Self::Arrows => SC_MARK_ARROWS,
            Self::Available => SC_MARK_AVAILABLE,
            Self::Background => SC_MARK_BACKGROUND,
            Self::Bookmark => SC_MARK_BOOKMARK,
            Self::BoxMinus => SC_MARK_BOXMINUS,
            Self::BoxMinusConnected => SC_MARK_BOXMINUSCONNECTED,
            Self::BoxPlus => SC_MARK_BOXPLUS,
            Self::BoxPlusConnected => SC_MARK_BOXPLUSCONNECTED,
            Self::Character(value) => value as u32 - SC_MARK_CHARACTER,
            Self::Circle => SC_MARK_CIRCLE,
            Self::CircleMinus => SC_MARK_CIRCLEMINUS,
            Self::CircleMinusConnected => SC_MARK_CIRCLEMINUSCONNECTED,
            Self::CirclePlus => SC_MARK_CIRCLEPLUS,
            Self::CirclePlusConnected => SC_MARK_CIRCLEPLUSCONNECTED,
            Self::DotDotDot => SC_MARK_DOTDOTDOT,
            Self::Empty => SC_MARK_EMPTY,
            Self::FullRect => SC_MARK_FULLRECT,
            Self::LCorner => SC_MARK_LCORNER,
            Self::LCornerCurve => SC_MARK_LCORNERCURVE,
            Self::LeftRect => SC_MARK_LEFTRECT,
            Self::Minus => SC_MARK_MINUS,
            Self::Pixmap => SC_MARK_PIXMAP,
            Self::Plus => SC_MARK_PLUS,
            Self::RgbaImage => SC_MARK_RGBAIMAGE,
            Self::RoundRect => SC_MARK_ROUNDRECT,
            Self::ShortArrow => SC_MARK_SHORTARROW,
            Self::SmallRect => SC_MARK_SMALLRECT,
            Self::TCorner => SC_MARK_TCORNER,
            Self::TCornerCurve => SC_MARK_TCORNERCURVE,
            Self::Underline => SC_MARK_UNDERLINE,
            Self::VLine => SC_MARK_VLINE,
        }
    }
}
