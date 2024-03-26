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

use crate::commander::keyboard::combo_keys::State::{DoublePress, LongPress, SinglePress};
use crate::commander::keyboard::combo_keys::{ComboKey, ComboKeyExt};
use std::sync::{Arc, Mutex};

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

    pub(crate) fn process_combo_key(&self, key: ComboKey, pressed: bool) -> Option<ComboKey> {
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

                // Todo
                //  200毫秒后， pressed_cache.count Default::default
                // 500毫秒后， 如果 release.count != Default::default State = LongPress

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
}
