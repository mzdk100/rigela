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
use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) type BrowserElement = Arc<dyn Browsable + Sync + Send>;

/// 窗口浏览器，使用虚拟焦点对象浏览窗口控件
pub struct FormBrowser {
    // 焦点控件索引
    index: RwLock<i32>,
    // 子控件索引
    child_index: RwLock<i32>,
    // 窗口控件集合
    container: RwLock<Vec<BrowserElement>>,
}

impl FormBrowser {
    pub fn new() -> Self {
        Self {
            index: RwLock::new(0),
            child_index: RwLock::new(-1),
            container: RwLock::new(Vec::new()),
        }
    }

    /**
     * 浏览器渲染操作，解析控件树。
     * `root` 根元素。
     * */
    pub(crate) async fn render(&self, root: BrowserElement) {
        {
            // 使用单独的块保证读写锁不会有过长的生命周期
            let mut index = self.index.write().await;
            *index = 0;
        }
        {
            let mut index = self.child_index.write().await;
            *index = 0;
        }
        let mut container = self.container.write().await;
        container.clear();
        for i in 0..root.get_child_count() {
            container.push(root.get_child(i).unwrap());
        }
    }

    /// 向后移动当前焦点
    pub(crate) async fn next(&self) -> &Self {
        self.move_cur_index(1).await
    }

    /// 向前移动当前焦点
    pub(crate) async fn prev(&self) -> &Self {
        self.move_cur_index(-1).await
    }

    /// 向后移动当前子控件
    pub(crate) async fn next_child(&self) -> &Self {
        self.move_child_index(1).await
    }

    /// 向前移动当前子控件
    pub(crate) async fn prev_child(&self) -> &Self {
        self.move_child_index(-1).await
    }

    /// 获取当前子控件
    pub(crate) async fn current_child(&self) -> Option<BrowserElement> {
        let cur_element = self.current().await;
        if cur_element.is_none() {
            return None;
        }
        let cur_element = cur_element.unwrap();
        let child_index = { *self.child_index.read().await };
        cur_element.get_child(child_index as usize)
    }

    /// 移动当前索引
    async fn move_cur_index(&self, diff: i32) -> &Self {
        let container = self.container.read().await;
        let len = container.len() as i32;
        if len <= 1 {
            return self;
        }
        let cur_index = { *self.index.read().await };
        let result = cur_index + diff;
        let result = match result {
            i if i < 0 => len - 1,
            i if i >= len => 0,
            i => i,
        };
        {
            let mut index = self.index.write().await;
            *index = result;
        }
        {
            let mut index = self.child_index.write().await;
            *index = -1;
        };
        self
    }

    /// 移动当前子控件索引
    pub(crate) async fn move_child_index(&self, diff: i32) -> &Self {
        let cur_element = self.current().await;
        if cur_element.is_none() {
            return self;
        }
        let cur_element = cur_element.unwrap();
        let len = cur_element.get_child_count() as i32;
        {
            let mut index = self.child_index.write().await;
            let result = *index + diff;
            *index = match result {
                i if i < 0 => len - 1,
                i if i >= len => 0,
                i => i,
            };
        }
        self
    }

    /// 获取当前焦点控件元素
    pub(crate) async fn current(&self) -> Option<BrowserElement> {
        let container = self.container.read().await;
        if container.is_empty() {
            return None;
        }
        let index = { *self.index.read().await };
        Some(container[index as usize].clone())
    }
}

unsafe impl Send for FormBrowser {}

unsafe impl Sync for FormBrowser {}
