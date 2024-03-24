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
use bitflags::bitflags;
use std::fmt;

bitflags! {
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

impl From<&str> for ModifierKeys {
    fn from(key: &str) -> ModifierKeys {
        match key {
            "RigelA" => ModifierKeys::RIGELA,
            "Ctrl" => ModifierKeys::CTRL,
            "Alt" => ModifierKeys::ALT,
            "Shift" => ModifierKeys::SHIFT,
            "Win" => ModifierKeys::WIN,
            "RigelA_Ctrl" => ModifierKeys::RIGELA | ModifierKeys::CTRL,
            "RigelA_Alt" => ModifierKeys::RIGELA | ModifierKeys::ALT,
            "RigelA_Shift" => ModifierKeys::RIGELA | ModifierKeys::SHIFT,
            "RigelA_Win" => ModifierKeys::RIGELA | ModifierKeys::WIN,
            "Ctrl_Alt" => ModifierKeys::CTRL | ModifierKeys::ALT,
            "Ctrl_Shift" => ModifierKeys::CTRL | ModifierKeys::SHIFT,
            "Ctrl_Win" => ModifierKeys::CTRL | ModifierKeys::WIN,
            "Alt_Shift" => ModifierKeys::ALT | ModifierKeys::SHIFT,
            "Alt_Win" => ModifierKeys::ALT | ModifierKeys::WIN,
            "Shift_Win" => ModifierKeys::SHIFT | ModifierKeys::WIN,
            "RigelA_Ctrl_Alt" => ModifierKeys::RIGELA | ModifierKeys::CTRL | ModifierKeys::ALT,
            "RigelA_Ctrl_Shift" => ModifierKeys::RIGELA | ModifierKeys::CTRL | ModifierKeys::SHIFT,
            "RigelA_Ctrl_Win" => ModifierKeys::RIGELA | ModifierKeys::CTRL | ModifierKeys::WIN,
            "RigelA_Alt_Shift" => ModifierKeys::RIGELA | ModifierKeys::ALT | ModifierKeys::SHIFT,
            "RigelA_Alt_Win" => ModifierKeys::RIGELA | ModifierKeys::ALT | ModifierKeys::WIN,
            "RigelA_Shift_Win" => ModifierKeys::RIGELA | ModifierKeys::SHIFT | ModifierKeys::WIN,
            "Ctrl_Alt_Shift" => ModifierKeys::CTRL | ModifierKeys::ALT | ModifierKeys::SHIFT,
            "Ctrl_Alt_Win" => ModifierKeys::CTRL | ModifierKeys::ALT | ModifierKeys::WIN,
            "Ctrl_Shift_Win" => ModifierKeys::CTRL | ModifierKeys::SHIFT | ModifierKeys::WIN,
            "Alt_Shift_Win" => ModifierKeys::ALT | ModifierKeys::SHIFT | ModifierKeys::WIN,
            "RigelA_Ctrl_Alt_Shift" => {
                ModifierKeys::RIGELA | ModifierKeys::CTRL | ModifierKeys::ALT | ModifierKeys::SHIFT
            }
            "RigelA_Ctrl_Alt_Win" => {
                ModifierKeys::RIGELA | ModifierKeys::CTRL | ModifierKeys::ALT | ModifierKeys::WIN
            }
            "RigelA_Ctrl_Shift_Win" => {
                ModifierKeys::RIGELA | ModifierKeys::CTRL | ModifierKeys::SHIFT | ModifierKeys::WIN
            }
            "RigelA_Alt_Shift_Win" => {
                ModifierKeys::RIGELA | ModifierKeys::ALT | ModifierKeys::SHIFT | ModifierKeys::WIN
            }
            "Ctrl_Alt_Shift_Win" => {
                ModifierKeys::CTRL | ModifierKeys::ALT | ModifierKeys::SHIFT | ModifierKeys::WIN
            }
            _ => ModifierKeys::empty(),
        }
    }
}

impl fmt::Display for ModifierKeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = vec![];
        if self.clone() & ModifierKeys::RIGELA != ModifierKeys::empty() {
            list.push("RigelA");
        }
        if self.clone() & ModifierKeys::CTRL != ModifierKeys::empty() {
            list.push("Ctrl");
        }
        if self.clone() & ModifierKeys::ALT != ModifierKeys::empty() {
            list.push("Alt");
        }
        if self.clone() & ModifierKeys::SHIFT != ModifierKeys::empty() {
            list.push("Shift");
        }
        if self.clone() & ModifierKeys::WIN != ModifierKeys::empty() {
            list.push("Win");
        }
        let text = if list.is_empty() {
            String::new()
        } else {
            list.join("_")
        };

        write!(f, "{text}")
    }
}
