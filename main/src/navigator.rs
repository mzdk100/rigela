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

pub(crate) mod element;
pub(crate) mod linear;

use std::{
    collections::HashSet,
    fmt::{Debug, Formatter},
    sync::{Arc, Weak},
};

use tokio::sync::Mutex;

use crate::navigator::element::UiElement;

/**
 * UI导航器。
 * */
pub(crate) struct UiNavigator {
    /// 最后访问的元素
    last_visit: Mutex<Option<Weak<UiElement<'static>>>>,
    /// 控件元素容器
    container: Mutex<HashSet<Arc<UiElement<'static>>>>,
}

#[allow(dead_code)]
impl UiNavigator {
    pub(crate) fn new() -> Self {
        Self {
            last_visit: None.into(),
            container: HashSet::new().into(),
        }
    }

    /**
     * 清除所有元素。
     * */
    pub(crate) async fn clear(&self) {
        let mut container = self.container.lock().await;
        container.clear();
    }

    //noinspection StructuralWrap
    /**
     * 添加元素到容器，解析控件树。
     * `root` 根元素。
     * */
    pub(crate) async fn add_all(&self, root: UiElement<'static>) {
        let mut container = self.container.lock().await;
        for i in 0..root.get_child_count() {
            if let Some(c) = root.get_child(i) {
                container.insert(c.into());
            }
        }
    }

    /**
     * 添加一个元素。
     * `element` 要添加的元素。
     * */
    pub(crate) async fn put(&self, element: UiElement<'static>) {
        let mut container = self.container.lock().await;
        container.insert(element.into());
    }

    /**
     * 移除一个元素。
     * `element` 要移除的元素。
     * */
    pub(crate) async fn remove(&self, element: UiElement<'static>) {
        let mut container = self.container.lock().await;
        container.remove(&element);
    }

    /**
     * 按条件移除元素。
     * `f` 一个闭包，返回true表示需要移除。
     * */
    pub(crate) async fn remove_by(&self, f: impl Fn(&UiElement<'static>) -> bool) {
        let mut container = self.container.lock().await;
        container.retain(|i| !f(i.as_ref()));
    }

    /**
     * 获取最后访问的元素。
     * */
    pub(crate) async fn get_last_visit(&self) -> Option<&UiElement> {
        if let Some(r) = self.last_visit.lock().await.as_ref() {
            return Some(unsafe { &*r.as_ptr() });
        }
        None
    }
}

unsafe impl Send for UiNavigator {}

unsafe impl Sync for UiNavigator {}

impl Debug for UiNavigator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UiNavigator()")
    }
}
