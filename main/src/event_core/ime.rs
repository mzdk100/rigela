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

use peeper::model::CandidateList;
use std::sync::Weak;
use win_wrap::input::{IME_CMODE_ALPHANUMERIC, IME_CMODE_FULLSHAPE, IME_CMODE_NATIVE};

use crate::{
    cancel_edge_handle,
    context::{Context, ContextAccessor},
    performer::sound::SoundArgument::Single,
};

pub(crate) const MS_IME_CLASS_NAME: &str = "Windows.UI.Core.CoreWindow";

//noinspection DuplicatedCode
pub(crate) async fn subscribe_ime_events(context: Weak<Context>) {
    // 订阅通用的输入法候选事件
    let ctx = context.clone();
    context
        .get_peeper_server()
        .add_on_ime_candidate_list_listener(move |candidate_list| {
            handle_ime_candidate(ctx.clone(), candidate_list);
        })
        .await;

    // 订阅输入法模式转换事件
    let ctx = context.clone();
    context
        .get_peeper_server()
        .add_on_ime_conversion_mode_listener(move |conversion_mode| {
            // https://learn.microsoft.com/zh-cn/windows/win32/intl/ime-conversion-mode-values
            let mode = if conversion_mode & IME_CMODE_NATIVE.0 == IME_CMODE_NATIVE.0 {
                t!("ime.native_input")
            } else if conversion_mode & IME_CMODE_ALPHANUMERIC.0 == IME_CMODE_ALPHANUMERIC.0 {
                t!("ime.alpha_numeric_input")
            } else if conversion_mode & IME_CMODE_FULLSHAPE.0 == IME_CMODE_FULLSHAPE.0 {
                t!("ime.fullshape_input")
            } else {
                return;
            };

            let ctx2 = ctx.clone();
            ctx.get_work_runtime()
                .spawn(async move { ctx2.get_performer().speak(&mode).await });
        })
        .await;

    // 订阅微软输入法的候选事件
    let ctx = context.clone();
    context
        .get_msaa()
        .add_on_object_selection_listener(move |src| {
            if src.get_class_name() != MS_IME_CLASS_NAME {
                // 此类事件不属于输入法事件
                return;
            }
            let Ok((obj, child)) = src.get_object() else {
                return;
            };
            let candidate_list = CandidateList {
                selection: 0,
                page_start: 0,
                list: vec![obj.get_name(child)],
            };
            handle_ime_candidate(ctx.clone(), candidate_list);
        });
}

fn handle_ime_candidate(context: Weak<Context>, candidate_list: CandidateList) {
    // 关闭编辑框键盘事件朗读
    cancel_edge_handle!(context);

    let ctx = context.clone();
    context.get_task_manager().push(
        "ime",
        context.get_work_runtime().spawn(async move {
            let candidate = candidate_list.list[candidate_list.selection as usize]
                .clone()
                .trim_end()
                .to_string();
            if candidate.is_empty() {
                return;
            }
            if !ctx.get_performer().speak(&candidate_list.clone()).await {
                // 如果语音被打断就不继续朗读候选的解释词
                return;
            }
            let Some(cache) = ctx.get_performer().get_cache() else {
                return;
            };
            if let Some(x) = unsafe { &*cache.as_ptr() }.make_word(&candidate) {
                ctx.get_performer().play_sound(Single("tip.wav")).await;
                // 朗读候选文字的解释词
                ctx.get_performer().speak(x).await;
            }
        }),
    );
}
