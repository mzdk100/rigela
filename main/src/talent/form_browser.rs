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

#[allow(unused_imports)]
use win_wrap::input::{VK_LEFT, VK_RIGHT};
use super::super::context::Context;
use rigela_macros::talent;

#[talent(doc="上一个控件", key=((VK_LEFT, false)))]
async fn prev_element(context: Arc<Context>) {
    let ele_text = "上一个控件";
    context.performer.speak_text(ele_text).await;
}