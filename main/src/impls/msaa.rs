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

use crate::{ext::role::AccessibleRoleExt, performer::Speakable};
use win_wrap::msaa::object::AccessibleObject;

impl Speakable for (AccessibleObject, i32) {
    fn get_sentence(&self) -> String {
        let mut text = vec![
            self.0.get_name(self.1),
            self.0.get_description(self.1),
            self.get_role_name(),
            self.0.get_value(self.1),
        ];
        text.retain(|i| !i.is_empty());
        text.join(", ")
    }
}
