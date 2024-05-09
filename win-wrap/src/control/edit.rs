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

use std::ffi::CString;

pub use windows::Win32::{
    Foundation::POINT,
    UI::{
        Controls::{
            Dialogs::{
                FINDREPLACE_FLAGS, FR_DIALOGTERM, FR_DOWN, FR_ENABLEHOOK, FR_ENABLETEMPLATE,
                FR_ENABLETEMPLATEHANDLE, FR_FINDNEXT, FR_HIDEMATCHCASE, FR_HIDEUPDOWN,
                FR_HIDEWHOLEWORD, FR_MATCHALEFHAMZA, FR_MATCHCASE, FR_MATCHDIAC, FR_MATCHKASHIDA,
                FR_NOMATCHCASE, FR_NOUPDOWN, FR_NOWHOLEWORD, FR_NOWRAPAROUND, FR_RAW, FR_REPLACE,
                FR_REPLACEALL, FR_SHOWHELP,
            },
            RichEdit::{
                CHARRANGE, EM_CANPASTE, EM_EXGETSEL, EM_EXLINEFROMCHAR, EM_EXSETSEL, EM_FINDTEXT,
                EM_GETSELTEXT, EM_GETTEXTRANGE, EM_HIDESELECTION, EM_SELECTIONTYPE,
                RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE, SEL_EMPTY, SEL_MULTICHAR, SEL_MULTIOBJECT,
                SEL_OBJECT, SEL_TEXT,
            },
            EM_CANUNDO, EM_CHARFROMPOS, EM_EMPTYUNDOBUFFER, EM_GETFIRSTVISIBLELINE, EM_GETLINE,
            EM_GETLINECOUNT, EM_GETMARGINS, EM_GETMODIFY, EM_GETRECT, EM_GETSEL, EM_LINEFROMCHAR,
            EM_LINEINDEX, EM_LINELENGTH, EM_LINESCROLL, EM_POSFROMCHAR, EM_REPLACESEL, EM_SCROLL,
            EM_SCROLLCARET, EM_SETMARGINS, EM_SETMODIFY, EM_SETREADONLY, EM_SETSEL,
        },
        WindowsAndMessaging::{
            EC_LEFTMARGIN, EC_RIGHTMARGIN, EC_USEFONTINFO, SB_BOTTOM, SB_ENDSCROLL, SB_LEFT,
            SB_LINEDOWN, SB_LINELEFT, SB_LINERIGHT, SB_LINEUP, SB_PAGEDOWN, SB_PAGELEFT,
            SB_PAGERIGHT, SB_PAGEUP, SB_RIGHT, SB_THUMBPOSITION, SB_THUMBTRACK, SB_TOP,
            SCROLLBAR_COMMAND,
        },
    },
};
use windows::{
    core::{
        imp::{heap_alloc, heap_free},
        HSTRING, PCWSTR, PWSTR,
    },
    Win32::UI::Controls::RichEdit::{FINDTEXTEXW, FINDTEXTW, TEXTRANGEW},
};

use crate::{
    common::{LPARAM, RECT, WPARAM},
    control::WindowControl,
    ext::StringExt,
};

pub trait Edit {
    fn get_line(&self, index: i32, len: u16) -> Option<String>;
    fn line_length(&self, position: i32) -> usize;
    fn line_index(&self, line_index: i32) -> i32;
    fn replace_sel(&self, allow_undo: bool, text: &str);
    fn set_readonly(&self, enable: bool) -> usize;
    fn get_text_range(&self, min: i32, max: i32) -> Option<String>;
    fn can_undo(&self) -> bool;
    fn empty_undo_buffer(&self);
    fn get_first_visible_line(&self) -> i32;
    fn get_line_count(&self) -> usize;
    fn get_modify(&self) -> bool;
    fn set_modify(&self, is_modified: bool);
    fn get_rect(&self) -> RECT;
    fn get_sel(&self) -> (i32, i32);
    fn get_sel_ex(&self) -> (i32, i32);
    fn set_sel(&self, start: i32, end: i32);
    fn set_sel_ex(&self, min: i32, max: i32) -> usize;
    fn get_sel_text(&self, len: usize) -> Option<String>;
    fn line_from_char(&self, position: i32) -> i32;
    fn line_from_char_ex(&self, position: i32) -> i32;
    fn scroll(&self, command: SCROLLBAR_COMMAND) -> isize;
    fn line_scroll(&self, column: i32, line: i32) -> bool;
    fn scroll_caret(&self);
    fn can_paste(&self) -> bool;
    fn char_from_pos(&self, x: i32, y: i32) -> i32;
    fn pos_from_char(&self, char_index: i32) -> (i32, i32);
    fn selection_type(&self) -> RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE;
    fn hide_selection(&self, hide: bool);
    fn find_text(&self, flags: FINDREPLACE_FLAGS, text: &str, min: i32, max: i32) -> i32;
    fn find_text_ex(&self, flags: FINDREPLACE_FLAGS, text: &str, min: i32, max: i32) -> (i32, i32);
    fn get_margins(&self) -> i32;
    fn set_margins(&self, margin: u32, value: i32);
}

