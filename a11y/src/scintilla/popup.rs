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

use scintilla_sys::{SC_POPUP_ALL, SC_POPUP_NEVER, SC_POPUP_TEXT};

#[derive(Debug, PartialEq)]
pub enum PopUpMode {
    /// 从不显示默认编辑菜单。
    Never,
    /// 如果单击 scintilla，则显示默认编辑菜单。
    All,
    /// 仅当单击文本区域时才显示默认编辑菜单。
    Text,
}

impl From<u32> for PopUpMode {
    fn from(value: u32) -> Self {
        match value {
            SC_POPUP_NEVER => Self::Never,
            SC_POPUP_ALL => Self::All,
            SC_POPUP_TEXT => Self::Text,
            _ => Self::Never,
        }
    }
}

impl Into<u32> for PopUpMode {
    fn into(self) -> u32 {
        match self {
            Self::Never => SC_POPUP_NEVER,
            Self::All => SC_POPUP_ALL,
            Self::Text => SC_POPUP_TEXT,
        }
    }
}
