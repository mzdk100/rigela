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

use crate::commander::keyboard::combo_keys::{ComboKey, ComboKeyExt, State};
use std::sync::Mutex;

#[allow(unused)]
pub(crate) struct ComboKeysManage {
    combo_key_cache: Mutex<ComboKeyExt>,
}

#[allow(unused)]
impl ComboKeysManage {
    pub(crate) fn new() -> ComboKeysManage {
        ComboKeysManage {
            combo_key_cache: Mutex::new(ComboKeyExt::default()),
        }
    }

    fn process_combo_key(&self, key: ComboKey, pressed: bool) -> ComboKey {
        let mut combo_key_cache = self.combo_key_cache.lock().unwrap();

        if key == combo_key_cache.clone().into() {
            match pressed {
                true => {}
                false => {}
            }
        }

        ComboKey {
            state: State::SinglePress,
            ..key
        }
    }
}
