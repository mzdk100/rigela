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

use crate::uia::element::UiAutomationElement;
use std::fmt::{Debug, Formatter};
use windows::{
    core::{Interface, BSTR},
    Win32::UI::Accessibility::{IUIAutomationValuePattern, UIA_ValuePatternId},
};

/// ValuePattern
/// 提供对控件的访问，该控件包含一个值，该值不跨越范围，并且可以表示为字符串。此字符串可能是可编辑的，也可能是不可编辑的，具体取决于控件及其设置。
pub struct UiAutomationValuePattern(IUIAutomationValuePattern);

/// https://learn.microsoft.com/en-us/windows/win32/api/uiautomationclient/nn-uiautomationclient-iuiautomationvaluepattern
impl UiAutomationValuePattern {
    /// 从UI元素获取此模式。
    pub fn obtain(value: &UiAutomationElement) -> Result<Self, String> {
        let pattern = unsafe { value.get_raw().GetCurrentPattern(UIA_ValuePatternId) };
        if let Err(e) = pattern {
            return Err(format!("Can't get the ItemContainerPattern. ({})", e));
        }
        let pattern = pattern
            .unwrap()
            .cast::<IUIAutomationValuePattern>()
            .unwrap();
        Ok(Self(pattern))
    }

    /**
     * 查询元素的值。
     * */
    pub fn get_value(&self) -> Result<String, String> {
        let value = unsafe { self.0.CurrentValue() };
        match value {
            Ok(v) => Ok(v.to_string()),
            Err(ee) => Err(format!("Can't get the value. ({})", ee)),
        }
    }

    /**
     * 设置元素的值。
     * */
    pub fn set_value(&self, value: &str) -> Result<(), String> {
        let value = BSTR::from(value);
        let value = unsafe { self.0.SetValue(&value) };
        if let Err(e) = value {
            return Err(format!("Can't set the value. ({})", e));
        }
        Ok(())
    }

    /**
     * 判断元素的值是否为只读。
     * */
    pub fn is_readonly(&self) -> Result<bool, String> {
        let value = unsafe { self.0.CurrentIsReadOnly() };
        match value {
            Ok(v) => Ok(v.0 != 0),
            Err(ee) => Err(format!("Can't get the value. ({})", ee)),
        }
    }
}

impl Debug for UiAutomationValuePattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UiAutomationValuePattern()")
    }
}

unsafe impl Send for UiAutomationValuePattern {}

unsafe impl Sync for UiAutomationValuePattern {}
