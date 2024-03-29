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

use crate::context::Context;
use std::collections::HashMap;
use std::sync::{Arc, Weak};
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;

/// 缓冲区
#[derive(Debug)]
pub(crate) struct Cache {
    data: Mutex<String>,
    char_list: Mutex<Vec<char>>,
    index: Mutex<Option<usize>>,
    words: Arc<HashMap<String, String>>,
}

impl Cache {
    //noinspection DuplicatedCode
    /// 创建缓存对象
    pub(crate) async fn build(context: Weak<Context>) -> Self {
        let words = match unsafe { &*context.as_ptr() }.resource_provider.open("words.txt").await {
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

        Self {
            data: Mutex::new("".to_string()),
            char_list: Mutex::new(vec![]),
            index: Mutex::new(None),
            words,
        }
    }

    /// 更新缓冲区
    pub(crate) async fn update(&self, value: String) {
        self.char_list.lock().await.clear();
        *self.index.lock().await = None;
        *self.data.lock().await = value;
    }

    /// 获取字符,参数可以是上一个，下一个，或者当前
    pub(crate) async fn get(&self, direction: Direction) -> String {
        let index = self.index.lock().await.clone();

        if index.is_none() {
            return self.get_first_char().await.into();
        }

        let len = self.char_list.lock().await.len();
        let index = index.unwrap();

        *self.index.lock().await = match direction {
            Direction::Forward if index == len - 1 => index.into(),
            Direction::Forward => (index + 1).into(),
            Direction::Backward if index == 0 => index.into(),
            Direction::Backward => (index - 1).into(),
            _ => index.into(),
        };

        let index = self.index.lock().await.clone().unwrap();
        let ch = self.char_list.lock().await.get(index).unwrap().clone();
        ch.into()
    }

    /// 获取缓冲区数据
    pub(crate) async fn get_data(&self) -> String {
        self.data.lock().await.clone()
    }

    /// 延时计算
    async fn get_first_char(&self) -> char {
        self.index.lock().await.replace(0);

        {
            let mut list = self.char_list.lock().await;
            for (_, c) in self.data.lock().await.char_indices() {
                list.push(c);
            }
        }

        self.char_list.lock().await.first().unwrap().clone()
    }

    pub(crate) fn get_words(&self) -> Arc<HashMap<String, String>> {
        self.words.clone()
    }
}

// 访问方向
#[derive(Debug, Copy, Clone)]
pub(crate) enum Direction {
    Current,
    Forward,
    Backward,
}
