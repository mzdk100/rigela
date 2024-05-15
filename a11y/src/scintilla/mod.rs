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

pub mod status;

use std::{ffi::c_char, os::raw::c_void};

use scintilla_sys::{Sci_CharacterRange, Sci_PositionCR, Sci_TextRange};
pub use scintilla_sys::{
    SCI_ADDSTYLEDTEXT, SCI_ADDTEXT, SCI_ALLOCATE, SCI_ALLOCATEEXTENDEDSTYLES, SCI_APPENDTEXT,
    SCI_CHANGEINSERTION, SCI_CLEARALL, SCI_CLEARDOCUMENTSTYLE, SCI_COUNTCHARACTERS,
    SCI_DELETERANGE, SCI_ENCODEDFROMUTF8, SCI_FINDCOLUMN, SCI_GETCHARAT, SCI_GETCOLUMN,
    SCI_GETLENGTH, SCI_GETLINE, SCI_GETLINECOUNT, SCI_GETLINEENDPOSITION, SCI_GETMODIFY,
    SCI_GETREADONLY, SCI_GETSTATUS, SCI_GETSTYLEAT, SCI_GETSTYLEDTEXT, SCI_GETTEXT,
    SCI_GETTEXTLENGTH, SCI_GETTEXTRANGE, SCI_INSERTTEXT, SCI_LINEFROMPOSITION, SCI_LINELENGTH,
    SCI_LINESONSCREEN, SCI_POINTXFROMPOSITION, SCI_POINTYFROMPOSITION, SCI_POSITIONAFTER,
    SCI_POSITIONBEFORE, SCI_POSITIONFROMPOINT, SCI_POSITIONFROMPOINTCLOSE, SCI_POSITIONRELATIVE,
    SCI_RELEASEALLEXTENDEDSTYLES, SCI_REPLACESEL, SCI_SETLENGTHFORENCODE, SCI_SETREADONLY,
    SCI_SETSAVEPOINT, SCI_SETSTATUS, SCI_SETTEXT, SCI_TARGETASUTF8, SCI_TEXTHEIGHT, SCI_TEXTWIDTH,
};

use crate::scintilla::status::Status;
use win_wrap::{
    common::{LPARAM, WPARAM},
    control::{edit::Edit, WindowControl},
    ext::StringExt,
    memory::InProcessMemory,
};

pub type Cell = u16;

pub trait Scintilla: Edit {
    /**
     * 返回从文档开头算起的指定数量的字符。
     * `length` 字符数，不包括'\0'
     * */
    fn get_text(&self, length: usize) -> Option<String>;

    /**
     * 这将用您传入的文本字符串替换文档中的所有文本。
     * `text` 文本内容。
     * */
    fn set_text(&self, text: String);

    /**
     * 此消息告诉Scintilla文档的当前状态未修改。这通常是在保存或加载文件时完成的，因此得名“保存点”。当Scintilla执行撤消和重做操作时，它会用SCN_SAVEPOINTRACHED和SCN_SAVEPOINTLEFT通知消息通知容器它已经进入或离开了保存点，从而使容器知道文件是否应该被视为脏文件。
     * */
    fn set_save_point(&self);

    /**
     * 获取某行文字长度。
     * `line` 行号。
     * */
    fn line_length(&self, line: usize) -> usize;

    /**
     * 返回某行的文本。
     * `line` 行号。
     * `length` 文本长度。
     * */
    fn get_line(&self, line: usize, length: usize) -> Option<String>;

    /**
     * 定位点和当前位置之间当前选定的文本将被text参数替换。如果定位点和目前位置相同，则文本将插入插入符号位置。插入符号位于插入的文本之后，插入符号滚动到视图中。
     * `text` 要替换的文本。
     */
    fn replace_sel(&self, text: String);

    /**
     * 获取文档的只读标志。如果将文档标记为只读，则尝试修改文本会导致SCN_MODIFYATTEMPTRO通知。
     * */
    fn get_readonly(&self) -> bool;

    /**
     * 设置文档的只读标志。如果将文档标记为只读，则尝试修改文本会导致SCN_MODIFYATTEMPTRO通知。
     * `val` 要设置的状态值。
     * */
    fn set_readonly(&self, val: bool);

