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
    SC_TECHNOLOGY_DEFAULT, SC_TECHNOLOGY_DIRECTWRITE, SC_TECHNOLOGY_DIRECTWRITEDC,
    SC_TECHNOLOGY_DIRECTWRITERETAIN,
};

/// 技术属性允许在不同的绘图API和选项之间进行选择。在大多数平台上，唯一的选择是SC_TECHNOLOGY_DEFAULT（0）。
/// 在Windows Vista或更高版本上，可以选择SC_TECHNOLOGY_DIRECTWRITE（1）、SC_TECHNOLOGY_DIRECTWRITETAIN（2）或SC_TECHNOLOGY_DIRECTWRITEDC（3）来使用Direct2D和DIRECTWRITE API以获得更高质量的抗锯齿图形。
/// SC_TECHNOLOGY_DIRECTWRITETAIN与SC_TECHNOLOGY_DIRCTWRITE的不同之处在于，它请求在呈现帧之后保留帧，这可以防止某些卡和驱动程序出现绘图故障。SC_TECHNOLOGY_DIRECTWRITED与SC_TECHNOLOGY_DIRECTWRITEDC的不同之处在于使用DIRECTWRITE绘制到GDI DC中。
/// 在Win32上，缓冲图形被设置为该技术的合理值：GDI为开，Direct2D为关，因为Direct2D执行自己的缓冲。这可以在使用SCI_SETBUFFEREDDRAW设置技术后更改。
/// 使用DirectWrite时，可以使用SCI_SETFONTLOCALE设置适当的字体区域设置，以使用预期的语言相关字形绘制文本。
#[derive(Debug, PartialEq)]
pub enum Technology {
    /// SC_TECHNOLOGY_DEFAULT（0）。
    Default,
    /// SC_TECHNOLOGY_DIRECTWRITE（1）
    DirectWrite,
    /// SC_TECHNOLOGY_DIRECTWRITERETAIN（2）
    DirectWriteRetain,
    /// SC_TECHNOLOGY_DIRECTWRITEDC（3）
    DirectWriteDC,
}

impl From<u32> for Technology {
    fn from(value: u32) -> Self {
        match value {
            SC_TECHNOLOGY_DEFAULT => Self::Default,
            SC_TECHNOLOGY_DIRECTWRITE => Self::DirectWrite,
            SC_TECHNOLOGY_DIRECTWRITEDC => Self::DirectWriteDC,
            SC_TECHNOLOGY_DIRECTWRITERETAIN => Self::DirectWriteRetain,
            _ => Self::Default,
        }
    }
}

impl Into<u32> for Technology {
    fn into(self) -> u32 {
        match self {
            Self::Default => SC_TECHNOLOGY_DEFAULT,
            Self::DirectWrite => SC_TECHNOLOGY_DIRECTWRITE,
            Self::DirectWriteDC => SC_TECHNOLOGY_DIRECTWRITEDC,
            Self::DirectWriteRetain => SC_TECHNOLOGY_DIRECTWRITERETAIN,
        }
    }
}
