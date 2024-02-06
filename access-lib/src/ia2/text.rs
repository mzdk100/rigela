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

pub use crate::IAccessible2Lib::AccessibleText::{IA2TextBoundaryType, IA2TextSegment};
use crate::{
    ia2::object::{IA2CoordinateType, IA2ScrollType},
    IAccessible2Lib::{AccessibleText::IAccessibleText, AccessibleText2::IAccessibleText2},
};
use std::fmt::{Debug, Formatter};
use win_wrap::msaa::object::AccessibleObject;
use windows::{
    core::{ComInterface, Error, Result, BSTR, HSTRING},
    Win32::{Foundation::S_FALSE, System::Com::IServiceProvider, UI::Accessibility::IAccessible},
};

pub struct AccessibleText {
    _iat: Option<IAccessibleText>,
    _iat2: Option<IAccessibleText2>,
}

impl AccessibleText {
    /**
     * 获取一个新实例。
     * `obj` 一个MSAA对象。
     * */
    pub fn from_accessible_object(obj: AccessibleObject) -> Option<Self> {
        if let Ok(sp) = obj.get_raw().cast::<IServiceProvider>() {
            let iat = match unsafe { sp.QueryService::<IAccessibleText>(&IAccessible::IID) } {
                Err(_) => None,
                Ok(x) => Some(x),
            };
            let iat2 = match unsafe { sp.QueryService::<IAccessibleText2>(&IAccessible::IID) } {
                Err(_) => None,
                Ok(x) => Some(x),
            };
            return Some(Self {
                _iat: iat,
                _iat2: iat2,
            });
        }
        None
    }

    /**
     * 返回与给定偏移处的给定属性过滤器相对应的范围（开始和结束偏移）和文本属性(start_offset, end_offset, attribute_values)。
     * `start_offset` 包含指定属性的文本的起始偏移量（从0开始）。
     * `end_offset` 从包含指定属性的文本的最后一个字符开始的偏移量（从0开始）。
     * `attribute_values` 请求的属性的值。
     * `offset` 搜索筛选器中指定的属性的偏移量。
     * `filter` 请求的属性名称。过滤器格式为“attribute1,attribute2”。
     * */
    pub fn attribute_range(&self, offset: i32, filter: &str) -> (i32, i32, String) {
        if self._iat2.is_none() {
            return (0, 0, String::new());
        }
        unsafe {
            let (mut start_offset, mut end_offset, mut attribute_values) = std::mem::zeroed();
            if self
                ._iat2
                .as_ref()
                .unwrap()
                .attributeRange(
                    offset,
                    BSTR::from(filter),
                    &mut start_offset,
                    &mut end_offset,
                    &mut attribute_values,
                )
                .is_err()
            {
                return (0, 0, String::new());
            }
            (start_offset, end_offset, attribute_values.to_string())
        }
    }

    /**
     * 添加文本选区。
     * `start_offset` 起始偏移量（从零开始）。
     * `end_offset` 新选区后第一个字符的偏移量（从零开始）。
     * 有关可以在%IAccessibleText方法中使用的特殊偏移的信息，请参阅@ref _specialOffsets“在IAccessibleText和IAccessibleEditableText方法中的特殊偏移”。
     * */
    pub fn add_selection(&self, start_offset: i32, end_offset: i32) -> bool {
        if self._iat.is_none() {
            return false;
        }
        unsafe {
            self._iat
                .as_ref()
                .unwrap()
                .addSelection(start_offset, end_offset)
                .is_ok()
        }
    }

    /**
     * 返回文本属性。
     * `start_offset` 字符范围的起始偏移量，在此范围内所有文本属性都与偏移量相匹配。（从0开始）
     * `end_offset` 超过字符范围的第一个字符的偏移量，在该字符范围内，所有文本属性都与偏移量相匹配。（从零开始）
     * `text_attributes` 描述文本的属性字符串。%IAccessible2网站上的文本属性规范描述了这些属性。
     * `offset` 文本偏移量（从零开始）。有关%IAccessibleText方法中可以使用的特殊偏移的信息，请参阅@ref _specialOffsets“在IAccessibleText和IAccessibleEditableText方法中使用的特殊偏置”。
     * */
    pub fn attributes(&self, offset: i32) -> (i32, i32, String) {
        if self._iat.is_none() {
            return (0, 0, String::new());
        }
        unsafe {
            let (mut start_offset, mut end_offset, mut text_attributes) = std::mem::zeroed();
            if self
                ._iat
                .as_ref()
                .unwrap()
                .attributes(
                    offset,
                    &mut start_offset,
                    &mut end_offset,
                    &mut text_attributes,
                )
                .is_err()
            {
                return (0, 0, String::new());
            }
            (start_offset, end_offset, text_attributes.to_string())
        }
    }