    /**
     * 这将收集位置min和max之间的文本。如果max为-1，则文本将返回到文档的末尾。
     * `min` 起始位置。
     * `max` 结束位置。
     * */
    fn get_text_range(&self, min: isize, max: isize) -> Option<String>;

    /**
     * 这将使用每个单元格的两个字节将样式文本收集到缓冲区中，字符位于每对的较低地址，样式字节位于较高地址。返回位置min和max之间的字符。不检查min或max的合理值。
     * `min` 起始位置。
     * `max` 结束位置。
     * */
    fn get_styled_text(&self, min: isize, max: isize) -> Option<Vec<Cell>>;

    /**
     * 分配一个足够大的文档缓冲区，以存储给定数量的字节。该文档不会小于其当前内容。
     * `bytes` 要分配的字节数。
     * */
    fn allocate(&self, bytes: usize);

    /**
     * 这将在当前位置插入字符串文本。当前位置设置在插入文本的末尾，但不会滚动到视图中。
     * `text` 要插入的文字。
     * */
    fn add_text(&self, text: String);

    /**
     * 其行为与SCI_ADDTEXT类似，但插入样式化的文本。
     * */
    fn add_styled_text(&self, text: &[Cell]);

    /**
     * 这会将字符串文本添加到文档的末尾。当前所选内容不会更改，新文本也不会滚动到视图中。
     * `text` 要插入的文字。
     * */
    fn append_text(&self, text: String);

    /**
     * 这将在位置pos或当前位置（如果pos为-1）插入文本字符串。如果当前位置在插入点之后，则它将与其周围的文本一起移动，但不执行滚动。
     * `pos` 要插入的位置。
     * `text` 要插入的文本。
     * */
    fn insert_text(&self, pos: usize, text: String);

    /**
     * 这只能从SC_MOD_INSERTCHECK通知处理程序调用，并将插入的文本更改为所提供的文本。
     * `text` 要更改的文字。
     * */
    fn change_insertion(&self, text: String);

    /**
     * 除非文档是只读的，否则会删除所有文本。
     * */
    fn clear_all(&self);

    /**
     * 删除文档中的一个文本范围。
     * `start` 开始位置。
     * `length` 要删除的字符数量。
     * */
    fn delete_range(&self, start: usize, length: usize);

    /**
     * 当想要完全重新设计文档样式时，例如在选择lexer之后，SCI_CLEARDOCUMENTSTYLE可用于清除所有样式信息并重置折叠状态。
     * */
    fn clear_document_style(&self);

    /**
     * 这将返回文档中pos处的字符，如果pos为负数或超过文档末尾，则返回0。
     * `pos` 要查询的位置。
     * */
    fn get_char_at(&self, pos: usize) -> isize;

    /**
     * 这将返回文档中pos处的样式，如果pos为负数或超过文档末尾，则返回0。SCI_GETSTYLEAT可以为超过127的样式返回负数，而SCI_GETSTYLEINDEXAT将仅返回正数。SCI_GETSTYLEINDEXAT应该是首选，因为它可以更一致地处理样式，并且可以避免定义超过128个样式的lexer出现问题。
     * */
    fn get_style_at(&self, pos: usize) -> isize;

    /**
     * 扩展样式用于文本边距和注释等功能，也用于Scintilla内部。它们不在用于与文档字节相关联的样式字节的0..255范围内。这些功能管理扩展样式的使用，以确保组件在定义样式时进行协作。
     * SCI_RELEASEALLEXTENDEDSTYLES释放容器分配的任何扩展样式。
     * SCI_ALLOCATEEXTENDEDSTYLES在字节样式值之后分配一系列样式编号，并返回第一个分配样式的编号。应在调用SCI_MARGENESTSTYLEOFFSET或SCI_ANNOTATIONSETSTYLEOFFSET之前分配边距和注释样式的范围。
     * */
    fn release_all_extended_styles(&self);

