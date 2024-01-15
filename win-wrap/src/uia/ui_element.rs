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

use std::fmt::{Display, Formatter};
use windows::{
    core::BSTR,
    Win32::UI::Accessibility::IUIAutomationElement
};

/// UiAutomationElement 的本地封装
#[derive(Clone)]
pub struct UiAutomationElement(pub IUIAutomationElement);

impl UiAutomationElement {
    /**
     * 获取元素的当前名称。
     * */
    pub fn get_name(&self) -> String {
        unsafe { self.0.CurrentName() }
            // 不需要手动释放BSTR类型的指针，windows-rs已经对BSTR类型实现drop特征
            .unwrap_or(BSTR::new())
            .to_string()
    }

    /**
     * 获取本土化的控件类型描述。
     * */
    pub fn get_localized_control_type(&self) -> String {
        unsafe { self.0.CurrentLocalizedControlType() }
            .unwrap_or(BSTR::new())
            .to_string()
    }

    /**
     * 获取元素的当前类名。
     * */
    #[allow(dead_code)]
    pub(crate) fn get_class_name(&self) -> String {
        unsafe { self.0.CurrentClassName() }
            .expect("Can't get the class name of element.")
            .to_string()
    }
}

impl From<&IUIAutomationElement> for UiAutomationElement {
    fn from(el: &IUIAutomationElement) -> Self {
        UiAutomationElement(el.clone())
    }
}

unsafe impl Send for UiAutomationElement {}
unsafe impl Sync for UiAutomationElement {}

impl Display for UiAutomationElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UiAutomationElement: {}", self.get_name())
    }
}
