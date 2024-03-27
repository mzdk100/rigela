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

use crate::commander::keyboard::keys::Keys;
use crate::commander::keyboard::keys::Keys::VkNone;
use crate::{
    commander::keyboard::combo_keys::{
        ComboKey,
        State::{DoublePress, LongPress, SinglePress},
    },
    context::Context,
};
use rust_i18n::AtomicStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    sync::{Arc, Weak},
    time::Duration,
};
use tokio::time::sleep;

pub(crate) struct ComboKeysManage {
    // 元组包含： 按键键名， 是否按下
    pressed_cache: Arc<(AtomicStr, AtomicBool)>,
    release_cache: Arc<(AtomicStr, AtomicBool)>,
}

#[allow(unused)]
impl ComboKeysManage {
    pub(crate) fn new() -> ComboKeysManage {
        let key: &str = VkNone.into();
        ComboKeysManage {
            pressed_cache: (AtomicStr::from(key), AtomicBool::new(false)).into(),
            release_cache: (AtomicStr::from(key), AtomicBool::new(false)).into(),
        }
    }

    /// 组合键处理
    pub(crate) fn process_combo_key(
        &self,
        context: Weak<Context>,
        key: &ComboKey,
        pressed: bool,
    ) -> Option<ComboKey> {
        let main_key: &str = key.main_key.into();
        match pressed {
            true if main_key.to_string() == self.pressed_cache.0.to_string().clone()
                && self.pressed_cache.1.load(Ordering::Acquire) =>
            {
                // 产生双击，把按压缓存的状态设为默认
                let key_none: &str = VkNone.into();
                self.pressed_cache.0.replace(key_none);
                self.pressed_cache.1.store(false, Ordering::Relaxed);

                Some(ComboKey {
                    state: DoublePress,
                    ..key.clone()
                })
            }

            true => {
                // 第一次按下某个键， 按压缓存设为按下，释放缓存设为松开， 启动延时任务。
                self.pressed_cache.0.replace(main_key);
                self.pressed_cache.1.store(true, Ordering::Relaxed);

                self.release_cache.0.replace(main_key);
                self.release_cache.1.store(false, Ordering::Relaxed);

                //  200毫秒后， 按压状态松开
                self.pressed_delay(context.clone());
                // 500毫秒后， 释放状态为长按
                self.release_delay(context.clone(), &key.main_key);

                Some(ComboKey {
                    state: SinglePress,
                    ..key.clone()
                })
            }

            false => {
                if main_key == self.release_cache.0.to_string().clone()
                    && self.release_cache.1.load(Ordering::Acquire)
                {
                    // 如果产生长按， 复原释放缓存
                    let key_none: &str = VkNone.into();
                    self.release_cache.0.replace(key_none);
                    self.release_cache.1.store(false, Ordering::Relaxed);

                    Some(ComboKey {
                        state: LongPress,
                        ..key.clone()
                    })
                } else {
                    // 松开任意键， 复原释放缓存
                    let key_none: &str = VkNone.into();
                    self.release_cache.0.replace(key_none);
                    self.release_cache.1.store(false, Ordering::Relaxed);

                    None
                }
            }
        }
    }

    // 按键按下延时处理
    fn pressed_delay(&self, context: Weak<Context>) {
        let pressed_cache = self.pressed_cache.clone();

        unsafe { &*context.as_ptr() }
            .work_runtime
            .spawn(async move {
                sleep(Duration::from_millis(200)).await;

                // 延时200毫秒后， 取消双击
                pressed_cache.1.store(false, Ordering::Relaxed);
            });
    }

    // 按键释放延时处理
    fn release_delay(&self, context: Weak<Context>, key: &Keys) {
        let release_cache = self.release_cache.clone();
        let key = key.clone();

        unsafe { &*context.as_ptr() }
            .work_runtime
            .spawn(async move {
                sleep(Duration::from_millis(500)).await;

                let key: &str = key.into();
                let key_cache = release_cache.0.to_string();
                if key == key_cache.as_str() {
                    // 如果持续500毫秒,释放缓存键和传入的按键相同，则释放的缓存状态为长按
                    release_cache.1.store(true, Ordering::Relaxed);
                }
            });
    }
}