    /**
     * 扩展样式用于文本边距和注释等功能，也用于Scintilla内部。它们不在用于与文档字节相关联的样式字节的0..255范围内。这些功能管理扩展样式的使用，以确保组件在定义样式时进行协作。
     * SCI_RELEASEALLEXTENDEDSTYLES释放容器分配的任何扩展样式。
     * SCI_ALLOCATEEXTENDEDSTYLES在字节样式值之后分配一系列样式编号，并返回第一个分配样式的编号。应在调用SCI_MARGENESTSTYLEOFFSET或SCI_ANNOTATIONSETSTYLEOFFSET之前分配边距和注释样式的范围。
     * `number` 样式数量。
     * */
    fn allocate_extended_styles(&self, number: isize) -> isize;

    /**
     * 此方法检索编码为UTF-8的目标值，UTF-8是GTK的默认编码，因此对于检索在用户界面的其他部分（如查找和替换对话框）中使用的文本非常有用。返回以字节为单位的编码文本的长度。Cocoa使用UTF-16，它很容易从UTF-8转换而来，因此这种方法可以用于执行更复杂的工作，即从支持的各种编码进行代码转换。
     * */
    fn target_as_utf8(&self) -> usize;

    /*
     * SCI_ENCODEDFROMUTF8将UTF-8 字符串转换为文档的编码，例如，这对于获取查找对话框的结果和接收可以在文档中搜索的字节字符串非常有用。
     * `text` 要转换的文字。
     * */
    fn encoded_from_utf8(&self, text: String) -> Vec<u8>;

    /**
     * 以字节为单位返回文档的长度。
     * */
    fn get_text_length(&self) -> usize;

    /**
     * 以字节为单位返回文档的长度。
     * */
    fn get_length(&self) -> usize;

    /**
     * 这将返回文档中的行数。空文档包含 1 行。一个 仅包含行尾序列的文档有 2 行。
     * */
    fn get_line_count(&self) -> usize;

    /**
     * 这将返回屏幕上可见的完整行数。对于恒定的线高度，这是可用的垂直空间除以线间距。除非将窗口的大小调整为整数行，否则视图底部可能会有一条可见的局部线。
     * */
    fn lines_on_screen(&self) -> usize;

    /**
     * 如果文档已修改，则返回true；如果未修改，则为false。文档的修改状态由相对于保存点的撤消位置决定。保存点由SCI_SETSAVEPOINT设置，通常在将数据保存到文件中时设置。
     * 如果您需要在文档被修改时得到通知，Scintilla会用SCN_SAVEPOINTRACHED和SCN_SAVEPOINTLEFT通知消息通知容器它已经进入或离开了保存点。
     * */
    fn get_modify(&self) -> bool;

    /**
     * 此消息返回包含文档中pos位置的行。如果<=0，则返回值为0。如果超出文档末尾，则返回值为最后一行。
     * `pos` 位置。
     * */
    fn line_from_position(&self, pos: usize) -> usize;

    /**
     * 这将返回行尾的位置，在任何行尾字符之前。如果是文档中的最后一行（没有任何行尾字符）或更大，则结果是文档的大小。如果为负数，则结果未定义。
     * `line` 行号。
     * */
    fn get_line_end_position(&self, line: usize) -> usize;

    /**
     * 此消息返回文档中某个位置的列号，并将制表符的宽度考虑在内。这将返回前一行最后一个制表符的列号，加上最后一个制表符和pos之间的字符数。如果行上没有制表符，则返回值为直到行上pos的位置为止的字符数。在这两种情况下，双字节字符都算作单个字符。这可能只适用于单空间字体。
     * `pos` 行号。
     * */
    fn get_column(&self, pos: usize) -> usize;

    /**
     * 此消息返回一行中列的位置，同时考虑制表符的宽度。它将多字节字符视为单列。列编号，如从0开始的行。
     * `line` 行号。
     * `column` 列号。
     */
    fn find_column(&self, line: usize, column: usize) -> usize;

    /**
     * 考虑到当前代码页，返回文档中另一个位置之前的位置。返回的最小位置为0，最大位置为文档中的最后一个位置。如果用多字节字符内的位置调用，则返回该字符的起始位置。
     * `pos` 位置。
     * */
    fn position_before(&self, pos: usize) -> usize;

    /**
     * 考虑到当前代码页，返回文档中另一个位置之后的位置。返回的最小位置为0，最大位置为文档中的最后一个位置。如果用多字节字符内的位置调用，则返回该字符的结束位置。
     * `pos` 位置。
     * */
    fn position_after(&self, pos: usize) -> usize;

