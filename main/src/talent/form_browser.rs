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
use crate::browser::get_form_browser;
#[allow(unused_imports)]
use win_wrap::input::{VK_CLEAR, VK_LEFT, VK_RIGHT};

//noinspection RsUnresolvedReference
#[talent(doc = "上一个控件", key = ((VK_LEFT, false)))]
async fn prev_element(context: Arc<Context>) {
    get_form_browser().lock().unwrap().prev();
    let name = get_form_browser().lock().unwrap().current().unwrap().get_name();
    let role = get_form_browser().lock().unwrap().current().unwrap().get_role();

    context
        .performer
        .speak_text(&format!("{} {}", name, role))
        .await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "下一个控件", key = ((VK_RIGHT, false)))]
async fn next_element(context: Arc<Context>) {
    get_form_browser().lock().unwrap().next();
    let name = get_form_browser().lock().unwrap().current().unwrap().get_name();
    let role = get_form_browser().lock().unwrap().current().unwrap().get_role();

    context
        .performer
        .speak_text(&format!("{} {}", name, role))
        .await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "当前控件", key = ((VK_CLEAR, false)))]
async fn curr_element(context: Arc<Context>) {
    let name = get_form_browser().lock().unwrap().current().unwrap().get_name();
    let role = get_form_browser().lock().unwrap().current().unwrap().get_role();

    context
        .performer
        .speak_text(&format!("{} {}", name, role))
        .await;

    // context.performer.speak(ele).await;
}
