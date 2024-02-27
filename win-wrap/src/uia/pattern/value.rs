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
use windows::core::{ComInterface, BSTR};
use windows::Win32::UI::Accessibility::{IUIAutomationValuePattern, UIA_ValuePatternId};

/// ValuePattern
pub struct UiAutomationValuePattern(IUIAutomationValuePattern);

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

    pub fn get_value(&self) -> Result<String, String> {
        let value = unsafe { self.0.CurrentValue() };
        match value {
            Ok(v) => Ok(v.to_string()),
            Err(ee) => Err(format!("Can't get the ItemContainerPattern. ({})", ee)),
        }
    }

    pub fn set_value(&self, value: &str) -> Result<(), String> {
        let value = BSTR::from(value);
        let value = unsafe { self.0.SetValue(&value) };
        if let Err(e) = value {
            return Err(format!("Can't get the ItemContainerPattern. ({})", e));
        }
        Ok(())
    }

    pub fn is_readonly(&self) -> Result<bool, String> {
        let value = unsafe { self.0.CurrentIsReadOnly() };
        match value {
            Ok(v) => Ok(v.0 != 0),
            Err(ee) => Err(format!("Can't get the ItemContainerPattern. ({})", ee)),
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
