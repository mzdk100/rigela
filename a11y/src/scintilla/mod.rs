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

pub mod selection;
pub mod status;

use std::{ffi::c_char, os::raw::c_void};

use scintilla_sys::{
    Sci_CharacterRange, Sci_PositionCR, Sci_TextRange, Sci_TextToFind, SCI_GETSELECTIONNANCHOR,
    SCI_GETSELECTIONNANCHORVIRTUALSPACE, SCI_GETSELECTIONSTART, SCI_SETSELECTIONNANCHOR,
    SCI_SETSELECTIONNANCHORVIRTUALSPACE, SCI_SETTARGETRANGE, SCI_TARGETFROMSELECTION,
};
pub use scintilla_sys::{
    SCFIND_CXX11REGEX, SCFIND_MATCHCASE, SCFIND_POSIX, SCFIND_REGEXP, SCFIND_WHOLEWORD,
    SCFIND_WORDSTART, SCI_ADDSELECTION, SCI_ADDSTYLEDTEXT, SCI_ADDTEXT, SCI_ALLOCATE,
    SCI_ALLOCATEEXTENDEDSTYLES, SCI_APPENDTEXT, SCI_CHANGEINSERTION, SCI_CHOOSECARETX,
    SCI_CLEARALL, SCI_CLEARDOCUMENTSTYLE, SCI_CLEARSELECTIONS, SCI_COUNTCHARACTERS,
    SCI_DELETERANGE, SCI_DROPSELECTIONN, SCI_ENCODEDFROMUTF8, SCI_FINDCOLUMN, SCI_FINDTEXT,
    SCI_GETADDITIONALCARETFORE, SCI_GETADDITIONALCARETSBLINK, SCI_GETADDITIONALCARETSVISIBLE,
    SCI_GETADDITIONALSELALPHA, SCI_GETADDITIONALSELECTIONTYPING, SCI_GETANCHOR, SCI_GETCHARAT,
    SCI_GETCOLUMN, SCI_GETCURLINE, SCI_GETCURRENTPOS, SCI_GETLENGTH, SCI_GETLINE, SCI_GETLINECOUNT,
    SCI_GETLINEENDPOSITION, SCI_GETLINESELENDPOSITION, SCI_GETLINESELSTARTPOSITION,
    SCI_GETMAINSELECTION, SCI_GETMODIFY, SCI_GETMOUSESELECTIONRECTANGULARSWITCH,
    SCI_GETMOVEEXTENDSSELECTION, SCI_GETMULTIPASTE, SCI_GETMULTIPLESELECTION, SCI_GETOVERTYPE,
    SCI_GETREADONLY, SCI_GETRECTANGULARSELECTIONANCHOR,
    SCI_GETRECTANGULARSELECTIONANCHORVIRTUALSPACE, SCI_GETRECTANGULARSELECTIONCARET,
    SCI_GETRECTANGULARSELECTIONCARETVIRTUALSPACE, SCI_GETRECTANGULARSELECTIONMODIFIER,
    SCI_GETSEARCHFLAGS, SCI_GETSELECTIONEMPTY, SCI_GETSELECTIONEND, SCI_GETSELECTIONMODE,
    SCI_GETSELECTIONNCARET, SCI_GETSELECTIONNCARETVIRTUALSPACE, SCI_GETSELECTIONNEND,
    SCI_GETSELECTIONNSTART, SCI_GETSELECTIONS, SCI_GETSELTEXT, SCI_GETSTATUS, SCI_GETSTYLEAT,
    SCI_GETSTYLEDTEXT, SCI_GETTAG, SCI_GETTARGETEND, SCI_GETTARGETSTART, SCI_GETTARGETTEXT,
    SCI_GETTEXT, SCI_GETTEXTLENGTH, SCI_GETTEXTRANGE, SCI_GETVIRTUALSPACEOPTIONS, SCI_GOTOLINE,
    SCI_GOTOPOS, SCI_HIDESELECTION, SCI_INSERTTEXT, SCI_LINEFROMPOSITION, SCI_LINELENGTH,
    SCI_LINESONSCREEN, SCI_MOVECARETINSIDEVIEW, SCI_MOVESELECTEDLINESDOWN, SCI_MOVESELECTEDLINESUP,
    SCI_MULTIPLESELECTADDEACH, SCI_MULTIPLESELECTADDNEXT, SCI_POINTXFROMPOSITION,
    SCI_POINTYFROMPOSITION, SCI_POSITIONAFTER, SCI_POSITIONBEFORE, SCI_POSITIONFROMPOINT,
    SCI_POSITIONFROMPOINTCLOSE, SCI_POSITIONRELATIVE, SCI_RELEASEALLEXTENDEDSTYLES, SCI_REPLACESEL,
    SCI_REPLACETARGET, SCI_REPLACETARGETRE, SCI_ROTATESELECTION, SCI_SEARCHANCHOR,
    SCI_SEARCHINTARGET, SCI_SEARCHNEXT, SCI_SEARCHPREV, SCI_SELECTALL, SCI_SELECTIONISRECTANGLE,
    SCI_SETADDITIONALCARETFORE, SCI_SETADDITIONALCARETSBLINK, SCI_SETADDITIONALCARETSVISIBLE,
    SCI_SETADDITIONALSELALPHA, SCI_SETADDITIONALSELBACK, SCI_SETADDITIONALSELECTIONTYPING,
    SCI_SETADDITIONALSELFORE, SCI_SETANCHOR, SCI_SETCURRENTPOS, SCI_SETEMPTYSELECTION,
    SCI_SETLENGTHFORENCODE, SCI_SETMAINSELECTION, SCI_SETMOUSESELECTIONRECTANGULARSWITCH,
    SCI_SETMULTIPASTE, SCI_SETMULTIPLESELECTION, SCI_SETOVERTYPE, SCI_SETREADONLY,
    SCI_SETRECTANGULARSELECTIONANCHOR, SCI_SETRECTANGULARSELECTIONANCHORVIRTUALSPACE,
    SCI_SETRECTANGULARSELECTIONCARET, SCI_SETRECTANGULARSELECTIONCARETVIRTUALSPACE,
    SCI_SETRECTANGULARSELECTIONMODIFIER, SCI_SETSAVEPOINT, SCI_SETSEARCHFLAGS, SCI_SETSEL,
    SCI_SETSELECTION, SCI_SETSELECTIONEND, SCI_SETSELECTIONMODE, SCI_SETSELECTIONNCARET,
    SCI_SETSELECTIONNCARETVIRTUALSPACE, SCI_SETSELECTIONNEND, SCI_SETSELECTIONNSTART,
    SCI_SETSELECTIONSTART, SCI_SETSTATUS, SCI_SETTARGETEND, SCI_SETTARGETSTART, SCI_SETTEXT,
    SCI_SETVIRTUALSPACEOPTIONS, SCI_SWAPMAINANCHORCARET, SCI_TARGETASUTF8, SCI_TARGETWHOLEDOCUMENT,
    SCI_TEXTHEIGHT, SCI_TEXTWIDTH, SCMOD_ALT, SCMOD_CTRL, SCMOD_META, SCMOD_NORM, SCMOD_SHIFT,
    SCVS_NONE, SCVS_NOWRAPLINESTART, SCVS_RECTANGULARSELECTION, SCVS_USERACCESSIBLE, SCI_CUT, SCI_COPY, SCI_PASTE, SCI_CLEAR, SCI_CANPASTE, SCI_COPYALLOWLINE, SCI_COPYRANGE, SCI_COPYTEXT, SCI_SETPASTECONVERTENDINGS, SCI_GETPASTECONVERTENDINGS,
};

use crate::scintilla::{selection::SelectionMode, status::Status};
use win_wrap::{
    common::{LPARAM, WPARAM},
    control::{edit::Edit, WindowControl},
    ext::StringExt,
    memory::InProcessMemory,
};

pub type Cell = u16;

