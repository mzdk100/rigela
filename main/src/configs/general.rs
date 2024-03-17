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

/// 常规配置项
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct GeneralConfig {
    pub(crate) desktop_shortcut: bool,
    pub(crate) run_on_startup: bool,
    pub(crate) auto_check_update: bool,
    pub(crate) lang: Lang,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            desktop_shortcut: false,
            run_on_startup: false,
            auto_check_update: true,
            lang: Lang::Zh,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialOrd, PartialEq)]
pub(crate) enum Lang {
    Zh,
    En,
}
