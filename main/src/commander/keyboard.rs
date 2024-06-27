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

pub(crate) mod combo_keys;
pub(crate) mod keys;
pub(crate) mod modify_keys;

use crate::{
    commander::keyboard::{
        combo_keys::{
            ComboKey,
            State::{DoublePress, LongPress, SinglePress},
        },
        keys::Keys,
    },
    context::{Context, ContextAccessor},
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, OnceLock, Weak,
    },
    time::Duration,
};
use tokio::time::sleep;
use win_wrap::input::{send_key, VK_CAPITAL};

pub(crate) type KeyCallbackFn = Arc<dyn Fn(Keys, bool) + Send + Sync>;

/// 键盘管理器
pub(crate) struct KeyboardManager {
    // 元组包含： 按键键名， 是否按下
    pressed_cache: Arc<(Mutex<Keys>, AtomicBool)>,
    release_cache: Arc<(Mutex<Keys>, AtomicBool)>,
    last_pressed_key: Mutex<Keys>,
    key_callback_fns: Mutex<Vec<(Vec<Keys>, KeyCallbackFn)>>,
    context: OnceLock<Weak<Context>>,
}

macro_rules! change_cache {
    ($cache:expr, $key_str:expr, $state: expr) => {{
        *$cache.0.lock().unwrap() = $key_str
    };
    $cache.1.store($state, Ordering::Relaxed);};
}

impl KeyboardManager {
    pub(crate) fn new() -> Self {
        Self {
            pressed_cache: (Keys::VkNone.into(), AtomicBool::new(false)).into(),
            release_cache: (Keys::VkNone.into(), AtomicBool::new(false)).into(),
            last_pressed_key: Keys::VkNone.into(),
            key_callback_fns: Mutex::new(vec![]),
            context: OnceLock::new(),
        }
    }

    //noinspection StructuralWrap
    /**
     * 设置上下文环境。
     * `context` 读屏的上下文环境。
     * */
    pub(crate) fn apply(&self, context: Weak<Context>) {
        self.context.set(context).unwrap();
    }

    /// 组合键处理
    pub(crate) fn process_combo_key(&self, key: &ComboKey, pressed: bool) -> Option<ComboKey> {
        match pressed {
            true if key.main_key == { self.pressed_cache.0.lock().unwrap().clone() }
                && self.pressed_cache.1.load(Ordering::Acquire) =>
            {
                // 产生双击，把按压缓存的状态设为默认
                change_cache!(self.pressed_cache, Keys::VkNone, false);
                Some(key.change_state(DoublePress))
            }

            true => {
                // 第一次按下某个键， 按压缓存设为按下，释放缓存设为松开， 启动延时任务。
                change_cache!(self.pressed_cache, key.main_key.clone(), true);
                change_cache!(self.release_cache, key.main_key.clone(), false);

                //  200毫秒后， 按压状态松开
                self.pressed_delay();
                // 500毫秒后， 释放状态为长按
                self.release_delay(&key.main_key);

                Some(key.change_state(SinglePress))
            }

            false => {
                // 松开任意键， 复原释放缓存
                change_cache!(self.release_cache, Keys::VkNone, false);

                match key.main_key == { *self.release_cache.0.lock().unwrap() }
                    && self.release_cache.1.load(Ordering::Acquire)
                {
                    true => Some(key.change_state(LongPress)),
                    false => None,
                }
            }
        }
    }

    /// 获取最后一次按下的键。
    pub(crate) fn get_last_pressed_key(&self) -> Keys {
        { *self.last_pressed_key.lock().unwrap() }.into()
    }

    //noinspection StructuralWrap
    /**
     * 设置最后一次按下的键。
     * `key` 键盘枚举。
     * */
    pub(crate) fn set_last_pressed_key(&self, key: &Keys) {
        *self.last_pressed_key.lock().unwrap() = key.clone();
    }

    /**
     * 添加键盘事件监听器
     * `keys` 要监听的热键。
     * `listener` 一个监听器函数。
     * */
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

    /// 获取键盘事件监听器
    pub(crate) fn get_key_callback_fns(&self) -> Vec<(Vec<Keys>, KeyCallbackFn)> {
        self.key_callback_fns.lock().unwrap().clone()
    }

    // 处理大小写锁定键
    pub(crate) fn capital_handle(&self, state: bool, hook_toggle: &AtomicBool) {
        hook_toggle.store(true, Ordering::Relaxed);
        send_key(VK_CAPITAL);
        hook_toggle.store(false, Ordering::Relaxed);

        let Some(context) = self.context.get() else {
            return;
        };
        let ctx = context.clone();
        context.get_work_runtime().spawn(async move {
            let info = if !state { "大写" } else { "小写" };
            ctx.get_performer().speak(&info.to_string()).await;
        });
    }

    // 按键按下延时处理
    fn pressed_delay(&self) {
        let Some(context) = self.context.get() else {
            return;
        };
        let pressed_cache = self.pressed_cache.clone();

        context.get_work_runtime().spawn(async move {
            // 延时200毫秒后， 取消双击
            sleep(Duration::from_millis(200)).await;
            pressed_cache.1.store(false, Ordering::SeqCst);
        });
    }

    // 按键释放延时处理
    fn release_delay(&self, key: &Keys) {
        let Some(context) = self.context.get() else {
            return;
        };
        let release_cache = self.release_cache.clone();
        let key2 = key.clone();

        context.get_work_runtime().spawn(async move {
            sleep(Duration::from_millis(500)).await;
            // 如果持续500毫秒,释放缓存键和传入的按键相同，则释放的缓存状态为长按
            if key2 == { *release_cache.0.lock().unwrap() } {
                release_cache.1.store(true, Ordering::SeqCst);
            }
        });
    }
}
