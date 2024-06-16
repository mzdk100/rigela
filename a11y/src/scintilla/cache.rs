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

use scintilla_sys::{SC_CACHE_CARET, SC_CACHE_DOCUMENT, SC_CACHE_NONE, SC_CACHE_PAGE};

/// 行缓存样式
#[derive(Debug, PartialEq)]
pub enum CacheMode {
    /// 没有缓存任何行。
    None,
    /// 缓存一行。这是默认设置。
    Caret,
    /// 可见行加上包含插入点的行。
    Page,
    /// 文档中的所有行。
    Document,
}

impl From<u32> for CacheMode {
    fn from(value: u32) -> Self {
        match value {
            SC_CACHE_NONE => Self::None,
            SC_CACHE_CARET => Self::Caret,
            SC_CACHE_PAGE => Self::Page,
            SC_CACHE_DOCUMENT => Self::Document,
            _ => Self::None,
        }
    }
}

impl Into<u32> for CacheMode {
    fn into(self) -> u32 {
        match self {
            Self::None => SC_CACHE_NONE,
            Self::Caret => SC_CACHE_CARET,
            Self::Page => SC_CACHE_PAGE,
            Self::Document => SC_CACHE_DOCUMENT,
        }
    }
}
