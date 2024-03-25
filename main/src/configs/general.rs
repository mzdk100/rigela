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
use std::fmt::Display;
use win_wrap::common::get_user_default_locale_name;

/// 常规配置项
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct GeneralConfig {
    pub(crate) run_on_startup: bool,
    pub(crate) auto_check_update: bool,
    pub(crate) lang: Lang,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            run_on_startup: false,
            auto_check_update: true,
            lang: Lang::Zh,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialOrd, PartialEq)]
pub(crate) enum Lang {
    FollowSystem,
    Zh,
    En,
}

impl Into<String> for Lang {
    fn into(self) -> String {
        match self {
            Self::Zh => "zh-CN".to_string(),
            Self::En => "en".to_string(),
            Self::FollowSystem => get_user_default_locale_name(),
        }
    }
}

impl<S: AsRef<str>> From<S> for Lang {
    fn from(value: S) -> Self {
        let value = value.as_ref();
        match value {
            "zh-CN" => Self::Zh,
            "en" => Self::En,
            _ => Self::FollowSystem,
        }
    }
}

impl Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FollowSystem => write!(f, "Lang ([FollowSystem]{})", self.to_string()),
            _ => write!(f, "Lang ({})", self.to_string()),
        }
    }
}

impl Default for Lang {
    fn default() -> Self {
        Self::FollowSystem
    }
}