impl Edit for WindowControl {
    /**
     * 从编辑控件复制一行文本。 可以将此消息发送到编辑控件或富编辑控件。
     * 富编辑控件： Microsoft Rich Edit 1.0 及更高版本中受支持。如果未复制文本，则消息会在缓冲区的开头放置一个 null 字符。 有关丰富编辑版本与各种系统版本的兼容性的信息，请参阅 关于丰富编辑控件。
     * `index` 要从多行编辑控件检索的行的从零开始的索引。值为零指定最上面的行。 单行编辑控件会忽略此参数。
     * `len` 要获取的字符数。
     * */
    fn get_line(&self, index: i32, len: u16) -> Option<String> {
        unsafe {
            let Ok(ptr) = heap_alloc((len * 2 + 1) as usize) else {
                return None;
            };
            ptr.write_bytes(b'\0', (len * 2 + 1) as usize);
            (ptr as *mut u16).write(len);
            let (_, _) =
                self.send_message(EM_GETLINE, WPARAM(index as usize), LPARAM(ptr as isize));
            let text = (ptr as *const u16).to_string_utf16();
            heap_free(ptr);
            Some(text)
        }
    }

    /**
     * 查询编辑控件中行的长度（以字符为单位）。
     * 对于多行编辑控件，返回值是 `position` 参数指定的行的长度。 对于 ANSI 文本，这是字节数;对于 Unicode 文本，这是字符数。 不包括行末尾的回车符。
     * 对于单行编辑控件，返回值是编辑控件中文本的长度。
     * 使用 EM_LINEINDEX 消息检索多行编辑控件中给定行号的字符索引。
     * 富编辑： Microsoft Rich Edit 1.0 及更高版本中受支持。 有关丰富编辑版本与各种系统版本的兼容性的信息，请参阅 关于丰富编辑控件。
     * `position` 行中要检索其长度的字符的字符索引。 如果此参数大于控件中的字符数，则返回值为零。此参数可以为 -1。 在这种情况下，消息返回包含选定字符的行上的未选定字符数。 例如，如果所选内容从下一行末尾的第四个字符扩展到第八个字符，则返回值为 10 (第一行的 3 个字符，下一行的) 7 个字符。
     * */
    fn line_length(&self, position: i32) -> usize {
        let (_, res) =
            self.send_message(EM_LINELENGTH, WPARAM(position as usize), LPARAM::default());
        res
    }

    /**
     * 获取多行编辑控件中指定行的第一个字符的字符索引。 字符索引是从编辑控件开头开始的字符的从零开始的索引。 可以将此消息发送到编辑控件或富编辑控件。
     * 返回值是 `line_index` 参数中指定的行的字符索引;如果指定的行号大于编辑控件中的行数，则返回值为 -1。
     * 富编辑： Microsoft Rich Edit 1.0 及更高版本中受支持。 有关丰富编辑版本与各种系统版本的兼容性的信息，请参阅 关于丰富编辑控件。
     * `line_index` 从零开始的行号。 值 -1 (当前行号指定包含插入点) 的行。
     * */
    fn line_index(&self, line_index: i32) -> i32 {
        let (_, res) =
            self.send_message(EM_LINEINDEX, WPARAM(line_index as usize), LPARAM::default());
        res as i32
    }

    /**
     * 将编辑控件或富编辑控件中的选定文本替换为指定的文本。
     * 使用 EM_REPLACESEL 消息仅替换编辑控件中文本的一部分。 若要替换所有文本，请使用 WM_SETTEXT 消息。
     * 如果没有选择，替换文本将插入插入点处。
     * Rich Edit： 在 Microsoft Rich Edit 1.0 及更高版本中受支持。 有关 Rich Edit 版本与各种系统版本的兼容性的信息，请参阅 关于 Rich Edit 控件。
     * 在丰富编辑控件中，替换文本采用插入点处的字符的格式，如果选定内容中第一个字符，则采用格式设置。
     * `allow_undo` 指定是否可以撤消替换操作。 如果为 true，则可以撤消该操作。 如果为 false ，则无法撤消操作。
     * `text` 要替换的文本。
     * */
    fn replace_sel(&self, allow_undo: bool, text: &str) {
        let text = CString::new(text).unwrap();
        let allow_undo = if allow_undo { WPARAM(1) } else { WPARAM(0) };
        self.send_message(EM_REPLACESEL, allow_undo, LPARAM(text.as_ptr() as isize));
    }

