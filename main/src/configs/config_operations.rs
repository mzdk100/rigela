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

use crate::configs::general::{GeneralConfig, Lang};
use crate::{commander::keys::Keys, configs::mouse::MouseConfig, context::Context};
use std::{collections::HashMap, sync::Weak};

macro_rules! get_cfg {
    ($context:expr) => {
        unsafe { &*($context.as_ptr()) }.config_manager.get_config()
    };
}

macro_rules! set_cfg {
    ($context:expr, $cfg: expr) => {
        let context = unsafe { &*($context.as_ptr()) };
        context.config_manager.set_config(&$cfg);
    };
}

// ------  鼠标配置  ------

/// 获取当前的朗读鼠标状态
pub(crate) fn get_mouse_read_state(context: Weak<Context>) -> bool {
    get_cfg!(context).mouse_config.is_read
}

/// 设置是否开启朗读鼠标
pub(crate) fn apply_mouse_config(context: Weak<Context>, is_read: bool) {
    let mut cfg = get_cfg!(context);
    cfg.mouse_config = MouseConfig { is_read };
    set_cfg!(context, cfg);
}

// ------  键盘配置  -------

/// 获取当前的热键配置
pub(crate) fn get_hotkeys(context: Weak<Context>) -> HashMap<String, Vec<Keys>> {
    get_cfg!(context).hotkeys_config.talent_keys.clone()
}

///  保存热键配置
pub(crate) fn save_hotkeys(context: Weak<Context>, hotkeys: HashMap<String, Vec<Keys>>) {
    let mut cfg = get_cfg!(context);
    cfg.hotkeys_config.talent_keys = hotkeys;
    set_cfg!(context, cfg);
}

// ------  常规配置  -------

/// 获取是否显示桌面快捷方式
pub(crate) fn get_is_display_shortcut(context: Weak<Context>) -> bool {
    get_cfg!(context).general_config.desktop_shortcut
}

/// 保存是否显示桌面快捷方式
pub(crate) fn save_is_display_shortcut(context: Weak<Context>, desktop_shortcut: bool) {
    let mut cfg = get_cfg!(context);
    cfg.general_config = GeneralConfig {
        desktop_shortcut,
        ..cfg.general_config
    };
    set_cfg!(context, cfg);
}

/// 获取是否开机自启
pub(crate) fn get_run_on_startup(context: Weak<Context>) -> bool {
    get_cfg!(context).general_config.run_on_startup
}

/// 保存是否开机自启
pub(crate) fn save_run_on_startup(context: Weak<Context>, run_on_startup: bool) {
    let mut cfg = get_cfg!(context);
    cfg.general_config = GeneralConfig {
        run_on_startup,
        ..cfg.general_config
    };
    set_cfg!(context, cfg);
}

/// 获取是否自动更新
pub(crate) fn get_auto_check_update(context: Weak<Context>) -> bool {
    get_cfg!(context).general_config.auto_check_update
}

/// 保存是否自动更新
pub(crate) fn save_auto_check_update(context: Weak<Context>, auto_check_update: bool) {
    let mut cfg = get_cfg!(context);
    cfg.general_config = GeneralConfig {
        auto_check_update,
        ..cfg.general_config
    };
    set_cfg!(context, cfg);
}

/// 获取当前语言
pub(crate) fn get_lang(context: Weak<Context>) -> Lang {
    get_cfg!(context).general_config.lang
}

/// 保存当前语言
pub(crate) fn save_lang(context: Weak<Context>, lang: &Lang) {
    let mut cfg = get_cfg!(context);
    cfg.general_config = GeneralConfig {
        lang: lang.clone(),
        ..cfg.general_config
    };
    set_cfg!(context, cfg);
}
