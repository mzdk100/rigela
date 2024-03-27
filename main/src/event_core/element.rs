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


use std::sync::Weak;
use crate::context::Context;

//noinspection SpellCheckingInspection
/**
 * 订阅元素改变事件。
 * `context` 读屏框架的上下文环境。
 * */
pub(crate) async fn subscribe_element_events(context: Weak<Context>) {
    let ctx = context.clone();
    unsafe { &*context.as_ptr() }.msaa.add_on_object_show_listener(move |src| {
        let Ok(obj) = src.get_object() else {
            return;
        };
        let navigator = unsafe { &*ctx.as_ptr() }.ui_navigator.clone();
        unsafe { &*ctx.as_ptr() }.work_runtime.spawn(async move {
            navigator.put(obj.into()).await;
        });
    });

    let ctx = context.clone();
    unsafe { &*context.as_ptr() }.msaa.add_on_object_hide_listener(move |src| {
        let Ok(obj) = src.get_object() else {
            return;
        };
        let navigator = unsafe { &*ctx.as_ptr() }.ui_navigator.clone();
        unsafe { &*ctx.as_ptr() }.work_runtime.spawn(async move {
            navigator.remove(obj.into()).await;
        });
    });
}