    /**
     * 设置或删除只读样式 (ES_READONLY 编辑控件的) 。 可以将此消息发送到编辑控件或富编辑控件。
     * 如果操作成功，则返回值为非零值。如果操作失败，则返回值为零。
     * 当编辑控件具有 ES_READONLY 样式时，用户无法更改编辑控件中的文本。
     * 若要确定编辑控件是否具有 ES_READONLY 样式，请使用带有 GWL_STYLE 标志的 get_window_long 函数。
     * 富编辑： Microsoft Rich Edit 1.0 及更高版本中受支持。 有关丰富编辑版本与各种系统版本的兼容性的信息，请参阅 关于丰富编辑控件。
     * `enable` 指定是设置还是删除 ES_READONLY 样式。 值为 true 设置 ES_READONLY 样式;值为 false 会删除 ES_READONLY 样式。
     * */
    fn set_readonly(&self, enable: bool) -> usize {
        let enable = if enable { WPARAM(1) } else { WPARAM(0) };
        let (_, res) = self.send_message(EM_SETREADONLY, enable, LPARAM::default());
        res
    }

    /**
     * 从丰富编辑控件中查询指定的字符范围。
     * `min` 最小索引。
     * `max` 最大索引。
     * */
    fn get_text_range(&self, min: i32, max: i32) -> Option<String> {
        let len = (max - min).abs();
        unsafe {
            let Ok(ptr) = heap_alloc((len * 2 + 1) as usize) else {
                return None;
            };
            ptr.write_bytes(b'\0', (len * 2 + 1) as usize);
            let mut range2: TEXTRANGEW = std::mem::zeroed();
            range2.chrg.cpMax = max;
            range2.chrg.cpMin = min;

            range2.lpstrText = PWSTR(ptr as *mut u16);
            self.send_message(
                EM_GETTEXTRANGE,
                WPARAM::default(),
                LPARAM(&mut range2 as *const TEXTRANGEW as isize),
            );
            let text = (ptr as *const u16).to_string_utf16();
            heap_free(ptr);
            Some(text)
        }
    }

    /**
     * 确定编辑控件的撤消队列中是否有任何操作。 可以将此消息发送到编辑控件或富编辑控件。
     * 如果撤消队列不为空，可以将 EM_UNDO 消息发送到控件以撤消最近的操作。
     * 编辑控件和 Rich Edit 1.0： 撤消队列仅包含最近的操作。
     * Rich Edit 2.0 及更高版本： 撤消队列可以包含多个操作。
     * Rich Edit： 在 Microsoft Rich Edit 1.0 及更高版本中受支持。 有关 Rich Edit 版本与各种系统版本的兼容性的信息，请参阅 关于 Rich Edit 控件。
     * */
    fn can_undo(&self) -> bool {
        let (_, res) = self.send_message(EM_CANUNDO, WPARAM::default(), LPARAM::default());
        res != 0
    }

    /**
     * 重置编辑控件的撤消标志。 每当可撤消编辑控件中的操作时，都会设置撤消标志。 可以将此消息发送到编辑控件或富编辑控件。
     * 每当编辑控件收到 WM_SETTEXT 或 EM_SETHANDLE 消息时，撤消标志都会自动重置。
     * 编辑控件和 Rich Edit 1.0： 控件只能撤消或重做最近的操作。
     * Rich Edit 2.0 及更高版本：EM_EMPTYUNDOBUFFER消息清空所有撤消和重做缓冲区。 丰富的编辑控件使用户能够撤消或重做多个操作。
     * Rich Edit： 在 Microsoft Rich Edit 1.0 及更高版本中受支持。 有关 Rich Edit 版本与各种系统版本的兼容性的信息，请参阅 关于 Rich Edit 控件。
     * */
    fn empty_undo_buffer(&self) {
        self.send_message(EM_EMPTYUNDOBUFFER, WPARAM::default(), LPARAM::default());
    }

