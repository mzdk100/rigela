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

use crate::browser::Browsable;
use std::sync::{Arc, Mutex, OnceLock};

pub type BrowserElement = Arc<dyn Browsable + Sync + Send>;

/// 窗口浏览器，使用虚拟焦点对象浏览窗口控件
struct FormBrowser {
    // 虚拟焦点索引
    index: i32,
    // 窗口控件集合
    container: Vec<BrowserElement>,
}

impl FormBrowser {
    pub fn new() -> Self {
        Self {
            index: 0,
            container: Vec::new(),
        }
    }

    /// 添加控件
    pub fn add(&mut self, element: BrowserElement) {
        self.container.push(element);
    }

    /// 清空控件
    pub fn clear(&mut self) {
        self.index = 0;
        self.container.clear();
    }

    pub fn next(&mut self) {
        self.next_index(1);
    }

    pub fn prev(&mut self) {
        self.next_index(-1);
    }

    /// 获取当前虚拟焦点控件元素
    pub fn current(&self) -> Option<BrowserElement> {
        if self.container.is_empty() {
            return None;
        }
        Some(Arc::clone(&self.container[self.index as usize]))
    }

    /// 循环移动索引， 参数（diff 传 -1 向后移动，传 1 向前移动）
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

// 获取FormBrowser的实例，FormBrowser是全局单例对象
fn get_form_browser() -> &'static Mutex<FormBrowser> {
    static INSTANCE: OnceLock<Mutex<FormBrowser>> = OnceLock::new();
    INSTANCE.get_or_init(|| Mutex::new(FormBrowser::new()))
}

// 封装 FormBrowser的接口方法，简化外部调用

pub(crate) fn clear() {
    get_form_browser().lock().unwrap().clear();
}

pub(crate) fn add(browsable: BrowserElement) {
    get_form_browser().lock().unwrap().add(browsable);
}

pub(crate) fn next() -> Option<BrowserElement> {
    get_form_browser().lock().unwrap().next();
    get_form_browser().lock().unwrap().current()
}

pub(crate) fn prev() -> Option<BrowserElement> {
    get_form_browser().lock().unwrap().prev();
    get_form_browser().lock().unwrap().current()
}

pub(crate) fn current() -> Option<BrowserElement> {
    get_form_browser().lock().unwrap().current()
}
