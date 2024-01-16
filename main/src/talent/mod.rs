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
mod program;
mod tts;

use crate::{
    commander::CommandType,
    context::Context,
    talent::form_browser::{
        CurrChildElementTalent, NextChildElementTalent, PrevChildElementTalent,
    },
    talent::tts::{IncreaseRTalent, ReduceRTalent},
    talent::{
        form_browser::{CurrElementTalent, ModeNextTalent, NextElementTalent, PrevElementTalent},
        program::{CurrentTimeTalent, ExitTalent},
        tts::{IncreaseTalent, ReduceTalent},
    },
};
use std::sync::Arc;

/**
 * 一个能力的抽象接口。
 * */
pub trait Talented {
    /**
     * 获取能力可支持的命令类型。
     * */
    fn get_supported_cmd_list(&self) -> Vec<CommandType>;

    /**
     * 执行能力的入口方法。
     * `context` 框架的上下文环境。
     * */
    fn perform(&self, context: Arc<Context>);
}

/// 技能访问器对象，包含所有技能对象列表
pub struct TalentAccessor {
    // 技能对象集合
    pub(crate) talents: Arc<Vec<Box<dyn Talented + Sync + Send>>>,
}

impl TalentAccessor {
    /**
     * 创建能力访问器。
     * */
    pub(crate) fn new() -> Self {
        let talents: Vec<Box<dyn Talented + Sync + Send>> = vec![
            Box::new(ExitTalent),
            Box::new(CurrentTimeTalent),
            Box::new(ModeNextTalent),
            Box::new(PrevElementTalent),
            Box::new(NextElementTalent),
            Box::new(CurrElementTalent),
            Box::new(PrevChildElementTalent),
            Box::new(NextChildElementTalent),
            Box::new(CurrChildElementTalent),
            Box::new(IncreaseTalent),
            Box::new(IncreaseRTalent),
            Box::new(ReduceTalent),
            Box::new(ReduceRTalent),
        ];
        Self {
            talents: talents.into(),
        }
    }
}
