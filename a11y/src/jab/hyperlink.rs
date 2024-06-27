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

use crate::jab::jab_lib::{
    packages::{AccessibleContext as AC, AccessibleContext, AccessibleHyperlinkInfo},
    JabLib,
};
use std::fmt::{Debug, Formatter};

pub struct AccessibleHyperlink<'lib> {
    _lib: &'lib JabLib,
    _ac: AC,
    _vm_id: i32,
    _info: AccessibleHyperlinkInfo,
}

impl<'lib> AccessibleHyperlink<'lib> {
    pub(crate) fn new(
        lib: &'lib JabLib,
        vm_id: i32,
        ac: AccessibleContext,
        info: AccessibleHyperlinkInfo,
    ) -> Self {
        Self {
            _lib: lib,
            _ac: ac,
            _vm_id: vm_id,
            _info: info,
        }
    }

    /**
     * 获取链接文本。
     * */
    pub fn get_text(&self) -> String {
        String::from_utf16_lossy(&self._info.text)
            .trim_matches('\0')
            .to_string()
    }

    /**
     * 链接开始的超文本文档中的索引
     * */
    pub fn get_start_index(&self) -> i32 {
        self._info.startIndex
    }

    /**
     * 链接结束的超文本文档中的索引
     * */
    pub fn get_end_index(&self) -> i32 {
        self._info.endIndex
    }

    /**
     * 请求激活超链接。
     * */
    pub fn activate(&self) -> bool {
        self._lib.activate_accessible_hyperlink(
            self._vm_id,
            self._ac,
            self._info.accessibleHyperlink,
        )
    }
}

impl<'lib> Drop for AccessibleHyperlink<'lib> {
    fn drop(&mut self) {
        self._lib
            .release_java_object(self._vm_id, self._info.accessibleHyperlink)
    }
}

impl<'lib> Debug for AccessibleHyperlink<'lib> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AccessibleHyperlink(text:{}, start_index:{}, end_index:{})",
            self.get_text(),
            self.get_start_index(),
            self.get_end_index()
        )
    }
}
