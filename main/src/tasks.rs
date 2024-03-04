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

use std::{collections::HashMap, sync::RwLock};
use tokio::task::JoinHandle;

/// 任务管理器
#[derive(Debug)]
pub(crate) struct TaskManager {
    tasks: RwLock<HashMap<&'static str, JoinHandle<()>>>,
}

impl TaskManager {
    /**
     * 创建一个实例。
     * */
    pub(crate) fn new() -> Self {
        Self {
            tasks: Default::default(),
        }
    }

    /**
     * 推送一个任务到管理器，并给任务命名，相同名称的任务只能按顺序运行。
     * `name` 任务名称，例如当前如果有一个名字为a的任务正在运行，如果push了一个新任务a，那么将放弃之前的任务。
     * `task` 一个任务句柄。
     * */
    pub(crate) fn push(&self, name: &'static str, task: JoinHandle<()>) {
        self.abort(name);
        self.tasks.write().unwrap().insert(name, task);
    }

    /**
     * 放弃一个任务的运行。
     * `name` 任务名称，例如当前如果有一个名字为a的任务正在运行，如果push了一个新任务a，那么将放弃之前的任务。
     * */
    pub(crate) fn abort(&self, name: &'static str) {
        if let Some(j) = self.tasks.read().unwrap().get(name) {
            j.abort();
        }
    }
}