    /**
     * 获取多行编辑控件中最上面的可见行的从零开始的索引。 可以将此消息发送到编辑控件或富编辑控件。
     * 返回值是多行编辑控件中最上面的可见行的从零开始的索引。
     * 编辑控件： 对于单行编辑控件，返回值是第一个可见字符的从零开始的索引。
     * 富编辑控件： 对于单行富编辑控件，返回值为零。
     * 编辑控件中的行数和行长度取决于控件的宽度和当前 Wordwrap 设置。
     * 富编辑： Microsoft Rich Edit 1.0 及更高版本中受支持。 有关丰富编辑版本与各种系统版本的兼容性的信息，请参阅 关于丰富编辑控件。
     * */
    fn get_first_visible_line(&self) -> i32 {
        let (_, res) =
            self.send_message(EM_GETFIRSTVISIBLELINE, WPARAM::default(), LPARAM::default());
        res as i32
    }

    /**
     * 获取多行编辑控件中的行数。 可以将此消息发送到编辑控件或富编辑控件。
     * 返回值是一个整数，用于指定多行编辑控件或富编辑控件中的文本行总数。 如果控件没有文本，则返回值为 1。 返回值永远不会小于 1。
     * EM_GETLINECOUNT消息检索文本行的总数，而不仅仅是当前可见的行数。
     * 如果启用了 Wordwrap 功能，则当编辑窗口的尺寸发生更改时，行数可能会更改。
     * Rich Edit： 在 Microsoft Rich Edit 1.0 及更高版本中受支持。 有关 Rich Edit 版本与各种系统版本的兼容性的信息，请参阅 关于 Rich Edit 控件。
     */
    fn get_line_count(&self) -> usize {
        let (_, res) = self.send_message(EM_GETLINECOUNT, WPARAM::default(), LPARAM::default());
        res
    }

    /**
     * 获取编辑控件的修改标志的状态。 标志指示是否已修改编辑控件的内容。 可以将此消息发送到编辑控件或富编辑控件。
     * 创建控件时，系统会自动将修改标志清除为零。 如果用户更改控件的文本，系统将标志设置为非零。 可以将 EM_SETMODIFY 消息发送到编辑控件以设置或清除标志。
     * 富编辑： Microsoft Rich Edit 1.0 及更高版本中受支持。 有关丰富编辑版本与各种系统版本的兼容性的信息，请参阅 关于丰富编辑控件。
     * */
    fn get_modify(&self) -> bool {
        let (_, res) = self.send_message(EM_GETMODIFY, WPARAM::default(), LPARAM::default());
        res != 0
    }

    /**
     * 设置或清除编辑控件的修改标志。 修改标志指示是否已修改编辑控件中的文本。 可以将此消息发送到编辑控件或富编辑控件。
     * 创建控件时，系统会自动将修改标志清除为零。 如果用户更改控件的文本，系统将标志设置为非零。 可以将 EM_GETMODIFY 消息发送到编辑控件，以检索标志的当前状态。
     * Rich Edit 1.0： 在没有 REO_DYNAMICSIZE 标志的情况下创建的对象会在修改标志设置为 FALSE 时锁定其盘区。
     * 富编辑： Microsoft Rich Edit 1.0 及更高版本中受支持。 有关丰富编辑版本与各种系统版本的兼容性的信息，请参阅 关于丰富编辑控件。
     * `is_modified` 修改标志的新值。 值为 true 表示文本已修改，值为 false 表示尚未修改。
     * */
    fn set_modify(&self, is_modified: bool) {
        let is_modified = if is_modified { WPARAM(1) } else { WPARAM(0) };
        self.send_message(EM_SETMODIFY, is_modified, LPARAM::default());
    }

    /**
     * 获取编辑控件 的格式设置矩形 。 格式设置矩形是控件在其中绘制文本的限制矩形。 限制矩形与编辑控件窗口的大小无关。 可以将此消息发送到编辑控件或富编辑控件。
     * 可以使用 EM_SETRECT 和 EM_SETRECTNP 消息来修改多行编辑控件的格式矩形。
     * 在某些情况下， EM_GETRECT 可能不会返回 EM_SETRECT 或 EM_SETRECTNP 设置的确切值，该值大致正确，但可能会关闭几个像素。
     * Rich Edit： 在 Microsoft Rich Edit 1.0 及更高版本中受支持。 格式设置矩形不包括选择栏，这是每个段落左侧的未标记区域。 单击后，选择栏将选择行。 有关 Rich Edit 版本与各种系统版本的兼容性的信息，请参阅 关于 Rich Edit 控件。
     * */
    fn get_rect(&self) -> RECT {
        let mut rect = unsafe { std::mem::zeroed() };
        self.send_message(
            EM_GETRECT,
            WPARAM::default(),
            LPARAM(&mut rect as *mut RECT as isize),
        );
        rect
    }

