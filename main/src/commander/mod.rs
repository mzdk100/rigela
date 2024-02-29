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
use std::{
    fmt::{Debug, Formatter},
    sync::{Arc, Mutex, OnceLock},
};
use win_wrap::hook::WindowsHook;

type Talent = Arc<dyn Talented + Send + Sync>;
type KeyCallbackFn = Arc<dyn Fn(Keys, bool) + Send + Sync>;

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
pub(crate) struct Commander {
    keyboard_hook: OnceLock<WindowsHook>,
    mouse_hook: OnceLock<WindowsHook>,
    last_pressed_key: Arc<Mutex<Keys>>,
    key_callback_fns: Mutex<Vec<(Vec<Keys>, KeyCallbackFn)>>,
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
            key_callback_fns: Mutex::new(Vec::new()),
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
    pub(crate) fn set_last_pressed_key(&self, key: &Keys) {
        *self.last_pressed_key.lock().unwrap() = key.clone();

        // let callbacks = { self.key_callback_fns.lock().unwrap().clone() };
        // 到这里lock已经没有生存期了，由于每一个callback的函数行为未知，所以一定要在没有锁的情况下执行他们
        // for callback in callbacks.iter() {
        //     if callback.0.contains(key) {
        //         callback.1(key.clone());
        //     }
        // }
    }

    pub(crate) fn add_key_event_listener(
        &self,
        keys: &[Keys],
        listener: impl Fn(Keys, bool) + Sync + Send + 'static,
    ) {
        self.key_callback_fns
            .lock()
            .unwrap()
            .push((Vec::from(keys), Arc::new(listener)));
    }

    pub(crate) fn get_key_callback_fns(&self) -> Vec<(Vec<Keys>, KeyCallbackFn)> {
        self.key_callback_fns.lock().unwrap().clone()
    }
}

impl Debug for Commander {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Commander").finish()
    }
}
