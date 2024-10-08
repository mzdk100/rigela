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

use crate::basic::Component;

/// 程序终结器组件
///
/// 这个组件用于控制程序的终止。当调用`exit`方法时，组件的布尔字段将被设置为`true`，表示程序应该终止。
pub struct Terminator(bool);

impl Terminator {
    /// 创建实例
    ///
    /// 创建一个新的`Terminator`实例，其布尔字段初始值为`false`。
    pub(crate) fn new() -> Self {
        Self(false)
    }

    /// 设置终止标志
    ///
    /// 将组件的布尔字段设置为`true`，表示程序应该终止。
    pub fn terminate(&mut self) {
        self.0 = true;
    }

    /// 判断是否设置了终止标志
    pub fn is_terminating(&self) -> bool {
        self.0
    }
}

impl Component for Terminator {}
