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

mod program;
mod form_browser;

use std::sync::Arc;
use crate::context::Context;
//noinspection RsUnresolvedReference
use crate::talent::program::all_talents;
use crate::talent::form_browser::PrevElementTalent;
use super::commander::CommandType;

/**
 * 一个能力的抽象接口。
 * `context` 框架的上下文环境。
 * */

pub trait Talented {
    fn get_supported_cmd_list(&self) -> Vec<CommandType>;
    fn perform(&self, context: Arc<Context>);
}

//noinspection RsUnresolvedReference
/**
 * 获取所有能力。
 * */
pub(crate) fn get_all_talents() -> Vec<Box<dyn Talented + Sync + Send>> {
    let mut result: Vec<Box<dyn Talented + Sync + Send>> = Vec::new();

    for i in all_talents(){
        result.push(Box::new(i));
    }
    // result.push(Box::new(PrevElementTalent));

    result
}