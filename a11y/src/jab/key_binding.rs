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

use crate::jab::jab_lib::packages::{
    AccessibleKeyBindingInfo, ACCESSIBLE_ALT_GRAPH_KEYSTROKE, ACCESSIBLE_ALT_KEYSTROKE,
    ACCESSIBLE_BUTTON1_KEYSTROKE, ACCESSIBLE_BUTTON2_KEYSTROKE, ACCESSIBLE_BUTTON3_KEYSTROKE,
    ACCESSIBLE_CONTROLCODE_KEYSTROKE, ACCESSIBLE_CONTROL_KEYSTROKE, ACCESSIBLE_FKEY_KEYSTROKE,
    ACCESSIBLE_META_KEYSTROKE, ACCESSIBLE_SHIFT_KEYSTROKE, ACCESSIBLE_VK_BACK_SPACE,
    ACCESSIBLE_VK_DELETE, ACCESSIBLE_VK_DOWN, ACCESSIBLE_VK_END, ACCESSIBLE_VK_HOME,
    ACCESSIBLE_VK_INSERT, ACCESSIBLE_VK_KP_DOWN, ACCESSIBLE_VK_KP_LEFT, ACCESSIBLE_VK_KP_RIGHT,
    ACCESSIBLE_VK_KP_UP, ACCESSIBLE_VK_LEFT, ACCESSIBLE_VK_PAGE_DOWN, ACCESSIBLE_VK_PAGE_UP,
    ACCESSIBLE_VK_RIGHT, ACCESSIBLE_VK_UP,
};

// 支持的控制码：
#[allow(unused)]
pub const CC_BACK_SPACE: u16 = ACCESSIBLE_VK_BACK_SPACE;
#[allow(unused)]
pub const CC_DELETE: u16 = ACCESSIBLE_VK_DELETE;
#[allow(unused)]
pub const CC_DOWN: u16 = ACCESSIBLE_VK_DOWN;
#[allow(unused)]
pub const CC_END: u16 = ACCESSIBLE_VK_END;
#[allow(unused)]
pub const CC_HOME: u16 = ACCESSIBLE_VK_HOME;
#[allow(unused)]
pub const CC_INSERT: u16 = ACCESSIBLE_VK_INSERT;
#[allow(unused)]
pub const CC_KP_DOWN: u16 = ACCESSIBLE_VK_KP_DOWN;
#[allow(unused)]
pub const CC_KP_LEFT: u16 = ACCESSIBLE_VK_KP_LEFT;
#[allow(unused)]
pub const CC_KP_RIGHT: u16 = ACCESSIBLE_VK_KP_RIGHT;
#[allow(unused)]
pub const CC_KP_UP: u16 = ACCESSIBLE_VK_KP_UP;
#[allow(unused)]
pub const CC_LEFT: u16 = ACCESSIBLE_VK_LEFT;
#[allow(unused)]
pub const CC_PAGE_DOWN: u16 = ACCESSIBLE_VK_PAGE_DOWN;
#[allow(unused)]
pub const CC_PAGE_UP: u16 = ACCESSIBLE_VK_PAGE_UP;
#[allow(unused)]
pub const CC_RIGHT: u16 = ACCESSIBLE_VK_RIGHT;
#[allow(unused)]
pub const CC_UP: u16 = ACCESSIBLE_VK_UP;

#[derive(Debug)]
pub struct AccessibleKeyBinding {
    _info: AccessibleKeyBindingInfo,
}

impl AccessibleKeyBinding {
    pub(crate) fn from(info: &AccessibleKeyBindingInfo) -> Self {
        Self {
            _info: info.clone(),
        }
    }

    /**
     * 是否有Shift修饰键
     * */
    pub fn is_shift_modifier(&self) -> bool {
        self._info.modifiers & ACCESSIBLE_SHIFT_KEYSTROKE == ACCESSIBLE_SHIFT_KEYSTROKE
    }

    /**
     * 是否有Control修饰键
     * */
    pub fn is_control_modifier(&self) -> bool {
        self._info.modifiers & ACCESSIBLE_CONTROL_KEYSTROKE == ACCESSIBLE_CONTROL_KEYSTROKE
    }

    /**
     * 是否有Meta修饰键
     * */
    pub fn is_meta_modifier(&self) -> bool {
        self._info.modifiers & ACCESSIBLE_META_KEYSTROKE == ACCESSIBLE_META_KEYSTROKE
    }

    /**
     * 是否有Alt修饰键
     * */
    pub fn is_alt_modifier(&self) -> bool {
        self._info.modifiers & ACCESSIBLE_ALT_KEYSTROKE == ACCESSIBLE_ALT_KEYSTROKE
    }

    /**
     * 是否有AltGraph修饰键
     * */
    pub fn is_alt_graph_modifier(&self) -> bool {
        self._info.modifiers & ACCESSIBLE_ALT_GRAPH_KEYSTROKE == ACCESSIBLE_ALT_GRAPH_KEYSTROKE
    }

    /**
     * 是否有Button1修饰键
     * */
    pub fn is_button1_modifier(&self) -> bool {
        self._info.modifiers & ACCESSIBLE_BUTTON1_KEYSTROKE == ACCESSIBLE_BUTTON1_KEYSTROKE
    }

    /**
     * 是否有Button2修饰键
     * */
    pub fn is_button2_modifier(&self) -> bool {
        self._info.modifiers & ACCESSIBLE_BUTTON2_KEYSTROKE == ACCESSIBLE_BUTTON2_KEYSTROKE
    }

    /**
     * 是否有Button3修饰键
     * */
    pub fn is_button3_modifier(&self) -> bool {
        self._info.modifiers & ACCESSIBLE_BUTTON3_KEYSTROKE == ACCESSIBLE_BUTTON3_KEYSTROKE
    }

    /**
     * 是否有F功能修饰键，character值包含1-24
     * */
    pub fn is_fkey_modifier(&self) -> bool {
        self._info.modifiers & ACCESSIBLE_FKEY_KEYSTROKE == ACCESSIBLE_FKEY_KEYSTROKE
    }

    /**
     * 是否有控制码修饰键，character值包含控制码
     * */
    pub fn is_control_code_modifier(&self) -> bool {
        self._info.modifiers & ACCESSIBLE_CONTROLCODE_KEYSTROKE == ACCESSIBLE_CONTROLCODE_KEYSTROKE
    }

    /**
     * 获取键的字符。
     * */
    pub fn get_character(&self) -> u16 {
        self._info.character
    }
}
