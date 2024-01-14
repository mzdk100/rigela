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

use crate::uia::{UiAutomation, UiAutomationElement};
use windows::Win32::UI::Accessibility::{IUIAutomation, TreeScope};

/// UIA 元素匹配器
pub struct UIMatcher<'a> {
    ui_automation: Box<&'a IUIAutomation>,
}

impl<'a> UIMatcher<'a> {
    pub fn new(ui_automation: &'a UiAutomation) -> Self {
        UIMatcher {
            ui_automation: Box::new(&ui_automation.0),
        }
    }

    /// 获取元素的子元素。
    pub fn get_child_elements(&self, element: &UiAutomationElement) -> Vec<UiAutomationElement> {
        let mut elements = Vec::new();

        let condition = unsafe { self.ui_automation.CreateTrueCondition() }.unwrap();
        let children = unsafe { element.0.FindAll(TreeScope(2), &condition) }.unwrap();

        let len = unsafe { children.Length() }.unwrap();
        for i in 0..len {
            let children = unsafe { children.GetElement(i) }.unwrap();
            elements.push(UiAutomationElement(children));
        }

        elements
    }
}
