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

use win_wrap::common::HMODULE;

#[derive(Clone, Debug)]
pub struct SafeModuleHandle(HMODULE);

impl SafeModuleHandle {
    /**
    创建新实例。
    */
    pub fn new(h_module: HMODULE) -> Self {
        Self(h_module)
    }
}

unsafe impl Send for SafeModuleHandle {}
unsafe impl Sync for SafeModuleHandle {}

impl std::ops::Deref for SafeModuleHandle {
    type Target = HMODULE;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