    /**
     * 获取编辑控件中当前所选内容的起始和结束字符位置。 可以将此消息发送到编辑控件或 Rich Edit 控件。
     * 如果没有选择内容，则起始值和结束值都是插入点的位置。
     * Rich Edit 控件：还可以使用 EM_EXGETSEL 消息来检索相同的信息。 EM_EXGETSEL 还以 32 位值的形式返回起始字符和结束字符位置。
     * Rich Edit：在 Microsoft Rich Edit 1.0 及更高版本中受支持。 若要了解 Rich Edit 版本与各种系统版本的兼容性，请参阅关于 Rich Edit 控件。
     * */
    fn get_sel(&self) -> (i32, i32) {
        let (mut start, mut end) = unsafe { std::mem::zeroed() };
        self.send_message(
            EM_GETSEL,
            WPARAM(&mut start as *mut i32 as usize),
            LPARAM(&mut end as *mut i32 as isize),
        );
        (start, end)
    }

    /**
     * 查询丰富编辑控件中所选内容的起始和结束字符位置。
     * */
    fn get_sel_ex(&self) -> (i32, i32) {
        let mut cr: CHARRANGE = unsafe { std::mem::zeroed() };
        self.send_message(
            EM_EXGETSEL,
            WPARAM::default(),
            LPARAM(&mut cr as *mut CHARRANGE as isize),
        );
        (cr.cpMin, cr.cpMax)
    }

    /**
     * 在编辑控件中选择一个字符范围。 可以将此消息发送到编辑控件或富编辑控件。
     * 起始值可以大于结束值。 两个值的下半部分指定所选内容中第一个字符的字符位置。 较高的值指定所选内容之外的第一个字符的位置。
     * 起始值是所选内容的定位点，结束值是活动结束。 如果用户使用 SHIFT 键调整所选内容的大小，活动端可以移动，但定位点保持不变。
     * 如果开头为 0，结尾为 -1，则选择编辑控件中的所有文本。 如果开始为 -1，则取消选择任何当前选择。
     * 编辑控件： 无论开始和结束的相对值如何，控件都会在结束位置显示闪烁的插入点。
     * Rich Edit： 在 Microsoft Rich Edit 1.0 及更高版本中受支持。 有关 Rich Edit 版本与各种系统版本的兼容性的信息，请参阅 关于 Rich Edit 控件。
     * 如果编辑控件具有 ES_NOHIDESEL 样式，则无论控件是否具有焦点，所选文本都会突出显示。 如果没有 ES_NOHIDESEL 样式，仅当编辑控件具有焦点时，才会突出显示所选文本。
     * `start` 所选内容的起始字符位置。
     * `end` 所选内容的结束字符位置。
     * */
    fn set_sel(&self, start: i32, end: i32) {
        self.send_message(EM_SETSEL, WPARAM(start as usize), LPARAM(end as isize));
    }

    /**
     * 选择一系列字符或组件对象模型 (COM) Microsoft Rich Edit 控件中的对象。
     * 返回值是实际设置的选定内容。
     * `min` 最小索引。
     * `max` 最大索引。
     * */
    fn set_sel_ex(&self, min: i32, max: i32) -> usize {
        let cr = CHARRANGE {
            cpMin: min,
            cpMax: max,
        };
        let (_, res) = self.send_message(
            EM_EXSETSEL,
            WPARAM::default(),
            LPARAM(&cr as *const CHARRANGE as isize),
        );
        res
    }

    /**
     * 查询格式编辑控件中当前选择的文本。
     * `len` 要获取的字符数量。
     * */
    fn get_sel_text(&self, len: usize) -> Option<String> {
        unsafe {
            let Ok(ptr) = heap_alloc(len * 2 + 1) else {
                return None;
            };
            ptr.write_bytes(b'\0', len * 2 + 1);
            self.send_message(EM_GETSELTEXT, WPARAM::default(), LPARAM(ptr as isize));
            let text = (ptr as *const u16).to_string_utf16();
            heap_free(ptr);
            Some(text)
        }
    }

    /**
     * 获取包含多行编辑控件中指定字符索引的行的索引。 字符索引是从编辑控件开头开始的字符的从零开始的索引。 可以将此消息发送到编辑控件或富编辑控件。
     * 返回值是包含  position  指定的字符索引的行的从零开始的行号。
     * Rich Edit：  在 Microsoft Rich Edit 1.0 及更高版本中受支持。 如果字符索引大于 64，000，请使用  EM_EXLINEFROMCHAR  消息。 有关 Rich Edit 版本与各种系统版本的兼容性的信息，请参阅  关于 Rich Edit 控件 。
     * `position` 要检索其编号的行中包含的字符的字符索引。 如果此参数为 -1，  EM_LINEFROMCHAR  将检索当前行的行号 (包含脱字号) 或者，如果有选定内容，则检索包含所选内容的开头的行号。
     * */
    fn line_from_char(&self, position: i32) -> i32 {
        let (_, res) = self.send_message(
            EM_LINEFROMCHAR,
            WPARAM(position as usize),
            LPARAM::default(),
        );
        res as i32
    }

