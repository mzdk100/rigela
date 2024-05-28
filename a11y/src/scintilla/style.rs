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
    SC_CASE_CAMEL, SC_CASE_LOWER, SC_CASE_MIXED, SC_CASE_UPPER, SC_IDLESTYLING_AFTERVISIBLE,
    SC_IDLESTYLING_ALL, SC_IDLESTYLING_NONE, SC_IDLESTYLING_TOVISIBLE,
};

/// 虽然一些样式设置消息会更改与文本相关联的样式号，但这些消息定义了如何直观地解释这些样式号。可以设置256种lexer样式，编号为0到STYLE_MAX（255）。还有一些从32开始的预定义编号样式，定义了以下STYLE_*常量。
pub use scintilla_sys::{
    STYLE_BRACEBAD, // 此样式设置在使用SCI_BRACEBADLIGHT消息标记不匹配的大括号时使用的显示属性。
    STYLE_BRACELIGHT, // 此样式设置使用SCI_BRACEHIGHLIGHT消息高亮显示大括号时以及使用SCI_SETHIGHLIGUITE高亮显示相应缩进时使用的属性。
    STYLE_CALLTIP, // 调用提示通常使用STYLE_DEFAULT定义的字体属性。SCI_CALLTIUSESTYLE的使用会导致调用提示改用此样式。仅使用字体名称、字体大小、前景和背景颜色以及字符集属性。
    STYLE_CONTROLCHAR, // 此样式设置绘制控制字符时使用的字体。仅使用字体、大小、粗体、斜体和字符集属性，而不使用颜色属性。另请参见：SCI_SETCONTROLCHARSSYMBOL。
    STYLE_DEFAULT,     // 此样式定义使用SCI_STYLECLEARALL消息时所有样式接收的属性。
    STYLE_FOLDDISPLAYTEXT, // 这是用于绘制附着到折叠文字的文字标记的样式。
    STYLE_INDENTGUIDE, // 此样式设置绘制缩进辅助线时使用的前景色和背景色。
    STYLE_LASTPREDEFINED, // 为了使客户端代码更容易发现预定义的样式范围，将其设置为上一个预定义样式的样式号。
    STYLE_LINENUMBER, // 此样式设置用于在行号边距中显示行号的文本的属性。此样式的背景色设置还为没有设置任何折叠掩码位的所有页边距设置背景色。也就是说，掩码&SC_MASK_FOLDERS为0的任何边距。有关遮罩的详细信息，请参见SCI_SETMARGINMASKN。
    STYLE_MAX, // 这不是样式，而是可以设置的最大样式数。可以使用介于STYLE_LASTPREDEFINED和STYLE_MAX之间的样式。
};

/*
对于每种样式，您可以设置字体名称、大小、粗体、斜体和下划线的使用、前景和背景色以及字符集。您也可以选择隐藏具有给定样式的文本，将所有字符显示为大写或小写，并从行上的最后一个字符填充到某行的末尾（对于嵌入式语言）。还有一个实验属性可以使文本只读。
如何使用样式完全取决于您自己。如果要使用语法着色，可以使用样式0表示空白，样式1表示数字，样式2表示关键字，样式3表示字符串，样式4表示预处理器，样式5表示运算符，等等。
*/

#[derive(Debug, PartialEq)]
pub enum IdleStyling {
    /// 默认情况下，SC_IDLESTYLING_NONE（0），在显示当前可见的所有文本之前对其执行语法样式设置。对于非常大的文件，这可能会使向下滚动速度变慢。
    None,
    /// 对于SC_IDLESTYLING_TOVISIBLE（1），在显示之前执行少量造型，然后在后台作为空闲时间任务逐步执行进一步的造型。这可能会导致文本最初看起来没有颜色，然后一段时间后，它被着色了。
    ToVisible,
    /// 当前可见部分之后的文本可以在背景中使用SC_IDLESTYLING_AFTERVISIBLE（2）进行样式化。
    AfterVisible,
    /// 要设置背景中可见文本前后的样式，请使用SC_IDLESTYLING_ALL（3）。
    All,
}

impl From<u32> for IdleStyling {
    fn from(value: u32) -> Self {
        match value {
            SC_IDLESTYLING_NONE => Self::None,
            SC_IDLESTYLING_TOVISIBLE => Self::ToVisible,
            SC_IDLESTYLING_AFTERVISIBLE => Self::AfterVisible,
            SC_IDLESTYLING_ALL => Self::All,
            _ => Self::None,
        }
    }
}

impl Into<u32> for IdleStyling {
    fn into(self) -> u32 {
        match self {
            Self::None => SC_IDLESTYLING_NONE,
            Self::ToVisible => SC_IDLESTYLING_TOVISIBLE,
            Self::AfterVisible => SC_IDLESTYLING_AFTERVISIBLE,
            Self::All => SC_IDLESTYLING_ALL,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Case {
    /// 大写（SC_CASE_UPER，1）
    Upper,
    /// 小写（SC_CASE_LOWER，2）
    Lower,
    /// 驼峰（SC_CASE_CAMEL，3）
    Camel,
    /// 正常显示（SC_CASE_MIXED，0）
    Mixed,
}

impl From<u32> for Case {
    fn from(value: u32) -> Self {
        match value {
            SC_CASE_MIXED => Self::Mixed,
            SC_CASE_UPPER => Self::Upper,
            SC_CASE_LOWER => Self::Lower,
            SC_CASE_CAMEL => Self::Camel,
            _ => Self::Mixed,
        }
    }
}

impl Into<u32> for Case {
    fn into(self) -> u32 {
        match self {
            Self::Upper => SC_CASE_UPPER,
            Self::Lower => SC_CASE_LOWER,
            Self::Camel => SC_CASE_CAMEL,
            Self::Mixed => SC_CASE_MIXED,
        }
    }
}
