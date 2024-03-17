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

use crate::JabLib::packages::AccessibleTextAttributesInfo;

#[derive(Debug)]
pub struct AccessibleTextAttributes {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub superscript: bool,
    pub subscript: bool,

    pub background_color: String,
    pub foreground_color: String,
    pub font_family: String,
    pub font_size: i32,

    pub alignment: i32,
    pub bidi_level: i32,

    pub first_line_indent: f32,
    pub left_indent: f32,
    pub right_indent: f32,
    pub line_spacing: f32,
    pub space_above: f32,
    pub space_below: f32,

    pub full_attributes_string: String,
}

impl AccessibleTextAttributes {
    /**
     * 创建一个实例。
     * */
    pub(crate) fn new(info: AccessibleTextAttributesInfo) -> Self {
        Self {
            bold: info.bold != 0,
            italic: info.italic != 0,
            underline: info.underline != 0,
            strikethrough: info.strikethrough != 0,
            superscript: info.superscript != 0,
            subscript: info.subscript != 0,
            background_color: String::from_utf16_lossy(&info.backgroundColor)
                .trim_matches('\0')
                .to_string(),
            foreground_color: String::from_utf16_lossy(&info.foregroundColor)
                .trim_matches('\0')
                .to_string(),
            font_family: String::from_utf16_lossy(&info.fontFamily)
                .trim_matches('\0')
                .to_string(),
            font_size: info.fontSize,
            alignment: info.alignment,
            bidi_level: info.bidiLevel,
            first_line_indent: info.firstLineIndent,
            left_indent: info.leftIndent,
            right_indent: info.rightIndent,
            line_spacing: info.lineSpacing,
            space_above: info.spaceAbove,
            space_below: info.spaceBelow,
            full_attributes_string: String::from_utf16_lossy(&info.fullAttributesString)
                .trim_matches('\0')
                .to_string(),
        }
    }
}