    /**
     * 这将返回在给定style中绘制的字符串的像素宽度，例如，可以使用该宽度来决定行号边距的宽度，以便显示给定数量的数字。
     * `style` 样式。
     * `text` 文字。
     * */
    fn text_width(&self, style: i32, text: String) -> i32;

    /**
     * 这将返回特定行的高度（以像素为单位）。目前所有线条的高度相同。
     * `line` 行号。
     * */
    fn text_height(&self, line: usize) -> i32;

    /**
     * 查找与某个点最接近的字符位置。
     * `x` X坐标。
     * `y` Y坐标。
     * */
    fn position_from_point(&self, x: i32, y: i32) -> usize;

    /**
     * 查找与某个点最接近的字符位置，如果该点在窗口之外或不靠近任何字符，则返回-1。
     * `x` X坐标。
     * `y` Y坐标。
     * */
    fn position_from_point_close(&self, x: i32, y: i32) -> isize;

    /**
     * 返回文档中文本pos位置处的像素点的X值。
     * `pos` 位置。
     * */
    fn point_x_from_position(&self, pos: usize) -> i32;

    /**
     * 返回文档中文本pos位置处的像素点的Y值。
     * `pos` 位置。
     * */
    fn point_y_from_position(&self, pos: usize) -> i32;

    //noinspection StructuralWrap
    /**
     * 计算参数位置之前或之后的完整字符数，然后返回该位置（按字符）。返回的最小位置为0，最大位置为文档中的最后一个位置。如果位置超过文档末尾，则返回0。
     * `pos` 起始位置。
     * `relative` 相对位置。
     * */
    fn position_relative(&self, pos: usize, relative: usize) -> usize;

    //noinspection StructuralWrap
    /**
     * 返回两个位置之间的完整字符数（按字符）。
     * `start` 开始位置。
     * `end` 结束位置。
     * */
    fn count_characters(&self, start: usize, end: usize) -> usize;

    /**
     * 设置状态。
     * `status` 状态值。
     * */
    fn set_status(&self, status: Status);

    /**
     * 获取状态。
     * */
    fn get_status(&self) -> Status;
}

impl Scintilla for WindowControl {
    fn get_text(&self, length: usize) -> Option<String> {
        let mem = InProcessMemory::new(self.get_pid(), length).unwrap();
        self.send_message(SCI_GETTEXT, WPARAM(length), LPARAM(mem.as_ptr() as isize));
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn set_text(&self, text: String) {
        let size = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_pid(), size).unwrap();
        mem.write(text.as_ptr() as *const c_void, size);
        self.send_message(SCI_SETTEXT, WPARAM(size), LPARAM(mem.as_ptr() as isize));
    }

    fn set_save_point(&self) {
        self.send_message(SCI_SETSAVEPOINT, WPARAM::default(), LPARAM::default());
    }

    fn line_length(&self, line: usize) -> usize {
        let (_, res) = self.send_message(SCI_LINELENGTH, WPARAM(line), LPARAM::default());
        res
    }

    fn get_line(&self, line: usize, length: usize) -> Option<String> {
        let mem = InProcessMemory::new(self.get_pid(), length).unwrap();
        self.send_message(SCI_GETLINE, WPARAM(line), LPARAM(mem.as_ptr() as isize));
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn replace_sel(&self, text: String) {
        let size = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_pid(), size).unwrap();
        mem.write(text.as_ptr() as *const c_void, size);
        self.send_message(SCI_REPLACESEL, WPARAM(size), LPARAM(mem.as_ptr() as isize));
    }

