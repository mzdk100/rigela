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


use windows::{
    core::{BSTR, ComInterface},
    Win32::UI::Accessibility::{IUIAutomationLegacyIAccessiblePattern, UIA_LegacyIAccessiblePatternId}
};
use crate::uia::element::UiAutomationElement;

pub struct UiAutomationLegacyIAccessiblePattern(IUIAutomationLegacyIAccessiblePattern);

impl UiAutomationLegacyIAccessiblePattern {
    /**
     * 获取元素名称。
     * */
    pub fn get_name(&self) -> String {
        unsafe { self.0.CurrentName() }
            .unwrap_or(BSTR::default())
            .to_string()
    }

    /**
     * 获取元素描述。
     * */
    pub fn get_description(&self) -> String {
        unsafe { self.0.CurrentDescription() }
            .unwrap_or(BSTR::default())
            .to_string()
    }
}

impl From<&UiAutomationElement> for UiAutomationLegacyIAccessiblePattern {
    fn from(value: &UiAutomationElement) -> Self {
        let pattern = unsafe { value.get_raw().GetCurrentPattern(UIA_LegacyIAccessiblePatternId) }
            .expect("Can't get the LegacyIAccessiblePattern.")
            .cast::<IUIAutomationLegacyIAccessiblePattern>()
            .unwrap();
        Self(pattern)
    }
}