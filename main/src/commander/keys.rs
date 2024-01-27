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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Keys {
    RigelA,
    Esc,

    None,
}

impl From<(i32, bool)> for Keys {
    fn from(vk: (i32, bool)) -> Self {
        match vk {
            (82, true) => Self::RigelA,
            (27, true) => Self::Esc,

            _ => Self::None,
        }
    }
}