    /**
     * 返回插入符号的位置。
     * 返回文本中插入符号的从0开始的偏移量。如果文本被实现为文本对象的树，其中嵌入字符在较高级别代表子文本对象的子字符串，并且插入符号在其中一个子文本对象中，则较高级别文本对象中的偏移量将在嵌入字符代表包含插入符号的子文本对象处。
     * 例如，如果字符串“一二三”实现为两个文本对象，其中一个顶层文本对象包含嵌入字符“一？三”，一个子文本对象包含“二”，并且如果插入符号在“二”中的“o”之前的子代对象中，则：
     * “一？三”对象的插入符号偏移量为4，与嵌入字符匹配
     * “two”的插入符号偏移量为2，与“o”匹配
     *插入符号位置/偏移量是逻辑上跟在它后面的字符的位置/偏移，例如，在从左到右的语言中，在它的右边，或者在从右到左的语言中在它的左边。
     * 返回的偏移量是相对于此对象表示的文本的。
     * 如果插入符号当前在此对象上不活动，即插入符号位于其他对象上，则返回的偏移值将为-1。
     * 如果插入符号位于文本对象或其子对象中，则不会返回（偏移量为-1）。
     * 注意： 上述翻译可能存在错误。
     * */
    pub fn caret_offset(&self) -> Result<i32> {
        if self._iat.is_none() {
            return Err(Error::new(S_FALSE, HSTRING::from("Not supported.")));
        }
        unsafe {
            let mut offset = std::mem::zeroed();
            self._iat
                .as_ref()
                .unwrap()
                .caretOffset(&mut offset)
                .from_abi(offset)
        }
    }

    /**
     * 返回指定位置的边界框(x, y, width, height)。
     * 在所表示的文本的最后一个字符之后的虚拟字符，即位置长度处的虚拟字符是一种特殊情况。它表示当前输入位置，因此AT通常比其他位置更频繁地查询它。因为它不表示现有字符，所以它的边界框是相对于前面的字符定义的。当插入到文本末尾时，它应该大致相当于某个字符的边界框。它的高度通常是文本中所有字符的最大高度或前一个字符的高度，它的宽度至少是一个像素，这样边界框就不会退化。
     * 请注意，索引“length”并不总是有效的。是否是取决于实现。通常情况下，当文本是可编辑的，或者在屏幕上时，插入符号可以放在文本后面。收到此索引的：IA2_EVENT_TEXT_CARET_MOVED事件后，可以确保该索引有效。
     * `x` 被引用字符的边界框的左上角的x坐标。
     * `y` 被引用字符的边界框的左上角的y坐标。
     * `width` 被引用字符的边框宽度。
     * `height` 被引用字符的边框高度。
     * `offset` 返回其边界框的字符的索引。有效范围是从零到长度。有关%IAccessibleText方法中可以使用的特殊偏移的信息，请参阅@ref _specialOffsets“在IAccessibleText和IAccessible EditableText方法中使用的特殊偏置”。
     * `coord_type` 指定坐标是相对于屏幕还是相对于父窗口。
     * */
    pub fn character_extents(
        &self,
        offset: i32,
        coord_type: IA2CoordinateType,
    ) -> (i32, i32, i32, i32) {
        if self._iat.is_none() {
            return (0, 0, 0, 0);
        }
        unsafe {
            let (mut x, mut y, mut width, mut height) = std::mem::zeroed();
            if self
                ._iat
                .as_ref()
                .unwrap()
                .characterExtents(offset, coord_type, &mut x, &mut y, &mut width, &mut height)
                .is_err()
            {
                return (0, 0, 0, 0);
            }
            (x, y, width, height)
        }
    }

    /**
     * 返回活动的非连续选区数
     * */
    pub fn n_selections(&self) -> i32 {
        if self._iat.is_none() {
            return 0;
        }
        unsafe {
            let mut num = std::mem::zeroed();
            if self._iat.as_ref().unwrap().nSelections(&mut num).is_err() {
                return 0;
            }
            num
        }
    }

