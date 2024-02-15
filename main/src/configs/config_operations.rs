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

use crate::{commander::keys::Keys, configs::mouse::MouseConfig, context::Context};
use std::{collections::HashMap, sync::Arc};

// ------  鼠标配置  ------

/// 设置是否开启朗读鼠标
pub(crate) fn apply_mouse_config(context: Arc<Context>, is_read: bool) {
    let mut config = context.config_manager.get_config();
    config.mouse_config = MouseConfig { is_read };
    context.config_manager.set_config(config);
}

// ------  键盘配置  -------

/// 获取当前的热键配置
pub(crate) fn get_hotkeys(context: Arc<Context>) -> HashMap<String, Vec<Keys>> {
    context
        .config_manager
        .get_config()
        .hotkeys_config
        .talent_keys
        .clone()
}

///  存储热键配置
pub(crate) fn save_hotkeys(context: Arc<Context>, hotkeys: HashMap<String, Vec<Keys>>) {
    let mut config = context.config_manager.get_config();
    config.hotkeys_config.talent_keys = hotkeys;
    context.config_manager.set_config(config);
}
