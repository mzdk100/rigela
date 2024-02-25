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
use std::{collections::HashMap, sync::Arc};

// ------  鼠标配置  ------

/// 获取当前的朗读鼠标状态
pub(crate) fn get_mouse_read_state(context: Arc<Context>) -> bool {
    context.config_manager.get_config().mouse_config.is_read
}

/// 设置是否开启朗读鼠标
pub(crate) fn apply_mouse_config(context: Arc<Context>, is_read: bool) {
    let mut root = context.config_manager.get_config();
    root.mouse_config = MouseConfig { is_read };
    context.config_manager.set_config(&root);
}

// ------  键盘配置  -------

/// 获取当前的热键配置
pub(crate) fn get_hotkeys(context: Arc<Context>) -> HashMap<String, Vec<Keys>> {
    let root = context.config_manager.get_config();
    root.hotkeys_config.talent_keys.clone()
}

///  保存热键配置
pub(crate) fn save_hotkeys(context: Arc<Context>, hotkeys: HashMap<String, Vec<Keys>>) {
    let mut config = context.config_manager.get_config();
    config.hotkeys_config.talent_keys = hotkeys;
    context.config_manager.set_config(&config);
}

// ------  常规配置  -------

/// 获取是否开机自启
pub(crate) fn get_run_on_startup(context: Arc<Context>) -> bool {
    let root = context.config_manager.get_config();
    root.general_config.run_on_startup
}

/// 保存是否开机自启
pub(crate) fn save_run_on_startup(context: Arc<Context>, run_on_startup: bool) {
    let mut root = context.config_manager.get_config();
    root.general_config = GeneralConfig {
        run_on_startup,
        ..root.general_config
    };
    context.config_manager.set_config(&root);
}

/// 获取是否自动更新
pub(crate) fn get_auto_check_update(context: Arc<Context>) -> bool {
    let root = context.config_manager.get_config();
    root.general_config.auto_check_update
}

/// 保存是否自动更新
pub(crate) fn save_auto_check_update(context: Arc<Context>, auto_check_update: bool) {
    let mut root = context.config_manager.get_config();
    root.general_config = GeneralConfig {
        auto_check_update,
        ..root.general_config
    };
    context.config_manager.set_config(&root);
}

/// 获取当前语言
pub(crate) fn get_lang(context: Arc<Context>) -> Lang {
    let root = context.config_manager.get_config();
    root.general_config.lang
}

/// 保存当前语言
pub(crate) fn save_lang(context: Arc<Context>, lang: &Lang) {
    let mut root = context.config_manager.get_config();
    root.general_config = GeneralConfig {
        lang: lang.clone(),
        ..root.general_config
    };
    context.config_manager.set_config(&root);
}
