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
    common::{BOOL, FALSE},
    uia::element::UiAutomationElement,
};
use windows::core::BSTR;
use windows::Win32::UI::Accessibility::{IUIAutomationTextPattern2, UIA_TextPattern2Id};
use windows::{
    core::Interface,
    Win32::{
        Foundation::POINT,
        UI::Accessibility::{
            IUIAutomationTextPattern, IUIAutomationTextRange, IUIAutomationTextRangeArray,
            TextPatternRangeEndpoint_End, TextPatternRangeEndpoint_Start, TextUnit_Character,
            TextUnit_Document, TextUnit_Format, TextUnit_Line, TextUnit_Page, TextUnit_Paragraph,
            TextUnit_Word, UIA_TextPatternId,
        },
    },
};

/**
 * 提供对包含文本的控件的访问。
 * */
pub struct UiAutomationTextPattern(IUIAutomationTextPattern);

/// https://learn.microsoft.com/en-us/windows/win32/api/uiautomationclient/nn-uiautomationclient-iuiautomationtextpattern
impl UiAutomationTextPattern {
    /**
     * 从UI元素获取此模式。
     * */
    pub fn obtain(value: &UiAutomationElement) -> Result<Self, String> {
        let pattern = unsafe { value.get_raw().GetCurrentPattern(UIA_TextPatternId) };
        if let Err(e) = pattern {
            return Err(format!("Can't get the TextPattern. ({})", e));
        }
        let pattern = pattern.unwrap().cast::<IUIAutomationTextPattern>().unwrap();
        Ok(Self(pattern))
    }

    /**
     * 查询包含文档正文的文本范围。
     * 此属性是只读的。
     * 某些辅助文本（如页眉、脚注或批注）可能不包括在内。
     * */
    pub fn document_range(&self) -> UiAutomationTextRange {
        unsafe { UiAutomationTextRange::obtain(&self.0.DocumentRange().unwrap()) }
    }

    //noinspection StructuralWrap
    /**
     * 查询文本范围的集合，该集合表示基于文本的控件中当前选定的文本。
     * 如果控件支持选择多个不连续的文本范围，则 ranges 集合将为每个选定的范围接收一个文本范围。
     * 如果控件仅包含所选文本的单个范围，则 ranges 集合将接收单个文本范围。
     * 如果控件包含文本插入点，但未选择任何文本，则 ranges 集合将在文本插入点的位置接收一个退化（空）文本区域。
     * 如果控件不包含文本插入点或不支持文本选择，则范围设置为 NULL。
     * 使用 supported_text_selection 属性测试控件是否支持文本选择。
     * */
    pub fn get_selection(&self) -> Vec<UiAutomationTextRange> {
        if let Ok(array) = unsafe { self.0.GetSelection() } {
            return array.to_vec();
        }
        vec![]
    }

    /**
     * 查询一个值，该值指定控件支持的文本选择的类型。
     * 此属性是只读的。
     * */
    pub fn supported_text_selection(&self) -> SupportedTextSelection {
        match unsafe { self.0.SupportedTextSelection() }.unwrap().0 {
            1 => SupportedTextSelection::Single,
            2 => SupportedTextSelection::Multiple,
            _ => SupportedTextSelection::None,
        }
    }

    /**
     * 从基于文本的控件中检索不相交的文本范围的数组，其中每个文本区域表示可见文本的连续范围。
     * */
    pub fn get_visible_ranges(&self) -> Vec<UiAutomationTextRange> {
        unsafe {
            if let Ok(array) = self.0.GetVisibleRanges() {
                return array.to_vec();
            }
        }
        vec![]
    }

    /**
     * 查询包含子元素（如图像、超链接、Microsoft Excel 电子表格或其他嵌入对象）的文本区域。
     * 如果区域中没有包含子元素的文本，则返回退化（空）区域。
     * `child` 要包含在文本范围中的子元素，可以是与 UiAutomationTextPattern 关联的元素的子参数，也可以是 UiAutomationTextRange 的子元素数组的子参数。
     * */
    pub fn range_from_child(&self, child: &UiAutomationElement) -> Option<UiAutomationTextRange> {
        if let Ok(c) = unsafe { self.0.RangeFromChild(child.get_raw()) } {
            return Some(UiAutomationTextRange::obtain(&c));
        }
        None
    }

