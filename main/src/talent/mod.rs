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

mod form_browser;
pub(crate) mod mouse;
mod program;
mod tts;

use crate::{
    commander::CommandType,
    context::Context,
    talent::{
        form_browser::{
            CurrChildElementTalent, CurrElementTalent, ModeNextTalent, ModePrevTalent,
            NextChildElementTalent, NextElementTalent, PrevChildElementTalent, PrevElementTalent,
        },
        mouse::{ClickTalent, ReadMouseTalent, RightClickTalent},
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
use std::{
    fmt::{Debug, Formatter},
    sync::{
        Arc,
        Weak,
    },
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

    /**
     * 执行能力的入口方法。
     * `context` 框架的上下文环境。
     * */
    async fn perform(&self, context: Weak<Context>);
}

/// 能力提供者，包含所有能力对象列表
pub(crate) struct TalentProvider {
    // 技能对象集合
    pub(crate) talents: Arc<Vec<Arc<dyn Talented + Send + Sync + 'static>>>,
}

impl TalentProvider {
    /**
     * 创建能力访问器。
     * */
    pub(crate) fn new() -> Self {
        let talents: Vec<Arc<dyn Talented + Send + Sync>> = vec![
            // 程序能力
            Arc::new(ExitTalent),
            Arc::new(CurrentTimeTalent),
            Arc::new(CurrentCpuUsageTalent),
            Arc::new(PopupMenuTalent),
            Arc::new(HotkeysTalent),
            Arc::new(ViewFocusTalent),
            Arc::new(ViewWindowTitleTalent),
            Arc::new(StopTtsOutputTalent),
            // 窗口浏览能力
            Arc::new(ModePrevTalent),
            Arc::new(ModeNextTalent),
            Arc::new(PrevElementTalent),
            Arc::new(NextElementTalent),
            Arc::new(CurrElementTalent),
            Arc::new(PrevChildElementTalent),
            Arc::new(NextChildElementTalent),
            Arc::new(CurrChildElementTalent),
            // 语音调节能力
            Arc::new(IncreaseTalent),
            Arc::new(ReduceTalent),
            Arc::new(NextPropTalent),
            Arc::new(PrevPropTalent),
            Arc::new(PrevCacheCharTalent),
            Arc::new(NextCacheCharTalent),
            Arc::new(TransCacheCharTalent),
            Arc::new(MakeWordCacheCharTalent),
            Arc::new(CacheToClipboardTalent),
            // 鼠标能力
            Arc::new(ClickTalent),
            Arc::new(RightClickTalent),
            Arc::new(ReadMouseTalent),
        ];

        Self {
            talents: talents.into(),
        }
    }
}

impl Debug for TalentProvider {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TalentAccessor({})", self.talents.len())
    }
}
