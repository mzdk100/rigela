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
    browser::{form_browser::BrowserElement, Browsable},
    performer::Speakable,
};
use std::sync::Arc;
use win_wrap::{
    common::RECT,
    uia::{
        element::UiAutomationElement,
        pattern::{
            legacy::UiAutomationIAccessiblePattern,
            text::UiAutomationTextRange,
            toggle::{ToggleState, UiAutomationTogglePattern},
        },
    },
};

trait ElementNameExt {
    fn get_name_better(&self) -> String;
}

impl ElementNameExt for UiAutomationElement {
    fn get_name_better(&self) -> String {
        let mut name = self.get_name();

        if name.is_empty() {
            let accessible = UiAutomationIAccessiblePattern::obtain(self);
            if accessible.is_err() {
                return String::new();
            }

            let accessible = accessible.unwrap();
            name = accessible.get_name();

            if name.is_empty() {
                name = accessible.get_description();
            }
        }
        name
    }
}

impl Browsable for UiAutomationElement {
    fn get_name(&self) -> String {
        self.get_name_better()
    }

    fn get_role(&self) -> String {
        self.get_localized_control_type()
    }

    fn get_child_count(&self) -> usize {
        self.get_child_count() as usize
    }

    fn get_child(&self, index: usize) -> Option<BrowserElement> {
        if let Some(x) = self.get_child(index as i32) {
            return Some(Arc::new(x));
        }
        None
    }

    fn get_rect(&self) -> RECT {
        self.get_rect()
    }
}

/// 给UIA元素实现朗读接口
impl Speakable for UiAutomationElement {
    fn get_sentence(&self) -> String {
        let mut text = self.get_name_better();

        let control_type = self.get_localized_control_type();
        if !control_type.is_empty() {
            text += format!(", {}", control_type).as_str()
        }

        let accelerator_key = self.get_accelerator_key();
        if !accelerator_key.is_empty() {
            text += format!(", {}", accelerator_key).as_str()
        }

        let access_key = self.get_access_key();
        if !access_key.is_empty() {
            text += format!(", {}", access_key).as_str()
        }

        let toggle = UiAutomationTogglePattern::obtain(self);
        if toggle.is_ok() {
            text += format!(
                ", {}",
                match toggle.unwrap().get_toggle_state() {
                    ToggleState::On => t!("uia.toggle_on"),
                    ToggleState::Off => t!("uia.toggle_off"),
                    ToggleState::Indeterminate => t!("uia.toggle_indeterminate"),
                }
            )
            .as_str();
        }

        text
    }
}

/// 给UIA文本范围实现朗读接口
impl Speakable for UiAutomationTextRange {
    fn get_sentence(&self) -> String {
        self.get_text(-1)
    }
}
