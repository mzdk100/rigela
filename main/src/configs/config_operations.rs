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
    configs::mouse::MouseConfig,
    configs::tts::{TtsConfig, TtsProperty},
    context::Context,
};
use std::ops::DerefMut;
use std::sync::{Arc, Mutex, OnceLock};

// ------ TTS 配置操作 ------

/// 设置TTS的参数。 `diff` 属性值的差值， 传0初始化tts属性值
pub(crate) async fn apply_tts_config(context: Arc<Context>, diff: i32) {
    let performer = &context.performer.clone();
    let mut config = context.config_manager.get_config().await;
    let mut tts_config = config.tts_config.clone();

    // 如果差值等于0，直接设置TTS属性值参数，返回
    if diff == 0 {
        performer.set_tts_properties_with_sapi5(
            tts_config.speed,
            tts_config.volume,
            tts_config.pitch,
        );
        return;
    }

    let restrict = |x| match x {
        i if i > 100 => 100,
        i if i < 0 => 0,
        i => i,
    };

    tts_config = match get_cur_tts_prop() {
        TtsProperty::Speed => TtsConfig {
            speed: restrict(tts_config.speed + diff),
            ..tts_config
        },
        TtsProperty::Volume => TtsConfig {
            volume: restrict(tts_config.volume + diff),
            ..tts_config
        },
        TtsProperty::Pitch => TtsConfig {
            pitch: restrict(tts_config.pitch + diff),
            ..tts_config
        },
    };

    performer.set_tts_properties_with_sapi5(tts_config.speed, tts_config.volume, tts_config.pitch);
    config.tts_config = tts_config;
    context.config_manager.set_config(config).await;
}

// 当前调节的TTS属性, 内部静态值，有对外公开方法
fn cur_tts_prop() -> &'static Mutex<TtsProperty> {
    static INSTANCE: OnceLock<Mutex<TtsProperty>> = OnceLock::new();
    INSTANCE.get_or_init(|| Mutex::new(TtsProperty::Speed))
}

/// 获取当前调节的TTS属性
pub(crate) fn get_cur_tts_prop() -> TtsProperty {
    cur_tts_prop().lock().unwrap().clone()
}

/// 后移当前需调节的TTS属性
pub(crate) fn next_tts_prop() {
    let prop = cur_tts_prop().lock().unwrap().next();
    *cur_tts_prop().lock().unwrap().deref_mut() = prop;
}

/// 前移当前需调节的TTS属性
pub(crate) fn prev_tts_prop() {
    let prop = cur_tts_prop().lock().unwrap().prev();
    *cur_tts_prop().lock().unwrap().deref_mut() = prop;
}

// ------  鼠标配置  ------

/// 设置是否开启朗读鼠标
pub(crate) async fn apply_mouse_config(context: Arc<Context>, is_read: bool) {
    let mut config = context.config_manager.get_config().await;
    config.mouse_config = MouseConfig { is_read };
    context.config_manager.set_config(config).await;
}
