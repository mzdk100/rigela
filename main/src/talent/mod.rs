/*
 * Copyright (c) 2023. The RigelA open source project team and
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

pub(crate) mod mouse;
mod navigator;
mod program;
mod tts;

use crate::commander::keyboard::combo_keys::ComboKey;
use crate::configs::config_operations::get_hotkeys;
use crate::{
    commander::CommandType,
    context::Context,
    talent::{
        mouse::{ClickTalent, ReadMouseTalent, RightClickTalent},
        navigator::{
            CurrChildElementTalent, CurrElementTalent, ModeNextTalent, ModePrevTalent,
            NextChildElementTalent, NextElementTalent, PrevChildElementTalent, PrevElementTalent,
        },
        program::{
            CurrentCpuUsageTalent, CurrentTimeTalent, ExitTalent, HotkeysTalent, PopupMenuTalent,
            StopTtsOutputTalent, ViewFocusTalent, ViewWindowTitleTalent,
        },
        tts::{
            CacheToClipboardTalent, IncreaseTalent, MakeWordCacheCharTalent, NextCacheCharTalent,
            NextPropTalent, PrevCacheCharTalent, PrevPropTalent, ReduceTalent,
            TransCacheCharTalent,
        },
    },
};
use async_trait::async_trait;
use std::collections::HashMap;
use std::{
    fmt::{Debug, Formatter},
    sync::{Arc, Mutex, Weak},
};

/**
 * 一个能力的抽象接口。
 * 可以使用rigela-macros中的talent属性宏标记在async fn函数上自动实现。
 * */
#[async_trait]
pub(crate) trait Talented {
    /**
     * 获取能力的ID。
     * */
    fn get_id(&self) -> String;

    /**
     * 获取能力的描述文字。
     * */
    fn get_doc(&self) -> String;

    /**
     * 获取能力可支持的命令类型。
     * */
    fn get_supported_cmd_list(&self) -> Vec<CommandType>;

    // 获取热键
    fn get_combo_key(&self) -> Option<ComboKey>;

    /**
     * 执行能力的入口方法。
     * `context` 框架的上下文环境。
     * */
    async fn perform(&self, context: Weak<Context>);
}

/// 能力提供者，包含所有能力对象列表
pub(crate) struct TalentProvider {
    // 技能对象集合
    talents: HashMap<String, Arc<dyn Talented + Send + Sync + 'static>>,
    combo_key_map: HashMap<ComboKey, String>,
    custom_combo_key_map: Mutex<HashMap<ComboKey, String>>,
}

macro_rules! add_talent {
    ($talents:ident, $combo_key_map: ident, $talent:expr) => {
        let talent = Arc::new($talent);
        let id = talent.get_id();
        let combo_key = talent.get_combo_key();

        $talents.insert(id, talent);

        if let Some(combo_key) = combo_key {
            $combo_key_map.insert(combo_key, id);
        }
    };
}

impl TalentProvider {
    /**
     * 创建能力访问器。
     * */
    pub(crate) fn new() -> Self {
        let mut talents = HashMap::<String, Arc<dyn Talented + Send + Sync + 'static>>::new();
        let mut combo_key_map = HashMap::<ComboKey, String>::new();

        add_talent!(talents, combo_key_map, ExitTalent);
        add_talent!(talents, combo_key_map, CurrentTimeTalent);
        add_talent!(talents, combo_key_map, CurrentCpuUsageTalent);
        add_talent!(talents, combo_key_map, PopupMenuTalent);
        add_talent!(talents, combo_key_map, HotkeysTalent);
        add_talent!(talents, combo_key_map, ViewFocusTalent);
        add_talent!(talents, combo_key_map, ViewWindowTitleTalent);
        add_talent!(talents, combo_key_map, StopTtsOutputTalent);
        // 窗口浏览能力
        add_talent!(talents, combo_key_map, ModePrevTalent);
        add_talent!(talents, combo_key_map, ModeNextTalent);
        add_talent!(talents, combo_key_map, PrevElementTalent);
        add_talent!(talents, combo_key_map, NextElementTalent);
        add_talent!(talents, combo_key_map, CurrElementTalent);
        add_talent!(talents, combo_key_map, PrevChildElementTalent);
        add_talent!(talents, combo_key_map, NextChildElementTalent);
        add_talent!(talents, combo_key_map, CurrChildElementTalent);
        // 语音调节能力
        add_talent!(talents, combo_key_map, IncreaseTalent);
        add_talent!(talents, combo_key_map, ReduceTalent);
        add_talent!(talents, combo_key_map, NextPropTalent);
        add_talent!(talents, combo_key_map, PrevPropTalent);
        add_talent!(talents, combo_key_map, PrevCacheCharTalent);
        add_talent!(talents, combo_key_map, NextCacheCharTalent);
        add_talent!(talents, combo_key_map, TransCacheCharTalent);
        add_talent!(talents, combo_key_map, MakeWordCacheCharTalent);
        add_talent!(talents, combo_key_map, CacheToClipboardTalent);
        // 鼠标能力
        add_talent!(talents, combo_key_map, ClickTalent);
        add_talent!(talents, combo_key_map, RightClickTalent);
        add_talent!(talents, combo_key_map, ReadMouseTalent);

        Self {
            talents,
            combo_key_map,
            custom_combo_key_map: Mutex::new(HashMap::<ComboKey, String>::new()),
        }
    }

    pub(crate) fn get_all_talents(&self) -> Vec<Arc<dyn Talented + Send + Sync + 'static>> {
        self.talents.values().cloned().collect()
    }

    pub(crate) fn get_talent_by_id(
        &self,
        id: &str,
    ) -> Option<Arc<dyn Talented + Send + Sync + 'static>> {
        self.talents.get(id).map(|t| t.clone())
    }

    pub(crate) fn update_custom_combo_key_map(&self, context: Weak<Context>) {
        *self.custom_combo_key_map.lock().unwrap() = get_hotkeys(context);
    }

    pub(crate) fn get_talent_by_combo_key(
        &self,
        combo_key: &ComboKey,
    ) -> Option<Arc<dyn Talented + Send + Sync + 'static>> {
        if let Some(id) = self.custom_combo_key_map.lock().unwrap().get(combo_key) {
            return self.get_talent_by_id(id);
        }
        self.combo_key_map
            .get(combo_key)
            .and_then(|id| self.get_talent_by_id(id))
    }
}
impl Debug for TalentProvider {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TalentAccessor({})", self.talents.len())
    }
}
