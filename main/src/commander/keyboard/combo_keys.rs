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

use crate::commander::keyboard::keys::Keys;
use crate::commander::keyboard::modify_keys::ModifierKeys;
use std::fmt;
use std::fmt::Formatter;
use std::hash::Hash;

/// 定义组合键
/// Example: combo_keys!("RigelA", Keys::VkEsc), combo_keys!("RigelA", Keys::F12, double),
#[allow(unused)]
macro_rules! combo_keys {
    ($mdf: literal, $key: path) => {
        ComboKey {
            main_key: $key,
            modify_keys: ModifierKeys::from($mdf),
            state: State::SinglePress,
        }
        ($mdf: literal, $key: path, double) =>{
            ComboKey {
                main_key: $key,
                modify_keys: ModifierKeys::from($mdf),
                state: State::DoublePress,
            }
            ($mdf: literal, $key: path, long) => {
                ComboKey {
                    main_key: $key,
                    modify_keys: ModifierKeys::from($mdf),
                    state: State::LongPress,
                }
            }
        }
    };
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum State {
    SinglePress,
    DoublePress,
    LongPress,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ComboKey {
    main_key: Keys,
    modify_keys: ModifierKeys,
    state: State,
}

impl Default for ComboKey {
    fn default() -> Self {
        ComboKey {
            main_key: Keys::VkNone,
            modify_keys: ModifierKeys::empty(),
            state: State::SinglePress,
        }
    }
}

impl fmt::Display for ComboKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let state = match self.state {
            State::SinglePress => "",
            State::DoublePress => "(Double)",
            State::LongPress => "(Long)",
        };
        write!(f, "{} + {}{state}", self.main_key, self.modify_keys)
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) struct ComboKeyExt {
    combokey: ComboKey,
    timestamp: u64,
    count: u32,
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
