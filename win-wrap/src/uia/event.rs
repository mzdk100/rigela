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
    common::{beep, Result},
    uia::{element::UiAutomationElement, pattern::text::UiAutomationTextRange},
};
use std::sync::Arc;
use windows::{
    core::implement,
    Win32::{
        System::Com::SAFEARRAY,
        UI::Accessibility::{
            IUIAutomation6, IUIAutomationActiveTextPositionChangedEventHandler,
            IUIAutomationActiveTextPositionChangedEventHandler_Impl, IUIAutomationElement,
            IUIAutomationEventHandlerGroup, IUIAutomationFocusChangedEventHandler,
            IUIAutomationFocusChangedEventHandler_Impl,
            IUIAutomationTextEditTextChangedEventHandler,
            IUIAutomationTextEditTextChangedEventHandler_Impl, IUIAutomationTextRange,
            TextEditChangeType, TextEditChangeType_None, TreeScope_Subtree,
        },
    },
};

pub struct UiAutomationEventHandlerGroup {
    _automation: Arc<IUIAutomation6>,
    _group: IUIAutomationEventHandlerGroup,
}

impl UiAutomationEventHandlerGroup {
    pub(crate) fn obtain(
        automation: Arc<IUIAutomation6>,
        group: &IUIAutomationEventHandlerGroup,
    ) -> Self {
        Self {
            _automation: automation.clone(),
            _group: group.clone(),
        }
    }

    /**
     * 注册一个活动文本位置改变时的通知函数。
     * 处理函数运行在单独的子线程中。
     * `func` 用于接收事件的函数。
     * */
    pub fn add_active_text_position_changed_listener<CB>(&self, func: CB)
    where
        CB: Fn(UiAutomationElement, UiAutomationTextRange) -> () + 'static,
    {
        let handler: IUIAutomationActiveTextPositionChangedEventHandler =
            OnActiveTextPositionChangedCallback::new(func, self._automation.clone()).into();
        unsafe {
            self._group
                .AddActiveTextPositionChangedEventHandler(TreeScope_Subtree, None, &handler)
        }
        .expect("Can't add the active text position changed listener.");
    }

    /**
     * 注册一个文本编辑文字改变时的通知函数。
     * 处理函数运行在单独的子线程中。
     * `func` 用于接收事件的函数。
     * */
    pub fn add_text_edit_text_changed_listener<CB>(&self, func: CB)
    where
        CB: Fn(UiAutomationElement) -> () + 'static,
    {
        let handler: IUIAutomationTextEditTextChangedEventHandler =
            OnTextEditTextChangedCallback::new(func, self._automation.clone()).into();
        unsafe {
            self._group.AddTextEditTextChangedEventHandler(
                TreeScope_Subtree,
                TextEditChangeType_None,
                None,
                &handler,
            )
        }
        .expect("Can't add the active text position changed listener.");
    }
}

#[implement(IUIAutomationFocusChangedEventHandler)]
pub(crate) struct OnFocusChangedCallback<CB>
where
    CB: Fn(UiAutomationElement) -> () + 'static,
{
    _automation: Arc<IUIAutomation6>,
    _cb: Box<CB>,
}

impl<CB> OnFocusChangedCallback<CB>
where
    CB: Fn(UiAutomationElement) -> () + 'static,
{
    pub(crate) fn new(func: CB, automation: Arc<IUIAutomation6>) -> Self {
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
    fn HandleFocusChangedEvent(
        &self,
        sender: Option<&IUIAutomationElement>,
    ) -> crate::common::Result<()> {
        let func = &*self._cb;
        func(UiAutomationElement::obtain(
            self._automation.clone(),
            sender.unwrap().clone(),
        ));
        Ok(())
    }
}

#[implement(IUIAutomationActiveTextPositionChangedEventHandler)]
struct OnActiveTextPositionChangedCallback<CB>
where
    CB: Fn(UiAutomationElement, UiAutomationTextRange) -> () + 'static,
{
    _automation: Arc<IUIAutomation6>,
    _cb: Box<CB>,
}

impl<CB> OnActiveTextPositionChangedCallback<CB>
where
    CB: Fn(UiAutomationElement, UiAutomationTextRange) -> () + 'static,
{
    fn new(func: CB, automation: Arc<IUIAutomation6>) -> Self {
        Self {
            _automation: automation,
            _cb: func.into(),
        }
    }
}

impl<CB> IUIAutomationActiveTextPositionChangedEventHandler_Impl
    for OnActiveTextPositionChangedCallback<CB>
where
    CB: Fn(UiAutomationElement, UiAutomationTextRange) -> () + 'static,
{
    #[allow(non_snake_case)]
    fn HandleActiveTextPositionChangedEvent(
        &self,
        sender: Option<&IUIAutomationElement>,
        range: Option<&IUIAutomationTextRange>,
    ) -> crate::common::Result<()> {
        let func = &*self._cb;
        let element =
            UiAutomationElement::obtain(self._automation.clone(), sender.unwrap().clone());
        let range = UiAutomationTextRange::obtain(range.unwrap());
        func(element, range);
        Ok(())
    }
}

#[implement(IUIAutomationTextEditTextChangedEventHandler)]
struct OnTextEditTextChangedCallback<CB>
where
    CB: Fn(UiAutomationElement) -> () + 'static,
{
    _automation: Arc<IUIAutomation6>,
    _cb: Box<CB>,
}

impl<CB> OnTextEditTextChangedCallback<CB>
where
    CB: Fn(UiAutomationElement) -> () + 'static,
{
    fn new(func: CB, automation: Arc<IUIAutomation6>) -> Self {
        Self {
            _automation: automation,
            _cb: func.into(),
        }
    }
}

impl<CB> IUIAutomationTextEditTextChangedEventHandler_Impl for OnTextEditTextChangedCallback<CB>
where
    CB: Fn(UiAutomationElement) -> () + 'static,
{
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    fn HandleTextEditTextChangedEvent(
        &self,
        sender: Option<&IUIAutomationElement>,
        text_edit_change_type: TextEditChangeType,
        event_strings: *const SAFEARRAY,
    ) -> Result<()> {
        beep(400, 40);
        Ok(())
    }
}
