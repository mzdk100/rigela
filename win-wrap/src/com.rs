/*
 * Copyright (c) 2023. The RigelA open source project team and
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

use windows::{core::*, Win32::System::Com::*};

/**
 * 使用多线程模型套间（Multi Thread Apartment, MTA）初始化COM调用。
 * MTA能充分利用多核CPU，提高程序性能，但要注意线程之间同步的安全问题。
 * */
pub fn co_initialize_multi_thread() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)?;
    }
    Ok(())
}
