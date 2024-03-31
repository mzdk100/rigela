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

use win_wrap::uia::{
    element::UiAutomationElement,
    pattern::text::{UiAutomationTextPattern, UiAutomationTextPattern2, UiAutomationTextRange},
};

pub(crate) trait UiAutomationElementExt {
    /**
     * 获取插入点
     * */
    fn get_caret(&self) -> Option<UiAutomationTextRange>;
}

impl UiAutomationElementExt for UiAutomationElement {
    fn get_caret(&self) -> Option<UiAutomationTextRange> {
        if let Ok(pattern) = UiAutomationTextPattern2::obtain(self) {
            if let Some((_, caret)) = pattern.get_caret_range() {
                Some(caret)
            } else {
                None
            }
        } else if let Ok(pattern) = UiAutomationTextPattern::obtain(self) {
            let selection = pattern.get_selection();
            if let Some(caret) = selection.first() {
                Some(caret.clone())
            } else {
                None
            }
        } else {
            return None;
        }
    }
}