    /**
     * 返回指定屏幕位置的文本位置。
     * 给定一个点，返回该点下字符的从零开始的索引。通过使用character_extents返回的每个字符的边界框，可以实现相同的功能。然而，此方法可以更有效地实现。
     * 返回给定点下的字符索引，如果该点无效或该点下没有字符，则为-1。
     * `x` 位置的x值，用于查找在该点呈现在显示器上的字符的索引。
     * `y` 位置的y值，用于查找在该点呈现在显示器上的字符的索引。
     * `coord_type` 屏幕坐标或窗口坐标。
     * */
    pub fn offset_at_point(&self, x: i32, y: i32, coord_type: IA2CoordinateType) -> i32 {
        if self._iat.is_none() {
            return 0;
        }
        unsafe {
            let mut offset = std::mem::zeroed();
            self._iat
                .as_ref()
                .unwrap()
                .offsetAtPoint(x, y, coord_type, &mut offset)
                .from_abi(offset)
                .unwrap_or(-1)
        }
    }

    /**
     * 返回指定选区的开始和结束偏移量(start_offset,end_offset)。
     * `start_offset` 第一个选定字符的基于0的偏移量。
     * `end_offset` 基于0的结束偏移量。
     * 如果结束选择偏移位于其中一个子文本对象中，则高级文本对象中的结束偏移将刚好位于表示包含结束选择偏移的子文本对象的嵌入字符之后。
     * 例如，如果字符串“一二三”实现为两个文本对象，其中顶层文本对象包含嵌入字符“一？三”，子文本对象包含“二”，如果选择是字符串“二”则：
     * “一？三”对象的startOffset为4，与嵌入字符匹配，endOffset为5。
     * “two”对象的startOffset为0，endOffset为3
     * 选择偏移是逻辑上跟随它的字符的偏移，例如，在从左到右的语言中，选择偏移在它的右边，或者在从右到左的语言中选择偏移在其左边。
     * `selection_index` 选区的索引（从零开始）。
     * */
    pub fn selection(&self, selection_index: i32) -> Result<(i32, i32)> {
        if self._iat.is_none() {
            return Err(Error::new(S_FALSE, HSTRING::from("Not supported.")));
        }
        unsafe {
            let (mut start_offset, mut end_offset) = std::mem::zeroed();
            let res = self._iat.as_ref().unwrap().selection(
                selection_index,
                &mut start_offset,
                &mut end_offset,
            );
            if res.is_err() {
                return Err(Error::new(S_FALSE, res.message()));
            }
            Ok((start_offset, end_offset))
        }
    }

    /**
     * 返回两个给定索引之间的子字符串。
     * 如果start_offset小于或等于end_offset，则子字符串从start_offset（包括start_offset）处的字符开始，直到end_offset（不包括end_offset）的字符。
     * 如果end_offset低于start_offset，则结果与交换两个参数的调用相同。
     * 可以通过传递索引0和n_characters来请求全文。如果两个索引的值相同，则返回一个空字符串。
     * 如果文本包含多字节字符，则返回的字符串可能长于endOffset startOffset字节。
     * 有关可以在%IAccessibleText方法中使用的特殊偏移的信息，请参阅@ref _specialOffsets“在IAccessibleText和IAccessible EditableText方法中的特殊偏移”。
     * `start_offset` 返回字符串中要包含的第一个字符的索引。有效范围是从零到长度。
     * `end_offset` 返回字符串中要排除的最后一个字符的索引。有效范围是从零到长度。
     * */
    pub fn text(&self, start_offset: i32, end_offset: i32) -> String {
        if self._iat.is_none() {
            return String::new();
        }
        unsafe {
            let mut text = std::mem::zeroed();
            if self
                ._iat
                .as_ref()
                .unwrap()
                .text(start_offset, end_offset, &mut text)
                .is_err()
            {
                return String::new();
            }
            text.to_string()
        }
    }

