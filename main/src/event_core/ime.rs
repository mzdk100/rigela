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

use crate::{context::Context, performer::sound::SoundArgument::Single};
use peeper::model::CandidateList;
use std::{collections::HashMap, sync::Arc};
use tokio::io::AsyncReadExt;

pub(crate) const MS_IME_CLASS_NAME: &str = "Windows.UI.Core.CoreWindow";

pub(crate) async fn subscribe_ime_events(context: Arc<Context>) {
    let words = match context.resource_provider.open("words.txt").await {
        Ok(mut f) => {
            let mut s = String::new();
            f.read_to_string(&mut s).await.unwrap_or(0);
            let mut v = HashMap::new();
            for i in s.lines() {
                let mut j = i.split("=");
                v.insert(j.next().unwrap().to_string(), j.next().unwrap().to_string());
            }
            Arc::new(v)
        }
        Err(_) => Arc::new(HashMap::new()),
    };

    // 订阅通用的输入法事件
    let ctx = context.clone();
    let words_map = words.clone();
    context
        .peeper_server
        .add_on_ime_candidate_list_listener(move |candidate_list| {
            handle_ime_candidate(ctx.clone(), candidate_list, words_map.clone());
        })
        .await;

    // 订阅微软输入法的候选事件
    let ctx = context.clone();
    let words_map = words.clone();
    context.msaa.add_on_object_selection_listener(move |src| {
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
        handle_ime_candidate(ctx.clone(), candidate_list, words_map.clone());
    });
}

fn handle_ime_candidate(
    context: Arc<Context>,
    candidate_list: CandidateList,
    words: Arc<HashMap<String, String>>,
) {
    let performer = context.performer.clone();

    context.task_manager.push(
        "ime",
        context.main_handler.spawn(async move {
            let candidate = candidate_list.list[candidate_list.selection as usize]
                .clone()
                .trim_end()
                .to_string();
            if candidate.is_empty() {
                return;
            }
            if !performer.speak(candidate_list.clone()).await {
                // 如果语音被打断就不继续朗读候选的解释词
                return;
            }
            if let Some(x) = words.get(&candidate) {
                performer.play_sound(Single("tip.wav")).await;
                // 朗读候选文字的解释词
                performer.speak(x.clone()).await;
            }
        }),
    );
}
