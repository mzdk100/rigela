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

use crate::context::{Context, ContextAccessor};
use arc_swap::ArcSwapAny;
use log::error;
use rust_i18n::AtomicStr;
use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc, Weak,
    },
};
use tokio::io::AsyncReadExt;

/// 缓冲区
pub(crate) struct Cache {
    data: AtomicStr,
    char_list: ArcSwapAny<Arc<Vec<char>>>,
    index: AtomicI64,
    word_map: Arc<HashMap<String, String>>,
}

impl Cache {
    //noinspection DuplicatedCode
    /// 创建缓存对象
    pub(crate) async fn build(context: Weak<Context>) -> Self {
        let word_map = match context.get_resource_provider().open("words.txt").await {
            Ok(mut f) => {
                let mut data: String = Default::default();
                if f.read_to_string(&mut data).await.is_err() {
                    error!("load words data error")
                }

                let mut result: HashMap<String, String> = Default::default();
                data.lines().for_each(|i| {
                    let mut j = i.split("=");
                    result.insert(j.next().unwrap().to_string(), j.next().unwrap().to_string());
                });

                Arc::new(result)
            }
            Err(_) => Default::default(),
        };

        Self {
            data: AtomicStr::new(""),
            char_list: Default::default(),
            index: AtomicI64::new(-1),
            word_map,
        }
    }

    /// 更新缓冲区
    pub(crate) fn update(&self, value: String) {
        self.char_list.store(Arc::new(vec![]));
        self.index.store(-1, Ordering::Release);
        self.data.replace(value);
    }

    /// 获取字符,参数可以是上一个，下一个，或者当前
    pub(crate) fn get(&self, direction: Direction) -> String {
        let index = self.index.load(Ordering::Acquire);
        match index {
            -1 => self.get_first_char().into(),
            _ => {
                let list = self.char_list.load();
                let len = list.len();

                let new_index = match direction {
                    Direction::Forward if index as usize == len - 1 => index,
                    Direction::Forward => index + 1,
                    Direction::Backward if index == 0 => index,
                    Direction::Backward => index - 1,
                    _ => index,
                };

                self.index.store(new_index, Ordering::Release);
                list.get(new_index as usize).unwrap().clone().into()
            }
        }
    }

    /// 获取缓冲区数据
    pub(crate) fn get_data(&self) -> String {
        self.data.to_string().clone()
    }

    /// 延时计算
    fn get_first_char(&self) -> char {
        self.index.store(0, Ordering::Release);

        let mut list = vec![];
        let data = self.data.to_string().clone();
        data.char_indices().for_each(|(_, ch)| list.push(ch));
        self.char_list.store(Arc::new(list.clone()));

        list.first().unwrap().clone()
    }

    pub(crate) fn get_current_char_words(&self) -> String {
        let char = self.get(Direction::Current);
        self.word_map.get(&char).unwrap_or(&char).clone()
    }

    /**
     * 获取解释词。
     * `origin` 原始字符串。
     * */
    pub(crate) fn make_word(&self, origin: &str) -> Option<&String> {
        self.word_map.get(origin)
    }
}

// 访问方向
#[derive(Debug, Copy, Clone)]
pub(crate) enum Direction {
    Current,
    Forward,
    Backward,
}

impl Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cache").finish()
    }
}
