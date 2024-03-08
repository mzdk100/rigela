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
use crate::commander::keys::Keys::*;
use crate::configs::config_operations::apply_mouse_config;
use crate::context::Context;
use async_trait::async_trait;
use rigela_macros::talent;
#[allow(unused_imports)]
use std::sync::Arc;
use win_wrap::input::{click, get_cur_mouse_point, right_click};

//noinspection RsUnresolvedReference
#[talent(doc = "鼠标单击", key = (VkNumPadDiv))]
async fn click(context: Arc<Context>) {
    let (x, y) = get_point(context.clone()).await;
    click(x, y);
    context.performer.speak(&t!("mouse.click")).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "鼠标右击", key = (VkNumPadMul))]
async fn right_click(context: Arc<Context>) {
    let (x, y) = get_point(context.clone()).await;
    right_click(x, y);
    context.performer.speak(&t!("mouse.right_click")).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "鼠标朗读", key = (VkRigelA, VkM))]
async fn read_mouse(context: Arc<Context>) {
    let is_read = !context.config_manager.get_config().mouse_config.is_read;
    apply_mouse_config(context.clone(), is_read);
    let state = match is_read {
        true => t!("mouse.state_on"),
        false => t!("mouse.state_off"),
    };
    context.performer.speak(&state).await;
}

async fn get_point(context: Arc<Context>) -> (i32, i32) {
    let ele = match context.form_browser.current_child().await {
        None => context.form_browser.current().await,
        e => e,
    };
    match ele {
        None => get_cur_mouse_point(),
        Some(e) => {
            if let Some(r) = e.get_rect() {
                (r.left, r.top)
            } else {
                get_cur_mouse_point()
            }
        }
    }
}

/// 朗读鼠标元素
pub(crate) fn mouse_read(context: Arc<Context>, x: i32, y: i32) {
    let uia = context.ui_automation.clone();
    let ele = uia.element_from_point(x, y).unwrap();
    let pf = context.performer.clone();
    let h = context.main_handler.clone();
    h.spawn(async move { pf.speak(&ele).await });
}
