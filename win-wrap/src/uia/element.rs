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
use std::sync::Arc;
use windows::{
    core::BSTR,
    Win32::UI::Accessibility::IUIAutomationElement
};
use windows::Win32::UI::Accessibility::{IUIAutomation, TreeScope_Children};

/// UiAutomationElement 的本地封装
#[derive(Clone)]
pub struct UiAutomationElement {
    _automation: Arc<IUIAutomation>,
    _current: IUIAutomationElement,
}

impl UiAutomationElement {
    /**
     * 获取原始的元素引用（不对外暴露）。
     * */
    pub(crate) fn get_raw(&self) -> &IUIAutomationElement {
        &self._current
    }

    pub(crate) fn obtain(automation: Arc<IUIAutomation>, element: IUIAutomationElement) -> Self {
        Self {
            _automation: automation,
            _current: element
        }
    }

    /**
     * 获取元素的当前名称。
     * */
    pub fn get_name(&self) -> String {
        unsafe { self._current.CurrentName() }
            // 不需要手动释放BSTR类型的指针，windows-rs已经对BSTR类型实现drop特征
            .unwrap_or(BSTR::new())
            .to_string()
    }

    /**
     * 获取本土化的控件类型描述。
     * */
    pub fn get_localized_control_type(&self) -> String {
        unsafe { self._current.CurrentLocalizedControlType() }
            .unwrap_or(BSTR::new())
            .to_string()
    }

    /// 获取子元素
    #[allow(unused_mut)]
    pub fn get_children(&self) -> Vec<UiAutomationElement> {
        let mut vec = Vec::new();
        let children = unsafe { self._current.FindAll(TreeScope_Children, &self._automation.CreateTrueCondition().unwrap()) }
            .unwrap();
        let len = unsafe { children.Length() }
            .unwrap();
        for i in 0..len {
            let c = unsafe { children.GetElement(i) }
                .unwrap();
            vec.push(UiAutomationElement::obtain(self._automation.clone(), c))
        }
        vec
    }

    /**
     * 获取元素的当前类名。
     * */
    #[allow(dead_code)]
    pub(crate) fn get_class_name(&self) -> String {
        unsafe { self._current.CurrentClassName() }
            .expect("Can't get the class name of element.")
            .to_string()
    }
}

unsafe impl Send for UiAutomationElement {}

unsafe impl Sync for UiAutomationElement {}

impl Display for UiAutomationElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UiAutomationElement: {}", self.get_name())
    }
}
