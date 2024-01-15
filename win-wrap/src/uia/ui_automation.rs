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
    core::{implement, Result},
    Win32::{
        System::Com::{CoCreateInstance, CLSCTX_ALL},
        Foundation::HWND,
        UI::Accessibility::{
            CUIAutomation, IUIAutomation, IUIAutomationElement, IUIAutomationFocusChangedEventHandler,
            IUIAutomationFocusChangedEventHandler_Impl,
        }
    }
};
use crate::{
    uia::ui_element::UiAutomationElement,
    common::get_foreground_window,
    uia::ui_matcher::UiMatcher
};

/// UIAutomation接口本地封装
#[derive(Clone)]
pub struct UiAutomation(pub IUIAutomation);

impl UiAutomation {
    /**
     * 创建一个UiAutomation对象。
     * */
    pub fn new() -> Self {
        let automation =
            unsafe { CoCreateInstance::<_, IUIAutomation>(&CUIAutomation, None, CLSCTX_ALL) }
                .expect("Can't create the ui automation.");
        UiAutomation { 0: automation }
    }

    /**
     * 获取UI根元素。
     * */
    pub fn get_root_element(&self) -> Result<UiAutomationElement> {
        let el = unsafe { self.0.GetRootElement() }?;
        Ok((&el).into())
    }

    /// 根据窗口句柄获取ui元素
    pub fn get_element_from_hwnd(&self, hwnd: HWND) -> Result<UiAutomationElement> {
        let el = unsafe { self.0.ElementFromHandle(hwnd) }?;
        Ok((&el).into())
    }

    /// 获取前台窗口控件元素
    pub fn get_foreground_window_elements(&self) -> Vec<UiAutomationElement> {
        let mut result = Vec::new();

        if let Ok(element) = self.get_element_from_hwnd(get_foreground_window()) {
            let elements = UiMatcher::new(self).get_child_elements(&element);
            result.extend(elements);
        }

        result
    }

    /**
     * 注册一个焦点改变时的通知函数
     * 处理函数运行在单独的子线程中。
     * `func` 用于接收事件的函数。
     * */
    pub fn add_focus_changed_listener<CB>(&self, func: CB)
    where
        CB: Fn(UiAutomationElement) -> () + 'static,
    {
        let handler: IUIAutomationFocusChangedEventHandler =
            OnFocusChangedCallback::new(func).into();
        unsafe { self.0.AddFocusChangedEventHandler(None, &handler) }
            .expect("Can't add the focus changed listener.")
    }
}

unsafe impl Sync for UiAutomation {}

unsafe impl Send for UiAutomation {}

#[implement(IUIAutomationFocusChangedEventHandler)]
struct OnFocusChangedCallback<CB>(Box<CB>)
where
    CB: Fn(UiAutomationElement) -> () + 'static;

impl<CB> OnFocusChangedCallback<CB>
where
    CB: Fn(UiAutomationElement) -> () + 'static,
{
    fn new(func: CB) -> Self {
        OnFocusChangedCallback(Box::new(func))
    }
}

impl<CB> IUIAutomationFocusChangedEventHandler_Impl for OnFocusChangedCallback<CB>
where
    CB: Fn(UiAutomationElement) -> () + 'static,
{
    #[allow(non_snake_case)]
    fn HandleFocusChangedEvent(&self, sender: Option<&IUIAutomationElement>) -> Result<()> {
        let func = &*self.0;
        func(sender.unwrap().into());
        Ok(())
    }
}
