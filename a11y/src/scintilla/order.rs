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

use scintilla_sys::{SC_ORDER_CUSTOM, SC_ORDER_PERFORMSORT, SC_ORDER_PRESORTED};

#[derive(Debug, PartialEq)]
pub enum Order {
    /// 默认设置 SC_ORDER_PRESORTED (0) 要求列表按字母顺序排列。
    Presorted,
    /// Scintilla 可以对列表进行排序，而不是使用 SC_ORDER_PERFORMSORT (1) 的应用程序。这将花费更多时间。
    PerformSort,
    /// 希望优先考虑某些值并按优先级而不是字母顺序显示列表的应用程序可以使用 SC_ORDER_CUSTOM (2)。
    Custom,
}

impl From<u32> for Order {
    fn from(value: u32) -> Self {
        match value {
            SC_ORDER_PRESORTED => Self::Presorted,
            SC_ORDER_PERFORMSORT => Self::PerformSort,
            SC_ORDER_CUSTOM => Self::Custom,
            _ => Self::Presorted,
        }
    }
}

impl Into<u32> for Order {
    fn into(self) -> u32 {
        match self {
            Self::Presorted => SC_ORDER_PRESORTED,
            Self::PerformSort => SC_ORDER_PERFORMSORT,
            Self::Custom => SC_ORDER_CUSTOM,
        }
    }
}
