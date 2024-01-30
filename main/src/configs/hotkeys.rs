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

use crate::commander::keys::Keys;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct HotKeysConfig {
    pub(crate) pairs: HashMap<String, Vec<Keys>>,
}

impl Default for HotKeysConfig {
    fn default() -> Self {
        let mut pairs = HashMap::new();

        pairs.insert("退出程序".to_string(), vec![Keys::VkRigelA, Keys::VkEscape]);
        pairs.insert("朗读时间".to_string(), vec![Keys::VkRigelA, Keys::VkF12]);

        Self { pairs }
    }
}
