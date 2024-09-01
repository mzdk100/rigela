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
    ext::{element::UiAutomationElementExt, role::AccessibleRoleExt},
    performer::Speakable,
};
use win_wrap::uia::{
    element::{ControlType, UiAutomationElement},
    pattern::{
        legacy::UiAutomationIAccessiblePattern,
        range::UiAutomationRangeValuePattern,
        text::{TextUnit::Line, UiAutomationTextRange},
        toggle::{ToggleState, UiAutomationTogglePattern},
        value::UiAutomationValuePattern,
        PatternCreator,
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

/// 给UIA元素实现朗读接口
impl Speakable for UiAutomationElement {
    fn get_sentence(&self) -> String {
        let mut text = vec![self.get_name_better(), self.get_role_name()];

        let control_type = self.get_control_type();
        if control_type != ControlType::Document && control_type != ControlType::Edit {
            if let Ok(pattern) = UiAutomationValuePattern::obtain(self) {
                text.push(pattern.get_value().unwrap())
            }
        }

        if let Some(caret) = self.get_caret() {
            caret.expand_to_enclosing_unit(Line);
            text.push(caret.get_text(-1))
        }

        text.push(self.get_accelerator_key());
        text.push(self.get_access_key());

        if let Ok(toggle) = UiAutomationTogglePattern::obtain(self) {
            text.push(
                match toggle.get_toggle_state() {
                    ToggleState::On => t!("uia.toggle_on"),
                    ToggleState::Off => t!("uia.toggle_off"),
                    ToggleState::Indeterminate => t!("uia.toggle_indeterminate"),
                }
                .to_string(),
            );
        }

        if let Ok(pattern) = UiAutomationRangeValuePattern::obtain(self) {
            let (m, n) = (pattern.get_minimum(), pattern.get_maximum());
            if n > m {
                text.push(format!(
                    "{}% ({}-{})",
                    (pattern.get_value() - m) / (n - m) * 100f64,
                    m,
                    n
                ));
            } else {
                text.push(format!("{} ({}-{})", pattern.get_value(), m, n))
            }
        }

        // 过滤空文本
        text.retain(|i| !i.is_empty());
        text.join(", ")
    }
}

/// 给UIA文本范围实现朗读接口
impl Speakable for UiAutomationTextRange {
    fn get_sentence(&self) -> String {
        self.get_text(-1)
    }
}
