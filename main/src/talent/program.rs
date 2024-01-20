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

/* 使用talent macro必须导入的条目，便于IDE进行代码提示 */
#[allow(unused_imports)]
use crate::context::Context;
use rigela_macros::talent;
#[allow(unused_imports)]
use std::sync::Arc;

/* 使用talent macro可选导入的条目 */
#[allow(unused_imports)]
use win_wrap::input::{VK_ESCAPE, VK_F12, VK_INSERT};

/* 业务逻辑使用的条目 */
use crate::performer::Speakable;
use chrono::prelude::{DateTime, Local};
use std::time::Duration;
use tokio::time::sleep;

//noinspection RsUnresolvedReference
#[talent(doc = "退出", key = ((VK_INSERT, false), (VK_ESCAPE, false)))]
async fn exit(context: Arc<Context>) {
    context
        .performer
        .speak_text(&t!("program.exit"))
        .await;
    sleep(Duration::from_millis(1000)).await;
    context.terminator.exit().await;
}

impl Speakable for DateTime<Local> {
    fn get_sentence(&self) -> String {
        format!("{}", self)
    }
}

//noinspection RsUnresolvedReference
#[talent(doc = "当前时间", key = ((VK_INSERT, false), (VK_F12, false)))]
async fn current_time(context: Arc<Context>) {
    context.performer.speak(&Local::now()).await;
}
