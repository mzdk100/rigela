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
use std::ops::Deref;
use std::sync::Arc;

type BrowserElement = Arc<dyn Browsable + Sync + Send>;

/// 窗口浏览器，使用虚拟焦点对象浏览窗口控件
pub struct FormBrowser {
    // 焦点控件索引
    index: i32,
    // 子控件索引
    child_index: i32,
    // 窗口控件集合
    container: Vec<BrowserElement>,
}

impl FormBrowser {
    pub fn new() -> Self {
        Self {
            index: 0,
            child_index: -1,
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
        self.child_index = -1;
        self.container.clear();
    }

    pub fn next(&mut self) {
        self.index = self.next_index(self.index, self.container.len(), 1);
        self.child_index = -1;
    }

    pub fn prev(&mut self) {
        self.index = self.next_index(self.index, self.container.len(), -1);
        self.child_index = -1;
    }

    pub fn next_child(&mut self) {
        let len = self
            .container
            .get(self.index as usize)
            .unwrap()
            .get_child_count();
        self.child_index = self.next_index(self.child_index, len, 1);
    }

    pub fn prev_child(&mut self) {
        let len = self
            .container
            .get(self.index as usize)
            .unwrap()
            .get_child_count();
        self.child_index = self.next_index(self.child_index, len, -1);
    }

    /// 获取当前焦点控件元素
    #[allow(unused)]
    pub fn current(&self) -> Option<BrowserElement> {
        if self.container.is_empty() {
            return None;
        }
        Some(Arc::clone(&self.container[self.index as usize]))
    }

    /// 循环移动索引， 参数（diff 传 -1 向后移动，传 1 向前移动）
    fn next_index(&self, cur_index: i32, length: usize, diff: i32) -> i32 {
        let len = length as i32;
        if len <= 1 {
            return 0;
        }

        let result = cur_index + diff;
        match result {
            i if i < 0 => len - 1,
            i if i >= len => 0,
            i => i,
        }
    }
}

impl Deref for FormBrowser {
    type Target = BrowserElement;
    fn deref(&self) -> &Self::Target {
        // todo: 这里如果container为空，需要做进一步处理，或者可以实现一个Empty BrowserElement
        &self.container.get(self.index as usize).unwrap()
    }
}

unsafe impl Send for FormBrowser {}

unsafe impl Sync for FormBrowser {}
