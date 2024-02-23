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
pub(crate) mod keys;

use crate::{
    commander::hooks::{set_keyboard_hook, set_mouse_hook},
    context::Context,
    talent::Talented,
};
use keys::Keys;
use std::sync::{Arc, Mutex, OnceLock};
use win_wrap::hook::WindowsHook;

type Talent = Arc<dyn Talented + Send + Sync>;

/**
 * 命令类型枚举。
 * */
#[allow(dead_code)]
pub(crate) enum CommandType {
    // 键盘命令
    Key(Vec<Keys>),
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
    keyboard_hook: OnceLock<WindowsHook>,
    mouse_hook: OnceLock<WindowsHook>,
    last_pressed_key: Arc<Mutex<Keys>>,
}

impl Commander {
    /**
     * 创建一个指挥官对象。
     * 负责收集用户的操作请求，例如快捷键、触摸动作、语音命令等，之后把这些命令调度给具体的任务。
     * */
    pub(crate) fn new() -> Self {
        Self {
            keyboard_hook: OnceLock::new(),
            mouse_hook: OnceLock::new(),
            last_pressed_key: Mutex::new(Keys::VkNone).into(),
        }
    }

    /**
     * 让指挥官开始工作。
     * `context` 框架上下文环境，可以通过此对象访问整个框架的所有API。
     * */
    pub(crate) fn apply(&self, context: Arc<Context>) {
        let talents = context.talent_provider.talents.clone();

        self.keyboard_hook
            .set(set_keyboard_hook(context.clone(), talents))
            .unwrap_or(());

        self.mouse_hook
            .set(set_mouse_hook(context.clone()))
            .unwrap_or(());
    }

    /**
     * 清理环境，后续不可以重复使用。
     * */
    pub(crate) fn dispose(&self) {
        self.keyboard_hook.get().unwrap().unhook();
        self.mouse_hook.get().unwrap().unhook();
    }

    /**
     * 获取最后一次按下的键。
     * */
    pub(crate) fn get_last_pressed_key(&self) -> Keys {
        *self.last_pressed_key.lock().unwrap()
    }

    /**
     * 设置最后一次按下的键。
     * `key` 键盘枚举。
     * */
    pub(crate) fn set_last_pressed_key(&self, key: Keys) {
        *self.last_pressed_key.lock().unwrap() = key;
    }
}
