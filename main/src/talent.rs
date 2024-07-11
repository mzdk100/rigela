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

use crate::{
    commander::{keyboard::combo_keys::ComboKey, CommandType},
    configs::operations::get_hotkeys,
    context::Context,
    talent::{
        mouse::{ClickTalent, ReadMouseTalent, RightClickTalent},
        navigator::{
            ElementColorSetTalent, ElementCurrentLineTalent, ElementCurrentTalent,
            ElementNextLineTalent, ElementNextTalent, ElementPrevLineTalent, ElementPrevTalent,
            ModeNextTalent, ModePrevTalent,
        },
        program::{
            CurrentCpuUsageTalent, CurrentDateTalent, CurrentTimeTalent, ExitTalent, HotkeysTalent,
            PopupMenuTalent, StopTtsOutputTalent, ViewFocusTalent, ViewWindowTitleTalent,
        },
        tts::{
            CacheToClipboardTalent, IncreaseTalent, MakeWordCacheCharTalent, NextCacheCharTalent,
            NextPropTalent, PrevCacheCharTalent, PrevPropTalent, ReduceTalent,
            TransCacheCharTalent,
        },
    },
};
use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
    sync::{Arc, Mutex, Weak},
};

pub(crate) type Talent = Arc<dyn Talented + Send + Sync + 'static>;

/**
 * 一个能力的抽象接口。
 * 可以使用rigela-macros中的talent属性宏标记在async fn函数上自动实现。
 * */
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
    fn perform(&self, context: Weak<Context>);
}

/// 能力提供者，包含所有能力对象列表
pub(crate) struct TalentProvider {
    // 能力对象集合
    talents: HashMap<String, Talent>,
    // 能力ID列表，使能力保持有序
    talent_ids: Vec<String>,
    // 热键能力映射，加速热键能力获取
    combo_key_map: Mutex<HashMap<ComboKey, String>>,
}

macro_rules! make_talents {
    ($talents:ident, $combo_key_map: ident, $($talent:expr),*) => {{
        let mut talent_ids = Vec::new();

        $(
            let talent = Arc::new($talent);
            let id = talent.get_id();
            let combo_key = talent.get_combo_key();

            $talents.insert(id.clone(), talent);
            talent_ids.push(id.clone());

            if let Some(combo_key) = combo_key {
                $combo_key_map.insert(combo_key, id.clone());
            }
        )*
        talent_ids
    }};
}

impl TalentProvider {
    /**
     * 创建能力访问器。
     * */
    pub(crate) fn new() -> Self {
        let mut talents = HashMap::<String, Talent>::new();
        let mut combo_key_map = HashMap::<ComboKey, String>::new();

        let talent_ids = make_talents!(
            talents,
            combo_key_map,
            // 常用能力
            ExitTalent,
            CurrentTimeTalent,
            CurrentDateTalent,
            CurrentCpuUsageTalent,
            PopupMenuTalent,
            HotkeysTalent,
            ViewFocusTalent,
            ViewWindowTitleTalent,
            StopTtsOutputTalent,
            // 导航器能力
            ModePrevTalent,
            ModeNextTalent,
            ElementPrevTalent,
            ElementNextTalent,
            ElementCurrentTalent,
            ElementPrevLineTalent,
            ElementNextLineTalent,
            ElementCurrentLineTalent,
            ElementColorSetTalent,
            // 语音调节能力
            IncreaseTalent,
            ReduceTalent,
            NextPropTalent,
            PrevPropTalent,
            // 语音缓冲区能力
            PrevCacheCharTalent,
            NextCacheCharTalent,
            TransCacheCharTalent,
            MakeWordCacheCharTalent,
            CacheToClipboardTalent,
            // 鼠标能力
            ClickTalent,
            RightClickTalent,
            ReadMouseTalent
        );

        Self {
            talents,
            talent_ids,
            combo_key_map: Mutex::new(combo_key_map),
        }
    }

    /// 获取全部能力项
    #[allow(unused)]
    pub(crate) fn get_talents(&self) -> Vec<Talent> {
        self.talents.values().cloned().collect()
    }

    /// 获取所有能力ID
    pub(crate) fn get_talent_ids(&self) -> Vec<String> {
        self.talent_ids.clone()
    }

    /// 通过ID获取能力
    pub(crate) fn get_talent_by_id(&self, id: &str) -> Option<Talent> {
        self.talents.get(id).map(|t| t.clone())
    }

    /// 更新自定义热键的能力映射
    pub(crate) fn update_custom_combo_key_map(&self, context: Weak<Context>) {
        let map = get_hotkeys(context);
        let map2: HashMap<ComboKey, String>;
        {
            map2 = self
                .combo_key_map
                .lock()
                .unwrap()
                .iter()
                .map(|(k, v)| {
                    if map.contains_key(v) {
                        (map.get(v).unwrap().clone(), v.clone())
                    } else {
                        (k.clone(), v.clone())
                    }
                })
                .collect();
        }

        *self.combo_key_map.lock().unwrap() = map2;
    }

    /// 通过热键获取能力
    pub(crate) fn get_talent_by_combo_key(&self, combo_key: &ComboKey) -> Option<Talent> {
        self.combo_key_map
            .lock()
            .unwrap()
            .get(combo_key)
            .and_then(|id| self.get_talent_by_id(id))
    }
}

impl Debug for TalentProvider {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TalentProvider({})", self.talents.len())
    }
}
