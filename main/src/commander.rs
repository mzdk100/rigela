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

pub(crate) mod hooks;
pub(crate) mod keyboard;

use crate::commander::keyboard::combo_keys::ComboKey;
use crate::commander::keyboard::KeyboardManager;
use crate::{
    commander::hooks::{set_keyboard_hook, set_mouse_hook},
    context::Context,
};
use std::{
    fmt::{Debug, Formatter},
    sync::{Arc, OnceLock, Weak},
};
use win_wrap::hook::WindowsHook;

/**
命令类型枚举。
*/
#[allow(dead_code)]
pub(crate) enum CommandType {
    // 键盘命令
    Key(ComboKey),
    // 触摸命令
    Touch,
    // 语音命令
    Voice,
}

/**
指挥官结构。
*/
pub(crate) struct Commander {
    keyboard_hook: OnceLock<WindowsHook>,
    mouse_hook: OnceLock<WindowsHook>,
    keyboard_manager: Arc<KeyboardManager>,
}

impl Commander {
    /**
    创建一个指挥官对象。
    负责收集用户的操作请求，例如快捷键、触摸动作、语音命令等，之后把这些命令调度给具体的任务。
    */
    pub(crate) fn new() -> Self {
        Self {
            keyboard_hook: Default::default(),
            mouse_hook: Default::default(),
            keyboard_manager: KeyboardManager::new().into(),
        }
    }

    /**
    让指挥官开始工作。
    `context` 框架上下文环境，可以通过此对象访问整个框架的所有API。
    */
    pub(crate) fn apply(&self, context: Weak<Context>) {
        self.keyboard_manager.apply(context.clone());
        self.keyboard_hook
            .set(set_keyboard_hook(context.clone()))
            .unwrap_or(());

        self.mouse_hook
            .set(set_mouse_hook(context.clone()))
            .unwrap_or(());
    }

    /// 获取键盘管理器
    pub(crate) fn get_keyboard_manager(&self) -> Arc<KeyboardManager> {
        self.keyboard_manager.clone()
    }
}

impl Drop for Commander {
    fn drop(&mut self) {
        self.keyboard_hook.get().unwrap().unhook();
        self.mouse_hook.get().unwrap().unhook();
    }
}

impl Debug for Commander {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Commander").finish()
    }
}
