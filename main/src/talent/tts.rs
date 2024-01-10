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

/* 使用talent macro必须导入的条目，便于IDE进行代码提示 */
#[allow(unused_imports)]
use crate::context::Context;
use rigela_macros::talent;
#[allow(unused_imports)]
use std::sync::Arc;

/* 使用talent macro可选导入的条目 */
#[allow(unused_imports)]
use win_wrap::input::{VK_INSERT, VK_OEM_MINUS, VK_OEM_PLUS};

//noinspection RsUnresolvedReference
#[talent(doc = "语音加速", key = ((VK_INSERT, false), (VK_OEM_PLUS, false)))]
async fn increase(context: Arc<Context>) {
    context.performer.apply_config(context.clone(), |c| {
        c.speed.replace(c.speed.unwrap() + 0.1);
    });
}

//noinspection RsUnresolvedReference
#[talent(doc = "语音减速", key = ((VK_INSERT, false), (VK_OEM_MINUS, false)))]
async fn reduce(context: Arc<Context>) {
    context.performer.apply_config(context.clone(), |c| {
        c.speed.replace(c.speed.unwrap() - 0.1);
    });
}
