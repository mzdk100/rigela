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
    basic::{AppProxy, Comp},
    builtin::Terminator,
};

/// 处理程序终止的系统
///
/// 这个系统用于处理一个`Terminator`类型的组件，并根据其状态决定是否退出程序。
///
/// # 参数
///
/// * `terminator` - 一个可变引用的`Comp<Terminator>`类型的参数，表示程序终止的组件。
///
/// # 返回值
///
/// 无
///
pub fn terminate(mut app_proxy: Comp<AppProxy>, terminator: Comp<Terminator>) {
    if terminator.is_terminating() {
        app_proxy.exit();
    }
}
