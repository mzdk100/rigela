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
use std::fmt::{Debug, Formatter};
use windows::{
    core::Interface,
    Win32::UI::Accessibility::{
        IUIAutomationTogglePattern, ToggleState_Indeterminate, ToggleState_Off, ToggleState_On,
        UIA_TogglePatternId,
    },
};

/**
 * 提供对控件的访问，该控件可以在一组状态之间循环，并在设置状态后保持状态。
 * */
pub struct UiAutomationTogglePattern(IUIAutomationTogglePattern);

/// <https://learn.microsoft.com/en-us/windows/win32/api/uiautomationclient/nn-uiautomationclient-iuiautomationtogglepattern>
impl UiAutomationTogglePattern {
    /**
     * 从UI元素获取此模式。
     * */
    pub fn obtain(value: &UiAutomationElement) -> Result<Self, String> {
        let pattern = unsafe { value.get_raw().GetCurrentPattern(UIA_TogglePatternId) };
        if let Err(e) = pattern {
            return Err(format!("Can't get the TogglePattern. ({})", e));
        }
        let pattern = pattern
            .unwrap()
            .cast::<IUIAutomationTogglePattern>()
            .unwrap();
        Ok(Self(pattern))
    }

    /**
     * 检索控件的状态。
     * */
    #[allow(non_upper_case_globals)]
    pub fn get_toggle_state(&self) -> ToggleState {
        let state = unsafe { self.0.CurrentToggleState() };
        if state.is_err() {
            return ToggleState::Indeterminate;
        }
        match state.unwrap() {
            ToggleState_On => ToggleState::On,
            ToggleState_Off => ToggleState::Off,
            ToggleState_Indeterminate => ToggleState::Indeterminate,
            _ => ToggleState::Indeterminate,
        }
    }

    /**
     * 在控件的切换状态之间循环。
     * 控件按以下顺序循环其状态：ToggleState::On、ToggleState::Off，以及（如果支持）ToggleState::Indentine。
     * */
    pub fn toggle(&self) {
        unsafe { self.0.Toggle().unwrap_or(()) }
    }
}

impl Debug for UiAutomationTogglePattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UiAutomationTogglePattern()")
    }
}

pub enum ToggleState {
    On,
    Off,
    Indeterminate,
}