    /**
     * 返回给定位置之前的文本部分(start_offset,end_offset,text)。
     * 返回指定文本类型的子字符串，该子字符串位于给定字符之前，但不包括该字符。此方法的结果应与text_at_offset的结果相同，但索引值应适当减小。
     * 例如，如果文本类型为：IA2_TEXT_BOUNDARY_WORD，则返回最靠近偏移量并位于偏移量之前的完整单词。
     * 如果索引有效，但未找到文本，则返回S_FALSE。当文本完全由空白组成时，这种情况会发生在字符以外的边界类型上。
     * `start_offset` 第一个字符的基于0的偏移量。
     * `end_offset` 最后一个字符后一个字符的基于0的偏移量。
     * `text` 返回请求的文本部分。当找不到合适的文本部分或文本类型无效时，此部分可能为空或无效。如果未实现请求的边界类型，例如IA2_TEXT_boundary_SENTENCE，或者没有任何可返回的内容，则值分别为0s和NULL。
     * `offset` 要返回其前面文本部分的字符的索引。索引字符不会是返回字符串的一部分。有效范围是从零到长度。有关特殊偏移的信息，请参阅@ref _specialOffsets“在IAccessibleText和IAccessible EditableText方法中使用的特殊偏移”
     * `boundary_type` 要返回的文本部分的类型。*有关完整列表，请参见：IA2TextBoundaryType。
     * */
    pub fn text_before_offset(
        &self,
        offset: i32,
        boundary_type: IA2TextBoundaryType,
    ) -> (i32, i32, String) {
        if self._iat.is_none() {
            return (0, 0, String::new());
        }
        unsafe {
            let (mut start_offset, mut end_offset, mut text) = std::mem::zeroed();
            if self
                ._iat
                .as_ref()
                .unwrap()
                .textBeforeOffset(
                    offset,
                    boundary_type,
                    &mut start_offset,
                    &mut end_offset,
                    &mut text,
                )
                .is_err()
            {
                return (0, 0, String::new());
            }
            (start_offset, end_offset, text.to_string())
        }
    }

    /**
     * 返回给定位置后的文本部分(start_offset, end_offset, text)。
     * 返回指定文本类型的子字符串，该子字符串位于给定字符之后，但不包括该字符。此方法的结果应与text_at_offset的结果相同，并适当增加索引值。
     * 例如，如果文本类型为：IA2_TEXT_BOUNDARY_WORD，则返回最接近偏移量并位于偏移量之后的完整单词。
     * 如果索引有效，但未找到文本，则返回S_FALSE。当文本完全由空白组成时，这种情况会发生在字符以外的边界类型上。
     * `start_offset` 第一个字符的基于0的偏移量。
     * `end_offset` 最后一个字符后一个字符的基于0的偏移量。
     * `text` 返回请求的文本部分。当找不到合适的文本部分或文本类型无效时，此部分可能为空或无效。
     * 如果未实现请求的边界类型，例如IA2_TEXT_boundary_SENTENCE，或者没有任何可返回的内容，则值分别为0s和NULL
     * `offset` 要返回其后面文本部分的字符的索引。索引字符不会是返回字符串的一部分。有效范围是从零到长度。有关信息，请参阅@ref _specialOffsets“在IAccessibleText和IAccessibleEditableText方法中使用的特殊偏移”
     * `boundary_type` 要返回的文本部分的类型。有关完整列表，请参见：IA2TextBoundaryType。
     * */
    pub fn text_after_offset(
        &self,
        offset: i32,
        boundary_type: IA2TextBoundaryType,
    ) -> (i32, i32, String) {
        if self._iat.is_none() {
            return (0, 0, String::new());
        }
        unsafe {
            let (mut start_offset, mut end_offset, mut text) = std::mem::zeroed();
            if self
                ._iat
                .as_ref()
                .unwrap()
                .textAfterOffset(
                    offset,
                    boundary_type,
                    &mut start_offset,
                    &mut end_offset,
                    &mut text,
                )
                .is_err()
            {
                return (0, 0, String::new());
            }
            (start_offset, end_offset, text.to_string())
        }
    }

