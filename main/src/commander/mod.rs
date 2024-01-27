/*
 * Copyright (c) 2023. The RigelA open source project team and
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

mod hooks;
mod keys;

use crate::commander::hooks::{set_keyboard_hook, set_mouse_hook};
use crate::{context::Context, talent::Talented};
use std::sync::{Arc, Mutex};
use win_wrap::{hook::WindowsHook, input::VirtualKey};

type Talent = Arc<dyn Talented + Send + Sync>;

/**
 * 命令类型枚举。
 * */
#[allow(dead_code)]
pub(crate) enum CommandType {
    // 键盘命令
    Key(Vec<(VirtualKey, bool)>),
    // 触摸命令
    Touch,
    // 语音命令
    Voice,
}

/**
 * 指挥官结构。
 * */
#[derive(Clone, Debug)]
pub(crate) struct Commander {
    keyboard_hook: Arc<Mutex<Option<WindowsHook>>>,
    mouse_hook: Arc<Mutex<Option<WindowsHook>>>,
}

impl Commander {
    /**
     * 创建一个指挥官对象。
     * 负责收集用户的操作请求，例如快捷键、触摸动作、语音命令等，之后把这些命令调度给具体的任务。
     * */
    pub(crate) fn new() -> Self {
        Self {
            keyboard_hook: Arc::new(Mutex::new(None)),
            mouse_hook: Arc::new(Mutex::new(None)),
        }
    }

    /**
     * 让指挥官开始工作。
     * `context` 框架上下文环境，可以通过此对象访问整个框架的所有API。
     * */
    pub(crate) fn apply(&self, context: Arc<Context>) {
        let talents = context.talent_accessor.talents.clone();

        self.keyboard_hook
            .lock()
            .unwrap()
            .replace(set_keyboard_hook(context.clone(), talents));

        self.mouse_hook
            .lock()
            .unwrap()
            .replace(set_mouse_hook(context.clone()));
    }

    /**
     * 清理环境，后续不可以重复使用。
     * */
    pub(crate) fn dispose(&self) {
        self.keyboard_hook.lock().unwrap().clone().unwrap().unhook();
        self.mouse_hook.lock().unwrap().clone().unwrap().unhook();
    }
}