    /**
     * 确定哪个行包含 Rich Edit 控件中的指定字符。
     * 此消息返回行的从零开始的索引。
     * `position` 字符的从零开始的索引。
     * */
    fn line_from_char_ex(&self, position: i32) -> i32 {
        let (_, res) = self.send_message(
            EM_EXLINEFROMCHAR,
            WPARAM::default(),
            LPARAM(position as isize),
        );
        res as i32
    }

    /**
     * 在多行编辑控件中垂直滚动文本。 此消息等效于将 WM\_VSCROLL 消息发送到编辑控件。 可以将此消息发送到编辑控件或富编辑控件。
     * 如果消息成功，则返回值的  HIWORD  为 TRUE， LOWORD  是命令滚动的行数。
     * 如果滚动移动到文本的开头或末尾，返回的数字可能与滚动的实际行数不同。 如果 command 参数指定的值无效，则返回值为 false。
     * 要滚动到特定行或字符位置，请使用 EM_LINESCROLL 消息。 要将插入点滚动到视图中，请使用 EM_SCROLLCARET 消息。
     * Rich Edit：在 Microsoft Rich Edit 1.0 及更高版本中受支持。 有关 Rich Edit 版本与各种系统版本的兼容性的信息，请参阅关于 Rich Edit 控件。
     * `command` 滚动条要执行的操作。 此参数的取值可为下列值之一。
     * - SB_LINEDOWN 向下滚动一行。
     * - SB_LINEUP 向上滚动一行。
     * - SB_PAGEDOWN 向下滚动一页。
     * - SB_PAGEUP 向上滚动一页。
     */
    fn scroll(&self, command: SCROLLBAR_COMMAND) -> isize {
        let (_, res) = self.send_message(EM_SCROLL, WPARAM(command.0 as usize), LPARAM::default());
        res as isize
    }

    /**
     * 滚动多行编辑控件中的文本。
     * 如果消息发送到多行编辑控件，则返回值为 true。如果消息发送到单行编辑控件，则返回值为 false。
     * 该控件不会垂直滚动到编辑控件中的最后一行文本。 如果当前行加上 line 参数指定的行数超过编辑控件中的总行数，则会调整该值，以便编辑控件的最后一行滚动到编辑控件窗口的顶部。
     * 编辑控件：EM_LINESCROLL消息在多行编辑控件中垂直或水平滚动文本。 EM_LINESCROLL消息可用于水平滚动超过任何行的最后一个字符。
     * 富编辑： Microsoft Rich Edit 1.0 及更高版本中受支持。 EM_LINESCROLL消息在多行编辑控件中垂直滚动文本。 有关丰富编辑版本与各种系统版本的兼容性的信息，请参阅 关于丰富编辑控件。
     * `column` 编辑控件： 要水平滚动的字符数。富编辑控件： 不使用此参数;它必须为零。
     * `line` 要垂直滚动的行数。
     * */
    fn line_scroll(&self, column: i32, line: i32) -> bool {
        let (_, res) = self.send_message(
            EM_LINESCROLL,
            WPARAM(column as usize),
            LPARAM(line as isize),
        );
        res != 0
    }

    /**
     * 将插入点滚动到编辑控件的视图中。 可以将此消息发送到编辑控件或富编辑控件。
     * 富编辑：  Microsoft Rich Edit 1.0 及更高版本中受支持。 有关丰富编辑版本与各种系统版本的兼容性的信息，请参阅  关于丰富编辑控件 。
     * */
    fn scroll_caret(&self) {
        self.send_message(EM_SCROLLCARET, WPARAM::default(), LPARAM::default());
    }

    /**
     * 确定丰富的编辑控件是否可以粘贴。
     * */
    fn can_paste(&self) -> bool {
        let (_, res) = self.send_message(EM_CANPASTE, WPARAM::default(), LPARAM::default());
        res != 0
    }

