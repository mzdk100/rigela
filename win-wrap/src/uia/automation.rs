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
use std::sync::Arc;
use windows::{
    core::{implement, Result},
    Win32::{
        Foundation::HWND,
        Foundation::POINT,
        System::Com::{CoCreateInstance, CLSCTX_ALL},
        UI::Accessibility::{
            CUIAutomation, IUIAutomation, IUIAutomationElement,
            IUIAutomationFocusChangedEventHandler, IUIAutomationFocusChangedEventHandler_Impl,
        },
    },
};

/// UIAutomation接口本地封装
#[derive(Clone, Debug)]
pub struct UiAutomation(Arc<IUIAutomation>);

impl UiAutomation {
    /**
     * 创建一个UiAutomation对象。
     * */
    pub fn new() -> Self {
        let automation =
            unsafe { CoCreateInstance::<_, IUIAutomation>(&CUIAutomation, None, CLSCTX_ALL) }
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
     * 注册一个焦点改变时的通知函数
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
}

unsafe impl Sync for UiAutomation {}

unsafe impl Send for UiAutomation {}

#[implement(IUIAutomationFocusChangedEventHandler)]
struct OnFocusChangedCallback<CB>
where
    CB: Fn(UiAutomationElement) -> () + 'static,
{
    _automation: Arc<IUIAutomation>,
    _cb: Box<CB>,
}

impl<CB> OnFocusChangedCallback<CB>
where
    CB: Fn(UiAutomationElement) -> () + 'static,
{
    fn new(func: CB, automation: Arc<IUIAutomation>) -> Self {
        Self {
            _automation: automation,
            _cb: func.into(),
        }
    }
}

impl<CB> IUIAutomationFocusChangedEventHandler_Impl for OnFocusChangedCallback<CB>
where
    CB: Fn(UiAutomationElement) -> () + 'static,
{
    #[allow(non_snake_case)]
    fn HandleFocusChangedEvent(&self, sender: Option<&IUIAutomationElement>) -> Result<()> {
        let func = &*self._cb;
        func(UiAutomationElement::obtain(
            self._automation.clone(),
            sender.unwrap().clone(),
        ));
        Ok(())
    }
}
