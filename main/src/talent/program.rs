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

use chrono::prelude::{DateTime, Local};
#[allow(unused_imports)]
use win_wrap::input::{VK_ESCAPE, VK_F12, VK_INSERT};
use crate::performer::Speakable;
#[allow(unused_imports)]
use super::super::context::Context;
use rigela_macros::talent;

#[talent(doc="退出", key=((VK_INSERT, false),(VK_ESCAPE, false)))]
async fn exit(context: Arc<Context>) {
    context
        .terminator
        .exit()
        .await;
}


impl Speakable for DateTime<Local> {
    fn get_sentence(&self) -> String {
        format!("{}", self)
    }
}
#[talent(doc="当前时间", key=((VK_INSERT, false),(VK_F12, false)))]
async fn current_time(context: Arc<Context>) {
    context
        .performer
        .speak(&Local::now())
        .await;
}