    /**
     * 获取有关距离编辑控件客户区中指定点最近的字符的信息。 可以将此消息发送到编辑控件或富编辑控件。
     * 富编辑控件：返回值指定了距指定点最近字符的从零开始的字符索引。 如果指定点超出控件中的最后一个字符，则返回值会指示编辑控件中的最后一个字符。
     * 编辑控件：LOWORD 指定距指定点最近字符的从零开始的索引。 此索引相对于控件的开头，而不是行的开头。 如果指定点超出了编辑控件中的最后一个字符，则返回值将指示控件中的最后一个字符。 HIWORD 指定包含字符的行的从零开始的索引。 对于单行编辑控件，此值为零。 如果指定点超出行中的最后一个可见字符，则索引会指示行分隔符。
     * 富编辑：在 Microsoft Rich Edit 1.0 及更高版本中受支持。 有关富编辑版本与各种系统版本兼容性的信息，请参阅关于富编辑控件。
     * 如果某个点位于编辑控件边界之外，则 lResult 为 (65535, 65535)。
     * `x` 某个点的X坐标。
     * `y` 某个点的Y坐标。
     * */
    fn char_from_pos(&self, x: i32, y: i32) -> i32 {
        let point = POINT { x, y };
        let (_, res) = self.send_message(
            EM_CHARFROMPOS,
            WPARAM::default(),
            LPARAM(&point as *const POINT as isize),
        );
        res as i32
    }

    /**
     * 查询编辑控件中指定字符的坐标。
     * 返回值是字符 （x， y） 的位置。对于单行编辑控件，y 坐标始终为零。
     * 最后一个字符之外的任何索引都返回 –1。
     * 如果字符已滚动到编辑控件的工作区之外，则返回的坐标可以为负数。坐标被截断为整数值。
     * 如果  char_index  是行分隔符的索引，则返回的坐标位于行中最后一个可见字符刚好经过的位置。
     * 如果  char_index  大于控件中最后一个字符的索引，则返回的坐标的位置刚好超过控件的最后一个字符。
     * `char_index` 指定字符的从零开始的索引。
     * */
    fn pos_from_char(&self, char_index: i32) -> (i32, i32) {
        let (_, res) = self.send_message(
            EM_POSFROMCHAR,
            WPARAM(char_index as usize),
            LPARAM::default(),
        );
        ((res & 0xffff) as i32, (res >> 16) as i32)
    }

