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

use scintilla_sys::{
    SCK_ADD, SCK_BACK, SCK_DELETE, SCK_DIVIDE, SCK_DOWN, SCK_END, SCK_ESCAPE, SCK_HOME, SCK_INSERT,
    SCK_LEFT, SCK_MENU, SCK_NEXT, SCK_PRIOR, SCK_RETURN, SCK_RIGHT, SCK_RWIN, SCK_SUBTRACT,
    SCK_TAB, SCK_UP, SCK_WIN,
};

/// 键代码是可见字符或控制字符，或来自 SCK_* 枚举的键
#[derive(Debug, PartialEq)]
pub enum KeyCode {
    Add,
    Back,
    Delete,
    Divide,
    Down,
    End,
    Escape,
    Home,
    Insert,
    Left,
    Menu,
    Next,
    Prior,
    Return,
    Right,
    RWin,
    Subtract,
    Tab,
    Up,
    Win,
    Other(u32),
}

impl From<u32> for KeyCode {
    fn from(value: u32) -> Self {
        match value {
            SCK_ADD => Self::Add,
            SCK_BACK => Self::Back,
            SCK_DELETE => Self::Delete,
            SCK_DIVIDE => Self::Divide,
            SCK_DOWN => Self::Down,
            SCK_END => Self::End,
            SCK_ESCAPE => Self::Escape,
            SCK_HOME => Self::Home,
            SCK_INSERT => Self::Insert,
            SCK_LEFT => Self::Left,
            SCK_MENU => Self::Menu,
            SCK_NEXT => Self::Next,
            SCK_PRIOR => Self::Prior,
            SCK_RETURN => Self::Return,
            SCK_RIGHT => Self::Right,
            SCK_RWIN => Self::RWin,
            SCK_SUBTRACT => Self::Subtract,
            SCK_TAB => Self::Tab,
            SCK_UP => Self::Up,
            SCK_WIN => Self::Win,
            _ => Self::Other(value),
        }
    }
}

impl Into<u32> for KeyCode {
    fn into(self) -> u32 {
        match self {
            Self::Add => SCK_ADD,
            Self::Back => SCK_BACK,
            Self::Delete => SCK_DELETE,
            Self::Divide => SCK_DIVIDE,
            Self::Down => SCK_DOWN,
            Self::End => SCK_END,
            Self::Escape => SCK_ESCAPE,
            Self::Home => SCK_HOME,
            Self::Insert => SCK_INSERT,
            Self::Left => SCK_LEFT,
            Self::Menu => SCK_MENU,
            Self::Next => SCK_NEXT,
            Self::Prior => SCK_PRIOR,
            Self::Return => SCK_RETURN,
            Self::Right => SCK_RIGHT,
            Self::RWin => SCK_RWIN,
            Self::Subtract => SCK_SUBTRACT,
            Self::Tab => SCK_TAB,
            Self::Up => SCK_UP,
            Self::Win => SCK_WIN,
            Self::Other(k) => k,
        }
    }
}
