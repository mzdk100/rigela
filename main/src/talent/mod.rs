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
mod mouse;
mod program;
mod tts;

use crate::talent::tts::{
    CacheToClipboardTalent, MakeWordCacheCharTalent, NextCacheCharTalent, PrevCacheCharTalent,
    TransCacheCharTalent,
};
use crate::{
    commander::CommandType,
    context::Context,
    talent::{
        form_browser::{
            CurrChildElementTalent, CurrElementTalent, ModeNextTalent, NextChildElementTalent,
            NextElementTalent, PrevChildElementTalent, PrevElementTalent,
        },
        mouse::{ClickTalent, ReadMouseTalent, RightClickTalent},
        program::{
            CurrentTimeTalent, ExitTalent, HotkeysTalent, PopupMenuTalent, ViewWindowTitleTalent,
        },
        tts::{IncreaseTalent, NextPropTalent, PrevPropTalent, ReduceTalent},
    },
};
use async_trait::async_trait;
use std::{
    fmt::{Debug, Formatter},
    sync::Arc,
};

/**
 * 一个能力的抽象接口。
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
    async fn perform(&self, context: Arc<Context>);
}

/// 技能访问器对象，包含所有技能对象列表
pub(crate) struct TalentAccessor {
    // 技能对象集合
    pub(crate) talents: Arc<Vec<Arc<dyn Talented + Send + Sync + 'static>>>,
}

impl TalentAccessor {
    /**
     * 创建能力访问器。
     * */
    pub(crate) fn new() -> Self {
        let talents: Vec<Arc<dyn Talented + Send + Sync>> = vec![
            // 程序技能
            Arc::new(ExitTalent),
            Arc::new(CurrentTimeTalent),
            Arc::new(PopupMenuTalent),
            Arc::new(HotkeysTalent),
            Arc::new(ViewWindowTitleTalent),
            // 窗口浏览技能
            Arc::new(ModeNextTalent),
            Arc::new(PrevElementTalent),
            Arc::new(NextElementTalent),
            Arc::new(CurrElementTalent),
            Arc::new(PrevChildElementTalent),
            Arc::new(NextChildElementTalent),
            Arc::new(CurrChildElementTalent),
            // 语音调节技能
            Arc::new(IncreaseTalent),
            Arc::new(ReduceTalent),
            Arc::new(NextPropTalent),
            Arc::new(PrevPropTalent),
            Arc::new(PrevCacheCharTalent),
            Arc::new(NextCacheCharTalent),
            Arc::new(TransCacheCharTalent),
            Arc::new(MakeWordCacheCharTalent),
            Arc::new(CacheToClipboardTalent),
            // 鼠标技能
            Arc::new(ClickTalent),
            Arc::new(RightClickTalent),
            Arc::new(ReadMouseTalent),
        ];
        Self {
            talents: talents.into(),
        }
    }
}

impl Debug for TalentAccessor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TalentAccessor({})", self.talents.len())
    }
}
