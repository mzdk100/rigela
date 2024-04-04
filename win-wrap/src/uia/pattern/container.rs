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
    core::{Interface, VARIANT},
    Win32::UI::Accessibility::{
        IUIAutomation6, IUIAutomationItemContainerPattern, UIA_ItemContainerPatternId,
    },
};

use crate::uia::{element::UiAutomationElement, property::UiaPropertyId};

//noinspection StructuralWrap
/**
 * 公开从容器（如虚拟列表）检索项的方法。
 * 此接口不限于虚拟化容器使用。任何可以实现高效名称查找的容器都可以支持此控件模式，使客户端能够比使用find_first等方法更快地查找名称，后者必须遍历Microsoft UI自动化树。
 * */
pub struct UiAutomationItemContainerPattern(
    Weak<IUIAutomation6>,
    IUIAutomationItemContainerPattern,
);

//noinspection StructuralWrap
/// <https://learn.microsoft.com/en-us/windows/win32/api/uiautomationclient/nn-uiautomationclient-iuiautomationitemcontainerpattern>
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
        Ok(Self(value.get_aut(), pattern))
    }

    /*
     * 根据指定的属性值检索包含元素中的元素。
     * 如果匹配元素被虚拟化，则提供程序可以返回实际的UiAutomationElement接口或占位符。
     * 如果请求的属性不是容器支持搜索的属性，则此方法返回E_INVALIDARG。预计大多数容器都将支持Name属性，如果适用于容器，则支持AutomationId和IsSelected。
     * 此方法可能很慢，因为它可能需要遍历多个对象才能找到匹配的对象。当在循环中用于返回多个项目时，只要每个项目只返回一次（即循环应终止），就不会定义特定的顺序。此方法也是以项目为中心，而不是以 UI 为中心，因此具有多个 UI 表示形式的项目只需点击一次。
     * 当 property_id 参数指定为 0（零）时，提供程序应返回 start_after 之后的下一项。如果将 start_after 指定为 None 且 property_id 为 0，则提供程序应返回容器中的第一项。当 property_id 指定为 0 时，value 参数应为 VT_EMPTY。
     * `start_after` 指向搜索开始的元素，或None搜索所有元素。
     * `property_id` 属性标识符。有关属性 ID 的列表，请参阅属性标识符。
     * `value` 属性值。
     * */
    pub fn find_item_by_property<T>(
        &self,
        start_after: Option<&UiAutomationElement>,
        property_id: UiaPropertyId,
        value: T,
    ) -> Option<UiAutomationElement>
        where
            VARIANT: From<T>,
    {
        unsafe {
            let Ok(r) = (match start_after {
                None => self.1.FindItemByProperty(
                    None,
                    property_id.into(),
                    &VARIANT::try_from(value).unwrap(),
                ),
                Some(x) => self.1.FindItemByProperty(
                    x.get_raw(),
                    property_id.into(),
                    &VARIANT::try_from(value).unwrap(),
                ),
            }) else {
                return None;
            };
            Some(UiAutomationElement::obtain(self.0.clone(), r))
        }
    }
}

impl Debug for UiAutomationItemContainerPattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UiAutomationItemContainerPattern()")
    }
}