    /**
     * 查询最接近指定屏幕坐标的退化（空）文本范围。
     * 如果屏幕坐标位于图像、超链接、Microsoft Excel 电子表格或其他嵌入对象的坐标内，则返回换行子对象的文本区域。
     * 由于不会忽略隐藏文本，因此此方法从最接近指定坐标的可见文本中检索退化范围。
     * Windows Internet Explorer 9 中 range_from_point 的实现不会返回预期的结果。相反，客户应该：
     * 1. 调用get_visible_ranges方法以检索可见文本范围的数组。
     * 2. 对于数组中的每个文本范围，调用get_bounding_rectangles以检索边界矩形。
     * 3. 检查边界矩形以查找占据特定屏幕坐标的文本范围。
     * */
    pub fn range_from_point(&self, x: i32, y: i32) -> Option<UiAutomationTextRange> {
        if let Ok(x) = unsafe { self.0.RangeFromPoint(POINT { x, y }) } {
            return Some(UiAutomationTextRange::obtain(&x));
        }
        None
    }
}

/// 扩展 UiAutomationTextPattern。
pub struct UiAutomationTextPattern2(IUIAutomationTextPattern2);

/// https://learn.microsoft.com/en-us/windows/win32/api/uiautomationclient/nn-uiautomationclient-iuiautomationtextpattern2
impl UiAutomationTextPattern2 {
    /**
     * 从UI元素获取此模式。
     * */
    pub fn obtain(value: &UiAutomationElement) -> Result<Self, String> {
        let result = unsafe { value.get_raw().GetCurrentPattern(UIA_TextPattern2Id) };

        match result {
            Ok(pattern) => Ok(UiAutomationTextPattern2(
                pattern.cast::<IUIAutomationTextPattern2>().unwrap(),
            )),

            Err(e) => Err(format!("Can't get the TextPattern2. ({})", e)),
        }
    }

    //noinspection StructuralWrap
    /**
     * 查询属于基于文本的控件的插入符号位置的零长度文本范围。
     * 此方法检索一个文本区域，客户端可以使用该文本区域查找属于基于文本的控件的插入符号的边界矩形，或查找插入符号附近的文本。
     * 返回(is_active,range)。
     * `is_active` 如果包含插入符号的基于文本的控件具有键盘焦点，则为 true，否则为 false。如果 is_active 为 false，则属于基于文本的控件的插入符号可能与系统插入符号位于同一位置。
     * `range` 接收一个文本范围，该范围表示属于基于文本的控件的插入符号的当前位置。
     * */
    pub fn get_caret_range(&self) -> (bool, UiAutomationTextRange) {
        let mut active = BOOL::default();
        let range = unsafe { self.0.GetCaretRange(&mut active).unwrap() };

        (active.as_bool(), UiAutomationTextRange::obtain(&range))
    }

    //noinspection StructuralWrap
    /**
     * 查询包含文本的文本范围，该文本是与指定批注元素关联的批注的目标。
     * `annotation` 要检索其目标文本的批注元素。此元素是实现文档的 UiAutomationTextPattern2 的元素的同级元素。
     * */
    pub fn range_from_annotation(&self, annotation: &UiAutomationElement) -> UiAutomationTextRange {
        let range = unsafe { self.0.RangeFromAnnotation(annotation.get_raw()).unwrap() };

        UiAutomationTextRange::obtain(&range)
    }
}

unsafe impl Send for UiAutomationTextPattern2 {}

unsafe impl Sync for UiAutomationTextPattern2 {}

trait TextRangeArray {
    fn to_vec(&self) -> Vec<UiAutomationTextRange>;
}

impl TextRangeArray for IUIAutomationTextRangeArray {
    fn to_vec(&self) -> Vec<UiAutomationTextRange> {
        let mut v = vec![];
        unsafe {
            for i in 0..self.Length().unwrap() {
                if let Ok(item) = self.GetElement(i) {
                    v.push(UiAutomationTextRange::obtain(&item));
                }
            }
        }
        v
    }
}

/**
 * 提供对支持 IUIAutomationTextPattern 接口的容器中连续文本范围的访问。
 * 客户端应用程序可以使用 IUIAutomationTextRange 接口从文本范围中选择、比较和查询嵌入对象。
 * 该接口使用两个端点来分隔文本范围的开始和结束位置。
 * 文本的不相交范围由 IUIAutomationTextRangeArray 接口表示。
 * */
