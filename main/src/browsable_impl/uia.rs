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

use crate::browser::Browsable;
use std::sync::Arc;
use win_wrap::uia::ui_element::UiAutomationElement;

impl Browsable for UiAutomationElement {
    fn get_name(&self) -> String {
        self.get_name()
    }

    fn get_role(&self) -> String {
        self.get_localized_control_type()
    }
    fn get_child_count(&self) -> usize {
        0
    }
    fn get_child(&self, index: i32) -> Option<Arc<dyn Browsable + Sync + Send>> {
        None
    }
}
