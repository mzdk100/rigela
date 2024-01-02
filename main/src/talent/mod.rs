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

use std::sync::Arc;
use crate::context::Context;
//noinspection RsUnresolvedReference
use crate::talent::{
    form_browser::{
        CurrElementTalent,
        PrevElementTalent,
        NextElementTalent
    },
    program::{
        CurrentTimeTalent,
        ExitTalent
    }
};
use super::commander::CommandType;

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

//noinspection RsUnresolvedReference
/**
 * 获取所有能力。
 * */
pub(crate) fn get_all_talents() -> Vec<Box<dyn Talented + Sync + Send>> {
    // 此处的代码后面将通过macro机制自动生成
    vec![
        Box::new(CurrElementTalent),
        Box::new(ExitTalent),
        Box::new(PrevElementTalent),
        Box::new(CurrentTimeTalent),
        Box::new(NextElementTalent),
    ]
}