#[derive(Clone, Debug)]
pub struct UiAutomationTextRange(IUIAutomationTextRange);

/// https://learn.microsoft.com/en-us/windows/win32/api/uiautomationclient/nn-uiautomationclient-iuiautomationtextrange
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
     * `src_start_endpoint` 源范围的端点类型，如果是true表示使用起始点，false表示使用结束点。
     * `range` 要比较的文本区域。
     * `target_start_endpoint` 目标范围的端点类型，如果是true表示使用起始点，false表示使用结束点。
     * */
    pub fn compare_endpoints(
        &self,
        src_start_endpoint: bool,
        range: &UiAutomationTextRange,
        target_start_endpoint: bool,
    ) -> i32 {
        let src_endpoint = if src_start_endpoint {
            TextPatternRangeEndpoint_Start
        } else {
            TextPatternRangeEndpoint_End
        };
        let target_endpoint = if target_start_endpoint {
            TextPatternRangeEndpoint_Start
        } else {
            TextPatternRangeEndpoint_End
        };
        unsafe {
            self.0
                .CompareEndpoints(src_endpoint, &range.0, target_endpoint)
        }
            .unwrap_or(0)
    }

    //noinspection StructuralWrap
    /**
     * 按指定的文本单位规范化文本范围。
     * 客户端应用程序（如屏幕阅读器）使用此方法检索插入点或插入符号位置处存在的完整单词、句子或段落。
     * 尽管名称如此，但 expand_to_enclosing_unit 方法不一定扩展文本范围。
     * 相反，它通过移动终结点来“规范化”文本范围，使该范围包含指定的文本单元。
     * 如果范围小于指定单位，则扩大范围，如果范围长于指定单位，则缩短范围。
     * 如果范围已经是指定单位的确切数量，则保持不变。
     * 如果控件不支持指定的文本单位，则 expand_to_enclosing_unit 默认为支持的下一个最大文本单元。
     * 从最小单位到最大单位的顺序如下： 字符、格式、词、行、段、页、文档。
     * expand_to_enclosing_unit 同时支持可见文本和隐藏文本。
     * Format，作为单位值，定位文本范围的边界，以根据范围内文本的共享文本属性（或格式）来扩展或移动范围。但是，文本单元不会在嵌入对象（如图像或超链接）的边界上移动或扩展文本范围。
     * 有关详细信息，请参阅 UI 自动化文本单元或文本内容的 UI 自动化支持。
     * `text_unit` 文本单位，例如行或段落。
     * */
    pub fn expand_to_enclosing_unit(&self, text_unit: TextUnit) {
        let unit = match text_unit {
            TextUnit::Character => TextUnit_Character,
            TextUnit::Format => TextUnit_Format,
            TextUnit::Word => TextUnit_Word,
            TextUnit::Line => TextUnit_Line,
            TextUnit::Paragraph => TextUnit_Paragraph,
            TextUnit::Page => TextUnit_Page,
            TextUnit::Document => TextUnit_Document,
        };
        unsafe { self.0.ExpandToEnclosingUnit(unit) }.unwrap_or(())
    }

    //noinspection StructuralWrap
    /**
     * 返回文本范围的纯文本。
     * `max_length` 要返回的字符串的最大长度，如果不需要限制，则为 -1。
     * */
    pub fn get_text(&self, max_length: i32) -> String {
        unsafe { self.0.GetText(max_length) }
            .unwrap_or(BSTR::new())
            .to_string()
    }
}

unsafe impl Sync for UiAutomationTextRange {}

unsafe impl Send for UiAutomationTextRange {}

/*
 * 包含指定用于导航的文本单位的值。
 * */
pub enum TextUnit {
    /// 字符
    Character,
    /// 格式
    Format,
    /// 单词
    Word,
    /// 行
    Line,
    /// 段落
    Paragraph,
    /// 页面
    Page,
    /// 文档
    Document,
}

pub enum SupportedTextSelection {
    /// 不支持文本选择。
    None,
    /// 支持单个连续文本选择。
    Single,
    /// 支持多个不相交的文本选择。
    Multiple,
}