    fn get_readonly(&self) -> bool {
        let (_, res) = self.send_message(SCI_GETREADONLY, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn set_readonly(&self, val: bool) {
        let val = if val { 1 } else { 0 };
        self.send_message(SCI_SETREADONLY, WPARAM(val), LPARAM::default());
    }

    fn get_text_range(&self, min: isize, max: isize) -> Option<String> {
        let mem = InProcessMemory::new(self.get_pid(), ((max - min).abs() + 1) as usize).unwrap();
        let param = Sci_TextRange {
            chrg: Sci_CharacterRange {
                cpMax: max as Sci_PositionCR,
                cpMin: min as Sci_PositionCR,
            },
            lpstrText: mem.as_ptr_mut() as *mut c_char,
        };
        let size = std::mem::size_of::<Sci_TextRange>();
        let mem2 = InProcessMemory::new(self.get_pid(), size).unwrap();
        mem2.write(&param as *const Sci_TextRange as *const c_void, size);
        self.send_message(
            SCI_GETTEXTRANGE,
            WPARAM::default(),
            LPARAM(mem2.as_ptr() as isize),
        );
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn get_styled_text(&self, min: isize, max: isize) -> Option<Vec<Cell>> {
        let mem =
            InProcessMemory::new(self.get_pid(), (2 * (max - min).abs() + 2) as usize).unwrap();
        let param = Sci_TextRange {
            chrg: Sci_CharacterRange {
                cpMax: max as Sci_PositionCR,
                cpMin: min as Sci_PositionCR,
            },
            lpstrText: mem.as_ptr_mut() as *mut c_char,
        };
        let size = std::mem::size_of::<Sci_TextRange>();
        let mem2 = InProcessMemory::new(self.get_pid(), size).unwrap();
        mem2.write(&param as *const Sci_TextRange as *const c_void, size);
        let (_, len) = self.send_message(
            SCI_GETSTYLEDTEXT,
            WPARAM::default(),
            LPARAM(mem2.as_ptr() as isize),
        );
        mem.read(|buf| {
            let mut p = buf as *const Cell;
            let mut v = Vec::with_capacity(len);
            for _ in 0..len {
                unsafe {
                    v.push(p.read());
                    p = p.add(1);
                }
            }
            v
        })
    }

    fn allocate(&self, bytes: usize) {
        self.send_message(SCI_ALLOCATE, WPARAM(bytes), LPARAM::default());
    }

    fn add_text(&self, text: String) {
        let size = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_pid(), size).unwrap();
        mem.write(text.as_ptr() as *const c_void, size);
        self.send_message(SCI_ADDTEXT, WPARAM(size), LPARAM(mem.as_ptr() as isize));
    }

    fn add_styled_text(&self, text: &[Cell]) {
        let size = text.len() * std::mem::size_of::<Cell>();
        let mem = InProcessMemory::new(self.get_pid(), size).unwrap();
        mem.write(text.as_ptr() as *const c_void, size);
        self.send_message(
            SCI_ADDSTYLEDTEXT,
            WPARAM(size),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn append_text(&self, text: String) {
        let size = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_pid(), size).unwrap();
        mem.write(text.as_ptr() as *const c_void, size);
        self.send_message(SCI_APPENDTEXT, WPARAM(size), LPARAM(mem.as_ptr() as isize));
    }

    fn insert_text(&self, pos: usize, text: String) {
        let size = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_pid(), size).unwrap();
        mem.write(text.as_ptr() as *const c_void, size);
        self.send_message(SCI_INSERTTEXT, WPARAM(pos), LPARAM(mem.as_ptr() as isize));
    }

    fn change_insertion(&self, text: String) {
        let size = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_pid(), size).unwrap();
        mem.write(text.as_ptr() as *const c_void, size);
        self.send_message(
            SCI_CHANGEINSERTION,
            WPARAM(size),
            LPARAM(mem.as_ptr() as isize),
        );
    }

    fn clear_all(&self) {
        self.send_message(SCI_CLEARALL, WPARAM::default(), LPARAM::default());
    }

    fn delete_range(&self, start: usize, length: usize) {
        self.send_message(SCI_DELETERANGE, WPARAM(start), LPARAM(length as isize));
    }

    fn clear_document_style(&self) {
        self.send_message(SCI_CLEARDOCUMENTSTYLE, WPARAM::default(), LPARAM::default());
    }

    fn get_char_at(&self, pos: usize) -> isize {
        let (_, res) = self.send_message(SCI_GETCHARAT, WPARAM(pos), LPARAM::default());
        res as isize
    }

    fn get_style_at(&self, pos: usize) -> isize {
        let (_, res) = self.send_message(SCI_GETSTYLEAT, WPARAM(pos), LPARAM::default());
        res as isize
    }

    fn release_all_extended_styles(&self) {
        self.send_message(
            SCI_RELEASEALLEXTENDEDSTYLES,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn allocate_extended_styles(&self, number: isize) -> isize {
        let (_, res) = self.send_message(
            SCI_ALLOCATEEXTENDEDSTYLES,
            WPARAM(number as usize),
            LPARAM::default(),
        );
        res as isize
    }

    fn target_as_utf8(&self) -> usize {
        let (_, res) = self.send_message(SCI_TARGETASUTF8, WPARAM::default(), LPARAM::default());
        res
    }

    fn encoded_from_utf8(&self, text: String) -> Vec<u8> {
        let size = text.as_bytes().len();
        self.send_message(SCI_SETLENGTHFORENCODE, WPARAM(size), LPARAM::default());
        let mem = InProcessMemory::new(self.get_pid(), size + 1).unwrap();
        mem.write(text.as_ptr() as *const c_void, size);
        let (_, len) = self.send_message(
            SCI_ENCODEDFROMUTF8,
            WPARAM(mem.as_ptr() as usize),
            LPARAM::default(),
        );
        let mem2 = InProcessMemory::new(self.get_pid(), len + 1).unwrap();
        self.send_message(
            SCI_ENCODEDFROMUTF8,
            WPARAM(mem.as_ptr() as usize),
            LPARAM(mem2.as_ptr() as isize),
        );
        mem2.read(|buf| {
            let mut p = buf as *const u8;
            let mut v = Vec::with_capacity(len);
            for _ in 0..len {
                unsafe {
                    v.push(p.read());
                    p = p.add(1);
                }
            }
            v
        })
            .unwrap_or(Vec::new())
    }

    fn get_text_length(&self) -> usize {
        let (_, res) = self.send_message(SCI_GETTEXTLENGTH, WPARAM::default(), LPARAM::default());
        res
    }

    fn get_length(&self) -> usize {
        let (_, res) = self.send_message(SCI_GETLENGTH, WPARAM::default(), LPARAM::default());
        res
    }

    fn get_line_count(&self) -> usize {
        let (_, res) = self.send_message(SCI_GETLINECOUNT, WPARAM::default(), LPARAM::default());
        res
    }

    fn lines_on_screen(&self) -> usize {
        let (_, res) = self.send_message(SCI_LINESONSCREEN, WPARAM::default(), LPARAM::default());
        res
    }

    fn get_modify(&self) -> bool {
        let (_, res) = self.send_message(SCI_GETMODIFY, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn line_from_position(&self, pos: usize) -> usize {
        let (_, res) = self.send_message(SCI_LINEFROMPOSITION, WPARAM(pos), LPARAM::default());
        res
    }

    fn get_line_end_position(&self, line: usize) -> usize {
        let (_, res) = self.send_message(SCI_GETLINEENDPOSITION, WPARAM(line), LPARAM::default());
        res
    }

    fn get_column(&self, pos: usize) -> usize {
        let (_, res) = self.send_message(SCI_GETCOLUMN, WPARAM(pos), LPARAM::default());
        res
    }

    fn find_column(&self, line: usize, column: usize) -> usize {
        let (_, res) = self.send_message(SCI_FINDCOLUMN, WPARAM(line), LPARAM(column as isize));
        res
    }

    fn position_before(&self, pos: usize) -> usize {
        let (_, res) = self.send_message(SCI_POSITIONBEFORE, WPARAM(pos), LPARAM::default());
        res
    }

    fn position_after(&self, pos: usize) -> usize {
        let (_, res) = self.send_message(SCI_POSITIONAFTER, WPARAM(pos), LPARAM::default());
        res
    }

    fn text_width(&self, style: i32, text: String) -> i32 {
        let size = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_pid(), size).unwrap();
        mem.write(text.as_ptr() as *const c_void, size);
        let (_, res) = self.send_message(
            SCI_TEXTWIDTH,
            WPARAM(style as usize),
            LPARAM(mem.as_ptr() as isize),
        );
        res as i32
    }

    fn text_height(&self, line: usize) -> i32 {
        let (_, res) = self.send_message(SCI_TEXTHEIGHT, WPARAM(line), LPARAM::default());
        res as i32
    }

    fn position_from_point(&self, x: i32, y: i32) -> usize {
        let (_, res) = self.send_message(
            SCI_POSITIONFROMPOINT,
            WPARAM(x as usize),
            LPARAM(y as isize),
        );
        res
    }

    fn position_from_point_close(&self, x: i32, y: i32) -> isize {
        let (_, res) = self.send_message(
            SCI_POSITIONFROMPOINTCLOSE,
            WPARAM(x as usize),
            LPARAM(y as isize),
        );
        res as isize
    }

    fn point_x_from_position(&self, pos: usize) -> i32 {
        let (_, res) = self.send_message(
            SCI_POINTXFROMPOSITION,
            WPARAM::default(),
            LPARAM(pos as isize),
        );
        res as i32
    }

    fn point_y_from_position(&self, pos: usize) -> i32 {
        let (_, res) = self.send_message(
            SCI_POINTYFROMPOSITION,
            WPARAM::default(),
            LPARAM(pos as isize),
        );
        res as i32
    }

    fn position_relative(&self, pos: usize, relative: usize) -> usize {
        let (_, res) =
            self.send_message(SCI_POSITIONRELATIVE, WPARAM(pos), LPARAM(relative as isize));
        res
    }

    fn count_characters(&self, start: usize, end: usize) -> usize {
        let (_, res) = self.send_message(SCI_COUNTCHARACTERS, WPARAM(start), LPARAM(end as isize));
        res
    }

    fn set_status(&self, status: Status) {
        self.send_message(
            SCI_SETSTATUS,
            WPARAM(<Status as Into<u32>>::into(status) as usize),
            LPARAM::default(),
        );
    }

    fn get_status(&self) -> Status {
        let (_, res) = self.send_message(SCI_GETSTATUS, WPARAM::default(), LPARAM::default());
        (res as u32).into()
    }
}

#[cfg(test)]
mod test_scintilla {
    use win_wrap::{
        common::{find_window_ex, HWND},
        control::WindowControl,
    };

    use crate::scintilla::status::Status;
    use crate::scintilla::Scintilla;

    #[test]
    fn main() {
        let h_wnd = find_window_ex(HWND::default(), HWND::default(), Some("Notepad++"), None);
        let h_wnd = find_window_ex(h_wnd, HWND::default(), Some("Scintilla"), None);

        let control = WindowControl::from(h_wnd);
        //Scintilla::set_text(&control, String::from("你好"));
        control.set_save_point();
        let line_length = control.line_length(0);
        dbg!(line_length);
        dbg!(control.get_line(0, line_length));
        control.replace_sel(String::from("hello"));
        dbg!(Scintilla::get_text(&control, 5));
        // control.set_readonly(true);
        // dbg!(control.get_readonly());
        // dbg!(control.get_text_range(1, 2));
        // dbg!(control.get_styled_text(1, 2));
        control.allocate(200);
        control.add_styled_text(&[65, 66, 67] /*ABC*/);
        control.append_text(String::from("abc"));
        // control.insert_text(4, String::from("123"));
        control.change_insertion(String::from("456"));
        // control.clear_all();
        // control.delete_range(2, 5);
        control.clear_document_style();
        dbg!(control.get_char_at(4));
        dbg!(control.get_style_at(4));
        control.release_all_extended_styles();
        dbg!(control.allocate_extended_styles(2));
        dbg!(control.target_as_utf8());
        dbg!(control.encoded_from_utf8(String::from("你好")));
        assert_eq!(control.get_length(), control.get_text_length());
        dbg!(control.get_line_count());
        dbg!(control.lines_on_screen());
        dbg!(control.get_modify());
        dbg!(control.line_from_position(3));
        dbg!(control.get_line_end_position(1));
        dbg!(control.get_column(7));
        dbg!(control.find_column(1, 10));
        dbg!(control.position_before(0));
        dbg!(control.position_after(0));
        dbg!(control.text_width(0, String::from("abc")));
        dbg!(control.text_height(1));
        dbg!(control.position_from_point(0, 400));
        dbg!(control.position_from_point_close(0, 400));
        dbg!(control.point_x_from_position(0));
        dbg!(control.point_y_from_position(0));
        dbg!(control.position_relative(0, 1));
        dbg!(control.count_characters(0, 4));
        control.set_status(Status::BadAlloc);
        assert_eq!(Status::BadAlloc, control.get_status());
        dbg!(control);
    }
}
