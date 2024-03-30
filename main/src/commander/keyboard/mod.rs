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

use crate::commander::keyboard::combo_keys::ComboKey;
use crate::commander::keyboard::combo_keys::State::{DoublePress, LongPress, SinglePress};
use crate::commander::keyboard::keys::Keys;
use crate::commander::keyboard::keys::Keys::VkNone;
use crate::context::Context;
use rust_i18n::AtomicStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Weak};
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};
use tokio::time::sleep;
use win_wrap::input::{send_key, VK_CAPITAL};

pub(crate) type KeyCallbackFn = Arc<dyn Fn(Keys, bool) + Send + Sync>;

/// 键盘管理器
pub(crate) struct Manager {
    // 元组包含： 按键键名， 是否按下
    pressed_cache: Arc<(AtomicStr, AtomicBool)>,
    release_cache: Arc<(AtomicStr, AtomicBool)>,
    last_pressed_key: AtomicStr,
    key_callback_fns: Mutex<Vec<(Vec<Keys>, KeyCallbackFn)>>,
    tokio_rt: Runtime,
}

macro_rules! change_cache {
    ($cache:expr, $key_str:expr, $state: expr) => {
        $cache.0.replace($key_str);
        $cache.1.store($state, Ordering::Relaxed);
    };
}

impl Manager {
    pub(crate) fn new() -> Manager {
        let key_none: &str = VkNone.into();

        Manager {
            pressed_cache: (AtomicStr::from(key_none), AtomicBool::new(false)).into(),
            release_cache: (AtomicStr::from(key_none), AtomicBool::new(false)).into(),
            last_pressed_key: AtomicStr::from(key_none),
            key_callback_fns: Mutex::new(vec![]),
            tokio_rt: Builder::new_multi_thread().enable_all().build().unwrap(),
        }
    }

    /// 组合键处理
    pub(crate) fn process_combo_key(&self, key: &ComboKey, pressed: bool) -> Option<ComboKey> {
        let main_key = Into::<&str>::into(key.main_key).to_string();
        let key_none: &str = VkNone.into();

        match pressed {
            true if main_key == self.pressed_cache.0.to_string()
                && self.pressed_cache.1.load(Ordering::Acquire) =>
            {
                // 产生双击，把按压缓存的状态设为默认
                change_cache!(self.pressed_cache, key_none, false);
                Some(key.change_state(DoublePress))
            }

            true => {
                // 第一次按下某个键， 按压缓存设为按下，释放缓存设为松开， 启动延时任务。
                change_cache!(self.pressed_cache, main_key.clone(), true);
                change_cache!(self.release_cache, main_key.clone(), false);

                //  200毫秒后， 按压状态松开
                self.pressed_delay();
                // 500毫秒后， 释放状态为长按
                self.release_delay(&key.main_key);

                Some(key.change_state(SinglePress))
            }

            false => {
                // 松开任意键， 复原释放缓存
                change_cache!(self.release_cache, key_none, false);

                match main_key == self.release_cache.0.to_string()
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
        let text = self.last_pressed_key.to_string();
        text.as_str().into()
    }

    /// 设置最后一次按下的键。
    /// `key` 键盘枚举。
    pub(crate) fn set_last_pressed_key(&self, key: &Keys) {
        let key: &str = key.clone().into();
        self.last_pressed_key.replace(key);
    }

    /// 添加键盘事件监听器
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
    pub(crate) fn capital_handle(context: Weak<Context>, state: bool, hook_toggle: &AtomicBool) {
        hook_toggle.store(true, Ordering::Relaxed);
        send_key(VK_CAPITAL);
        hook_toggle.store(false, Ordering::Relaxed);

        let context = unsafe { &*context.as_ptr() };
        let performer = context.performer.clone();
        context.work_runtime.spawn(async move {
            let info = if !state { "大写" } else { "小写" };
            performer.speak(&info.to_string()).await;
        });
    }

    // 按键按下延时处理
    fn pressed_delay(&self) {
        let pressed_cache = self.pressed_cache.clone();

        self.tokio_rt.spawn(async move {
            // 延时200毫秒后， 取消双击
            sleep(Duration::from_millis(200)).await;
            pressed_cache.1.store(false, Ordering::SeqCst);
        });
    }

    // 按键释放延时处理
    fn release_delay(&self, key: &Keys) {
        let release_cache = self.release_cache.clone();
        let key = Into::<&str>::into(key.clone()).to_string();

        self.tokio_rt.spawn(async move {
            sleep(Duration::from_millis(500)).await;
            // 如果持续500毫秒,释放缓存键和传入的按键相同，则释放的缓存状态为长按
            if key == release_cache.0.to_string() {
                release_cache.1.store(true, Ordering::SeqCst);
            }
        });
    }
}
