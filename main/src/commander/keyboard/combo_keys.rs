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

use std::{
    fmt::{Display, Formatter},
    hash::Hash,
};

use serde::{Deserialize, Serialize};

use crate::commander::keyboard::{keys::Keys, modify_keys::ModifierKeys};

/// 定义组合键
/// Example: combo_keys!("RigelA", Keys::VkEsc), combo_keys!("RigelA", Keys::F12, double),
#[macro_export]
macro_rules! combo_key {
    ($key: path) => {
        ComboKey::new($key, ModifierKeys::empty(), State::SinglePress)
    };
    ($key: path, double) => {
        ComboKey::new($key, ModifierKeys::empty(), State::DoublePress)
    };
    ($key: path, long) => {
        ComboKey::new($key, ModifierKeys::empty(), State::LongPress)
    };
    ($mdf: literal, $key: path) => {
        ComboKey::new($key, ModifierKeys::from($mdf), State::SinglePress)
    };
    ($mdf: literal, $key: path, double) => {
        ComboKey::new($key, ModifierKeys::from($mdf), State::DoublePress)
    };
    ($mdf: literal, $key: path, long) => {
        ComboKey::new($key, ModifierKeys::from($mdf), State::LongPress)
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) enum State {
    Idle,
    SinglePress,
    DoublePress,
    LongPress,
}

impl Default for State {
    fn default() -> Self {
        State::Idle
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub(crate) struct ComboKey {
    pub(crate) main_key: Keys,
    pub(crate) modify_keys: ModifierKeys,
    pub(crate) state: State,
}

impl ComboKey {
    pub(crate) fn new(main_key: Keys, modify_keys: ModifierKeys, state: State) -> Self {
        ComboKey {
            main_key,
            modify_keys,
            state,
        }
    }

    pub(crate) fn change_state(self, state: State) -> Self {
        ComboKey {
            main_key: self.main_key,
            modify_keys: self.modify_keys,
            state,
        }
    }
}

impl From<Vec<Keys>> for ComboKey {
    fn from(keys: Vec<Keys>) -> Self {
        let mut mdf = ModifierKeys::empty();
        let mut main = Keys::VkNone;
        for k in keys {
            match k.is_modifierkey() {
                true => mdf |= ModifierKeys::from(k),
                false => main = k,
            }
        }

        ComboKey::new(main, mdf, State::Idle)
    }
}

impl Into<Vec<Keys>> for ComboKey {
    fn into(self) -> Vec<Keys> {
        let mut keys: Vec<Keys> = self.modify_keys.into();
        keys.push(self.main_key);
        keys
    }
}

impl Display for ComboKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let state = match self.state {
            State::SinglePress => "",
            State::DoublePress => "(Double)",
            State::LongPress => "(Long)",
            _ => "",
        };
        let modify = if self.modify_keys.is_empty() {
            String::from("")
        } else {
            format!("{} +", self.modify_keys)
        };
        write!(f, "{modify}{}{state}", self.main_key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) struct ComboKeyExt {
    pub(crate) combokey: ComboKey,
    pub(crate) timestamp: u64,
    pub(crate) count: u32,
}

impl From<ComboKey> for ComboKeyExt {
    fn from(combokey: ComboKey) -> Self {
        ComboKeyExt {
            combokey,
            timestamp: 0,
            count: 0,
        }
    }
}