/**
 * 一些搜索例程使用标志选项，其中包括一个简单的正则表达式搜索。通过添加标志选项来组合它们：
 * 搜索标志
 * SCFIND_NONE | 默认设置为不区分大小写的文字匹配。
 * SCFIND_MATCHCASE | 只有与搜索字符串大小写匹配的文本才会匹配。
 * SCFIND_WHOLEWORD | 只有当前面和后面的字符不是SCI_SETWORDCHARS定义的单词字符时，才会发生匹配。
 * SCFIND_WORDSTART | 只有当前面的字符不是SCI_SETWORDCHARS定义的单词字符时，才会发生匹配。
 * SCFIND_REGEXP | 搜索字符串应被解释为正则表达式。除非与SCFIND_CXC11REGEX结合使用，否则使用Scintilla的基本实现。
 * SCFIND_POSIX | 通过为标记节而不是\(和\)解释bare(和)，以更兼容POSIX的方式处理正则表达式。设置SCFIND_CXC11REGEX时无效。
 * SCFIND_CXC11REGEX | 此标志可以设置为使用C++11＜regex＞，而不是Scintilla的基本正则表达式。如果正则表达式无效，则返回-1，状态设置为SC_STATUS_WARN_REGEX。ECMAScript标志是在regex对象上设置的，UTF-8文档将表现出符合Unicode的行为。对于MSVC，其中wchar_t是16位，正则表达式“..”将匹配单个星体平面字符。编译器之间可能还有其他差异。还必须设置SCFIND_REGEXP。
 *
 * 在正则表达式中，使用Scintilla的基本实现，解释的特殊字符为：
 * 正则表达式 | 概要
 * . | 匹配任何字符
 * \( | 这标志着用于标记匹配项的区域的开始。
 * \) | 这标志着标记区域的结束。
 * \n | 其中n是1到9指代替换时的第一到第九标记区域。例如，如果搜索字符串是Fred\（[1-9]\）XXX，而替换字符串是Sam\1YYY，当应用于Fred2XXX时，这将生成Sam2YYY\0表示所有匹配的文本。
 * \< | 这与使用Scintilla的单词定义的单词开头相匹配。
 * \> | 这与使用Scintilla的单词定义的单词结尾相匹配。
 * \x | 这允许您使用具有特殊含义的字符x。例如，\[将被解释为[，而不是字符集的开头。
 * [...] | 这表示一组字符，例如，[abc]表示字符a、b或c中的任何一个。您也可以使用范围，例如[a-z]表示任何小写字符。
 * [^...] | 集合中字符的补码。例如，[^A-Za-z]表示除字母字符以外的任何字符。
 * ^ | 这与一行的开头匹配（除非在集合中使用，请参见上文）。
 * $ | 这与一行的末尾相匹配。
 * * | 这匹配了0次或更多次。例如，Sa*m匹配Sm、Sam、Saam、Saam等等。
 * + | 这匹配1次或多次。例如，Sa+m匹配Sam、Saam、Saam等等。
 * 正则表达式将只匹配单行内的范围，而不会匹配多行。当使用SCFIND_CXX11REGEX时，可以使用更多的功能，通常类似于JavaScript中的正则表达式支持。有关支持内容的详细信息，请参阅C++运行时的文档。
 * */
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

    //noinspection StructuralWrap
    /**
     * 此消息设置锚点和当前位置。如果插入符号为负数，则表示文档的末尾。如果锚点为负数，则表示删除任何选择（即将锚点设置为与插入符号相同的位置）。执行此操作后，插入符号将滚动到视图中。
     * `anchor` 锚点。
     * `caret` 插入点。
     * */
    fn set_sel(&self, anchor: usize, caret: usize);

    /**
     * 这将删除任何选择，将caret设置为插入符号，并在必要时滚动视图以使插入符号可见。它相当于SCI_SETSEL(caret, caret)。锚点位置设置为与当前位置相同。
     * `caret` 插入点。
     * */
    fn goto_pos(&self, caret: usize);

    //noinspection StructuralWrap
    /**
     * 这将删除任何选择，并在行号line的开头设置插入符号，并滚动视图（如果需要）使其可见。锚点位置设置为与当前位置相同。如果line在文档中的行之外（第一行为0），则line集为第一行或最后一行。
     * `line` 行号。
     * */
    fn goto_line(&self, line: usize);

    //noinspection StructuralWrap
    /**
     * 这将设置当前位置，并在锚点和当前位置之间创建一个选择。插入符号未滚动到视图中。
     * 另请参阅：SCI_SCROLLCARET
     * `caret` 插入点。
     * */
    fn set_current_pos(&self, caret: usize);

    /**
     * 这将返回当前位置。
     * */
    fn get_current_pos(&self) -> usize;

    //noinspection StructuralWrap
    /**
     * 这将设置锚点位置，并在锚点位置和当前位置之间创建一个选择。插入符号未滚动到视图中。
     * 另请参阅：SCI_SCROLLCARET
     * `anchor` 锚点。
     * */
    fn set_anchor(&self, anchor: usize);

    /**
     * 这将返回当前的锚点位置。
     * */
    fn get_anchor(&self) -> usize;

    /**
     * 基于锚位置小于当前位置的假设来设置选择。它们不会使插入符号可见。该表显示了锚点的位置和使用这些消息后的当前位置。
     * 设置选择插入符号定位
     * 新值 | 锚点 | 插入点
     * SCI_SETSELECTIONSTART | anchor | Max(anchor, current)
     * SCI_SETSELECTIONEND | Min(anchor, caret) | caret
     * 另请参阅：SCI_SCROLLCARET
     * */
    fn set_selection_start(&self, anchor: usize);

    /**
     * 基于锚位置小于当前位置的假设来设置选择。它们不会使插入符号可见。该表显示了锚点的位置和使用这些消息后的当前位置。
     * 设置选择插入符号定位
     * 新值 | 锚点 | 插入点
     * SCI_SETSELECTIONSTART | anchor | Max(anchor, current)
     * SCI_SETSELECTIONEND | Min(anchor, caret) | caret
     * 另请参阅：SCI_SCROLLCARET
     * */
    fn set_selection_end(&self, caret: usize);

    /**
     * 返回选择的开始，而不考虑当前位置和锚点。返回当前位置或锚点位置中较小的一个。
     * */
    fn get_selection_start(&self) -> usize;

    /**
     * 返回选择的结束，而不考虑当前位置和锚点。返回当前位置或锚点中较大的一个。
     * */
    fn get_selection_end(&self) -> usize;

    /**
     * 这将删除任何选择并将插入点设置为caret。插入点未滚动到视图中。
     * `caret` 插入点。
     * */
    fn set_empty_selection(&self, caret: usize);

    /**
     * 这将选择文档中的所有文本。当前位置不会滚动到视图中。
     * */
    fn select_all(&self);

    /**
     * 正常状态是通过按照SCI_SETSELFORE、SCI_SETSELLACK和相关调用的设置绘制选择，使其可见。但是，如果隐藏所选内容，则会将其绘制为普通文本。
     * `hide` 是否隐藏。
     * */
    fn hide_selection(&self, hide: bool);

    /**
     * 获取当前选定的文本。这允许矩形和不连续的选择以及简单的选择。有关如何复制多重选择和矩形选择以及虚拟空间的信息，请参见多重选择。
     * 另请参见：SCI_GETCURLINE、SCI_GETLINE、SCI_GETTEXT、SCI_GetSTYLETEXT、SCI_GETTEXTRANGE
     * */
    fn get_sel_text(&self) -> Option<String>;

    /**
     * 查询包含插入符号的某行的文本，并返回插入符号所在行中的位置。
     * 另请参见：SCI_GETSELTEXT、SCI_GETLINE、SCI_GETTEXT、SCI_GETSTYLEDTEXT和SCI_GETTEXTRANGE
     * */
    fn get_cur_line(&self) -> (Option<String>, usize);

    /**
     * 如果当前选择处于矩形模式，则返回true，否则返回false。
     * */
    fn selection_is_rectangle(&self) -> bool;

    /**
     * 设置选择模式）。当SCI_SETSELECTIONMODE设置这些模式时，常规插入符号移动将扩展或减少选择，直到具有相同值的调用、SCI_CANCEL或SCI_SETMOVEXTENDSSELECTION取消该模式。SCI_CHANGESELECTIONMODE设置模式，但不使常规插入符号移动扩展或减少选择。
     * */
    fn set_selection_mode(&self, selection_mode: SelectionMode);

    /**
     * 返回当前模式，即使选择是通过鼠标或常规扩展移动进行的。SC_SEL_THIN是在键入矩形选择后的模式，可确保不选择任何字符。
     * */
    fn get_selection_mode(&self) -> SelectionMode;

    /**
     * 这控制了常规插入符号移动是否扩展所选内容而不改变定位点。如果常规插入符号移动将扩展或减少所选内容，则为true，否则为false。SCI_SETSELECTIONMODE可在打开和关闭之间切换此设置。
     * */
    fn get_move_extends_selection(&self) -> bool;

    /**
     * 查询给定行的选择开始的位置，如果该行上没有选择，则返回INVALID_POSITION。
     * `line` 行号。
     * */
    fn get_line_sel_start_position(&self, line: usize) -> usize;

    /**
     * 查询给定行的选择结束的位置，如果该行上没有选择，则返回INVALID_POSITION。
     * `line` 行号。
     * */
    fn get_line_sel_end_position(&self, line: usize) -> usize;

    /**
     * 如果插入点偏离视图的顶部或底部，则会将其移动到其当前位置可见的最近一行。任何选择都将丢失。
     * */
    fn move_caret_inside_view(&self);

    /**
     * Scintilla会记住用户明确水平移动到的最后一个位置的x值，然后在垂直移动时使用该值，例如使用上下键。此消息将插入符号的当前x位置设置为记忆值。
     * */
    fn choose_caret_x(&self);

    /**
     * 将选定的行向上移动一行，选择后将行移到上面。所选内容将自动扩展到所选内容的第一行的开头和最后一行的末尾。如果未选择任何内容，则将选择光标当前所在的行。
     * */
    fn move_selected_lines_up(&self);

    /**
     * 将选定的行向下移动一行，将该行移动到选定之前的下方。所选内容将自动扩展到所选内容的第一行的开头和最后一行的末尾。如果未选择任何内容，则将选择光标当前所在的行。
     * */
    fn move_selected_lines_down(&self);

    //noinspection StructuralWrap
    /**
     * 启用或禁用在使用鼠标进行选择时切换到矩形选择模式的功能。启用此选项后，可以通过按下相应的修改器键将流模式下的鼠标选择切换到矩形模式。然后，即使再次释放修改器键，它们也会坚持矩形模式。禁用此选项后，鼠标选择将始终保持在启动选择时的模式。默认情况下，此选项处于禁用状态。
     * `mouse_selection_rectangular_switch` 鼠标选择矩形开关。
     * */
    fn set_mouse_selection_rectangular_switch(&self, mouse_selection_rectangular_switch: bool);

    /**
     * 获取在使用鼠标进行选择时矩形选择模式。
     * */
    fn get_mouse_selection_rectangular_switch(&self) -> bool;

    /**
     * 启用或禁用多选。禁用多选时，无法通过按住Ctrl键同时用鼠标拖动来选择多个范围。
     * `multiple_selection` 启用多选。
     * */
    fn set_multiple_selection(&self, multiple_selection: bool);

    /**
     * 获取多选开关状态。禁用多选时，无法通过按住Ctrl键同时用鼠标拖动来选择多个范围。
     * */
    fn get_multiple_selection(&self) -> bool;

    //noinspection StructuralWrap
    /**
     * 无论是打字、换行、光标左/右/上/下、退格、删除、主页还是同时结束多项选择。还允许选择和字和行删除命令。
     * `additional_selection_typing` 附加选择类型。
     * */
    fn set_additional_selection_typing(&self, additional_selection_typing: bool);

    /**
     * 获取附加选择类型状态。无论是打字、换行、光标左/右/上/下、退格、删除、主页还是同时结束多项选择。还允许选择和字和行删除命令。
     * */
    fn get_additional_selection_typing(&self) -> bool;

    /**
     * 粘贴到多个选择中时，粘贴的文本可以仅进入multi_paste=false的主选择中，也可以进入multi_paste=true的每个选择中。false是默认值。
     * `multi_paste` 是否启用多粘贴方式。
     * */
    fn set_multi_paste(&self, multi_paste: bool);

    /**
     * 获取多粘贴方式。粘贴到多个选择中时，粘贴的文本仅进入主选择返回false，进入每个选择返回true。
     * */
    fn get_multi_paste(&self) -> bool;

    /**
     * 虚拟空间可以针对矩形选择启用或禁用，也可以在其他情况下启用或禁用。有三个位标志SCVS_RECTANGULARSELECTION=1、SCVS_USERACCESSABLE=2和SCVS_NOWRAPLINESTART=4，它们可以独立设置。SCVS_NONE=0（默认值）将禁用所有虚拟空间的使用。
     * SCVS_NOWRAPLINESTART防止左箭头移动和选择换行到前一行。这通常与虚拟空间结合使用，但它是一个独立的设置，因此在没有虚拟空间的情况下也能工作。
     * `virtual_space` 虚拟空间位标志。
     * */
    fn set_virtual_space_options(&self, virtual_space: u32);

    /**
     * 获取虚拟空间位标志。
     * 虚拟空间可以针对矩形选择启用或禁用，也可以在其他情况下启用或禁用。有三个位标志SCVS_RECTANGULARSELECTION=1、SCVS_USERACCESSABLE=2和SCVS_NOWRAPLINESTART=4，它们可以独立设置。SCVS_NONE=0（默认值）将禁用所有虚拟空间的使用。
     * SCVS_NOWRAPLINESTART防止左箭头移动和选择换行到前一行。这通常与虚拟空间结合使用，但它是一个独立的设置，因此在没有虚拟空间的情况下也能工作。
     * */
    fn get_virtual_space_options(&self) -> u32;

    /**
     * 在GTK和Qt上，可以设置用于指示与鼠标拖动组合时应创建矩形选择的键。三个可能的值是SCMOD_CTRL=2、SCMOD_ALT=4（默认值）或SCMOD_SUPER=8。由于SCMOD_ALT可能已经被窗口管理器使用，因此窗口管理器可能需要进行配置以允许此选择。SCMOD_SUPER通常是一个依赖于系统的修改键，例如Windows键盘上的“左Windows”键或Mac上的“命令”键。
     * `modifier` 修饰为标志。
     * */
    fn set_rectangular_selection_modifier(&self, modifier: u32);

    /**
     * 在GTK和Qt上，可以设置用于指示与鼠标拖动组合时应创建矩形选择的键。三个可能的值是SCMOD_CTRL=2、SCMOD_ALT=4（默认值）或SCMOD_SUPER=8。由于SCMOD_ALT可能已经被窗口管理器使用，因此窗口管理器可能需要进行配置以允许此选择。SCMOD_SUPER通常是一个依赖于系统的修改键，例如Windows键盘上的“左Windows”键或Mac上的“命令”键。
     * */
    fn get_rectangular_selection_modifier(&self) -> u32;

    /**
     * 返回当前活动的选择数。始终至少有一个选择。
     * */
    fn get_selections(&self) -> i32;

    /**
     * 如果每个选定范围为空，则返回true，否则返回false。
     * */
    fn get_selection_empty(&self) -> bool;

    /**
     * 将0处的单个空选择设置为唯一选择。
     * */
    fn clear_selections(&self);

    //noinspection StructuralWrap
    /**
     * 将从锚点到插入点的单个选择设置为唯一选择。
     * `caret` 插入点。
     * `archor` 锚点。
     * */
    fn set_selection(&self, caret: usize, anchor: usize);

    /**
     * 从锚点到插入点添加一个新的选择作为主选择，保留所有其他选择作为附加选择。由于总是至少有一个选择，因此要设置选择列表，第一个选择应添加SCI_SETSELECTION，随后的选择应添加SCI_ADDSELECTION。
     * `caret` 插入点。
     * `archor` 锚点。
     * */
    fn add_selection(&self, caret: usize, anchor: usize);

    //noinspection StructuralWrap
    /**
     * 如果有多个选择，则删除指示的选择。如果这是主选择，则将上一个选择作为主选择；如果是第一个选择，则最后一个选择成为主选择。如果只有一个选择，或者没有选择，则没有效果。
     * `selection` 选择区。
     * */
    fn drop_selection_n(&self, selection: i32);

    //noinspection StructuralWrap
    /**
     * 其中一个选项是主选项，用于确定自动可见的文本范围。主选择可以用不同的颜色显示，或者用不同样式的插入点显示。只有已存在的选择才能成为主要选择。
     * `selection` 选择区。
     * */
    fn set_main_selection(&self, selection: i32);

    /**
     * 获取主要选择，用于确定自动可见的文本范围。主选择可以用不同的颜色显示，或者用不同样式的插入点显示。
     * */
    fn get_main_selection(&self) -> i32;

    /**
     * 设置每个已存在选择的插入点的位置。
     * `selection` 选择区。
     * `caret` 插入点。
     * */
    fn set_selection_n_caret(&self, selection: i32, caret: usize);

    //noinspection StructuralWrap
    /**
     * 查询每个已存在选择的插入点的位置。
     * `selection` 选择区。
     * */
    fn get_selection_n_caret(&self, selection: i32) -> usize;

    /**
     * 设置每个已存在选择的插入点的虚拟空间数量。
     * `selection` 选择区。
     * `space` 虚拟空间。
     * */
    fn set_selection_n_caret_virtual_space(&self, selection: i32, space: usize);

    //noinspection StructuralWrap
    /**
     * 查询每个已存在选择的插入点的虚拟空间数量。
     * `selection` 选择区。
     * */
    fn get_selection_n_caret_virtual_space(&self, selection: i32) -> usize;

    /**
     * 设置每个已存在选择的锚点的位置。
     * `selection` 选择区。
     * `anchor` 锚点。
     * */
    fn set_selection_n_anchor(&self, selection: i32, anchor: usize);

    //noinspection StructuralWrap
    /**
     * 查询每个已存在选择的锚点的位置。
     * `selection` 选择区。
     * */
    fn get_selection_n_anchor(&self, selection: i32) -> usize;

    /**
     * 设置每个已存在选择的锚点的虚拟空间数量。
     * `selection` 选择区。
     * `space` 虚拟空间。
     * */
    fn set_selection_n_anchor_virtual_space(&self, selection: i32, space: usize);

    //noinspection StructuralWrap
    /**
     * 查询每个已存在选择的锚点的虚拟空间数量。
     * `selection` 选择区。
     * */
    fn get_selection_n_anchor_virtual_space(&self, selection: i32) -> usize;

    //noinspection StructuralWrap
    /**
     * 设置每个已存在选择的开始位置。主要用于查询每个范围的文本。选择参数为零。
     * `selection` 选择区。
     * `anchor` 锚点。
     * */
    fn set_selection_n_start(&self, selection: i32, anchor: usize);

    //noinspection StructuralWrap
    /**
     * 查询每个已存在选择的开始位置。主要用于查询每个范围的文本。选择参数为零。
     * `selection` 选择区。
     * */
    fn get_selection_n_start(&self, selection: i32) -> usize;

    //noinspection StructuralWrap
    /**
     * 设置每个已存在选择的结束位置。主要用于查询每个范围的文本。选择参数为零。
     * `selection` 选择区。
     * `caret` 插入点。
     * */
    fn set_selection_n_end(&self, selection: i32, caret: usize);

    //noinspection StructuralWrap
    /**
     * 查询每个已存在选择的结束位置。主要用于查询每个范围的文本。选择参数为零。
     * `selection` 选择区。
     * */
    fn get_selection_n_end(&self, selection: i32) -> usize;

    //noinspection StructuralWrap
    /**
     * 设置矩形选择的插入点的位置。设置矩形选择后，会将其分解为多个选择，每行一个。
     * `caret` 插入点。
     * */
    fn set_rectangular_selection_caret(&self, caret: usize);

    /**
     * 查询矩形选择的插入点的位置。矩形选择会将其分解为多个选择，每行一个。
     * */
    fn get_rectangular_selection_caret(&self) -> usize;

    //noinspection StructuralWrap
    /**
     * 设置矩形选择的插入点的虚拟空间数量。设置矩形选择后，会将其分解为多个选择，每行一个。
     * `space` 虚拟空间。
     * */
    fn set_rectangular_selection_caret_virtual_space(&self, space: usize);

    /**
     * 查询矩形选择的插入点的虚拟空间数量。矩形选择会将其分解为多个选择，每行一个。
     * */
    fn get_rectangular_selection_caret_virtual_space(&self) -> usize;

    //noinspection StructuralWrap
    /**
     * 设置矩形选择的锚点的位置。设置矩形选择后，会将其分解为多个选择，每行一个。
     * `anchor` 锚点。
     * */
    fn set_rectangular_selection_anchor(&self, anchor: usize);

    /**
     * 查询矩形选择的锚点的位置。矩形选择会将其分解为多个选择，每行一个。
     * */
    fn get_rectangular_selection_anchor(&self) -> usize;

    //noinspection StructuralWrap
    /**
     * 设置矩形选择的锚点的虚拟空间数量。设置矩形选择后，会将其分解为多个选择，每行一个。
     * `space` 虚拟空间。
     * */
    fn set_rectangular_selection_anchor_virtual_space(&self, space: usize);

    /**
     * 查询矩形选择的锚点的虚拟空间数量。矩形选择会将其分解为多个选择，每行一个。
     * */
    fn get_rectangular_selection_anchor_virtual_space(&self) -> usize;

    /**
     * 修改附加选择的外观，以便将其与外观设置为SC_ELEMENT_SELECTION_TEXT、SC_ELEMENT_SELECTION_BACK、SCI_SETSELALPHA、SCI_GETSELALPHA、SCI_SETSELFORE和SCI_SETSELBACK的主选择区分开来。首选元素API，不鼓励使用以下消息。附加选择背景绘制在SCI_SETSELECTIONLAYER为所有选择背景定义的图层上。只有在useSetting值设置为true的情况下调用SCI_SETSELFORE和SCI_SETSELBACK后，SCI_SETADDITIONALSELFORE与SCI_SETADDITIONALSELBACK调用才会生效。随后对SCI_SETSELFORE和SCI_SETSELBACK的调用将覆盖SCI_SETADDITIONALSEL*函数设置的值。
     * `alpha` 外观。
     * */
    fn set_additional_sel_alpha(&self, alpha: i32);

    /**
     * 获取附加选择的外观。
     * */
    fn get_additional_sel_alpha(&self) -> i32;

    /**
     * 修改附加选择的外观，以便将其与外观设置为SC_ELEMENT_SELECTION_TEXT、SC_ELEMENT_SELECTION_BACK、SCI_SETSELALPHA、SCI_GETSELALPHA、SCI_SETSELFORE和SCI_SETSELBACK的主选择区分开来。首选元素API，不鼓励使用以下消息。附加选择背景绘制在SCI_SETSELECTIONLAYER为所有选择背景定义的图层上。只有在useSetting值设置为true的情况下调用SCI_SETSELFORE和SCI_SETSELBACK后，SCI_SETADDITIONALSELFORE与SCI_SETADDITIONALSELBACK调用才会生效。随后对SCI_SETSELFORE和SCI_SETSELBACK的调用将覆盖SCI_SETADDITIONALSEL*函数设置的值。
     * `fore` 外观。
     * */
    fn set_additional_sel_fore(&self, fore: i32);

    /**
     * 修改附加选择的外观，以便将其与外观设置为SC_ELEMENT_SELECTION_TEXT、SC_ELEMENT_SELECTION_BACK、SCI_SETSELALPHA、SCI_GETSELALPHA、SCI_SETSELFORE和SCI_SETSELBACK的主选择区分开来。首选元素API，不鼓励使用以下消息。附加选择背景绘制在SCI_SETSELECTIONLAYER为所有选择背景定义的图层上。只有在useSetting值设置为true的情况下调用SCI_SETSELFORE和SCI_SETSELBACK后，SCI_SETADDITIONALSELFORE与SCI_SETADDITIONALSELBACK调用才会生效。随后对SCI_SETSELFORE和SCI_SETSELBACK的调用将覆盖SCI_SETADDITIONALSEL*函数设置的值。
     * `back` 外观。
     * */
    fn set_additional_sel_back(&self, back: i32);

    /**
     * 修改附加插入符号的外观，以便将其与外观设置为SC_ELEMENT_CARET、SCI_SETCARETFORE、SCI_GETCARETFORE、SCI_SETCARETPERIOD和SCI_GETCARETPERIOD的主插入符号区分开来。
     * `fore` 外观。
     * */
    fn set_additional_caret_fore(&self, fore: i32);

    /**
     * 获取附加插入符号的外观。
     * */
    fn get_additional_caret_fore(&self) -> i32;

    /**
     * 修改附加插入符号的外观，以便将其与外观设置为SC_ELEMENT_CARET、SCI_SETCARETFORE、SCI_GETCARETFORE、SCI_SETCARETPERIOD和SCI_GETCARETPERIOD的主插入符号区分开来。
     * `additional_carets_blink` 可选插入点闪烁。
     * */
    fn set_additional_carets_blink(&self, additional_carets_blink: bool);

    /**
     * 获取附加插入符号的外观。
     * */
    fn get_additional_carets_blink(&self) -> bool;

    /**
     * 确定是否显示其他插入符号（默认为true）。
     * `visible` 是否显示。
     * */
    fn set_additional_carets_visible(&self, visible: bool);

    /**
     * 确定是否显示其他插入符号（默认为true）。
     * */
    fn get_additional_carets_visible(&self) -> bool;

    /**
     * 将插入符号移动到主选择的另一端。
     * */
    fn swap_main_anchor_caret(&self);

    /**
     * 使下一个选择成为主选择。
     * */
    fn rotate_selection(&self);

    /**
     * 将目标中下一次出现的主选择添加到主选择集。如果当前选择为空，则选择插入符号周围的单词。使用当前的search_flags，因此应用程序可以选择区分大小写和单词搜索选项。
     * */
    fn multiple_select_add_next(&self);

    /**
     * 类似于multiple_select_add_next，但添加了多个引用，而不是仅添加一个。
     * */
    fn multiple_select_add_each(&self);

    //noinspection StructuralWrap
    /**
     * 设置改写模式。启用改写后，每个键入的字符都会替换文本插入符号右侧的字符。如果禁用了改写，则会在插入符号处插入字符。如果改写处于活动状态，SCI_GETOVERTYPE将返回true（1），否则将返回false（0）。
     * `over_type` 改写模式。
     * */
    fn set_over_type(&self, over_type: bool);

    /**
     * 获取改写模式。启用改写后，每个键入的字符都会替换文本插入符号右侧的字符。如果禁用了改写，则会在插入符号处插入字符。如果改写处于活动状态，SCI_GETOVERTYPE将返回true（1），否则将返回false（0）。
     * */
    fn get_over_type(&self) -> bool;

    /**
     * 设置目标的开始点。搜索时，可以将“开始”设置为大于“结束”，以查找目标中的最后一个匹配文本，而不是第一个匹配文本。使用SCI_SETTARGETSTART、SCI_SETTARGETEND或SCI_SETTARGETRANGE设置目标位置会将虚拟空间设置为0。该目标也是由成功的SCI_SEARCHINTARGET设定的。
     * `start` 开始点。
     * */
    fn set_target_start(&self, start: usize);

    /**
     * 获取目标的开始点。
     * */
    fn get_target_start(&self) -> usize;

    /**
     * 设置目标的结束点。搜索时，可以将“开始”设置为大于“结束”，以查找目标中的最后一个匹配文本，而不是第一个匹配文本。使用SCI_SETTARGETSTART、SCI_SETTARGETEND或SCI_SETTARGETRANGE设置目标位置会将虚拟空间设置为0。该目标也是由成功的SCI_SEARCHINTARGET设定的。
     * `end` 结束点。
     * */
    fn set_target_end(&self, end: usize);

    /**
     * 获取目标的结束点。
     * */
    fn get_target_end(&self) -> usize;

    /**
     * 设置目标的范围。搜索时，可以将“开始”设置为大于“结束”，以查找目标中的最后一个匹配文本，而不是第一个匹配文本。使用SCI_SETTARGETSTART、SCI_SETTARGETEND或SCI_SETTARGETRANGE设置目标位置会将虚拟空间设置为0。该目标也是由成功的SCI_SEARCHINTARGET设定的。
     * `start` 开始点。
     * `end` 结束点。
     * */
    fn set_target_range(&self, start: usize, end: usize);

    /**
     * 将目标起点和终点设置为选择的起点和终点位置。
     * */
    fn target_from_selection(&self);

    /**
     * 将目标起点设置为文档起点，将目标终点设置为文档终点。
     * */
    fn target_whole_document(&self);

    /**
     * 设置SCI_SEARCHINTARGET使用的search_flags。有几个选项标志，包括一个简单的正则表达式搜索。
     * `search_flags` 搜索标志。
     * */
    fn set_search_flags(&self, search_flags: u32);

    /**
     * 获取SCI_SEARCHINTARGET使用的search_flags。有几个选项标志，包括一个简单的正则表达式搜索。
     * */
    fn get_search_flags(&self) -> u32;

    /**
     * 这将搜索SCI_SETTARGETSTART和SCI_SETTARGETEND定义的目标中第一个出现的文本字符串。搜索由SCI_SETSEARCHFLAGS设置的搜索标志进行修改。如果搜索成功，则将目标设置为找到的文本，返回值为匹配文本的起始位置。如果搜索失败，结果为-1。
     * `text` 要搜索的文字。
     * */
    fn search_in_target(&self, text: String) -> usize;

    //noinspection StructuralWrap
    /**
     * 查询目标中的值。
     * `length` 字符数，不包括'\0'
     * */
    fn get_target_text(&self, length: usize) -> Option<String>;

    //noinspection StructuralWrap
    /**
     * 替换后，目标范围是指替换文本。返回值是替换字符串的长度。
     * 请注意，删除文档中文本的建议方法是将目标设置为要删除的文本，并用空字符串替换目标。
     * `text` 要替换的文字。
     * */
    fn replace_target(&self, text: String) -> usize;

    /**
     * 这将使用正则表达式替换目标。替换字符串由文本字符串组成，其中\1到\9的任何序列都被最近正则表达式搜索中的标记匹配项替换\0将替换为最近搜索中的所有匹配文本。替换后，目标范围是指替换文本。返回值是替换字符串的长度。
     * */
    fn replace_target_re(&self, text: String) -> usize;

    //noinspection StructuralWrap
    /**
     * 发现正则表达式搜索中标记的表达式匹配了哪些文本。如果应用程序想要解释替换字符串本身，这将非常有用。
     * 另请参阅：SCI_FINDTEXT
     * `tag_number` 标记序号。
     * `length` 字符数，不包括'\0'
     * */
    fn get_tag(&self, tag_number: i32, length: usize) -> (i32, Option<String>);

    /**
     * 消息搜索文档中的文本。它们不使用或移动当前选择。search_flags参数控制搜索类型，其中包括正则表达式搜索。
     * 通过在开始之前设置搜索范围的末尾，可以向后搜索以查找搜索字符串的前一个出现。
     * 设置min和max以及要搜索的文档中的位置范围。您可以通过将max设置为小于min来向后搜索。
     * 如果搜索失败，则返回值为-1；如果搜索成功，则返回所找到文本的起始位置。
     * 另请参阅：SCI_SEARCHINTARGET
     * `text` 要搜索的文字或正规表达式。
     * `min` 搜索开始位置。
     * `max` 搜索结束位置。
     * `search_flags` 搜索标志。
     * */
    fn find_text(
        &self,
        text: String,
        min: isize,
        max: isize,
        search_flags: u32,
    ) -> (usize, Option<(usize, usize)>);

    /**
     * 消息提供可重定位搜索支持。这允许宏记录多个增量交互式搜索，同时仍将选择设置为已找到的文本，因此查找/选择操作是自包含的。如果启用了宏录制，这三条消息将发送SCN_MACRORECORD通知。
     * 将SCI_SEARCHNEXT和SCI_SEARCHPREV使用的搜索起点设置为当前选择的起点，即更接近文档起点的选择的终点。在调用SCI_SEARCHNEXT或SCI_SEARCHPREV之前，应始终调用此函数。
     * */
    fn search_anchor(&self);

    //noinspection StructuralWrap
    /**
     * 消息提供可重定位搜索支持。这允许宏记录多个增量交互式搜索，同时仍将选择设置为已找到的文本，因此查找/选择操作是自包含的。如果启用了宏录制，这三条消息将发送SCN_MACRORECORD通知。
     * 搜索上一个。搜索由search_flags修改。
     * 如果未找到任何内容，则返回值为-1，否则返回值为匹配文本的起始位置。所选内容会更新以显示匹配的文本，但不会滚动到视图中。
     * 另请参见：SCI_SEARCHINTARGET、SCI_FINDTEXT
     * `search_flags` 搜索标志。
     * */
    fn search_prev(&self, search_flags: u32, text: String) -> usize;

    //noinspection StructuralWrap
    /**
     * 消息提供可重定位搜索支持。这允许宏记录多个增量交互式搜索，同时仍将选择设置为已找到的文本，因此查找/选择操作是自包含的。如果启用了宏录制，这三条消息将发送SCN_MACRORECORD通知。
     * 搜索下一个。搜索由search_flags修改。
     * 如果未找到任何内容，则返回值为-1，否则返回值为匹配文本的起始位置。所选内容会更新以显示匹配的文本，但不会滚动到视图中。
     * 另请参见：SCI_SEARCHINTARGET、SCI_FINDTEXT
     * `search_flags` 搜索标志。
     * */
    fn search_next(&self, search_flags: u32, text: String) -> usize;

    /**
     * 将数据剪切。如果您需要“可以复制”或“可以剪切”，请使用SCI_GETSELECTIONEMPTY()，如果有任何非空的选择范围意味着复制或剪切到剪贴板应该可以工作，则该值将为零。
     * */
    fn cut(&self);

    /**
     * 将数据复制到剪贴板。如果您需要“可以复制”或“可以剪切”，请使用SCI_GETSELECTIONEMPTY（），如果有任何非空的选择范围意味着复制或剪切到剪贴板应该可以工作，则该值将为零。
     * */
    fn copy(&self);

    /**
     * 从剪贴板粘贴到文档。
     * 在X上，剪贴板是异步的，可能需要在目标应用程序和源应用程序之间发送多条消息。来自SCI_PASTE的数据不会立即到达文档中。
     * */
    fn paste(&self);

    /**
     * 清除文档。
     * */
    fn clear(&self);

    /**
     * 查询是否可以粘贴。如果文档不是只读的，并且所选内容不包含受保护的文本，SCI_CANPASTE将返回非零。如果您需要“可以复制”或“可以剪切”，请使用SCI_GETSELECTIONEMPTY（），如果有任何非空的选择范围意味着复制或剪切到剪贴板应该可以工作，则该值将为零。
     * GTK并不真正支持SCI_CANPASTE，并且总是返回true，除非文档是只读的。
     * */
    fn can_paste(&self) -> bool;

    /**
     * 工作原理与SCI_COPY相同，只是如果选择为空，则复制当前行。在Windows上，会将一个额外的“MSDEVLineSelect”标记添加到剪贴板，然后在SCI_PASTE中使用该标记将整行粘贴到当前行之前。
     * */
    fn copy_allow_line(&self);

    //noinspection StructuralWrap
    /**
     * 将一系列文本从文档复制到系统剪贴板。
     * `start` 开始点。
     * `end` 结束点。
     * */
    fn copy_range(&self, start: usize, end: usize);

    //noinspection StructuralWrap
    /**
     * 将提供的文本复制到系统剪切板。
     * `text` 要复制的文字。
     * */
    fn copy_text(&self, text: String);

    /**
     * 如果设置了此属性，则在粘贴文本时，将转换任何行尾以匹配使用SCI_SETEOLMODE设置的文档的行尾模式。默认为true。
     * `convert` 转换模式。
     * */
    fn set_paste_convert_endings(&self, convert: bool);

    /**
     * 获取粘贴时行尾转换模式。如果设置了此属性，则在粘贴文本时，将转换任何行尾以匹配使用SCI_SETEOLMODE设置的文档的行尾模式。默认为true。
     * */
    fn get_paste_convert_endings(&self) -> bool;
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

    fn set_sel(&self, anchor: usize, caret: usize) {
        self.send_message(SCI_SETSEL, WPARAM(anchor), LPARAM(caret as isize));
    }

    fn goto_pos(&self, caret: usize) {
        self.send_message(SCI_GOTOPOS, WPARAM(caret), LPARAM::default());
    }

    fn goto_line(&self, line: usize) {
        self.send_message(SCI_GOTOLINE, WPARAM(line), LPARAM::default());
    }

    fn set_current_pos(&self, caret: usize) {
        self.send_message(SCI_SETCURRENTPOS, WPARAM(caret), LPARAM::default());
    }

    fn get_current_pos(&self) -> usize {
        let (_, res) = self.send_message(SCI_GETCURRENTPOS, WPARAM::default(), LPARAM::default());
        res
    }

    fn set_anchor(&self, anchor: usize) {
        self.send_message(SCI_SETANCHOR, WPARAM(anchor), LPARAM::default());
    }

    fn get_anchor(&self) -> usize {
        let (_, res) = self.send_message(SCI_GETANCHOR, WPARAM::default(), LPARAM::default());
        res
    }

    fn set_selection_start(&self, anchor: usize) {
        self.send_message(SCI_SETSELECTIONSTART, WPARAM(anchor), LPARAM::default());
    }

    fn set_selection_end(&self, caret: usize) {
        self.send_message(SCI_SETSELECTIONEND, WPARAM(caret), LPARAM::default());
    }

    fn get_selection_start(&self) -> usize {
        let (_, res) =
            self.send_message(SCI_GETSELECTIONSTART, WPARAM::default(), LPARAM::default());
        res
    }

    fn get_selection_end(&self) -> usize {
        let (_, res) = self.send_message(SCI_GETSELECTIONEND, WPARAM::default(), LPARAM::default());
        res
    }

    fn set_empty_selection(&self, caret: usize) {
        self.send_message(SCI_SETEMPTYSELECTION, WPARAM(caret), LPARAM::default());
    }

    fn select_all(&self) {
        self.send_message(SCI_SELECTALL, WPARAM::default(), LPARAM::default());
    }

    fn hide_selection(&self, hide: bool) {
        let hide = if hide { 1 } else { 0 };
        self.send_message(SCI_HIDESELECTION, WPARAM(hide), LPARAM::default());
    }

    fn get_sel_text(&self) -> Option<String> {
        let (_, len) = self.send_message(SCI_GETSELTEXT, WPARAM::default(), LPARAM::default());
        let mem = InProcessMemory::new(self.get_pid(), len + 1).unwrap();
        self.send_message(
            SCI_GETSELTEXT,
            WPARAM::default(),
            LPARAM(mem.as_ptr() as isize),
        );
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn get_cur_line(&self) -> (Option<String>, usize) {
        let (_, len) = self.send_message(SCI_GETCURLINE, WPARAM::default(), LPARAM::default());
        let mem = InProcessMemory::new(self.get_pid(), len + 1).unwrap();
        let (_, caret) =
            self.send_message(SCI_GETCURLINE, WPARAM(len), LPARAM(mem.as_ptr() as isize));
        (mem.read(|buf| (buf as *const u8).to_string()), caret)
    }

    fn selection_is_rectangle(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_SELECTIONISRECTANGLE,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn set_selection_mode(&self, selection_mode: SelectionMode) {
        self.send_message(
            SCI_SETSELECTIONMODE,
            WPARAM(<SelectionMode as Into<u32>>::into(selection_mode) as usize),
            LPARAM::default(),
        );
    }

    fn get_selection_mode(&self) -> SelectionMode {
        let (_, res) =
            self.send_message(SCI_GETSELECTIONMODE, WPARAM::default(), LPARAM::default());
        SelectionMode::from(res as u32)
    }

    fn get_move_extends_selection(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_GETMOVEEXTENDSSELECTION,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn get_line_sel_start_position(&self, line: usize) -> usize {
        let (_, res) =
            self.send_message(SCI_GETLINESELSTARTPOSITION, WPARAM(line), LPARAM::default());
        res
    }

    fn get_line_sel_end_position(&self, line: usize) -> usize {
        let (_, res) =
            self.send_message(SCI_GETLINESELENDPOSITION, WPARAM(line), LPARAM::default());
        res
    }

    fn move_caret_inside_view(&self) {
        self.send_message(
            SCI_MOVECARETINSIDEVIEW,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn choose_caret_x(&self) {
        self.send_message(SCI_CHOOSECARETX, WPARAM::default(), LPARAM::default());
    }

    fn move_selected_lines_up(&self) {
        self.send_message(
            SCI_MOVESELECTEDLINESUP,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn move_selected_lines_down(&self) {
        self.send_message(
            SCI_MOVESELECTEDLINESDOWN,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn set_mouse_selection_rectangular_switch(&self, mouse_selection_rectangular_switch: bool) {
        let mode = if mouse_selection_rectangular_switch {
            1
        } else {
            0
        };
        self.send_message(
            SCI_SETMOUSESELECTIONRECTANGULARSWITCH,
            WPARAM(mode),
            LPARAM::default(),
        );
    }

    fn get_mouse_selection_rectangular_switch(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_GETMOUSESELECTIONRECTANGULARSWITCH,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn set_multiple_selection(&self, multiple_selection: bool) {
        let multiple_selection = if multiple_selection { 1 } else { 0 };
        self.send_message(
            SCI_SETMULTIPLESELECTION,
            WPARAM(multiple_selection),
            LPARAM::default(),
        );
    }

    fn get_multiple_selection(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_GETMULTIPLESELECTION,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn set_additional_selection_typing(&self, additional_selection_typing: bool) {
        let additional_selection_typing = if additional_selection_typing { 1 } else { 0 };
        self.send_message(
            SCI_SETADDITIONALSELECTIONTYPING,
            WPARAM(additional_selection_typing),
            LPARAM::default(),
        );
    }

    fn get_additional_selection_typing(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_GETADDITIONALSELECTIONTYPING,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn set_multi_paste(&self, multi_paste: bool) {
        let multi_paste = if multi_paste { 1 } else { 0 };
        self.send_message(SCI_SETMULTIPASTE, WPARAM(multi_paste), LPARAM::default());
    }

    fn get_multi_paste(&self) -> bool {
        let (_, res) = self.send_message(SCI_GETMULTIPASTE, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn set_virtual_space_options(&self, virtual_space: u32) {
        self.send_message(
            SCI_SETVIRTUALSPACEOPTIONS,
            WPARAM(virtual_space as usize),
            LPARAM::default(),
        );
    }

    fn get_virtual_space_options(&self) -> u32 {
        let (_, res) = self.send_message(
            SCI_GETVIRTUALSPACEOPTIONS,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as u32
    }

    fn set_rectangular_selection_modifier(&self, modifier: u32) {
        self.send_message(
            SCI_SETRECTANGULARSELECTIONMODIFIER,
            WPARAM(modifier as usize),
            LPARAM::default(),
        );
    }

    fn get_rectangular_selection_modifier(&self) -> u32 {
        let (_, res) = self.send_message(
            SCI_GETRECTANGULARSELECTIONMODIFIER,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as u32
    }

    fn get_selections(&self) -> i32 {
        let (_, res) = self.send_message(SCI_GETSELECTIONS, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn get_selection_empty(&self) -> bool {
        let (_, res) = self.send_message(SCI_GETSELECTIONS, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn clear_selections(&self) {
        self.send_message(SCI_CLEARSELECTIONS, WPARAM::default(), LPARAM::default());
    }

    fn set_selection(&self, caret: usize, anchor: usize) {
        self.send_message(SCI_SETSELECTION, WPARAM(caret), LPARAM(anchor as isize));
    }

    fn add_selection(&self, caret: usize, anchor: usize) {
        self.send_message(SCI_ADDSELECTION, WPARAM(caret), LPARAM(anchor as isize));
    }

    fn drop_selection_n(&self, selection: i32) {
        self.send_message(
            SCI_DROPSELECTIONN,
            WPARAM(selection as usize),
            LPARAM::default(),
        );
    }

    fn set_main_selection(&self, selection: i32) {
        self.send_message(
            SCI_SETMAINSELECTION,
            WPARAM(selection as usize),
            LPARAM::default(),
        );
    }

    fn get_main_selection(&self) -> i32 {
        let (_, res) =
            self.send_message(SCI_GETMAINSELECTION, WPARAM::default(), LPARAM::default());
        res as i32
    }

    fn set_selection_n_caret(&self, selection: i32, caret: usize) {
        self.send_message(
            SCI_SETSELECTIONNCARET,
            WPARAM(selection as usize),
            LPARAM(caret as isize),
        );
    }

    fn get_selection_n_caret(&self, selection: i32) -> usize {
        let (_, res) = self.send_message(
            SCI_GETSELECTIONNCARET,
            WPARAM(selection as usize),
            LPARAM::default(),
        );
        res
    }

    fn set_selection_n_caret_virtual_space(&self, selection: i32, space: usize) {
        self.send_message(
            SCI_SETSELECTIONNCARETVIRTUALSPACE,
            WPARAM(selection as usize),
            LPARAM(space as isize),
        );
    }

    fn get_selection_n_caret_virtual_space(&self, selection: i32) -> usize {
        let (_, res) = self.send_message(
            SCI_GETSELECTIONNCARETVIRTUALSPACE,
            WPARAM(selection as usize),
            LPARAM::default(),
        );
        res
    }

    fn set_selection_n_anchor(&self, selection: i32, anchor: usize) {
        self.send_message(
            SCI_SETSELECTIONNANCHOR,
            WPARAM(selection as usize),
            LPARAM(anchor as isize),
        );
    }

    fn get_selection_n_anchor(&self, selection: i32) -> usize {
        let (_, res) = self.send_message(
            SCI_GETSELECTIONNANCHOR,
            WPARAM(selection as usize),
            LPARAM::default(),
        );
        res
    }

    fn set_selection_n_anchor_virtual_space(&self, selection: i32, space: usize) {
        self.send_message(
            SCI_SETSELECTIONNANCHORVIRTUALSPACE,
            WPARAM(selection as usize),
            LPARAM(space as isize),
        );
    }

    fn get_selection_n_anchor_virtual_space(&self, selection: i32) -> usize {
        let (_, res) = self.send_message(
            SCI_GETSELECTIONNANCHORVIRTUALSPACE,
            WPARAM(selection as usize),
            LPARAM::default(),
        );
        res
    }

    fn set_selection_n_start(&self, selection: i32, anchor: usize) {
        self.send_message(
            SCI_SETSELECTIONNSTART,
            WPARAM(selection as usize),
            LPARAM(anchor as isize),
        );
    }

    fn get_selection_n_start(&self, selection: i32) -> usize {
        let (_, res) = self.send_message(
            SCI_GETSELECTIONNSTART,
            WPARAM(selection as usize),
            LPARAM::default(),
        );
        res
    }

    fn set_selection_n_end(&self, selection: i32, caret: usize) {
        self.send_message(
            SCI_SETSELECTIONNEND,
            WPARAM(selection as usize),
            LPARAM(caret as isize),
        );
    }

    fn get_selection_n_end(&self, selection: i32) -> usize {
        let (_, res) = self.send_message(
            SCI_GETSELECTIONNEND,
            WPARAM(selection as usize),
            LPARAM::default(),
        );
        res
    }

    fn set_rectangular_selection_caret(&self, caret: usize) {
        self.send_message(
            SCI_SETRECTANGULARSELECTIONCARET,
            WPARAM(caret),
            LPARAM::default(),
        );
    }

    fn get_rectangular_selection_caret(&self) -> usize {
        let (_, res) = self.send_message(
            SCI_GETRECTANGULARSELECTIONCARET,
            WPARAM::default(),
            LPARAM::default(),
        );
        res
    }

    fn set_rectangular_selection_caret_virtual_space(&self, space: usize) {
        self.send_message(
            SCI_SETRECTANGULARSELECTIONCARETVIRTUALSPACE,
            WPARAM(space),
            LPARAM::default(),
        );
    }

    fn get_rectangular_selection_caret_virtual_space(&self) -> usize {
        let (_, res) = self.send_message(
            SCI_GETRECTANGULARSELECTIONCARETVIRTUALSPACE,
            WPARAM::default(),
            LPARAM::default(),
        );
        res
    }

    fn set_rectangular_selection_anchor(&self, anchor: usize) {
        self.send_message(
            SCI_SETRECTANGULARSELECTIONCARET,
            WPARAM(anchor),
            LPARAM::default(),
        );
    }

    fn get_rectangular_selection_anchor(&self) -> usize {
        let (_, res) = self.send_message(
            SCI_GETRECTANGULARSELECTIONANCHOR,
            WPARAM::default(),
            LPARAM::default(),
        );
        res
    }

    fn set_rectangular_selection_anchor_virtual_space(&self, space: usize) {
        self.send_message(
            SCI_SETRECTANGULARSELECTIONCARET,
            WPARAM(space),
            LPARAM::default(),
        );
    }

    fn get_rectangular_selection_anchor_virtual_space(&self) -> usize {
        let (_, res) = self.send_message(
            SCI_GETRECTANGULARSELECTIONANCHORVIRTUALSPACE,
            WPARAM::default(),
            LPARAM::default(),
        );
        res
    }

    fn set_additional_sel_alpha(&self, alpha: i32) {
        self.send_message(
            SCI_SETADDITIONALSELALPHA,
            WPARAM(alpha as usize),
            LPARAM::default(),
        );
    }

    fn get_additional_sel_alpha(&self) -> i32 {
        let (_, res) = self.send_message(
            SCI_GETADDITIONALSELALPHA,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as i32
    }

    fn set_additional_sel_fore(&self, fore: i32) {
        self.send_message(
            SCI_SETADDITIONALSELFORE,
            WPARAM(fore as usize),
            LPARAM::default(),
        );
    }

    fn set_additional_sel_back(&self, back: i32) {
        self.send_message(
            SCI_SETADDITIONALSELBACK,
            WPARAM(back as usize),
            LPARAM::default(),
        );
    }

    fn set_additional_caret_fore(&self, fore: i32) {
        self.send_message(
            SCI_SETADDITIONALCARETFORE,
            WPARAM(fore as usize),
            LPARAM::default(),
        );
    }

    fn get_additional_caret_fore(&self) -> i32 {
        let (_, res) = self.send_message(
            SCI_GETADDITIONALCARETFORE,
            WPARAM::default(),
            LPARAM::default(),
        );
        res as i32
    }

    fn set_additional_carets_blink(&self, additional_carets_blink: bool) {
        let additional_carets_blink = if additional_carets_blink { 1 } else { 0 };
        self.send_message(
            SCI_SETADDITIONALCARETSBLINK,
            WPARAM(additional_carets_blink as usize),
            LPARAM::default(),
        );
    }

    fn get_additional_carets_blink(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_GETADDITIONALCARETSBLINK,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn set_additional_carets_visible(&self, visible: bool) {
        let visible = if visible { 1 } else { 0 };
        self.send_message(
            SCI_SETADDITIONALCARETSVISIBLE,
            WPARAM(visible as usize),
            LPARAM::default(),
        );
    }

    fn get_additional_carets_visible(&self) -> bool {
        let (_, res) = self.send_message(
            SCI_GETADDITIONALCARETSVISIBLE,
            WPARAM::default(),
            LPARAM::default(),
        );
        res != 0
    }

    fn swap_main_anchor_caret(&self) {
        self.send_message(
            SCI_SWAPMAINANCHORCARET,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn rotate_selection(&self) {
        self.send_message(SCI_ROTATESELECTION, WPARAM::default(), LPARAM::default());
    }

    fn multiple_select_add_next(&self) {
        self.send_message(
            SCI_MULTIPLESELECTADDNEXT,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn multiple_select_add_each(&self) {
        self.send_message(
            SCI_MULTIPLESELECTADDEACH,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn set_over_type(&self, over_type: bool) {
        let over_type = if over_type { 1 } else { 0 };
        self.send_message(SCI_SETOVERTYPE, WPARAM(over_type), LPARAM::default());
    }

    fn get_over_type(&self) -> bool {
        let (_, res) = self.send_message(SCI_GETOVERTYPE, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn set_target_start(&self, start: usize) {
        self.send_message(SCI_SETTARGETSTART, WPARAM(start), LPARAM::default());
    }

    fn get_target_start(&self) -> usize {
        let (_, res) = self.send_message(SCI_GETTARGETSTART, WPARAM::default(), LPARAM::default());
        res
    }

    fn set_target_end(&self, end: usize) {
        self.send_message(SCI_SETTARGETEND, WPARAM(end), LPARAM::default());
    }

    fn get_target_end(&self) -> usize {
        let (_, res) = self.send_message(SCI_GETTARGETEND, WPARAM::default(), LPARAM::default());
        res
    }

    fn set_target_range(&self, start: usize, end: usize) {
        self.send_message(SCI_SETTARGETRANGE, WPARAM(start), LPARAM(end as isize));
    }

    fn target_from_selection(&self) {
        self.send_message(
            SCI_TARGETFROMSELECTION,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn target_whole_document(&self) {
        self.send_message(
            SCI_TARGETWHOLEDOCUMENT,
            WPARAM::default(),
            LPARAM::default(),
        );
    }

    fn set_search_flags(&self, search_flags: u32) {
        self.send_message(
            SCI_SETSEARCHFLAGS,
            WPARAM(search_flags as usize),
            LPARAM::default(),
        );
    }

    fn get_search_flags(&self) -> u32 {
        let (_, res) = self.send_message(SCI_GETSEARCHFLAGS, WPARAM::default(), LPARAM::default());
        res as u32
    }

    fn search_in_target(&self, text: String) -> usize {
        let length = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_pid(), length + 1).unwrap();
        mem.write(text.as_ptr() as *const c_void, length);
        let (_, res) = self.send_message(
            SCI_SEARCHINTARGET,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
        res
    }

    fn get_target_text(&self, length: usize) -> Option<String> {
        let mem = InProcessMemory::new(self.get_pid(), length).unwrap();
        self.send_message(
            SCI_GETTARGETTEXT,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
        mem.read(|buf| (buf as *const u8).to_string())
    }

    fn replace_target(&self, text: String) -> usize {
        let length = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_pid(), length + 1).unwrap();
        mem.write(text.as_ptr() as *const c_void, length);
        let (_, res) = self.send_message(
            SCI_REPLACETARGET,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
        res
    }

    fn replace_target_re(&self, text: String) -> usize {
        let length = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_pid(), length + 1).unwrap();
        mem.write(text.as_ptr() as *const c_void, length);
        let (_, res) = self.send_message(
            SCI_REPLACETARGETRE,
            WPARAM(length),
            LPARAM(mem.as_ptr() as isize),
        );
        res
    }

    fn get_tag(&self, tag_number: i32, length: usize) -> (i32, Option<String>) {
        let mem = InProcessMemory::new(self.get_pid(), length).unwrap();
        let (_, res) = self.send_message(
            SCI_GETTAG,
            WPARAM(tag_number as usize),
            LPARAM(mem.as_ptr() as isize),
        );
        (res as i32, mem.read(|buf| (buf as *const u8).to_string()))
    }

    fn find_text(
        &self,
        text: String,
        min: isize,
        max: isize,
        search_flags: u32,
    ) -> (usize, Option<(usize, usize)>) {
        let length = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_pid(), length + 1).unwrap();
        mem.write(text.as_ptr() as *const c_void, length);
        let param = Sci_TextToFind {
            chrg: Sci_CharacterRange {
                cpMax: max as Sci_PositionCR,
                cpMin: min as Sci_PositionCR,
            },
            lpstrText: mem.as_ptr() as *const c_char,
            chrgText: Sci_CharacterRange { cpMin: 0, cpMax: 0 },
        };
        let size = std::mem::size_of::<Sci_TextToFind>();
        let mem2 = InProcessMemory::new(self.get_pid(), size).unwrap();
        mem2.write(&param as *const Sci_TextToFind as *const c_void, size);
        let (_, res) = self.send_message(
            SCI_FINDTEXT,
            WPARAM(search_flags as usize),
            LPARAM(mem2.as_ptr() as isize),
        );
        let range = mem2.read(|buf| unsafe { (buf as *const Sci_TextToFind).read().chrgText });
        match range {
            None => (res, None),
            Some(r) => (res, Some((r.cpMin as usize, r.cpMax as usize))),
        }
    }

    fn search_anchor(&self) {
        self.send_message(SCI_SEARCHANCHOR, WPARAM::default(), LPARAM::default());
    }

    fn search_prev(&self, search_flags: u32, text: String) -> usize {
        let length = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_pid(), length + 1).unwrap();
        mem.write(text.as_ptr() as *const c_void, length);
        let (_, res) = self.send_message(
            SCI_SEARCHPREV,
            WPARAM(search_flags as usize),
            LPARAM(mem.as_ptr() as isize),
        );
        res
    }

    fn search_next(&self, search_flags: u32, text: String) -> usize {
        let length = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_pid(), length + 1).unwrap();
        mem.write(text.as_ptr() as *const c_void, length);
        let (_, res) = self.send_message(
            SCI_SEARCHNEXT,
            WPARAM(search_flags as usize),
            LPARAM(mem.as_ptr() as isize),
        );
        res
    }

    fn cut(&self) {
        self.send_message(SCI_CUT, WPARAM::default(), LPARAM::default());
    }

    fn copy(&self) {
        self.send_message(SCI_COPY, WPARAM::default(), LPARAM::default());
    }

    fn paste(&self) {
        self.send_message(SCI_PASTE, WPARAM::default(), LPARAM::default());
    }

    fn clear(&self) {
        self.send_message(SCI_CLEAR, WPARAM::default(), LPARAM::default());
    }

    fn can_paste(&self) -> bool {
        let (_, res) = self.send_message(SCI_CUT, WPARAM::default(), LPARAM::default());
        res != 0
    }

    fn copy_allow_line(&self) {
        self.send_message(SCI_COPYALLOWLINE, WPARAM::default(), LPARAM::default());
    }

    fn copy_range(&self, start: usize, end: usize) {
        self.send_message(SCI_COPYRANGE, WPARAM(start), LPARAM(end as isize));
    }

    fn copy_text(&self, text: String) {
        let lentth = text.as_bytes().len();
        let mem = InProcessMemory::new(self.get_pid(), lentth + 1).unwrap();
        mem.write(text.as_ptr() as *const c_void, lentth);
        self.send_message(SCI_COPYTEXT, WPARAM(lentth), LPARAM(mem.as_ptr() as isize));
    }

    fn set_paste_convert_endings(&self, convert: bool) {
        let convert = if convert { 1 } else { 0 };
        self.send_message(SCI_SETPASTECONVERTENDINGS, WPARAM(convert), LPARAM::default());
    }

    fn get_paste_convert_endings(&self) -> bool {
        let (_, res) = self.send_message(SCI_GETPASTECONVERTENDINGS, WPARAM::default(), LPARAM::default());
        res != 0
    }
}

#[cfg(test)]
mod test_scintilla {
    use win_wrap::{
        common::{find_window_ex, HWND},
        control::WindowControl,
    };

    use crate::scintilla::{
        selection::SelectionMode, status::Status, Scintilla, SCFIND_MATCHCASE, SCMOD_META,
        SCVS_USERACCESSIBLE,
    };

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
        // has bugs
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
        control.set_sel(6, 7);
        control.goto_pos(9);
        control.goto_line(2);
        control.set_current_pos(6);
        assert_eq!(6, control.get_current_pos());
        control.set_anchor(8);
        assert_eq!(8, control.get_anchor());
        control.set_selection_start(10);
        assert_eq!(10, control.get_selection_start());
        control.set_selection_end(11);
        assert_eq!(11, control.get_selection_end());
        control.set_empty_selection(7);
        control.select_all();
        control.hide_selection(true);
        dbg!(control.get_sel_text());
        dbg!(control.get_cur_line());
        dbg!(control.selection_is_rectangle());
        control.set_selection_mode(SelectionMode::Rectangle);
        assert_eq!(SelectionMode::Rectangle, control.get_selection_mode());
        dbg!(control.get_move_extends_selection());
        dbg!(control.get_line_sel_start_position(1));
        dbg!(control.get_line_sel_end_position(1));
        control.move_caret_inside_view();
        control.choose_caret_x();
        control.move_selected_lines_up();
        control.move_selected_lines_down();
        control.set_mouse_selection_rectangular_switch(true);
        assert_eq!(true, control.get_mouse_selection_rectangular_switch());
        control.set_multiple_selection(true);
        assert_eq!(true, control.get_multiple_selection());
        control.set_additional_selection_typing(true);
        assert_eq!(true, control.get_additional_selection_typing());
        control.set_multi_paste(true);
        assert_eq!(true, control.get_multi_paste());
        control.set_virtual_space_options(SCVS_USERACCESSIBLE);
        assert_eq!(SCVS_USERACCESSIBLE, control.get_virtual_space_options());
        control.set_rectangular_selection_modifier(SCMOD_META);
        dbg!(control.get_rectangular_selection_modifier());
        dbg!(control.get_selections());
        dbg!(control.get_selection_empty());
        control.clear_selections();
        control.set_selection(1, 2);
        control.add_selection(3, 5);
        control.drop_selection_n(1);
        control.set_main_selection(1);
        assert_eq!(1, control.get_main_selection());
        control.set_selection_n_caret(1, 3);
        assert_eq!(3, control.get_selection_n_caret(1));
        control.set_selection_n_caret_virtual_space(1, 0);
        assert_eq!(0, control.get_selection_n_caret_virtual_space(1));
        control.set_selection_n_anchor(1, 5);
        assert_eq!(5, control.get_selection_n_anchor(1));
        control.set_selection_n_anchor_virtual_space(1, 0);
        assert_eq!(0, control.get_selection_n_anchor_virtual_space(1));
        control.set_selection_n_start(1, 3);
        assert_eq!(3, control.get_selection_n_start(1));
        control.set_selection_n_end(1, 5);
        assert_eq!(5, control.get_selection_n_end(1));
        control.set_rectangular_selection_caret(1);
        assert_eq!(1, control.get_rectangular_selection_caret());
        control.set_rectangular_selection_caret_virtual_space(1);
        assert_eq!(1, control.get_rectangular_selection_caret_virtual_space());
        control.set_rectangular_selection_anchor(0);
        assert_eq!(0, control.get_rectangular_selection_anchor());
        control.set_rectangular_selection_anchor_virtual_space(0);
        assert_eq!(0, control.get_rectangular_selection_anchor_virtual_space());
        control.set_additional_sel_alpha(1);
        dbg!(control.get_additional_sel_alpha());
        control.set_additional_sel_fore(1);
        control.set_additional_sel_back(1);
        control.set_additional_caret_fore(32);
        assert_eq!(32, control.get_additional_caret_fore());
        control.set_additional_carets_blink(true);
        assert_eq!(true, control.get_additional_carets_blink());
        control.set_additional_carets_visible(true);
        assert_eq!(true, control.get_additional_carets_visible());
        control.swap_main_anchor_caret();
        control.rotate_selection();
        control.multiple_select_add_next();
        control.multiple_select_add_each();
        control.set_over_type(true);
        assert_eq!(true, control.get_over_type());
        control.set_target_start(3);
        assert_eq!(3, control.get_target_start());
        control.set_target_start(4);
        assert_eq!(4, control.get_target_start());
        control.set_target_range(5, 6);
        control.target_from_selection();
        control.target_whole_document();
        control.set_search_flags(0);
        assert_eq!(0, control.get_search_flags());
        dbg!(control.search_in_target("ll".to_string()));
        dbg!(control.get_target_text(5));
        dbg!(control.replace_target("world".to_string()));
        dbg!(control.replace_target_re(r"worl\1".to_string()));
        dbg!(control.get_tag(0, 3));
        // has bugs
        // dbg!(control.find_text("lo".to_string(), 0, 10, SCFIND_MATCHCASE));
        control.search_anchor();
        dbg!(control.search_prev(SCFIND_MATCHCASE, "h".to_string()));
        dbg!(control.search_next(SCFIND_MATCHCASE, "o".to_string()));
        control.cut();
        control.copy();
        control.paste();
        control.clear();
        dbg!(control.can_paste());
        control.copy_allow_line();
        control.copy_range(1, 5);
        control.copy_text("hello".to_string());
        control.set_paste_convert_endings(true);
        assert_eq!(true, control.get_paste_convert_endings());
        dbg!(control);
    }
}
