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

mod dialog;
pub(crate) mod editor;
mod element;
mod focus;
mod ime;
mod input;
mod progress;

use std::{
    fmt::{Debug, Formatter},
    sync::{Arc, Weak},
    time::{Duration, SystemTime},
};

use tokio::sync::Mutex;

use crate::{
    context::Context,
    event_core::{
        dialog::subscribe_dialog_events, editor::subscribe_editor_events,
        element::subscribe_element_events, focus::subscribe_focus_events,
        ime::subscribe_ime_events, input::subscribe_input_events,
        progress::subscribe_progress_events,
    },
};

/// 事件过滤器
#[derive(Debug)]
pub(crate) struct EventItem {
    same: String,
    time: SystemTime,
}

/// 事件处理中心
#[derive(Clone)]
pub(crate) struct EventCore {
    filter: Arc<Mutex<Vec<EventItem>>>,
}

impl EventCore {
    pub(crate) fn new() -> Self {
        Self {
            filter: Arc::new(vec![].into()),
        }
    }

    //noinspection StructuralWrap
    /**
     * 给定一个事件的特征，判断是否应该忽略此事件。
     * `same` 事件的特征文字。
     * `interval` 一个时间内如果此事件出现过，则表示他应该被忽略。
     * */
    pub(crate) async fn should_ignore(&self, same: String, interval: Duration) -> bool {
        let item = EventItem {
            same: same.clone(),
            time: SystemTime::now(),
        };
        let mut lock = self.filter.lock().await;
        for i in lock.iter() {
            if i.same == same && i.time.elapsed().unwrap() < interval {
                return true;
            }
        }
        for (i, j) in lock.iter().enumerate() {
            if j.same == same {
                lock.remove(i);
                break;
            }
        }
        lock.push(item);
        return false;
    }

    /// 启动事件监听
    pub(crate) async fn run(&self, context: Weak<Context>) {
        // 订阅UIA的焦点元素改变事件
        subscribe_focus_events(context.clone()).await;

        // 订阅对话框事件
        subscribe_dialog_events(context.clone()).await;

        // 订阅元素改变事件
        subscribe_element_events(context.clone()).await;

        // 订阅输入事件
        subscribe_input_events(context.clone()).await;

        // 订阅输入法候选事件
        subscribe_ime_events(context.clone()).await;

        // 订阅编辑框事件
        subscribe_editor_events(context.clone()).await;

        // 订阅进度栏事件
        subscribe_progress_events(context.clone()).await;
    }

    /**
     * 停止所有事件处理。
     * */
    pub(crate) fn shutdown(&self) {}
}

impl Debug for EventCore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventCore").finish()
    }
}