    /**
     * 返回跨越给定位置的文本部分(start_offset, end_offset, text)。
     * 返回由指定边界类型在指定偏移量处定义的子字符串。有关更多详细信息，请参阅IA2TextBoundaryType。
     * 对于单词边界类型，如果偏移量在单词内部，则返回的字符串将包含偏移量处的单词；如果偏移量不在单词内，则返回字符串将包含在偏移量之前的单词。从单词的第一个字符到最后一个字符的所有偏移都被考虑在单词内部。句子和段落的边界类型应该表现出相似的行为。
     * 如果索引有效，但未找到文本，则返回S_FALSE。当文本完全由空白组成时，这种情况会发生在字符以外的边界类型上。
     * `start_offset`第一个字符的基于0的偏移量。
     * `end_offset` 最后一个字符后一个字符的基于0的偏移量。
     * `text` 返回请求的文本部分。当找不到合适的文本部分或文本类型无效时，此部分可能为空或无效。
     * 如果未实现请求的边界类型，例如IA2_TEXT_boundary_SENTENCE，或者没有任何可返回的内容，则值分别为0s和NULL。
     * `offset` 返回所属文本部分的字符索引。有效范围为从零到长度。请参阅@ref _specialOffsets“IAccess中使用的特殊偏移
     * `boundary_type` 要返回的文本部分的类型。有关完整列表，请参见：IA2TextBoundaryType。
     * */
    pub fn text_at_offset(
        &self,
        offset: i32,
        boundary_type: IA2TextBoundaryType,
    ) -> (i32, i32, String) {
        if self._iat.is_none() {
            return (0, 0, String::new());
        }
        unsafe {
            let (mut start_offset, mut end_offset, mut text) = std::mem::zeroed();
            if self
                ._iat
                .as_ref()
                .unwrap()
                .textAtOffset(
                    offset,
                    boundary_type,
                    &mut start_offset,
                    &mut end_offset,
                    &mut text,
                )
                .is_err()
            {
                return (0, 0, String::new());
            }
            (start_offset, end_offset, text.to_string())
        }
    }

    /**
     * 取消选择一系列文本。
     * `selection_index` 要删除选区的索引（从零开始）。
     * */
    pub fn remove_selection(&self, selection_index: i32) -> bool {
        if self._iat.is_none() {
            return false;
        }
        unsafe {
            self._iat
                .as_ref()
                .unwrap()
                .removeSelection(selection_index)
                .is_ok()
        }
    }

    /**
     * 设置插入符号的位置。
     * 插入符号位置/偏移量是逻辑上跟在它后面的字符的位置/偏移，例如，在从左到右的语言中，在它的右边。
     * 设置插入符号位置可能会也可能不会改变当前选择。选择的更改将通过IA2_EVENT_TEXT_SELECTION_CHANGED事件通知给辅助功能事件侦听器。
     * 当新插入符号位置与旧插入符号位置不同时（这是标准情况），将通过IA2_EVENT_TEXT_CARET_MOVED事件通知辅助功能事件侦听器。
     * `offset` 插入符号的新索引。这个插入符号实际上被放置在具有该索引的字符的左侧。索引0放置插入符号，以便下一次插入位于第一个字符之前。n_characters的索引导致插入最后一个字符之后。有关特殊偏移的信息，请参阅@ref _specialOffsets“在IAccessibleText和IAccessibleEditableText方法中使用的特殊偏移”
     * */
    pub fn set_caret_offset(&self, offset: i32) -> bool {
        if self._iat.is_none() {
            return false;
        }
        unsafe { self._iat.as_ref().unwrap().setCaretOffset(offset).is_ok() }
    }

    /**
     * 更改现有选区的边界。
     * `selection_index` 要更改的选区的索引（从零开始）
     * `start_offset` 新的起始偏移量（从零开始）。
     * `end_offset` 新地结束偏移量（从零开始）-字符刚好超过所选最后一个字符的偏移量。
     * */
    pub fn set_selection(&self, selection_index: i32, start_offset: i32, end_offset: i32) -> bool {
        if self._iat.is_none() {
            return false;
        }
        unsafe {
            self._iat
                .as_ref()
                .unwrap()
                .setSelection(selection_index, start_offset, end_offset)
                .is_ok()
        }
    }

    /**
     * 返回字符总数。
     * 请注意，如果文本包含多字节字符，则这可能与存储文本所需的总字节数不同。
     * */
    pub fn n_characters(&self) -> i32 {
        if self._iat.is_none() {
            return 0;
        }
        unsafe {
            let mut num = std::mem::zeroed();
            self._iat
                .as_ref()
                .unwrap()
                .nCharacters(&mut num)
                .from_abi(num)
                .unwrap_or(0)
        }
    }

