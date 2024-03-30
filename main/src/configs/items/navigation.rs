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

use serde::{Deserialize, Serialize};

/**
 * 导航模式。
 * */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) enum NavigationMode {
    /// 线性模式
    Linear,
    /// 平面模式
    Plane,
    /// 树状模式
    Tree,
}

impl Default for NavigationMode {
    fn default() -> Self {
        Self::Linear
    }
}

/// 导航配置项
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct NavigationConfig {
    /// 导航模式
    pub(crate) mode: NavigationMode,
}

impl Default for NavigationConfig {
    fn default() -> Self {
        Self {
            mode: NavigationMode::default(),
        }
    }
}
