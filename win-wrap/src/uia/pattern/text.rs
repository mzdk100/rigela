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

use crate::common::{BOOL, FALSE};
use windows::Win32::UI::Accessibility::{IUIAutomationTextRange, TextPatternRangeEndpoint};

/**
 * 提供对支持 IUIAutomationTextPattern 接口的容器中连续文本范围的访问。
 * 客户端应用程序可以使用 IUIAutomationTextRange 接口从文本范围中选择、比较和查询嵌入对象。
 * 该接口使用两个端点来分隔文本范围的开始和结束位置。
 * 文本的不相交范围由 IUIAutomationTextRangeArray 接口表示。
 * */
pub struct UiAutomationTextRange(IUIAutomationTextRange);

impl UiAutomationTextRange {
    /* 获取一个实例。 */
    pub(crate) fn obtain(range: &IUIAutomationTextRange) -> Self {
        Self(range.clone())
    }

    /**
     * 查询一个值，该值指定此文本区域是否与另一个文本区域具有相同的端点。
     * `range` 指向要与此范围进行比较的文本范围。
     * */
    pub fn compare(&self, range: &UiAutomationTextRange) -> BOOL {
        unsafe { self.0.Compare(&range.0) }.unwrap_or(FALSE)
    }

    /**
     * 将文本区域添加到支持所选文本的多个不相交范围的控件中的选定文本范围的集合中。
     * */
    pub fn add_to_selection(&self) {
        unsafe { self.0.AddToSelection() }.unwrap_or(())
    }

    /**
     * 查询一个值，该值指定此文本区域的起始或结束端点是否与另一个文本区域的起始或结束端点相同。
     * `src_endpoint` 一个值，指示是要比较此文本范围的起点还是终点。
     * `range` 要比较的文本区域。
     * `target_endpoint` 一个值，指示是要比较范围的起点还是终点。
     * */
    pub fn compare_endpoints(
        &self,
        src_endpoint: TextPatternRangeEndpoint,
        range: UiAutomationTextRange,
        target_endpoint: TextPatternRangeEndpoint,
    ) -> i32 {
        unsafe {
            self.0
                .CompareEndpoints(src_endpoint, &range.0, target_endpoint)
        }
        .unwrap_or(0)
    }
}
