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

use crate::basic::{
    app::AppRunner,
    component::{Comp, Component},
};

/// 系统结构体，用于封装闭包。
pub struct System(Box<dyn Fn(&AppRunner)>);

impl System {
    /// 创建一个新的 `System` 实例。
    ///
    /// # 参数
    /// * `func` - 一个闭包，接受一个 `&AppRunner` 参数。
    ///
    /// # 返回值
    /// 返回一个新的 `System` 实例。
    fn new<F: Fn(&AppRunner) + 'static>(func: F) -> Self {
        Self(Box::new(func))
    }

    /// 调用封装的闭包。
    ///
    /// # 参数
    /// * `app` - 一个 `&AppRunner` 参数，传递给闭包。
    pub(crate) fn call(&self, app: &AppRunner) {
        self.0(app);
    }
}

/// 将任意类型转换为 `System` trait 对象。
pub trait ToSystem<P> {
    /// 将类型转换为 `System`。
    ///
    /// # 返回值
    /// 返回一个 `System` 实例。
    fn to_system(self) -> System;
}

/// 宏，为不同数量参数的函数实现 `ToSystem` 特征。
///
/// # 参数
/// * `$($comp:ident),*` - 类型参数名称。
///
/// # 注意
/// - 宏的实现中，闭包 `self` 被调用时，会通过 `app.get_component::<$comp>()` 获取组件的实例。
macro_rules! impl_to_system {
    ($($comp:ident),* $(,)?) => {
        impl<$($comp),*, T> ToSystem<($($comp,)*)> for T
        where
            T: Fn($(Comp<$comp>),*) + 'static,
            $($comp: Component + 'static),*
        {
            #[track_caller]
            fn to_system(self) -> System {
                System::new(move |app| self($(app.get_component::<$comp>()),*))
            }
        }
    };
}

impl_to_system!(A);
impl_to_system!(A, B);
impl_to_system!(A, B, C);
impl_to_system!(A, B, C, D);
impl_to_system!(A, B, C, D, E);
impl_to_system!(A, B, C, D, E, F);
impl_to_system!(A, B, C, D, E, F, G);
impl_to_system!(A, B, C, D, E, F, G, H);
impl_to_system!(A, B, C, D, E, F, G, H, I);
impl_to_system!(A, B, C, D, E, F, G, H, I, J);
