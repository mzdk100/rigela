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

use parking_lot::MutexGuard;
use std::{
    any::Any,
    fmt::{Debug, Formatter},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

/// 组件 trait，所有组件必须实现这个 trait。
pub trait Component {}

/// 组件封装，用于安全地访问组件。
pub struct Comp<'a, C: Component> {
    /// 组件的 `MutexGuard`。
    inner: MutexGuard<'a, Box<dyn Any>>,
    /// 泛型参数的标记，用于确保类型安全。
    _c: PhantomData<C>,
}

impl<'a, C: Component> Comp<'a, C> {
    /// 创建一个新的 `Comp` 实例。
    ///
    /// # 参数
    /// * `inner` - 组件的 `MutexGuard`。
    ///
    /// # 返回值
    /// 返回一个新的 `Comp` 实例。
    pub(crate) fn new(value: MutexGuard<'a, Box<dyn Any>>) -> Self {
        Self {
            inner: value,
            _c: Default::default(),
        }
    }
}

impl<'a, C: Component + Debug + 'static> Debug for Comp<'a, C> {
    /// 格式化 `Comp` 实例为字符串。
    ///
    /// # 返回值
    /// 返回成功与否。
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.inner.downcast_ref::<C>() {
            None => write!(f, "InvalidComponent."),
            Some(c) => Debug::fmt(c, f),
        }
    }
}

impl<'a, C: Component + 'static> Deref for Comp<'a, C> {
    type Target = C;

    /// 解引用 `Comp` 实例为 `C` 类型的引用。
    ///
    /// # 返回值
    /// 返回 `C` 类型的引用。
    fn deref(&self) -> &Self::Target {
        match self.inner.downcast_ref::<C>() {
            None => panic!("Component not found."),
            Some(c) => c,
        }
    }
}

impl<'a, C: Component + 'static> DerefMut for Comp<'a, C> {
    /// 解引用 `Comp` 实例为 `C` 类型的可变引用。
    ///
    /// # 返回值
    /// 返回 `C` 类型的可变引用。
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self.inner.downcast_mut::<C>() {
            None => panic!("Component not found."),
            Some(c) => c,
        }
    }
}
