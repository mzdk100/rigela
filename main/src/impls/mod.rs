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

use crate::performer::Speakable;
use std::borrow::Cow;

mod peeper;
mod uia;

impl Speakable for u16 {
    fn get_sentence(&self) -> String {
        String::from_utf16_lossy(&[*self])
    }
}

impl Speakable for String {
    fn get_sentence(&self) -> String {
        self.to_string()
    }
}

impl Speakable for Cow<'_, str> {
    fn get_sentence(&self) -> String {
        self.to_string()
    }
}
