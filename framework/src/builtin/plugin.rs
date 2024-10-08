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

use crate::{
    basic::{AppRunner, Plugin},
    builtin::{terminate, Terminator},
};

/// DefaultPlugin 是一个默认的插件，它在初始化时向应用程序添加一个 Terminator 组件和一个 terminate 系统。
pub struct DefaultPlugin;

impl Plugin for DefaultPlugin {
    /// 初始化插件，向应用程序添加组件和系统。
    ///
    /// # 参数
    ///
    /// * `app` - 一个可变的引用，指向应用程序的 AppRunner 实例。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use rigela_framework::{
    /// basic::AppRunner,
    /// builtin::DefaultPlugin
    /// };
    /// AppRunner::new().add_plugin(DefaultPlugin).run();
    /// ```
    ///
    /// 在上面的示例中，DefaultPlugin 的 init 方法被调用，向应用程序添加了 Terminator 组件和 terminate 系统。
    fn init(&self, app: &mut AppRunner) {
        app.add_component(Terminator::new()).add_system(terminate);
    }
}
