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

use std::collections::HashSet;
use std::sync::Arc;

use tokio::sync::MutexGuard;

use crate::navigator::{element::UiElement, UiNavigator};

static mut INDEX: i32 = 0;

/// 移动当前索引
async fn move_cur_index<'a>(
    container: &'a MutexGuard<'a, HashSet<Arc<UiElement<'static>>>>,
    diff: i32,
) -> bool {
    let len = container.len() as i32;
    if len <= 1 {
        return false;
    }
    let cur_index = unsafe { INDEX };
    let result = cur_index + diff;
    let result = match result {
        i if i < 0 => len - 1,
        i if i >= len => 0,
        i => i,
    };
    unsafe {
        INDEX = result;
    }
    true
}

pub(crate) trait LinearNavigator {
    async fn current(&self) -> Option<Arc<UiElement<'_>>>;
    async fn next(&self) -> &Self;
    async fn prev(&self) -> &Self;
}

impl LinearNavigator for UiNavigator {
    /// 获取当前焦点控件元素
    async fn current(&self) -> Option<Arc<UiElement<'_>>> {
        let container = self.container.lock().await;
        if container.is_empty() {
            return None;
        }
        let index = unsafe { INDEX };
        for (i, j) in container.iter().enumerate() {
            if (i as i32) == index {
                return Some(j.clone());
            }
        }
        None
    }

    /// 向后移动当前焦点
    async fn next(&self) -> &Self {
        let container = self.container.lock().await;
        move_cur_index(&container, 1).await;
        self
    }

    /// 向前移动当前焦点
    async fn prev(&self) -> &Self {
        let container = self.container.lock().await;
        move_cur_index(&container, -1).await;
        self
    }
}
