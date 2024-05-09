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

use crate::msaa::object::AccessibleObject;
use crate::uia::element::UiAutomationElement;
use std::fmt::{Debug, Formatter};
use windows::{
    core::{Interface, Result, BSTR},
    Win32::UI::Accessibility::{
        IUIAutomationLegacyIAccessiblePattern, UIA_LegacyIAccessiblePatternId,
    },
};

/**
 * 公开使 Microsoft UI 自动化客户端能够从 Microsoft Active Accessibility （MSAA） 服务器检索 UI 信息的方法和属性。
 * 此接口的获取方式与任何其他控制模式一样。它使 UI 自动化客户端能够利用缓存系统更有效地收集 MSAA 属性，还使 UI 自动化客户端能够与支持 IAccessible 接口的本机 Microsoft Active Accessibility 服务器进行交互。
 * */
pub struct UiAutomationIAccessiblePattern(IUIAutomationLegacyIAccessiblePattern);

/// <https://learn.microsoft.com/en-us/windows/win32/api/uiautomationclient/nn-uiautomationclient-iuiautomationlegacyiaccessiblepattern>
impl UiAutomationIAccessiblePattern {
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

    /**
     * 获取元素帮助。
     * */
    pub fn get_help(&self) -> String {
        unsafe { self.0.CurrentHelp() }
            .unwrap_or(BSTR::default())
            .to_string()
    }

    /**
     * 获取元素状态。
     * */
    pub fn get_state(&self) -> u32 {
        unsafe { self.0.CurrentState() }.unwrap_or(0)
    }

    /**
     * 获取元素的角色。
     * */
    pub fn get_role(&self) -> u32 {
        unsafe { self.0.CurrentRole() }.unwrap_or(0)
    }

    /**
     * 获取对应的MSAA对象。
     * */
    pub fn get_object(&self) -> Result<AccessibleObject> {
        match unsafe { self.0.GetIAccessible() } {
            Ok(o) => Ok(AccessibleObject::from_raw(
                o,
                unsafe { self.0.CurrentChildId() }.unwrap_or(0),
            )),
            Err(e) => Err(e),
        }
    }

    /**
     * 从UI元素获取此模式。
     * */
    pub fn obtain(value: &UiAutomationElement) -> Result<Self> {
        let pattern = unsafe {
            value
                .get_raw()
                .GetCurrentPattern(UIA_LegacyIAccessiblePatternId)
        };
        if let Err(e) = pattern {
            return Err(e);
        }
        let pattern = pattern
            .unwrap()
            .cast::<IUIAutomationLegacyIAccessiblePattern>()
            .unwrap();
        Ok(Self(pattern))
    }
}

impl Debug for UiAutomationIAccessiblePattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UiAutomationIAccessiblePattern(name:{}, description:{}, role:{})",
            self.get_name(),
            self.get_description(),
            self.get_role()
        )
    }
}
