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

use crate::commander::keyboard::combo_keys::{ComboKey, ComboKeyExt};
use crate::commander::keyboard::keys::Keys;
use crate::commander::keyboard::modify_keys::ModifierKeys;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

type EventHandler = Box<dyn Fn()>;

pub(crate) struct ComboKeysManage {
    combokeys: Mutex<HashSet<ComboKeyExt>>,
    key_bindings: Mutex<HashMap<ComboKey, EventHandler>>,
}

impl ComboKeysManage {
    pub(crate) fn new() -> ComboKeysManage {
        ComboKeysManage {
            combokeys: Default::default(),
            key_bindings: Default::default(),
        }
    }

    fn add_binding(&mut self, combo: &ComboKey, handler: EventHandler) {
        self.combokeys.lock().unwrap().insert(combo.clone().into());
        self.key_bindings
            .lock()
            .unwrap()
            .insert(combo.clone(), handler);
    }

    fn process_key_event(&self, key: Keys, down: bool, modifiers: ModifierKeys) {}
}
