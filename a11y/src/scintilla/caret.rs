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

use scintilla_sys::{SC_CARETSTICKY_OFF, SC_CARETSTICKY_ON, SC_CARETSTICKY_WHITESPACE};

#[derive(Debug, PartialEq)]
pub enum CaretSticky {
    /// 当设置为SC_CARETSTICKY_OFF（0）时，粘性标志关闭；当移动到不同的行时，所有文本更改（以及所有插入符号位置更改）都会记住插入符号的新水平位置。这是默认设置。
    Off,
    /// 当设置为SC_CARETSTICKY_ON（1）时，粘性标志打开，唯一能使编辑器记住水平插入符号位置的是用鼠标或键盘（左/右箭头键、行首/行尾键等）移动插入符号。
    On,
    /// 当设置为SC_CARETSTICKY_WHITESPACE（2）时，插入符号的行为类似于模式0（粘性关闭），除非在一种特殊情况下；插入空格或制表符时。（包括只粘贴空格/制表符——撤消、重做等不会表现出这种行为。）。
    WhiteSpace,
}

impl From<u32> for CaretSticky {
    fn from(value: u32) -> Self {
        match value {
            SC_CARETSTICKY_OFF => Self::Off,
            SC_CARETSTICKY_ON => Self::On,
            SC_CARETSTICKY_WHITESPACE => Self::WhiteSpace,
            _ => Self::Off,
        }
    }
}

impl Into<u32> for CaretSticky {
    fn into(self) -> u32 {
        match self {
            Self::Off => SC_CARETSTICKY_OFF,
            Self::On => SC_CARETSTICKY_ON,
            Self::WhiteSpace => SC_CARETSTICKY_WHITESPACE,
        }
    }
}