    /**
     * 使字符串的特定部分在屏幕上可见。
     * `start_index` 从零开始的字符偏移量。
     * `end_index` 从零开始的字符偏移量-刚好超过字符串最后一个字符的字符的偏移量。
     * `scroll_type` 定义对象应放在屏幕上的位置。
     * 有关可以在%IAccessibleText方法中使用的特殊偏移的信息，请参阅@ref _specialOffsets“在IAccessibleText和IAccessible EditableText方法中的特殊偏移”。
     * */
    pub fn scroll_substring_to(
        &self,
        start_index: i32,
        end_index: i32,
        scroll_type: IA2ScrollType,
    ) -> bool {
        if self._iat.is_none() {
            return false;
        }
        unsafe {
            self._iat
                .as_ref()
                .unwrap()
                .scrollSubstringTo(start_index, end_index, scroll_type)
                .is_ok()
        }
    }

    /**
     * 将子字符串的左上角移动到指定位置。
     * `start_index` 从零开始的字符偏移量。
     * `end_index` 从零开始的字符偏移量-刚好超过字符串最后一个字符的字符的偏移量。
     * `coordinate_type` 指定坐标是相对于屏幕还是相对于父对象。
     * `x` 定义x坐标。
     * `y` 定义y坐标。
     * 如果对象已在指定位置，则返回S_FALSE。
     * 有关可以在%IAccessibleText方法中使用的特殊偏移的信息，请参阅@ref _specialOffsets“在IAccessibleText和IAccessible EditableText方法中的特殊偏移”。
     * */
    pub fn scroll_substring_to_point(
        &self,
        start_index: i32,
        end_index: i32,
        coordinate_type: IA2CoordinateType,
        x: i32,
        y: i32,
    ) -> bool {
        if self._iat.is_none() {
            return false;
        }
        unsafe {
            self._iat
                .as_ref()
                .unwrap()
                .scrollSubstringToPoint(start_index, end_index, coordinate_type, x, y)
                .is_ok()
        }
    }

    /**
     * 返回任何插入的文本。
     * 提供给IA2_EVENT_TEXT_INSERTED和IA2_EVENT_TEXT_UPDATED事件处理程序使用。
     * 只有当通知事件的线程继续时，才能保证此数据有效。一旦处理程序返回，数据的有效性取决于服务器如何管理其对象的生命周期。
     * 此外，请注意，根据控件是否管理其子控件，服务器可能具有不同的控件生命周期管理策略。列表、树和表可以有大量的子对象，因此这些控件的子对象可能只会根据需要创建。服务器应记录其生命周期策略，因为这将对辅助技术或脚本引擎访问进程外或其他线程的数据感兴趣。服务器只需要保存最后插入的文本块，整个应用程序的范围就足够了。
     * */
    pub fn new_text(&self) -> Result<IA2TextSegment> {
        if self._iat.is_none() {
            return Err(Error::new(S_FALSE, HSTRING::from("Not supported.")));
        }
        unsafe {
            let mut text = std::mem::zeroed();
            let res = self._iat.as_ref().unwrap().newText(&mut text);
            if res.is_err() {
                return Err(Error::new(S_FALSE, res.message()));
            }
            Ok(text)
        }
    }

    /**
     * 返回任何删除的文本。
     * 提供给IA2_EVENT_TEXT_REMOVED/UPDATED事件处理程序使用。
     * 只有当通知事件的线程继续时，才能保证此数据有效。一旦处理程序返回，数据的有效性就取决于服务器如何管理其对象的生命周期。
     * 此外，请注意，根据控件是否管理其子控件，服务器可能具有不同的控件生命周期管理策略。列表、树和表可以有大量的子对象，因此这些控件的子对象可能只会根据需要创建。服务器应记录其生命周期策略，因为这将对辅助技术或脚本引擎访问进程外或其他线程的数据感兴趣。服务器只需要保存最后删除的文本块，整个应用程序的范围就足够了。
     * */
    pub fn old_text(&self) -> Result<IA2TextSegment> {
        if self._iat.is_none() {
            return Err(Error::new(S_FALSE, HSTRING::from("Not supported.")));
        }
        unsafe {
            let mut text = std::mem::zeroed();
            let res = self._iat.as_ref().unwrap().oldText(&mut text);
            if res.is_err() {
                return Err(Error::new(S_FALSE, res.message()));
            }
            Ok(text)
        }
    }
}

impl Debug for AccessibleText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AccessibleText(caret:{}, n_characters:{}, n_selections:{})",
            self.caret_offset().unwrap_or(0),
            self.n_characters(),
            self.n_selections()
        )
    }
}