    /**
     * 确定 Rich Edit 控件的选择类型。
     * 如果所选内容为空，则返回值SEL_EMPTY。
     * 如果所选内容不为空，则返回值是一组包含以下一个或多个值的标志。
     * 对于无底丰富编辑控件的父控件 ，此 消息在WM_SIZE处理期间非常有用。
     * - SEL_TEXT 文本。
     * - SEL_OBJECT 至少一个 COM 对象。
     * - SEL_MULTICHAR 文本的多个字符。
     * - SEL_MULTIOBJECT 多个 COM 对象。
     * */
    fn selection_type(&self) -> RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
        let (_, res) = self.send_message(EM_SELECTIONTYPE, WPARAM::default(), LPARAM::default());
        RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(res as u16)
    }

    /**
     * 隐藏或显示富编辑控件中的选定内容。
     * `hide` 是否隐藏。
     * */
    fn hide_selection(&self, hide: bool) {
        let hide = if hide { WPARAM(1) } else { WPARAM(0) };
        self.send_message(EM_HIDESELECTION, hide, LPARAM::default());
    }

    /**
     * 在 Rich Edit 控件中查找文本。
     * 如果找到目标字符串，则返回值为匹配的第一个字符的从零开始的位置。 如果未找到目标，则返回值为 -1。
     * 向后搜索时，  min  必须等于或大于  max 。 向前搜索时，  max  中的值 -1 会将搜索范围扩展到文本的末尾。
     * `flags` 指定搜索操作的参数。 此参数可使用以下一个或多个值。
     * - FR_DOWN Microsoft Rich Edit 2.0 及更高版本：如果设置，则搜索从当前所选内容的末尾到文档末尾。 如果未设置，则搜索将从当前所选内容的末尾到文档的开头。Microsoft Rich Edit 1.0：忽略FR_DOWN标志。 搜索始终从当前所选内容的末尾到文档的末尾。
     * - FR_MATCHALEFHAMZA Microsoft Rich Edit 3.0 及更高版本：如果设置，搜索将区分具有不同口音的阿拉伯语 alef。 如果未设置，则由 alef 字符单独匹配所有 alef。
     * - FR_MATCHDIAC Microsoft Rich Edit 3.0 及更高版本：如果设置，搜索操作将考虑阿拉伯语和希伯来语音调符号。 如果未设置，则忽略音调符号。
     * - FR_MATCHKASHIDA Microsoft Rich Edit 3.0 及更高版本：如果设置，则搜索操作将视为阿拉伯语 kashida。 如果未设置，则忽略 kashidas。
     * - FR_MATCHWIDTH Windows 8：如果设置，则同一字符的单字节和双字节版本被视为不相等。
     * - FR_WHOLEWORD 如果已设置，则操作仅搜索与搜索字符串匹配的整个单词。 如果未设置，该操作还会搜索与搜索字符串匹配的单词片段。
     * `text` 要查找的文本。
     * `min` 最小索引。
     * `max` 最大索引。
     * */
    fn find_text(&self, flags: FINDREPLACE_FLAGS, text: &str, min: i32, max: i32) -> i32 {
        let find = FINDTEXTW {
            chrg: CHARRANGE {
                cpMin: min,
                cpMax: max,
            },
            lpstrText: PCWSTR(HSTRING::from(text).as_ptr()),
        };
        let (_, res) = self.send_message(
            EM_FINDTEXT,
            WPARAM(flags.0 as usize),
            LPARAM(&find as *const FINDTEXTW as isize),
        );
        res as i32
    }

    /**
     * 查找格式编辑控件中的文本。
     * `flags` 指定搜索操作的行为。 此参数可使用以下一个或多个值。
     * - FR_DOWN Microsoft Rich Edit 2.0 及更高版本：如果设置，则从 FINDTEXTEX.chrg.cpMin 向前搜索;如果未设置，则从 FINDTEXTEX.chrg.cpMin 向后搜索。 Microsoft Rich Edit 1.0：忽略FR_DOWN标志。 搜索始终向前。
     * - FR_MATCHALEFHAMZA Microsoft Rich Edit 3.0 及更高版本：如果设置，搜索将区分具有不同重音的阿拉伯语和希伯来语。 如果未设置，则所有 alef 都单独与 alef 字符匹配。
     * - FR_MATCHCASE 如果设置，则搜索操作区分大小写。 如果未设置，则搜索操作不区分大小写。
     * - FR_MATCHDIAC Microsoft Rich Edit 3.0 及更高版本：如果设置，则搜索操作将考虑阿拉伯语和希伯来语音调符号。 如果未设置，则忽略音调符号。
     * - FR_MATCHKASHIDA Microsoft Rich Edit 3.0 及更高版本：如果设置，则搜索操作将考虑阿拉伯语和希伯来语 kashidas。 如果未设置，则忽略 kashidas。
     * - FR_WHOLEWORD 如果已设置，则操作仅搜索与搜索字符串匹配的整个单词。 如果未设置，该操作还会搜索与搜索字符串匹配的单词片段。
     * 如果找到目标字符串，则返回值是匹配项的第一个字符的从零开始的位置。 如果未找到目标，则返回值为 -1。
     * 向后搜索时， min 必须等于或大于 max。 向前搜索时， max 中的值 -1 会将搜索范围扩展到文本的末尾。
     * `text` 要查找的文本。
     * `min` 最小索引。
     * `max` 最大索引。
     * */
    fn find_text_ex(&self, flags: FINDREPLACE_FLAGS, text: &str, min: i32, max: i32) -> (i32, i32) {
        let mut find = FINDTEXTEXW {
            chrg: CHARRANGE {
                cpMin: min,
                cpMax: max,
            },
            lpstrText: PCWSTR(HSTRING::from(text).as_ptr()),
            chrgText: CHARRANGE::default(),
        };
        let (_, _) = self.send_message(
            EM_FINDTEXT,
            WPARAM(flags.0 as usize),
            LPARAM(&mut find as *mut FINDTEXTEXW as isize),
        );
        (find.chrgText.cpMin, find.chrgText.cpMax)
    }

    /**
     * 获取编辑控件的左边距和右边距的宽度。
     * 返回 LOWORD 中左边距的宽度，以及 HIWORD 中右边距的宽度。
     * Rich Edit：  不支持  EM_GETMARGINS  消息。
     * */
    fn get_margins(&self) -> i32 {
        let (_, res) = self.send_message(EM_GETMARGINS, WPARAM::default(), LPARAM::default());
        res as i32
    }

    /**
     * 设置编辑控件的左边距和右边距的宽度。该消息将重新绘制控件以反映新的边距。
     * `margin` 指定要设置的边距。下表显示了可以组合的可能值。
     * - EC_LEFTMARGIN 设置左边距。
     * - EC_RIGHTMARGIN 设置右边距。
     * `value` LOW指定左边距的宽度（以像素为单位）。如果 margin 不包括 EC_LEFTMARGIN，则忽略此值。HIGH 指定右边距的宽度（以像素为单位）。如果 margin 不包括 EC_RIGHTMARGIN，则忽略此值。
     * */
    fn set_margins(&self, margin: u32, value: i32) {
        self.send_message(
            EM_SETMARGINS,
            WPARAM(margin as usize),
            LPARAM(value as isize),
        );
    }
}
