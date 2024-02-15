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

use crate::{
    commander::keys::Keys::*,
    context::Context,
    gui::{hotkeys, popup_menu},
    performer::Speakable,
};
use rigela_macros::talent;
use std::{thread, time::Duration};
/* 业务逻辑使用的条目 */
use async_trait::async_trait;
use chrono::prelude::{DateTime, Local};
use tokio::time::sleep;
use win_wrap::{common::get_foreground_window, msaa::object::AccessibleObject};

//noinspection RsUnresolvedReference
#[talent(doc = "退出", key = (VkRigelA, VkEscape))]
async fn exit(context: Arc<Context>) {
    context.performer.speak(t!("program.exit")).await;
    sleep(Duration::from_millis(1000)).await;
    context.terminator.exit().await;
}

impl Speakable for DateTime<Local> {
    fn get_sentence(&self) -> String {
        self.format("%Y年%m月%d日 %H:%M:%S").to_string()
    }
}

//noinspection RsUnresolvedReference
#[talent(doc = "当前时间", key = (VkRigelA, VkF12))]
async fn current_time(context: Arc<Context>) {
    context.performer.speak(Local::now()).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "弹出菜单", key = (VkRigelA, VkR))]
async fn popup_menu(context: Arc<Context>) {
    thread::spawn(|| popup_menu::show());
}

//noinspection RsUnresolvedReference
#[talent(doc = "自定义热键", key = (VkRigelA, VkK))]
async fn hotkeys(context: Arc<Context>) {
    let context = context.clone();
    thread::spawn(|| hotkeys::show(context));
}

//noinspection RsUnresolvedReference
#[talent(doc = "查看前景窗口标题", key = (VkRigelA, VkT))]
async fn view_window_title(context: Arc<Context>) {
    let obj = AccessibleObject::from_window(get_foreground_window()).unwrap();
    context.performer.speak((obj, 0)).await;
}
