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

use std::{
    fmt::{Debug, Formatter},
    sync::Weak,
};

use windows::{
    core::Interface,
    Win32::UI::Accessibility::{
        IUIAutomation6, IUIAutomationTableItemPattern, IUIAutomationTablePattern,
        RowOrColumnMajor_ColumnMajor, RowOrColumnMajor_Indeterminate, RowOrColumnMajor_RowMajor,
        UIA_TableItemPatternId, UIA_TablePatternId,
    },
};

use crate::{ext::VecExt, uia::element::UiAutomationElement};

/**
 * 提供对控件的访问，该控件充当子元素集合的容器。此元素的子元素支持 UiAutomationTableItemPattern，并按行和列遍历的二维逻辑坐标系进行组织。
 * */
pub struct UiAutomationTablePattern {
    _automation: Weak<IUIAutomation6>,
    _pattern: IUIAutomationTablePattern,
}

/// https://learn.microsoft.com/en-us/windows/win32/api/uiautomationclient/nn-uiautomationclient-iuiautomationtablepattern
impl UiAutomationTablePattern {
    /**
     * 从UI元素获取此模式。
     * */
    pub fn obtain(value: &UiAutomationElement) -> Result<Self, String> {
        let pattern = unsafe { value.get_raw().GetCurrentPattern(UIA_TablePatternId) };
        if let Err(e) = pattern {
            return Err(format!("Can't get the TablePattern. ({})", e));
        }
        let pattern = pattern
            .unwrap()
            .cast::<IUIAutomationTablePattern>()
            .unwrap();
        Ok(Self {
            _automation: value.get_aut(),
            _pattern: pattern,
        })
    }

    /**
     * 查询表的主要遍历方向。此属性是只读的。
     * */
    #[allow(non_upper_case_globals)]
    pub fn row_or_column_major(&self) -> RowOrColumnMajor {
        unsafe {
            match self._pattern.CurrentRowOrColumnMajor() {
                Ok(x) => match x {
                    RowOrColumnMajor_ColumnMajor => RowOrColumnMajor::Column,
                    RowOrColumnMajor_Indeterminate => RowOrColumnMajor::Indeterminate,
                    RowOrColumnMajor_RowMajor => RowOrColumnMajor::Row,
                    _ => RowOrColumnMajor::None,
                },
                Err(_) => RowOrColumnMajor::None,
            }
        }
    }

    /**
     * 查询表示表中所有列标题的 UI 自动化元素的集合。
     * */
    pub fn column_headers(&self) -> Vec<UiAutomationElement> {
        unsafe {
            match self._pattern.GetCurrentColumnHeaders() {
                Ok(x) => x
                    .to_vec()
                    .iter()
                    .map(|x| UiAutomationElement::obtain(self._automation.clone(), x.clone()))
                    .collect(),
                Err(_) => vec![],
            }
        }
    }
}

pub enum RowOrColumnMajor {
    Row,
    Column,
    Indeterminate,
    None,
}

/// 提供对支持 UiAutomationTablePattern 的容器中的子元素的访问。
/// 支持此接口的元素还必须支持 UiAutomationGridItemPattern，以提供不特定于表的属性。
pub struct UiAutomationTableItemPattern(Weak<IUIAutomation6>, IUIAutomationTableItemPattern);

/// https://learn.microsoft.com/en-us/windows/win32/api/uiautomationclient/nn-uiautomationclient-iuiautomationtableitempattern
impl UiAutomationTableItemPattern {
    /**
     * 从UI元素获取此模式。
     * */
    pub fn obtain(value: &UiAutomationElement) -> Result<Self, String> {
        let pattern = unsafe { value.get_raw().GetCurrentPattern(UIA_TableItemPatternId) };
        if let Err(e) = pattern {
            return Err(format!("Can't get the TableItemPattern. ({})", e));
        }
        let pattern = pattern
            .unwrap()
            .cast::<IUIAutomationTableItemPattern>()
            .unwrap();
        Ok(Self(value.get_aut(), pattern))
    }

    /**
     * 查询与表项或单元格关联的列标题。
     * */
    pub fn get_column_header_items(&self) -> Option<Vec<UiAutomationElement>> {
        unsafe {
            if let Ok(arr) = self.1.GetCurrentColumnHeaderItems() {
                return Some(
                    arr.to_vec()
                        .iter()
                        .map(|i| UiAutomationElement::obtain(self.0.clone(), i.clone()))
                        .collect(),
                );
            }
            None
        }
    }

    /**
     * 查询与表项或单元格关联的行标题。
     * */
    pub fn get_row_header_items(&self) -> Option<Vec<UiAutomationElement>> {
        unsafe {
            if let Ok(arr) = self.1.GetCurrentRowHeaderItems() {
                return Some(
                    arr.to_vec()
                        .iter()
                        .map(|i| UiAutomationElement::obtain(self.0.clone(), i.clone()))
                        .collect(),
                );
            }
            None
        }
    }
}

impl Debug for UiAutomationTableItemPattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UiAutomationTableItemPattern()")
    }
}
