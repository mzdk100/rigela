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

use crate::browser::Browseable;
use std::sync::{LazyLock, Mutex};
use win_wrap::common::{get_foreground_window, HWND};

pub struct FormBrowser {
    hwnd: HWND,
    index: i32,
    container: Vec<Box<dyn Browseable>>,
}

pub static FORM_BROWSER: LazyLock<Mutex<FormBrowser>> =
    LazyLock::new(|| Mutex::new(FormBrowser::new()));

impl FormBrowser {
    pub fn new() -> Self {
        Self {
            hwnd: HWND::default(),
            index: 0,
            container: Vec::new(),
        }
    }

    pub fn is_foreground_window_changed(&self) -> bool {
        self.hwnd != get_foreground_window()
    }

    pub fn update_hwnd(&mut self) {
        self.hwnd = get_foreground_window();
    }

    pub fn set_hwnd(&mut self, hwnd: HWND) {
        self.hwnd = hwnd;
    }

    pub fn add(&mut self, element: Box<dyn Browseable>) {
        self.container.push(element);
    }

    pub fn clear(&mut self) {
        self.container.clear();
    }

    pub fn next(&mut self) {
        self.next_index(1);
    }

    pub fn prev(&mut self) {
        self.next_index(-1);
    }

    pub fn current(&self) -> Option<&dyn Browseable> {
        if self.container.is_empty() {
            return None;
        }
        self.container.get(self.index as usize).map(|e| &**e)
    }

    fn next_index(&mut self, diff: i32) {
        let len = self.container.len() as i32;
        if len <= 1 {
            return;
        }

        self.index = self.index + diff;

        self.index = match self.index {
            i if i < 0 => len - 1,
            i if i >= len => 0,
            i => i,
        }
    }
}

unsafe impl Send for FormBrowser {}
unsafe impl Sync for FormBrowser {}
