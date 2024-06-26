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
use serde::{Deserialize, Serialize};
use std::fmt;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
    #[serde(transparent)]
    pub(crate) struct ModifierKeys: u8 {
        const RIGELA = 0b0000_0001;
        const CTRL = 0b0000_0010;
        const ALT = 0b0000_0100;
        const SHIFT = 0b0000_1000;
        const WIN = 0b0001_0000;
    }
}

impl From<Keys> for ModifierKeys {
    fn from(key: Keys) -> ModifierKeys {
        match key {
            Keys::VkShift => ModifierKeys::SHIFT,
            Keys::VkCtrl => ModifierKeys::CTRL,
            Keys::VkAlt => ModifierKeys::ALT,
            Keys::VkWin => ModifierKeys::WIN,
            Keys::VkRigelA => ModifierKeys::RIGELA,
            _ => ModifierKeys::empty(),
        }
    }
}

impl Into<Vec<Keys>> for ModifierKeys {
    fn into(self) -> Vec<Keys> {
        let mut keys = Vec::new();
        if self.contains(ModifierKeys::RIGELA) {
            keys.push(Keys::VkRigelA);
        }
        if self.contains(ModifierKeys::CTRL) {
            keys.push(Keys::VkCtrl);
        }
        if self.contains(ModifierKeys::ALT) {
            keys.push(Keys::VkAlt);
        }
        if self.contains(ModifierKeys::SHIFT) {
            keys.push(Keys::VkShift);
        }
        if self.contains(ModifierKeys::WIN) {
            keys.push(Keys::VkWin);
        }

        keys
    }
}

impl From<&str> for ModifierKeys {
    fn from(key: &str) -> ModifierKeys {
        let key = key.to_ascii_lowercase();
        match key.as_str() {
            "rigela" => ModifierKeys::RIGELA,
            "ctrl" => ModifierKeys::CTRL,
            "alt" => ModifierKeys::ALT,
            "shift" => ModifierKeys::SHIFT,
            "win" => ModifierKeys::WIN,
            "rigela_ctrl" => ModifierKeys::RIGELA | ModifierKeys::CTRL,
            "rigela_alt" => ModifierKeys::RIGELA | ModifierKeys::ALT,
            "rigela_shift" => ModifierKeys::RIGELA | ModifierKeys::SHIFT,
            "rigela_win" => ModifierKeys::RIGELA | ModifierKeys::WIN,
            "ctrl_alt" => ModifierKeys::CTRL | ModifierKeys::ALT,
            "ctrl_shift" => ModifierKeys::CTRL | ModifierKeys::SHIFT,
            "ctrl_win" => ModifierKeys::CTRL | ModifierKeys::WIN,
            "alt_shift" => ModifierKeys::ALT | ModifierKeys::SHIFT,
            "alt_win" => ModifierKeys::ALT | ModifierKeys::WIN,
            "shift_win" => ModifierKeys::SHIFT | ModifierKeys::WIN,
            "rigela_ctrl_alt" => ModifierKeys::RIGELA | ModifierKeys::CTRL | ModifierKeys::ALT,
            "rigela_ctrl_shift" => ModifierKeys::RIGELA | ModifierKeys::CTRL | ModifierKeys::SHIFT,
            "rigela_ctrl_win" => ModifierKeys::RIGELA | ModifierKeys::CTRL | ModifierKeys::WIN,
            "rigela_alt_shift" => ModifierKeys::RIGELA | ModifierKeys::ALT | ModifierKeys::SHIFT,
            "rigela_alt_win" => ModifierKeys::RIGELA | ModifierKeys::ALT | ModifierKeys::WIN,
            "rigela_shift_win" => ModifierKeys::RIGELA | ModifierKeys::SHIFT | ModifierKeys::WIN,
            "ctrl_alt_shift" => ModifierKeys::CTRL | ModifierKeys::ALT | ModifierKeys::SHIFT,
            "ctrl_alt_win" => ModifierKeys::CTRL | ModifierKeys::ALT | ModifierKeys::WIN,
            "ctrl_shift_win" => ModifierKeys::CTRL | ModifierKeys::SHIFT | ModifierKeys::WIN,
            "alt_shift_win" => ModifierKeys::ALT | ModifierKeys::SHIFT | ModifierKeys::WIN,
            "rigela_ctrl_alt_shift" => {
                ModifierKeys::RIGELA | ModifierKeys::CTRL | ModifierKeys::ALT | ModifierKeys::SHIFT
            }
            "rigela_ctrl_alt_win" => {
                ModifierKeys::RIGELA | ModifierKeys::CTRL | ModifierKeys::ALT | ModifierKeys::WIN
            }
            "rigela_ctrl_shift_win" => {
                ModifierKeys::RIGELA | ModifierKeys::CTRL | ModifierKeys::SHIFT | ModifierKeys::WIN
            }
            "rigela_alt_shift_win" => {
                ModifierKeys::RIGELA | ModifierKeys::ALT | ModifierKeys::SHIFT | ModifierKeys::WIN
            }
            "ctrl_alt_shift_win" => {
                ModifierKeys::CTRL | ModifierKeys::ALT | ModifierKeys::SHIFT | ModifierKeys::WIN
            }
            _ => ModifierKeys::empty(),
        }
    }
}

impl fmt::Display for ModifierKeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = vec![];

        let list2 = [
            (ModifierKeys::RIGELA, "RigelA"),
            (ModifierKeys::CTRL, "Ctrl"),
            (ModifierKeys::ALT, "Alt"),
            (ModifierKeys::SHIFT, "Shift"),
            (ModifierKeys::WIN, "Win"),
        ];
        for (k, v) in list2 {
            if self.contains(k) {
                list.push(v)
            }
        }

        write!(f, "{}", list.join(" + "))
    }
}

impl Default for ModifierKeys {
    fn default() -> Self {
        ModifierKeys::empty()
    }
}
