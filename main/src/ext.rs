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

use win_wrap::{
    common::{get_foreground_window, Result},
    msaa::object::{AccessibleObject, ROLE_SYSTEM_DIALOG},
};

pub(crate) trait AccessibleObjectExt {
    type Output;

    /**
     * 获取对话框内容。
     * */
    fn get_dialog_content(&self) -> String;

    /**
     * 从前景窗口创建对象。
     * */
    fn from_foreground_window() -> Result<Self::Output>;
}

impl AccessibleObjectExt for AccessibleObject {
    type Output = Self;

    fn get_dialog_content(&self) -> String {
        let mut content = String::new();
        for i in self.children(0, self.child_count()).unwrap() {
            if i.get_role(0) == ROLE_SYSTEM_DIALOG {
                for j in i.children(0, i.child_count()).unwrap() {
                    content += j.get_name(0).as_str();
                }
                break;
            }
        }
        content
    }

    fn from_foreground_window() -> Result<Self::Output> {
        AccessibleObject::from_window(get_foreground_window())
    }
}
