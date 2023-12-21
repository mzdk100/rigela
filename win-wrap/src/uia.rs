/*
 * Copyright (c) 2023. The RigelA open source project team and
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
use windows::core::{implement, Result};
use windows::Win32::System::Com::{CLSCTX_ALL, CoCreateInstance};
use windows::Win32::UI::Accessibility::{CUIAutomation, IUIAutomation, IUIAutomationElement, IUIAutomationFocusChangedEventHandler, IUIAutomationFocusChangedEventHandler_Impl};
#[implement(IUIAutomationFocusChangedEventHandler)]
struct OnFocusChangedCallback<CB>(Box<CB>) where CB: Fn(UiAutomationElement) -> () + 'static;
impl<CB> OnFocusChangedCallback<CB>
    where CB: Fn(UiAutomationElement) -> () + 'static {
    fn new(func: CB) -> Self {
        OnFocusChangedCallback(Box::new(func))
    }
}
impl<CB> IUIAutomationFocusChangedEventHandler_Impl for OnFocusChangedCallback<CB>
    where CB: Fn(UiAutomationElement) -> () + 'static {
    #[allow(non_snake_case)]
    fn HandleFocusChangedEvent(&self, sender: Option<&IUIAutomationElement>) -> Result<()> {
        let func = &*self.0;
        func(UiAutomationElement::from(sender.unwrap()));
        Ok(())
    }
}
pub struct UiAutomation(IUIAutomation);
unsafe impl Sync for UiAutomation {}
unsafe impl Send for UiAutomation {}
#[derive(Clone)]
pub struct UiAutomationElement(IUIAutomationElement);
impl UiAutomation {
    /**
     * 获取UI根元素。
     * */
    pub fn get_root_element(&self) -> UiAutomationElement {
        let el = unsafe { self.0.GetRootElement() }
            .expect("Can't get the root element.");
        UiAutomationElement::from(&el)
    }

    /**
     * 创建一个UiAutomation对象。
     * */
    pub fn new() -> Self {
        let automation = unsafe { CoCreateInstance::<_, IUIAutomation>(&CUIAutomation, None, CLSCTX_ALL) }
            .expect("Can't create the ui automation.");
        UiAutomation {
            0: automation
        }
    }

    /**
     * 注册一个焦点改变时的通知函数
     * 处理函数运行在单独的子线程中。
     * `func` 用于接收事件的函数。
     * */
    pub fn add_focus_changed_listener<CB>(&self, func: CB)
        where CB: Fn(UiAutomationElement) -> () + 'static {
        let handler: IUIAutomationFocusChangedEventHandler = OnFocusChangedCallback::new(func).into();
        unsafe { self.0.AddFocusChangedEventHandler(None, &handler) }
            .expect("Can't add the focus changed listener.")
    }
}
impl UiAutomationElement {
    fn from(el: &IUIAutomationElement) -> Self {
        UiAutomationElement(el.clone())
    }

    /**
     * 获取元素的当前名称。
     * */
    pub fn get_name(&self) -> String {
        unsafe { self.0.CurrentName() }
            // 不需要手动释放BSTR类型的指针，windows-rs已经对BSTR类型实现drop特征
            .expect("Can't get the element name.")
            .to_string()
    }

    /**
     * 获取元素的当前类名。
     * */
    pub(crate) fn get_class_name(&self) -> String {
        unsafe { self.0.CurrentClassName() }
            .expect("Can't get the class name of element.")
            .to_string()
    }

    /**
     * 获取本土化的控件类型描述。
     * */
    pub fn get_localized_control_type(&self) -> String {
        unsafe { self.0.CurrentLocalizedControlType() }
            .unwrap()
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
