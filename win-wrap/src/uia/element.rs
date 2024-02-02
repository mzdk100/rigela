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
    Win32::{
        Foundation::RECT,
        UI::Accessibility::{IUIAutomation6, IUIAutomationElement, TreeScope_Children},
    },
};

/// UiAutomationElement 的本地封装
#[derive(Clone)]
pub struct UiAutomationElement {
    _automation: Arc<IUIAutomation6>,
    _current: IUIAutomationElement,
}

impl UiAutomationElement {
    /**
     * 获取原始的元素引用（不对外暴露）。
     * */
    pub(crate) fn get_raw(&self) -> &IUIAutomationElement {
        &self._current
    }

    pub(crate) fn obtain(automation: Arc<IUIAutomation6>, element: IUIAutomationElement) -> Self {
        Self {
            _automation: automation,
            _current: element,
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

    /**
     * 获取子元素数量。
     * */
    pub fn get_child_count(&self) -> i32 {
        if let Ok(children) = unsafe {
            self._current.FindAll(
                TreeScope_Children,
                &self._automation.CreateTrueCondition().unwrap(),
            )
        } {
            return unsafe { children.Length() }.unwrap();
        }
        0
    }

    /**
     * 获取子元素。
     * `index` 序号。
     * */
    pub fn get_child(&self, index: i32) -> Option<UiAutomationElement> {
        if let Ok(children) = unsafe {
            self._current.FindAll(
                TreeScope_Children,
                &self._automation.CreateTrueCondition().unwrap(),
            )
        } {
            if let Ok(el) = unsafe { children.GetElement(index) } {
                return Some(UiAutomationElement::obtain(self._automation.clone(), el));
            }
        }
        None
    }

    /// 获取元素的坐标
    pub fn get_rect(&self) -> RECT {
        unsafe { self._current.CurrentBoundingRectangle() }
            .expect("Can't get the location of element.")
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
