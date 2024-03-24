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

use clipboard::{ClipboardContext, ClipboardProvider};

/**
 * 获取剪贴板文本数据。
 * */
pub fn get_clipboard_text() -> String {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.get_contents().unwrap()
}

/**
 * 设置剪贴板文本数据。
 * `text` 数据内容。
 * */
pub fn set_clipboard_text(text: String) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(text).unwrap()
}
