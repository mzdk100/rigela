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
use std::sync::{Arc, Mutex, OnceLock};
use win_wrap::common::{get_foreground_window, HWND};

pub type BrowserElement = Arc<dyn Browseable + Sync + Send>;

struct FormBrowser {
    hwnd: HWND,
    index: i32,
    container: Vec<BrowserElement>,
}

impl FormBrowser {
    pub fn new() -> Self {
        Self {
            hwnd: HWND::default(),
            index: 0,
            container: Vec::new(),
        }
    }

    pub fn get_hwnd(&self) -> HWND {
        self.hwnd
    }

    pub fn set_hwnd(&mut self, hwnd: HWND) {
        self.hwnd = hwnd;
    }

    pub fn add(&mut self, element: BrowserElement) {
        self.container.push(element);
    }

    pub fn clear(&mut self) {
        self.container.clear();
        self.index = 0;
        self.hwnd = HWND::default();
    }

    pub fn next(&mut self) {
        self.next_index(1);
    }

    pub fn prev(&mut self) {
        self.next_index(-1);
    }

    pub fn current(&self) -> Option<BrowserElement> {
        if self.container.is_empty() {
            return None;
        }
        Some(Arc::clone(&self.container[self.index as usize]))
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

fn get_form_browser() -> &'static Mutex<FormBrowser> {
    static INSTANCE: OnceLock<Mutex<FormBrowser>> = OnceLock::new();
    INSTANCE.get_or_init(|| Mutex::new(FormBrowser::new()))
}

pub(crate) fn is_foreground_window_changed() -> bool {
    get_form_browser().lock().unwrap().get_hwnd() != get_foreground_window()
}

pub(crate) fn update_form_browser_hwnd() {
    get_form_browser()
        .lock()
        .unwrap()
        .set_hwnd(get_foreground_window());
}

pub(crate) fn clear_browseable() {
    get_form_browser().lock().unwrap().clear();
}

pub(crate) fn add_browseable(browseable: BrowserElement) {
    get_form_browser().lock().unwrap().add(browseable);
}

pub(crate) fn next_browseable() -> Option<BrowserElement> {
    get_form_browser().lock().unwrap().next();
    get_form_browser().lock().unwrap().current()
}

pub(crate) fn prev_browseable() -> Option<BrowserElement> {
    get_form_browser().lock().unwrap().prev();
    get_form_browser().lock().unwrap().current()
}

pub(crate) fn current_browseable() -> Option<BrowserElement> {
    get_form_browser().lock().unwrap().current()
}
