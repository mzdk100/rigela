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

use crate::{
    common::Result,
    uia::{
        element::UiAutomationElement,
        event::{OnFocusChangedCallback, UiAutomationEventHandlerGroup},
    },
};
use std::sync::Arc;
use windows::Win32::{
    Foundation::{HWND, POINT},
    System::Com::{CoCreateInstance, CLSCTX_ALL},
    UI::Accessibility::{CUIAutomation8, IUIAutomation6, IUIAutomationFocusChangedEventHandler},
};

/// UIAutomation接口本地封装
#[derive(Clone, Debug)]
pub struct UiAutomation(Arc<IUIAutomation6>);

impl UiAutomation {
    /**
     * 创建一个UiAutomation对象。
     * */
    pub fn new() -> Self {
        let automation =
            unsafe { CoCreateInstance::<_, IUIAutomation6>(&CUIAutomation8, None, CLSCTX_ALL) }
                .expect("Can't create the ui automation.");
        UiAutomation {
            0: automation.into(),
        }
    }

    /**
     * 获取UI根元素。
     * */
    pub fn get_root_element(&self) -> UiAutomationElement {
        let el = unsafe { self.0.GetRootElement() }.expect("Can't get the root element.");
        UiAutomationElement::obtain(self.0.clone(), el)
    }

    /**
     * 获取UI焦点元素。
     * */
    pub fn get_focused_element(&self) -> UiAutomationElement {
        let el = unsafe { self.0.GetFocusedElement() }.expect("Can't get the root element.");
        UiAutomationElement::obtain(self.0.clone(), el)
    }

    /// 根据窗口句柄获取ui元素
    pub fn element_from_handle(&self, hwnd: HWND) -> Option<UiAutomationElement> {
        let el = unsafe { self.0.ElementFromHandle(hwnd) };
        if el.is_err() {
            return None;
        }
        Some(UiAutomationElement::obtain(self.0.clone(), el.unwrap()))
    }

    /// 根据坐标获取ui元素
    pub fn element_from_point(&self, x: i32, y: i32) -> Option<UiAutomationElement> {
        let el = unsafe {
            self.0
                .ElementFromPoint(POINT { x, y })
                .expect("Can't get the element from point.")
        };
        Some(UiAutomationElement::obtain(self.0.clone(), el))
    }

    /**
     * 创建事件处理器组。
     * */
    pub fn create_event_handler_group(&self) -> UiAutomationEventHandlerGroup {
        unsafe {
            UiAutomationEventHandlerGroup::obtain(
                self.0.clone(),
                &self.0.CreateEventHandlerGroup().unwrap(),
            )
        }
    }

    //noinspection StructuralWrap
    /**
     * 添加事件处理器组，以便于处理各种事件。
     * `element` 要监听的元素。
     * `group` 通过调用create_event_handler_group函数返回的事件处理器组。
     * */
    pub fn add_event_handler_group(
        &self,
        element: &UiAutomationElement,
        group: &UiAutomationEventHandlerGroup,
    ) -> Result<()> {
        unsafe {
            self.0
                .AddEventHandlerGroup(element.get_raw(), group.get_raw())
        }
    }

    /**
     * 注册一个焦点改变时的通知函数。
     * 处理函数运行在单独的子线程中。
     * `func` 用于接收事件的函数。
     * */
    pub fn add_focus_changed_listener<CB>(&self, func: CB)
    where
        CB: Fn(UiAutomationElement) -> () + 'static,
    {
        let handler: IUIAutomationFocusChangedEventHandler =
            OnFocusChangedCallback::new(func, self.0.clone()).into();
        unsafe { self.0.AddFocusChangedEventHandler(None, &handler) }
            .expect("Can't add the focus changed listener.")
    }

    /**
     * 移除已经注册的所有监听器。
     * */
    pub fn remove_all_event_listeners(&self) {
        unsafe { self.0.RemoveAllEventHandlers() }.unwrap_or(());
    }

    //noinspection StructuralWrap
    /**
     * 移除一个事件处理器组。
     * `group` 一个事件处理器组对象的引用。
     * */
    pub fn remove_event_handler_group(
        &self,
        element: &UiAutomationElement,
        group: &UiAutomationEventHandlerGroup,
    ) {
        unsafe {
            self.0
                .RemoveEventHandlerGroup(element.get_raw(), group.get_raw())
        }
        .unwrap_or(())
    }
}

unsafe impl Sync for UiAutomation {}

unsafe impl Send for UiAutomation {}
