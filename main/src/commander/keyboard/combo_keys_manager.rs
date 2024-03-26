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

use crate::{
    commander::keyboard::combo_keys::{
        ComboKey, ComboKeyExt,
        State::{DoublePress, LongPress, SinglePress},
    },
    context::Context,
};
use std::{
    sync::{Arc, Mutex, Weak},
    time::Duration,
};
use tokio::time::sleep;

#[allow(unused)]
pub(crate) struct ComboKeysManage {
    pressed_cache: Arc<Mutex<ComboKeyExt>>,
    release_cache: Arc<Mutex<ComboKeyExt>>,
}

#[allow(unused)]
impl ComboKeysManage {
    pub(crate) fn new() -> ComboKeysManage {
        ComboKeysManage {
            pressed_cache: Arc::new(Mutex::new(ComboKeyExt::default())),
            release_cache: Arc::new(Mutex::new(ComboKeyExt::default())),
        }
    }

    /// 组合键处理
    pub(crate) fn process_combo_key(
        &self,
        context: Weak<Context>,
        key: ComboKey,
        pressed: bool,
    ) -> Option<ComboKey> {
        let mut pressed_cache = self.pressed_cache.lock().unwrap();
        let mut release_cache = self.release_cache.lock().unwrap();

        match pressed {
            true if key.main_key == pressed_cache.main_key
                && pressed_cache.state == SinglePress =>
                {
                    *pressed_cache = Default::default();
                    Some(ComboKey {
                        state: DoublePress,
                        ..key
                    })
                }
            true => {
                *pressed_cache = ComboKey {
                    state: SinglePress,
                    ..key
                }
                    .into();
                *release_cache = key.clone().into();

                //  200毫秒后， pressed_cache.count Default::default
                self.pressed_delay(context.clone());
                // 500毫秒后， 如果 release.count != Default::default State = LongPress
                self.release_delay(context.clone(), &key);

                Some(ComboKey {
                    state: SinglePress,
                    ..key
                })
            }
            false => {
                if release_cache.main_key == key.main_key && release_cache.state == LongPress {
                    *release_cache = Default::default();
                    Some(ComboKey {
                        state: LongPress,
                        ..key
                    })
                } else {
                    *release_cache = Default::default();
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
                *pressed_cache.lock().unwrap() = Default::default();
            });
    }

    // 按键释放延时处理
    fn release_delay(&self, context: Weak<Context>, combo_key: &ComboKey) {
        let release_cache = self.release_cache.clone();
        let combo_key = combo_key.clone();

        unsafe { &*context.as_ptr() }
            .work_runtime
            .spawn(async move {
                sleep(Duration::from_millis(500)).await;
                let k = { release_cache.lock().unwrap().main_key.clone() };
                if combo_key.main_key == k {
                    let mut ck = release_cache.lock().unwrap();
                    *ck = ComboKey {
                        state: LongPress,
                        ..combo_key
                    }
                        .into();
                }
            });
    }
}
