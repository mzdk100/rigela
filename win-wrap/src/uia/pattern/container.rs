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
    core::ComInterface,
    Win32::UI::Accessibility::{IUIAutomationItemContainerPattern, UIA_ItemContainerPatternId},
};

/// https://learn.microsoft.com/en-us/windows/win32/api/uiautomationclient/nn-uiautomationclient-iuiautomationitemcontainerpattern
pub struct UiAutomationItemContainerPattern(IUIAutomationItemContainerPattern);

impl UiAutomationItemContainerPattern {
    /**
     * 从UI元素获取此模式。
     * */
    pub fn obtain(value: &UiAutomationElement) -> Result<Self, String> {
        let pattern = unsafe {
            value
                .get_raw()
                .GetCurrentPattern(UIA_ItemContainerPatternId)
        };
        if let Err(e) = pattern {
            return Err(format!("Can't get the ItemContainerPattern. ({})", e));
        }
        let pattern = pattern
            .unwrap()
            .cast::<IUIAutomationItemContainerPattern>()
            .unwrap();
        Ok(Self(pattern))
    }
}

impl Debug for UiAutomationItemContainerPattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UiAutomationItemContainerPattern()")
    }
}
