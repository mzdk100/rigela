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

mod ext;
mod internal;

pub mod accessibility;
pub mod annotation;
pub mod autoc;
pub mod bidirectional;
pub mod cache;
pub mod caret;
pub mod character;
pub mod edge;
pub mod eol;
pub mod folding;
pub mod ime;
pub mod indentation;
pub mod indicator;
pub mod key;
pub mod margin;
pub mod marker;
pub mod order;
pub mod phases;
pub mod popup;
pub mod print;
pub mod selection;
pub mod space;
pub mod status;
pub mod style;
pub mod technology;
pub mod typing;
pub mod wrap;

pub use crate::scintilla::internal::*;
use crate::scintilla::{
    accessibility::Accessibility,
    annotation::Annotation,
    autoc::MultiAutoc,
    bidirectional::Bidirectional,
    cache::CacheMode,
    caret::CaretSticky,
    character::CharacterSet,
    edge::EdgeMode,
    eol::EolMode,
    folding::{FoldAction, FoldDisplayText},
    ime::Ime,
    indentation::IndentView,
    indicator::Indicator,
    key::KeyCode,
    margin::MarginOptions,
    marker::Mark,
    order::Order,
    phases::Phases,
    popup::PopUpMode,
    print::PrintMode,
    selection::SelectionMode,
    space::{TabDrawMode, WhiteSpace},
    status::Status,
    style::{Case, IdleStyling},
    technology::Technology,
    typing::PropertyType,
    wrap::{WrapIndent, WrapMode},
};
use win_wrap::control::edit::Edit;

/**
 * 搜索标志
 * 一些搜索例程使用标志选项，其中包括一个简单的正则表达式搜索。通过添加标志选项来组合它们：
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
 * [...] | 这表示一组字符，例如，`[abc]`表示字符a、b或c中的任何一个。您也可以使用范围，例如[a-z]表示任何小写字符。
 * [^...] | 集合中字符的补码。例如，[^A-Za-z]表示除字母字符以外的任何字符。
 * ^ | 这与一行的开头匹配（除非在集合中使用，请参见上文）。
 * $ | 这与一行的末尾相匹配。
 * * | 这匹配了0次或更多次。例如，Sa*m匹配Sm、Sam、Saam、Saam等等。
 * + | 这匹配1次或多次。例如，Sa+m匹配Sam、Saam、Saam等等。
 * 正则表达式将只匹配单行内的范围，而不会匹配多行。当使用SCFIND_CXX11REGEX时，可以使用更多的功能，通常类似于JavaScript中的正则表达式支持。有关支持内容的详细信息，请参阅C++运行时的文档。
 *
 * 后台保存
 * 想要在后台保存的应用程序应使用 SCI_SETREADONLY(1) 锁定文档以防止修改，并使用 SCI_GETCHARACTERPOINTER 检索指向统一文档内容的指针。锁定文档的缓冲区不会移动，因此指针在应用程序调用 SCI_SETREADONLY(0) 之前有效。如果用户在文档锁定时尝试执行修改，则会向应用程序发送 SCN_MODIFYATTEMPTRO 通知。然后，应用程序可能会决定忽略修改或终止后台保存线程并在从通知返回之前重新启用修改。
 *
 * 文档接口
 * 应用程序可能想要操作不可见的文档，临时的 IDocumentEditable 接口可用于此目的。 IDocumentEditable 允许更直接地访问功能，并且比调用 Scintilla API 更快。 IDocumentEditable 指针由 SCI_CREATEDOCUMENT、SCI_GETDOCPOINTER 和 ILoader::ConvertToDocument 返回。它们可以传递给 SCI_ADDREFDOCUMENT、SCI_RELEASEDOCUMENT 和 SCI_SETDOCPOINTER。
 * ```cpp
 * class IDocumentEditable {
 * public:
 *         // Allow this interface to add methods over time and discover whether new methods available.
 *         virtual int SCI_METHOD DEVersion() const noexcept = 0;
 *
 *         // Lifetime control
 *         virtual int SCI_METHOD AddRef() noexcept = 0;
 *         virtual int SCI_METHOD Release() = 0;
 * };
 * ```
 * IDocumentEditable 接口正在开发中，将来会添加更多方法。方法也可能会更改签名或被删除。因此该功能是临时的，用户应该知道他们可能必须修改客户端代码以响应这些更改。当 IDocumentEditable 是临时的时，DEVersion 将返回 0，而对于第一个稳定版本，它将返回 1。此后，当添加新方法时，它将递增。
 *
 * 词法分析器对象
 * 词法分析器被编程为实现 ILexer5 接口的对象，并通过 IDocument 接口与它们正在词法分析的文档进行交互。以前，词法分析器是通过提供词法分析和折叠函数来定义的，但创建一个对象来处理词法分析器与文档的交互允许词法分析器存储可在词法分析期间使用的状态信息。例如，C++ 词法分析器可以存储一组预处理器定义或变量声明，并根据其角色对其进行样式设置。
 * ILexer4 使用 ILexer5 接口进行了扩展，以支持使用 Lexilla。一组辅助类允许在 Scintilla 中使用由函数定义的旧词法分析器。
 * ```cpp
 * class ILexer4 {
 * public:
 *         virtual int SCI_METHOD Version() const = 0;
 *         virtual void SCI_METHOD Release() = 0;
 *         virtual const char * SCI_METHOD PropertyNames() = 0;
 *         virtual int SCI_METHOD PropertyType(const char *name) = 0;
 *         virtual const char * SCI_METHOD DescribeProperty(const char *name) = 0;
 *         virtual Sci_Position SCI_METHOD PropertySet(const char *key, const char *val) = 0;
 *         virtual const char * SCI_METHOD DescribeWordListSets() = 0;
 *         virtual Sci_Position SCI_METHOD WordListSet(int n, const char *wl) = 0;
 *         virtual void SCI_METHOD Lex(Sci_PositionU startPos, Sci_Position lengthDoc, int initStyle, IDocument *pAccess) = 0;
 *         virtual void SCI_METHOD Fold(Sci_PositionU startPos, Sci_Position lengthDoc, int initStyle, IDocument *pAccess) = 0;
 *         virtual void * SCI_METHOD PrivateCall(int operation, void *pointer) = 0;
 *         virtual int SCI_METHOD LineEndTypesSupported() = 0;
 *         virtual int SCI_METHOD AllocateSubStyles(int styleBase, int numberStyles) = 0;
 *         virtual int SCI_METHOD SubStylesStart(int styleBase) = 0;
 *         virtual int SCI_METHOD SubStylesLength(int styleBase) = 0;
 *         virtual int SCI_METHOD StyleFromSubStyle(int subStyle) = 0;
 *         virtual int SCI_METHOD PrimaryStyleFromStyle(int style) = 0;
 *         virtual void SCI_METHOD FreeSubStyles() = 0;
 *         virtual void SCI_METHOD SetIdentifiers(int style, const char *identifiers) = 0;
 *         virtual int SCI_METHOD DistanceToSecondaryStyles() = 0;
 *         virtual const char * SCI_METHOD GetSubStyleBases() = 0;
 *         virtual int SCI_METHOD NamedStyles() = 0;
 *         virtual const char * SCI_METHOD NameOfStyle(int style) = 0;
 *         virtual const char * SCI_METHOD TagsOfStyle(int style) = 0;
 *         virtual const char * SCI_METHOD DescriptionOfStyle(int style) = 0;
 * };
 * class ILexer5 : public ILexer4 {
 * public:
 *         virtual const char * SCI_METHOD GetName() = 0;
 *         virtual int SCI_METHOD  GetIdentifier() = 0;
 *         virtual const char * SCI_METHOD PropertyGet(const char *key) = 0;
 * };
 * ```
 * Sci_Position 和 Sci_PositionU 类型用于文档中的位置和行号。64 位版本将它们定义为 64 位类型，以允许大于 2 GB 的文档。将字符串返回为 const char * 的方法不需要无限期地维护单独的分配：词法分析器实现可能拥有一个缓冲区，该缓冲区可在每次调用时重复使用。调用者应立即复制返回的字符串。 PropertySet 和 WordListSet 的返回值用于指示更改是否需要对文档的任何部分执行词法分析或折叠。它是重新开始词法分析和折叠的位置，如果更改不需要对文档进行任何额外工作，则返回 -1。
 * 一种简单的方法是，如果更改有可能需要再次对文档进行词法分析，则返回 0，而优化可能是记住设置首先影响文档的位置并返回该位置。 Version 返回一个枚举值，指定实现哪个版本的接口：对于 ILexer5 为 lvRelease5，对于 ILexer4 为 lvRelease4。对于 Scintilla 版本 5.0 或更高版本，必须提供 ILexer5。调用 Release 来销毁词法分析器对象。
 * PrivateCall 允许应用程序和词法分析器之间直接通信。例如，应用程序维护一个包含有关系统头文件（如 Windows.h）的符号信息的单个大型数据结构，并将其提供给词法分析器，然后将其应用于每个文档。这避免了为每个文档构建系统头文件信息的成本。这是通过 SCI_PRIVATELEXERCALL API 调用的。
 * 使用需要折叠的确切范围调用 Fold。以前，使用从需要折叠的范围前一行开始的范围调用词法分析器，因为这样可以修复上次折叠的最后一行。新方法允许词法分析器决定是回溯还是更有效地处理这个问题。当请求的子样式多于可用子样式时，AllocateSubStyles 返回负数。 NamedStyles、NameOfStyle、TagsOfStyle 和 DescriptionOfStyle 用于提供有关此词法分析器使用的样式集的信息。
 * NameOfStyle 是 C 语言标识符，如“SCE_LUA_COMMENT”。TagsOfStyle 是一组以标准化方式描述样式的标签，如“文字字符串多行原始”。这里描述了一组常用标签和将它们组合在一起的约定。DescriptionOfStyle 是样式的英文描述，如“函数或方法名称定义”。可以调用 GetName 和 GetIdentifier 来发现词法分析器的身份，并用于实现 SCI_GETLEXERLANGUAGE 和 SCI_GETLEXER。可以调用 PropertyGet 来发现词法分析器存储的属性的值，并用于实现 SCI_GETPROPERTY。
 * ```cpp
 * class IDocument {
 * public:
 *         virtual int SCI_METHOD Version() const = 0;
 *         virtual void SCI_METHOD SetErrorStatus(int status) = 0;
 *         virtual Sci_Position SCI_METHOD Length() const = 0;
 *         virtual void SCI_METHOD GetCharRange(char *buffer, Sci_Position position, Sci_Position lengthRetrieve) const = 0;
 *         virtual char SCI_METHOD StyleAt(Sci_Position position) const = 0;
 *         virtual Sci_Position SCI_METHOD LineFromPosition(Sci_Position position) const = 0;
 *         virtual Sci_Position SCI_METHOD LineStart(Sci_Position line) const = 0;
 *         virtual int SCI_METHOD GetLevel(Sci_Position line) const = 0;
 *         virtual int SCI_METHOD SetLevel(Sci_Position line, int level) = 0;
 *         virtual int SCI_METHOD GetLineState(Sci_Position line) const = 0;
 *         virtual int SCI_METHOD SetLineState(Sci_Position line, int state) = 0;
 *         virtual void SCI_METHOD StartStyling(Sci_Position position) = 0;
 *         virtual bool SCI_METHOD SetStyleFor(Sci_Position length, char style) = 0;
 *         virtual bool SCI_METHOD SetStyles(Sci_Position length, const char *styles) = 0;
 *         virtual void SCI_METHOD DecorationSetCurrentIndicator(int indicator) = 0;
 *         virtual void SCI_METHOD DecorationFillRange(Sci_Position position, int value, Sci_Position fillLength) = 0;
 *         virtual void SCI_METHOD ChangeLexerState(Sci_Position start, Sci_Position end) = 0;
 *         virtual int SCI_METHOD CodePage() const = 0;
 *         virtual bool SCI_METHOD IsDBCSLeadByte(char ch) const = 0;
 *         virtual const char * SCI_METHOD BufferPointer() = 0;
 *         virtual int SCI_METHOD GetLineIndentation(Sci_Position line) = 0;
 *         virtual Sci_Position SCI_METHOD LineEnd(Sci_Position line) const = 0;
 *         virtual Sci_Position SCI_METHOD GetRelativePosition(Sci_Position positionStart, Sci_Position characterOffset) const = 0;
 *         virtual int SCI_METHOD GetCharacterAndWidth(Sci_Position position, Sci_Position *pWidth) const = 0;
 * };
 * ```
 * Scintilla 尝试将修改文本的后果降至最低，尽可能只重新排列并重新绘制更改的行。词法分析器对象包含自己的私有额外状态，这可能会影响后面的行。例如，如果 C++ 词法分析器使不活动的代码段变灰，则将语句 #define BEOS 0 更改为 #define BEOS 1 可能需要重新设置样式并重新显示文档的后面部分。
 * 词法分析器可以调用 ChangeLexerState 向文档发出信号，表示它应该重新排列并显示更多内容。对于 StartStyling，mask 参数无效。它在 3.4.2 及更早版本中使用。 SetErrorStatus 用于通知文档异常。不应在构建边界上抛出异常，因为两侧可能使用不同的编译器或不兼容的异常选项构建。
 * 为了允许词法分析器确定行的结束位置，从而更容易支持 Unicode 行结束符，IDocument 包含 LineEnd，应该使用它而不是测试特定的行结束符。 GetRelativePosition 按整个字符浏览文档，如果超出文档的开始和结束，则返回 INVALID_POSITION。 GetCharacterAndWidth 提供从 UTF-8 字节到 UTF-32 字符或从 DBCS 到 16 位值的标准转换。
 * 无效 UTF-8 中的字节单独报告为值 0xDC80+byteValue，这不是有效的 Unicode 代码点。如果调用者不需要知道字符中的字节数，则 pWidth 参数可以为 NULL。 ILexer5 和 IDocument 接口将来可能会扩展为扩展版本（ILexer6...）。Version 方法指示实现了哪个接口，从而可以调用哪些方法。
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

    /**
     * 撤消一个操作，或者如果撤消缓冲区已达到SCI_ENDUNDOACTION点，则所有操作都返回到相应的SCI_BEGINUNDOACTION。
     * */
    fn undo(&self);

    /**
     * 如果没有要撤消的内容，将返回false，如果有，则返回true。您通常会使用此消息的结果来启用/禁用“编辑”菜单的“撤消”命令。
     * */
    fn can_undo(&self) -> bool;

    /**
     * 取消最后一次SCI_UNDO操作的效果。
     * */
    fn redo(&self);

    /**
     * 如果没有要重做的操作，将返回false，如果有要重做的撤消操作，则返回true。您通常可以使用此消息的结果来启用/禁用“编辑”菜单“重做”命令。
     * */
    fn can_redo(&self) -> bool;

    /**
     * 此命令告诉Scintilla忘记任何保存的撤消或重做历史记录。它还将保存点设置为撤消缓冲区的起点，这样文档看起来就没有修改。这不会导致SCN_SAVEPOINTRACHED通知被发送到容器。
     * 另请参阅：SCI_SETSAVEPOINT
     * */
    fn empty_undo_buffer(&self);

    /**
     * 控制Scintilla是否收集撤消信息。传入true（1）以收集信息，传入false（0）以停止收集。如果停止收集，还应使用SCI_EMPTUNDOBUFFER来避免撤消缓冲区与缓冲区中的数据不同步。
     * 如果使用Scintilla存储程序生成的文本（日志视图）或在经常删除和重新生成文本的显示窗口中，则可能希望关闭保存撤消信息。
     * `collect_undo` 是否收集撤销记录。
     * */
    fn set_undo_collection(&self, collect_undo: bool);

    /**
     * 获取是否收集撤销信息。
     * 如果使用Scintilla存储程序生成的文本（日志视图）或在经常删除和重新生成文本的显示窗口中，则可能希望关闭保存撤消信息。
     * */
    fn get_undo_collection(&self) -> bool;

    /**
     * 以标记一组操作的开始，操作要全部撤消为一个操作，但必须生成为多个操作。或者，您可以使用begin_undo_action和end_undo_action标记一组操作，如果这些操作已撤消，则不希望与前面或后面的操作组合。
     * */
    fn begin_undo_action(&self);

    /**
     * 以标记一组操作的结束，操作要全部撤消为一个操作，但必须生成为多个操作。或者，您可以使用begin_undo_action和end_undo_action标记一组操作，如果这些操作已撤消，则不希望与前面或后面的操作组合。
     * */
    fn end_undo_action(&self);

    /**
     * 将自己的操作添加到撤消堆栈中，并且当需要撤消（SC_PERFORMED_UNDO）或重做（SC_PERFORMED_REDO）操作时，将向具有SC_MOD_CONTAINER标志的容器发送SCN_MODIFIED通知。提供的token参数在通知的token字段中返回。
     * 例如，如果容器希望允许撤消和重做“切换书签”命令，则可以在每次执行该命令时调用SCI_ADDUNDOACTION(line, 0)。然后，当它收到撤消或重做的通知时，它会在令牌字段给出的行上切换书签。如果有不同类型的命令或参数需要存储到撤消堆栈中，则容器应为文档维护自己的堆栈，并使用该堆栈中的当前位置作为SCI_ADDUNDOACTION的参数（line）。除非与SCI_BEGINUNDOACTION和SCI_ENDUNDOACTION组合在一起，否则SCI_ADDUNDOACTION命令不会组合成单个撤消事务。
     * 如果容器操作可以与任何插入和删除操作合并为单个复合操作，则flags参数可以是UNDO_MAY_COALESCE（1），否则为UNDO_NONE（0）。合并将可合并容器操作视为透明的，因此仍然只将看起来像键入的插入或看起来像多次使用Backspace或Delete键的删除分组在一起。
     * `token` 令牌。
     * `flags` 标志。
     * */
    fn add_undo_action(&self, token: i32, flags: u32);

    //noinspection StructuralWrap
    /**
     * 设置闪烁体视图中第一条可见行的行号。文档中的第一行编号为0。该值是可见行，而不是文档行。
     * `line` 行号。
     * */
    fn set_first_visible_line(&self, line: usize);

    /**
     * 获取闪烁体视图中第一条可见线的行号。文档中的第一行编号为0。该值是可见行，而不是文档行。
     * */
    fn get_first_visible_line(&self) -> usize;

    /**
     * 设置x偏移量。x_offset是文本视图开始处的水平滚动位置（以像素为单位）。值0是第一个文本列在视图左侧可见的正常位置。
     * `x_offset` x坐标。
     * */
    fn set_x_offset(&self, x_offset: i32);

    /**
     * 获取x偏移量。x_offset是文本视图开始处的水平滚动位置（以像素为单位）。值0是第一个文本列在视图左侧可见的正常位置。
     * */
    fn get_x_offset(&self) -> i32;

    //noinspection StructuralWrap
    /**
     * 这将尝试按指定的列数和行数滚动显示。正值会增加屏幕顶部的行号（即，它们会将文本向上移动到用户所关心的位置），负值则相反。
     * 列度量值是默认样式中空间的宽度。正值会增加视图左边缘的列（即，它们会将文本向左移动到用户所关心的位置）。负值则相反。
     * 另请参阅：SCI_SETXOFFSET
     * `columns` 要滚动的列数。
     * `lines` 要滚动的行数。
     * */
    fn line_scroll(&self, columns: usize, lines: usize);

    /**
     * 如果当前位置（如果没有选择，则为插入符号）不可见，则根据当前插入符号策略滚动视图使其可见。
     * */
    fn scroll_caret(&self);

    //noinspection StructuralWrap
    /**
     * 将自变量位置及其之间的范围滚动到视图中，优先考虑主要位置，然后是次要位置。该行为类似于SCI_SCROLLCARET，使用的是主位置而不是插入符号。然后努力确保二次位置和二次位置之间的范围也是可见的。这可以用于使搜索匹配可见。
     * `secondary` 次要位置。
     * `primary` 主要位置。
     * */
    fn scroll_range(&self, secondary: usize, primary: usize);

    /**
     * 设置插入符号策略。caretPolicy的值是CARET_SLOP、CARET_STRICT、CARET_JUMPS和CARET_EVEN的组合。
     * `caret_policy` 参数可以是下列值的组合：
     * CARET_SLOP | 如果设置，我们可以定义一个SLOP值：caretSlop。此值定义了一个不需要的区域（UZ），其中插入符号是…不需要的。此区域定义为垂直边距附近的像素数和水平边距附近的行数。通过使插入符号远离边缘，可以在其上下文中看到插入符号。这使得可以完全看到插入符号所在的标识符，并且可以看到当前行及其后面的一些行，这些行通常依赖于该行。
     * CARET_STRICT | 如果设置了CARET_SLOP设置的策略，则强制执行。。。严格地如果未设置插入符号Slop，则插入符号将居中显示；如果设置了插入符号Slopt，则不能进入UZ。
     * CARET_JUMPS | 如果设置，则会更有力地移动显示，以便在再次应用策略之前，插入符号可以在同一方向上移动更长时间。'3UZ’表示法用于将UZ的大小表示为到边缘的距离的三倍。
     * CARET_EVEN  | 如果未设置，则左侧和底部的UZ将分别向上延伸到右侧和顶部的UZ，而不是具有对称的UZ。通过这种方式，我们倾向于显示有用的信息：大多数代码所在的行的开头，以及插入符号之后的行，例如函数的正文。
     * `caret_slop` 可以是下列位标志：
     * slop | strict | jumps | even | Caret可以到达边缘 | 当达到极限（离开视线或进入UZ）时，显示。。。
     * 0 | 0 | 0 | 0  | 是 | 移动以将插入符号放在顶部/右侧
     * 0 | 0 | 0 | 1 | 是 | 移动了一个位置
     * 0 | 0 | 1 | 0 | 是 | 移动以将插入符号放在顶部/右侧
     * 0 | 0 | 1 | 1 | 是 | 以插入符号为中心
     * 0 | 1 | - | 0 | 光标始终位于显示的顶部/右侧 | -
     * 0 | 1 | - | 1 | 否，插入符号始终居中 | -
     * 1 | 0 | 0 | 0 | 是 | 移动以将插入符号移出不对称的UZ
     * 1 | 0 | 0 | 1 | 是 | 移动以将插入符号移出UZ
     * 1 | 0 | 1 | 0 | 是 | 移动到将插入符号放在上页边距或右页边距的3UZ处
     * 1 | 0 | 1 | 1 | 是 | 移动到将插入符号放在边距的3UZ处
     * 1 | 1 | - | 0 | Caret始终位于上页边距/右页边距的UZ处 | -
     * 1 | 1 | 0 | 1 | 否，避开UZ | 移动了一个位置
     * 1 | 1 | 1 | 0 | 否，避开UZ | 移动以将插入符号放在边距的3UZ处
     * */
    fn set_x_caret_policy(&self, caret_policy: u32, caret_slop: i32);

    /**
     * 设置插入符号策略。caretPolicy的值是CARET_SLOP、CARET_STRICT、CARET_JUMPS和CARET_EVEN的组合。
     * `caret_policy` 参数可以是下列值的组合：
     * CARET_SLOP | 如果设置，我们可以定义一个SLOP值：caretSlop。此值定义了一个不需要的区域（UZ），其中插入符号是…不需要的。此区域定义为垂直边距附近的像素数和水平边距附近的行数。通过使插入符号远离边缘，可以在其上下文中看到插入符号。这使得可以完全看到插入符号所在的标识符，并且可以看到当前行及其后面的一些行，这些行通常依赖于该行。
     * CARET_STRICT | 如果设置了CARET_SLOP设置的策略，则强制执行。。。严格地如果未设置插入符号Slop，则插入符号将居中显示；如果设置了插入符号Slopt，则不能进入UZ。
     * CARET_JUMPS | 如果设置，则会更有力地移动显示，以便在再次应用策略之前，插入符号可以在同一方向上移动更长时间。'3UZ’表示法用于将UZ的大小表示为到边缘的距离的三倍。
     * CARET_EVEN  | 如果未设置，则左侧和底部的UZ将分别向上延伸到右侧和顶部的UZ，而不是具有对称的UZ。通过这种方式，我们倾向于显示有用的信息：大多数代码所在的行的开头，以及插入符号之后的行，例如函数的正文。
     * `caret_slop` 可以是下列位标志：
     * slop | strict | jumps | even | Caret可以到达边缘 | 当达到极限（离开视线或进入UZ）时，显示。。。
     * 0 | 0 | 0 | 0  | 是 | 移动以将插入符号放在顶部/右侧
     * 0 | 0 | 0 | 1 | 是 | 移动了一个位置
     * 0 | 0 | 1 | 0 | 是 | 移动以将插入符号放在顶部/右侧
     * 0 | 0 | 1 | 1 | 是 | 以插入符号为中心
     * 0 | 1 | - | 0 | 光标始终位于显示的顶部/右侧 | -
     * 0 | 1 | - | 1 | 否，插入符号始终居中 | -
     * 1 | 0 | 0 | 0 | 是 | 移动以将插入符号移出不对称的UZ
     * 1 | 0 | 0 | 1 | 是 | 移动以将插入符号移出UZ
     * 1 | 0 | 1 | 0 | 是 | 移动到将插入符号放在上页边距或右页边距的3UZ处
     * 1 | 0 | 1 | 1 | 是 | 移动到将插入符号放在边距的3UZ处
     * 1 | 1 | - | 0 | Caret始终位于上页边距/右页边距的UZ处 | -
     * 1 | 1 | 0 | 1 | 否，避开UZ | 移动了一个位置
     * 1 | 1 | 1 | 0 | 否，避开UZ | 移动以将插入符号放在边距的3UZ处
     * */
    fn set_y_caret_policy(&self, caret_policy: u32, caret_slop: i32);

    /**
     * 这决定了在调用SCI_ENSUREVISIBLEENFORCEPOLICY时如何确定垂直定位。它接受visible_policy参数的VISIBLE_SLOP和VISIBLE_STRICT标志。它在操作上类似于SCI_SETYCARETPOLICY(int caret_policy, int caret_slop)。
     * `visible_policy` 可见策略。
     * `visible_slop` 位标志。
     * */
    fn set_visible_policy(&self, visible_policy: u32, visible_slop: i32);

    /**
     * 设置水平滚动条可见性。水平滚动条仅在假定宽度需要时才显示。如果您从不希望看到它，请调用SCI_SETHSCROLLBAR（0）。请使用SCI_SETHSCROLLBAR（1）再次启用它。SCI_GETHSCROLLBAR返回当前状态。默认状态是在需要时显示它。
     * 另请参见：SCI_SETSCROLLWIDTH。
     * `visible` 是否可见。
     * */
    fn set_h_scroll_bar(&self, visible: bool);

    /**
     * 获取水平滚动条可见性。水平滚动条仅在假定宽度需要时才显示。如果您从不希望看到它，请调用SCI_SETHSCROLLBAR（0）。请使用SCI_SETHSCROLLBAR（1）再次启用它。SCI_GETHSCROLLBAR返回当前状态。默认状态是在需要时显示它。
     * 另请参见：SCI_SETSCROLLWIDTH。
     * */
    fn get_h_scroll_bar(&self) -> bool;

    /**
     * 设置垂直滚动条可见性。默认情况下，在需要时始终显示垂直滚动条。您可以选择使用SCI_SETVSCROLLBAR隐藏或显示它，并使用SCI_GETVSCLOLLBAR获取当前状态。
     * 另请参阅：SCI_LINESCROLL
     * `visible` 是否可见。
     * */
    fn set_v_scroll_bar(&self, visible: bool);

    /**
     * 获取垂直滚动条可见性。默认情况下，在需要时始终显示垂直滚动条。您可以选择使用SCI_SETVSCROLLBAR隐藏或显示它，并使用SCI_GETVSCLOLLBAR获取当前状态。
     * 另请参阅：SCI_LINESCROLL
     * */
    fn get_v_scroll_bar(&self) -> bool;

    /**
     * 为了提高性能，Scintilla不会通过测量文档的显示宽度来确定水平滚动条的属性。而是使用假定的宽度。此消息设置Scintilla假设的文档宽度（以像素为单位）。默认值为2000。要确保当前可见行的宽度可以滚动，请使用SCI_SETSCROLLWIDTHTRACKING
     * `pixel_width` 宽度。
     * */
    fn set_scroll_width(&self, pixel_width: i32);

    /**
     * 为了提高性能，Scintilla不会通过测量文档的显示宽度来确定水平滚动条的属性。而是使用假定的宽度。此消息获取Scintilla假设的文档宽度（以像素为单位）。默认值为2000。要确保当前可见行的宽度可以滚动，请使用SCI_SETSCROLLWIDTHTRACKING
     * */
    fn get_scroll_width(&self) -> i32;

    //noinspection StructuralWrap
    /**
     * 设置滚动宽度跟踪。如果启用了滚动宽度跟踪，则会调整滚动宽度，以确保当前显示的所有行都可以完全滚动。此模式从不将滚动宽度调整得更窄。
     * `tracking` 是否跟踪。
     * */
    fn set_scroll_width_tracking(&self, tracking: bool);

    /**
     * 获取滚动宽度跟踪。如果启用了滚动宽度跟踪，则会调整滚动宽度，以确保当前显示的所有行都可以完全滚动。此模式从不将滚动宽度调整得更窄。
     * */
    fn get_scroll_width_tracking(&self) -> bool;

    /**
     * 设置滚动范围，使最大滚动位置的最后一行位于视图底部（默认值）。将其设置为false可在最后一行下方滚动一页。
     * `end_at_last_line` 最后一行是否为视图末尾。
     * */
    fn set_end_at_last_line(&self, end_at_last_line: bool);

    /**
     * 获取滚动范围，判断最大滚动位置的最后一行是否位于视图底部（默认值）。false表示可在最后一行下方滚动一页。
     * */
    fn get_end_at_last_line(&self) -> bool;

    //noinspection StructuralWrap
    /**
     * 设置空白显示模式。可以使空白可见，这对于空白很重要的语言（如Python）可能很有用。空格字符显示为居中的小圆点，制表符显示为指向右侧的轻箭头。还有一些方法可以控制行尾字符的显示。
     * `view_ws` 显示模式。
     * */
    fn set_view_ws(&self, view_ws: WhiteSpace);

    /**
     * 获取空白显示模式。空格字符显示为居中的小圆点，制表符显示为指向右侧的轻箭头。
     * */
    fn get_view_ws(&self) -> WhiteSpace;

    /**
     * 默认情况下，可见白色空间的颜色由使用中的lexer决定。所有可见白色空间的前景和/或背景颜色可以全局设置，用SC_ELEMENT_WHITE_SPACE和SC_ELELEMENT_WHITE_SPACE_BACK覆盖lexer的颜色。
     * SCI_SETWHITESPACEFORE和SCI_SETWHITESPACEBACK也会更改空白颜色，但元素API是首选，SC_ELEMENTWHITE_SPACE允许半透明。
     * `use_setting` 使用设置。
     * `fore` 前景颜色。
     * */
    fn set_white_space_fore(&self, use_setting: bool, fore: i32);

    /**
     * 默认情况下，可见白色空间的颜色由使用中的lexer决定。所有可见白色空间的前景和/或背景颜色可以全局设置，用SC_ELEMENT_WHITE_SPACE和SC_ELELEMENT_WHITE_SPACE_BACK覆盖lexer的颜色。
     * SCI_SETWHITESPACEFORE和SCI_SETWHITESPACEBACK也会更改空白颜色，但元素API是首选，SC_ELEMENTWHITE_SPACE允许半透明。
     * `use_setting` 使用设置。
     * `back` 背景颜色。
     * */
    fn set_white_space_back(&self, use_setting: bool, back: i32);

    //noinspection StructuralWrap
    /**
     * 设置用于标记空间字符的点的大小。值0是有效的，并且使点不可见。
     * `size` 大小值。
     * */
    fn set_white_space_size(&self, size: i32);

    /**
     * 查询当前空白字符大小。值0是有效的，并且使点不可见。
     * */
    fn get_white_space_size(&self) -> i32;

    //noinspection StructuralWrap
    /**
     * 设置空白时制表符的绘制方式。
     * `tab_draw_mode` 制表符绘制模式。
     * */
    fn set_tab_draw_mode(&self, tab_draw_mode: TabDrawMode);

    /**
     * 获取空白时制表符的绘制方式。
     * */
    fn get_tab_draw_mode(&self) -> TabDrawMode;

    //noinspection StructuralWrap
    /**
     * 将空格添加到最大上升（SCI_SETEXTRAASCENT），以允许线之间有更多的空间。这样做可以使文本更容易阅读或容纳下划线或高亮显示。
     * 文本是以“基线”上每个字符的底部绘制的。线的高度是从任何样式延伸到基线以上的最大值（其“上升”）加上任何样式延伸至基线以下的最大值，（其“下降”）得出的。
     * 额外的上升值可能是负值，但应小心操作，因为当线路共享空间时，可能会导致意外干扰。
     * `ascent` 上升值。
     * */
    fn set_extra_ascent(&self, ascent: i32);

    /**
     * 获取空格额外的上升值。
     * */
    fn get_extra_ascent(&self) -> i32;

    //noinspection StructuralWrap
    /**
     * 将空格添加到最大下降（SCI_SETEXTRADESCENT），以允许线之间有更多的空间。这样做可以使文本更容易阅读或容纳下划线或高亮显示。
     * 文本是以“基线”上每个字符的底部绘制的。线的高度是从任何样式延伸到基线以上的最大值（其“上升”）加上任何样式延伸至基线以下的最大值，（其“下降”）得出的。
     * 额外的下降值可能是负值，但应小心操作，因为当线路共享空间时，可能会导致意外干扰。
     * `descent` 下降值。
     * */
    fn set_extra_descent(&self, descent: i32);

    /**
     * 获取空格额外的下降值。
     * */
    fn get_extra_descent(&self) -> i32;

    /**
     * 设置光标类型。光标通常是以上下文敏感的方式选择的，因此在边距上的光标与在文本上的光标不同。执行慢速操作时，您可能希望更改为等待光标。
     * `cursor_type` 光标值在1到7之间，但只有SC_CURSORWAIT是有效可控的，其他值会导致显示指针。可以是：
     * SC_CURSORNORMAL | -1 | 显示正常光标。
     * SC_CURSORWAIT | 4 | 当鼠标位于Scintilla窗口上方或为其所有时，将显示等待光标。
     * */
    fn set_cursor(&self, cursor_type: u32);

    /**
     * 返回您设置的最后一个光标类型，如果您没有设置光标类型，则返回SC_CURSORNORMAL（-1）。
     * */
    fn get_cursor(&self) -> u32;

    /**
     * 设置鼠标按下捕获模式。当鼠标在闪烁体内部按下时，它会被捕获，以便将来的鼠标移动事件被发送到闪烁体。可以使用SCI_SETMOUSEDOWNCAPTURES（0）关闭此行为。
     * `captures` 是否捕获。
     * */
    fn set_mouse_down_captures(&self, captures: bool);

    /**
     * 获取鼠标按下捕获模式。当鼠标在闪烁体内部按下时，它会被捕获，以便将来的鼠标移动事件被发送到闪烁体。可以使用SCI_SETMOUSEDOWNCAPTURES（0）关闭此行为。
     * */
    fn get_mouse_down_captures(&self) -> bool;

    /**
     * 设置鼠标滚轮捕获模式。在Windows上，即使鼠标指针不在Scintilla编辑器窗口附近，如果Scintilla有焦点，它也会捕获所有WM_MOUSEWHEEL消息。可以使用SCI_SETMOUSEWHEELCAPTURES（0）更改此行为，以便Scintilla将WM_MOUSEWHEEL消息传递到其父窗口。如果鼠标指针位于编辑器窗口上方，闪烁体仍将对鼠标滚轮做出反应。
     * `captures` 是否捕获。
     * */
    fn set_mouse_wheel_captures(&self, captures: bool);

    /**
     * 获取鼠标滚轮捕获模式。在Windows上，即使鼠标指针不在Scintilla编辑器窗口附近，如果Scintilla有焦点，它也会捕获所有WM_MOUSEWHEEL消息。可以使用SCI_SETMOUSEWEHEELCAPTURES（0）更改此行为，以便Scintilla将WM_MOUSEWHEEL消息传递到其父窗口。如果鼠标指针位于编辑器窗口上方，闪烁体仍将对鼠标滚轮做出反应。
     * */
    fn get_mouse_wheel_captures(&self) -> bool;

    /**
     * 设置用户按Enter键时添加到文档中的字符。
     * `eol_mode` 换行模式。
     * */
    fn set_eol_mode(&self, eol_mode: EolMode);

    /**
     * 获取换行模式。
     * */
    fn get_eol_mode(&self) -> EolMode;

    /**
     * 更改文档中的所有行尾字符以匹配eol_mode。
     * `eol_mode` 换行模式。
     * */
    fn convert_eols(&self, eol_mode: EolMode);

    /**
     * 通常，行尾字符是隐藏的，但SCI_SETVIEWEOL允许您通过设置visible true（或false）来显示（或隐藏）它们。行尾字符的可见渲染类似于（CR）、（LF）或（CR）（LF）。SCI_GETVIEWEOL返回当前状态。
     * `visible` 是否显示。
     * */
    fn set_view_eol(&self, visible: bool);

    /**
     * 获取换行显示状态。
     * */
    fn get_view_eol(&self) -> bool;

    /**
     * 报告当前lexer支持的不同类型的行尾。这是一个位集，尽管目前只有SC_LINE_END_TYPE_DEFAULT（0）或SC_LINEEND_TYPE_UNICODE（1）的单一选择。这些值也被其他与Unicode行尾有关的消息使用。
     * */
    fn get_line_end_types_supported(&self) -> u32;

    /**
     * 默认情况下，仅解释ASCII行尾。可以使用SCI_SETLINEENDTYPESALLOWED(SC_LINE_END_TYPE_UNICODE)请求Unicode行结束，但除非lexer也允许您使用Unicode行结束否则这将无效。
     * SCI_GETLINEENDTYPESALLOWED返回当前状态。
     * `line_end_bit_set` 行尾类型位标志。
     * */
    fn set_line_end_types_allowed(&self, line_end_bit_set: u32);

    /**
     * 获取行尾类型位标志。
     * */
    fn get_line_end_types_allowed(&self) -> u32;

    /**
     * 报告当前由Scintilla解释的一组行结束。它是SCI_GETLINEENDTYPESSUPPORTED和SCI_GETLINEENDTYPESALLOWED。
     * */
    fn get_line_end_types_active(&self) -> u32;

    /**
     * 使用与闪烁体内部使用的单词定义相同的单词定义返回单词的开头。您可以使用SCI_SETWORDCHARS设置自己的字符列表，这些字符算作单词。位置设置开始或搜索，搜索结束时向前，搜索开始时向后。
     * `pos` 位置。
     * `only_word_characters` 仅单词字符，请参考[官方文档](https://www.scintilla.org/ScintillaDoc.html#SCI_WORDSTARTPOSITION)。
     * */
    fn word_start_position(&self, pos: usize, only_word_characters: bool) -> usize;

    /**
     * 使用与闪烁体内部使用的单词定义相同的单词定义返回单词的结尾。您可以使用SCI_SETWORDCHARS设置自己的字符列表，这些字符算作单词。位置设置开始或搜索，搜索结束时向前，搜索开始时向后。
     * `pos` 位置。
     * `only_word_characters` 仅单词字符，请参考[官方文档](https://www.scintilla.org/ScintillaDoc.html#SCI_WORDENDPOSITION)。
     * */
    fn word_end_position(&self, pos: usize, only_word_characters: bool) -> usize;

    //noinspection StructuralWrap
    /**
     * 范围是从一个单词或一组单词开始到结束？此消息检查开始是否在单词开始转换处，结束是否在单词结束转换处。它不检查范围内是否有空格。
     * `start` 开始点。
     * `end` 结束点。
     * */
    fn is_range_word(&self, start: usize, end: usize) -> bool;

    /**
     * 此消息定义哪些字符是单词类别的成员。在处理此函数之前，将字符类别设置为默认值。例如，如果不允许在字符集中使用“_”，请使用：
     * SCI_SETWORDCHARS(0, "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
     * `characters` 字符串。
     * */
    fn set_word_chars(&self, characters: String);

    /**
     * 获取单词中包含的所有字符。对于多字节编码，此API将不会返回0x80及以上的有意义的值。
     * */
    fn get_word_chars(&self) -> Option<String>;

    /**
     * 与SCI_SETWORDCHARS类似，此消息允许用户定义Scintilla将哪些字符视为空白字符。通过设置空白字符，用户可以微调Scintilla的行为，比如将光标移动到单词的开头或结尾；例如，通过将标点符号定义为空白，当用户按下ctrl+left或ctrl+right时，它们将被跳过。此函数应在SCI_SETWORDCHARS之后调用，因为它会将空白字符重置为默认集。
     * `characters` 字符串。
     * */
    fn set_white_space_chars(&self, characters: String);

    /**
     * 获取空白字符串。行为与SCI_GETWORDCHARS类似。
     * */
    fn get_white_space_chars(&self) -> Option<String>;

    /**
     * 与SCI_SETWORDCHARS和SCI_SETWHITESPACECHARS类似，此消息允许用户定义Scintilla将哪些字符视为标点符号。
     * */
    fn set_punctuation_chars(&self, characters: String);

    /**
     * 获取标点字符串。行为与SCI_GETWORDCHARS类似。
     * */
    fn get_punctuation_chars(&self) -> Option<String>;

    /**
     * 使用默认的单词和空白字符集。这将空白设置为空格、制表符和其他代码小于0x20的字符，单词字符设置为字母数字和“_”。
     * */
    fn set_chars_default(&self);

    /**
     * 单词左移。
     * */
    fn word_left(&self);

    /**
     * SCI_WORDLEFTEXTEND.
     * */
    fn word_left_extend(&self);

    /**
     * 单词右移。
     * */
    fn word_right(&self);

    /**
     * SCI_WORDRIGHTEXTEND.
     * */
    fn word_right_extend(&self);

    /**
     * 单词结束左移。
     * */
    fn word_left_end(&self);

    /**
     * SCI_WORDLEFTENDEXTEND.
     * */
    fn word_left_end_extend(&self);

    /**
     * 单词结束右移。
     * */
    fn word_right_end(&self);

    /**
     * SCI_WORDRIGHTENDEXTEND.
     * */
    fn word_right_end_extend(&self);

    /**
     * 在以大写字母（aCamelCaseIdentifier）或下划线（an_under_bar_ident）标记的词段之间左移。
     * */
    fn word_part_left(&self);

    /**
     * SCI_WORDPARTLEFTEXTEND.
     * */
    fn word_part_left_extend(&self);

    /**
     * 在以大写字母（aCamelCaseIdentifier）或下划线（an_under_bar_ident）标记的词段之间右移。
     * */
    fn word_part_right(&self);

    /**
     * SCI_WORDPARTRIGHTEXTEND.
     * */
    fn word_part_right_extend(&self);

    /**
     * SCI_DELWORDLEFT.
     * */
    fn del_word_left(&self);

    /**
     * SCI_DELWORDRIGHT.
     * */
    fn del_word_right(&self);

    /**
     * SCI_DELWORDRIGHTEND.
     * */
    fn del_word_right_end(&self);

    /**
     * Scintilla会记录最后一个可能被正确设置样式的字符。当设置其后面的字符的样式时，它会向前移动，如果更改其前面文档的文本，它会向后移动。在绘制文本之前，会检查此位置以查看是否需要任何样式，如果需要，则会向容器发送SCN_STYLENEDED通知消息。容器可以发送SCI_GETENDSTYLED来确定它需要在哪里开始设置样式。闪烁体总是要求整条线的样式。
     * */
    fn get_end_styled(&self) -> usize;

    /**
     * 这是通过将样式设置位置的起始位置设置为起始位置来准备样式设置的。在SCI_STARTSTYLING之后，为每个要样式化或发送的词法实体发送多条SCI_SETSTYLING消息。
     * `start` 开始点。
     * */
    fn start_styling(&self, start: usize);

    /**
     * 从样式位置开始设置长度字符的样式，然后按长度增加样式位置，为下一次调用做好准备。应在第一次调用之前调用SCI_STARTSTYLEING。
     * `length` 要设置样式的字符长度。
     * `style` 样式。
     * */
    fn set_styling(&self, length: usize, style: i32);

    /**
     * 作为对每个字节应用相同样式的SCI_SETSTYLING的替代方案，您可以使用此消息，该消息从样式位置指定每个长度字节的样式，然后按长度增加样式位置，为下一次调用做好准备。应在第一次调用之前调用SCI_STARTSTYLEING。
     * `style` 样式。
     * */
    fn set_styling_ex(&self, styles: &[u8]);

    //noinspection StructuralWrap
    /**
     * 设置空闲样式。
     * 由于换行还需要执行样式设置，并且还需要使用空闲时间，因此当文档显示为换行时，此设置不起作用。
     * `idle_styling` 样式。
     * */
    fn set_idle_styling(&self, idle_styling: IdleStyling);

    /**
     * 获取空闲样式。
     * */
    fn get_idle_styling(&self) -> IdleStyling;

    /**
     * 设置某行状态。除了为每个字符存储的8位词法状态外，还为每行存储一个整数。这可以用于寿命更长的解析状态，例如ASP页面中的当前脚本语言。使用SCI_SETLINESTATE设置整数值，使用SCI_GETLINESTATE获取值。更改该值会产生SC_MOD_CHANGELINESTATE通知。
     * `line` 行号。
     * `state` 状态值。
     * */
    fn set_line_state(&self, line: usize, state: i32);

    /**
     * 获取某行状态。
     * `line` 行号。
     * */
    fn get_line_state(&self, line: usize) -> i32;

    /**
     * 这将返回最后一行具有任何行状态的值。如果设置了任何行的状态，则总是为所有行进行分配的优化使这一点变得不那么有用。它仍然可以区分从未为任何行设置行状态的情况。
     * */
    fn get_max_line_state(&self) -> i32;

    /**
     * 将STYLE_DEFAULT重置为闪烁体初始化时的状态。
     * */
    fn style_reset_default(&self);

    /**
     * 此消息将所有样式设置为具有与STYLE_DEFAULT相同的属性。如果您正在为语法着色设置Scintilla，那么您设置的词汇样式很可能非常相似。设置样式的一种方法是：
     * 1.将STYLE_DEFAULT设置为所有样式的共同功能。
     * 2.使用SCI_STYLECLEARALL将其复制到所有样式。
     * 3.设置使你的词汇风格不同的风格属性。
     * */
    fn style_clear_all(&self);

    /**
     * 样式设置字体。
     * `style` 样式。
     * `font` 包含字体的名称。在Windows下，只使用名称的前32个字符，名称被解码为UTF-8，并且名称不区分大小写。对于内部缓存，Scintilla按名称跟踪字体，并关心字体名称的大小写，因此请保持一致。在GTK上，Pango用于显示文本，并且名称直接发送给Pango而不进行转换。在Qt上，名称被解码为UTF-8。在Cocoa上，这个名字被解码为MacRoman。
     * */
    fn style_set_font(&self, style: i32, font: String);

    /**
     * 样式获取字体。
     * `style` 样式。
     * */
    fn style_get_font(&self, style: i32) -> Option<String>;

    /**
     * 样式设置大小。
     * `style` 样式。
     * `size_points` 大小为整数个像素点。
     * */
    fn style_set_size(&self, style: i32, size_points: i32);

    /**
     * 样式获取大小。
     * `style` 样式。
     * */
    fn style_get_size(&self, style: i32) -> i32;

    /**
     * 样式设置大小。
     * `style` 样式。
     * `size_hundredth_points` 将大小乘以100（SC_FONT_SIZE_MULTIPLIER）以点的百分之一为单位设置小数点大小。例如，使用SCI_STYLESETSIZEFRACTIONAL（style，940）设置9.4像素点的文字大小。
     * */
    fn style_set_size_fractional(&self, style: i32, size_hundredth_points: i32);

    /**
     * 样式获取大小。
     * `style` 样式。
     * */
    fn style_get_size_fractional(&self, style: i32) -> i32;

    /**
     * 样式设置粗体。
     * `style` 样式。
     * `bold` 粗体。
     * */
    fn style_set_bold(&self, style: i32, bold: bool);

    /**
     * 样式获取粗体。
     * `style` 样式。
     * */
    fn style_get_bold(&self, style: i32) -> bool;

    /**
     * 样式设置权重(字体粗细）。
     * `style` 样式。
     * `weight` 介于1和999之间的数字，其中1非常轻，999非常重。虽然可以使用任何值，但字体通常只支持2到4个权重，其中三个权重足够常见，可以具有符号名称：SC_WEIGHT_NORMAL（400）、SC_WEIGHT_SEMIBOLD（600）和SC_WEIGHT_BOLD（700）。
     * */
    fn style_set_weight(&self, style: i32, weight: i32);

    //noinspection StructuralWrap
    /**
     * 样式获取权重（字体粗细）。
     * `style` 样式。
     * */
    fn style_get_weight(&self, style: i32) -> i32;

    /**
     * 样式设置斜体。
     * `style` 样式。
     * `italic` 斜体。
     * */
    fn style_set_italic(&self, style: i32, italic: bool);

    /**
     * 样式获取斜体。
     * `style` 样式。
     * */
    fn style_get_italic(&self, style: i32) -> bool;

    //noinspection StructuralWrap
    /**
     * 样式设置下划线。下划线是用前景色画的。样式包含下划线属性的所有字符都会加下划线，即使它们是空白字符。
     * `style` 样式。
     * `underline` 下划线。
     * */
    fn style_set_underline(&self, style: i32, underline: bool);

    /**
     * 样式获取下划线。
     * `style` 样式。
     * */
    fn style_get_underline(&self, style: i32) -> bool;

    //noinspection StructuralWrap
    /**
     * 样式设置前景色。文本以前景色绘制。
     * `style` 样式。
     * `fore` 前景色。
     * */
    fn style_set_fore(&self, style: i32, fore: i32);

    /**
     * 样式获取前景色。
     * `style` 样式。
     * */
    fn style_get_fore(&self, style: i32) -> i32;

    //noinspection StructuralWrap
    /**
     * 样式设置背景色。每个字符单元格中未被该字符占用的空间以背景色绘制。
     * `style` 样式。
     * `back` 背景色。
     * */
    fn style_set_back(&self, style: i32, back: i32);

    /**
     * 样式获取背景色。
     * `style` 样式。
     * */
    fn style_get_back(&self, style: i32) -> i32;

    /**
     * 样式设置行尾填充。如果行中的最后一个字符具有此属性集的样式，则该行直到窗口右边缘的其余部分将填充为最后一个角色设置的背景色。当文档包含另一种语言的嵌入部分时，例如具有嵌入JavaScript的HTML页面，这一点非常有用。通过将eol_filled设置为true，并为所有JavaScript样式设置一致的背景颜色（不同于为HTML样式设置的背景颜色），可以轻松地将JavaScript部分与HTML区分开来。
     * `style` 样式。
     * `eol_filled` 行尾填充。
     * */
    fn style_set_eol_filled(&self, style: i32, eol_filled: bool);

    /**
     * 样式获取行尾填充。
     * `style` 样式。
     * */
    fn style_get_eol_filled(&self, style: i32) -> bool;

    /**
     * 样式设置字符集。可以将样式设置为使用与默认字符集不同的字符集。这些字符集可能有用的地方是注释和文字字符串。例如，SCI_STYLESETCHARACTERSET(SCE_C_STRING, SC_CHARSET_RUSSIAN)将确保俄语字符串在C和C++中正确显示（SCE_C_STRING是C和C++lexer用于显示文字字符串的样式号；其值为6）。此功能在Windows和GTK上的工作方式不同。
     * SC_CHARSET_ANSI和SC_CHARSET_DEFAULT指定欧洲Windows代码页1252，除非设置了代码页。
     * `style` 样式。
     * `character_set` 字符集。
     * */
    fn style_set_character_set(&self, style: i32, charset_set: CharacterSet);

    /**
     * 样式获取字符集。
     * `style` 样式。
     * */
    fn style_get_character_set(&self, style: i32) -> CharacterSet;

    //noinspection StructuralWrap
    /**
     * 样式设置大小写形式。这不会更改存储的文本，只会更改文本的显示方式。
     * `style` 样式。
     * `case_visible` 显示方式。
     * */
    fn style_set_case(&self, style: i32, case_visible: Case);

    //noinspection StructuralWrap
    /**
     * 样式获取大小写形式。
     * `style` 样式。
     * */
    fn style_get_case(&self, style: i32) -> Case;

    /**
     * 样式设置可见性。文本通常可见。但是，您可以通过将可见设置为false的样式来完全隐藏它。这可以用来隐藏HTML或XML中嵌入的格式化指令或超文本关键字。用户操作可能无法删除不可见的文本，但应用程序可以通过调用SCI_DELETERANGE来删除不可见文本。
     * `style` 样式。
     * `visible` 是否可见。
     * */
    fn style_set_visible(&self, style: i32, visible: bool);

    /**
     * 样式获取可见性。
     * `style` 样式。
     * */
    fn style_get_visible(&self, style: i32) -> bool;

    /**
     * 样式设置可变性。这是一个实验性的、未完全实现的样式属性。默认设置是可变的，设置为true，但设置为false时，文本将变为只读。用户不能在不可更改的文本中移动插入符号，也不能删除不可更改文本。应用程序可以通过调用SCI_DELETERANGE来删除不可更改的文本。
     * `style` 样式。
     * `changeable` 是否可变。
     * */
    fn style_set_changeable(&self, style: i32, changeable: bool);

    /**
     * 样式获取可变性。
     * `style` 样式。
     * */
    fn style_get_changeable(&self, style: i32) -> bool;

    /**
     * 样式设置热点。此样式用于标记可以检测鼠标单击的文本范围。光标变为切换热点，前景和背景颜色可能会发生变化，并出现下划线以指示这些区域对点击敏感。这可以用于允许指向其他文档的超链接。
     * `style` 样式。
     * `hotspot` 热点。
     * */
    fn style_set_hotspot(&self, style: i32, hotspot: bool);

    /**
     * 样式获取热点。
     * `style` 样式。
     * */
    fn style_get_hotspot(&self, style: i32) -> bool;

    /**
     * 设置选区前景色。
     * `use_setting` 设置为true，则使用您提供的颜色。如果设置为false，则使用默认样式的着色，fore参数无效。
     * `fore` 前景色。
     * */
    fn set_sel_fore(&self, use_setting: bool, fore: i32);

    /**
     * 设置选区背景色。
     * `use_setting` 设置为true，则使用您提供的颜色。如果设置为false，则使用默认样式的着色，back参数无效。
     * `back` 背景色。
     * */
    fn set_sel_back(&self, use_setting: bool, back: i32);

    /**
     * 设置选区半透明。
     * `alpha` 半透明。
     * */
    fn set_sel_alpha(&self, alpha: i32);

    /**
     * 获取选区半透明。
     * */
    fn get_sel_alpha(&self) -> i32;

    //noinspection StructuralWrap
    /**
     * 通过设置此属性，可以将所选内容绘制到右侧边框。
     * `filled` 是否填充。
     * */
    fn set_sel_eol_filled(&self, filled: bool);

    /**
     * 获取所选内容是否绘制到右侧边框。
     * */
    fn get_sel_eol_filled(&self) -> bool;

    //noinspection StructuralWrap
    /**
     * 设置插入点的前景色。
     * `fore` 前景色。
     * */
    fn set_caret_fore(&self, fore: i32);

    /**
     * 获取插入点的前景色。
     * */
    fn get_caret_fore(&self) -> i32;

    //noinspection StructuralWrap
    /**
     * 设置插入点的行的显示。
     * `show` 是否显示。
     * */
    fn set_caret_line_visible(&self, show: bool);

    /**
     * 获取插入点的行的显示。
     * */
    fn get_caret_line_visible(&self) -> bool;

    /**
     * 设置包含插入点的行的背景颜色，然后使用SCI_SETACRETLINEVISIBLE(true)启用效果。您可以使用SCI_SETCARETLINEVISIBLE(false)取消效果。当线条具有会改变背景颜色的标记时，这种形式的背景颜色具有最高优先级。
     * `back` 背景色。
     * */
    fn set_caret_line_back(&self, back: i32);

    /**
     * 获取插入点的行的背景色。
     * */
    fn get_caret_line_back(&self) -> i32;

    //noinspection StructuralWrap
    /**
     * 设置插入点的行的半透明。插入点的行也可以半透明地绘制，这允许其他背景颜色显示出来。当alpha不是SC_ALPHA_NOALPHA时，插入点的行会在所有其他特征之后绘制，因此会影响所有其他特征的颜色。
     * `alpha` 半透明。
     * */
    fn set_caret_line_back_alpha(&self, alpha: i32);

    /**
     * 获取插入点的行的半透明。
     * */
    fn get_caret_line_back_alpha(&self) -> i32;

    //noinspection StructuralWrap
    /**
     * 设置显示插入点的行边框，而不是填充整个背景。设置width!=0可启用此选项，width=0可禁用此选项。
     * `width` 宽度。
     * */
    fn set_caret_line_frame(&self, width: i32);

    /**
     * 获取显示插入点的行边框。
     * */
    fn get_caret_line_frame(&self) -> i32;

    /**
     * 设置插入点的行的始终显示。选择使插入点的行始终可见，即使窗口不在焦点上。默认行为SCI_SETCARETLINEVISIBLEALWAYS(false)插入点的行仅在窗口聚焦时可见。
     * `always_visible` 是否始终显示。
     * */
    fn set_caret_line_visible_always(&self, always_visible: bool);

    /**
     * 获取插入点的行的始终显示。
     * */
    fn get_caret_line_visible_always(&self) -> bool;

    //noinspection StructuralWrap
    /**
     * 设置插入点闪烁的速率，它确定在更改状态之前插入符号可见或不可见的时间（以毫秒为单位）。将period设置为0将停止插入点闪烁。默认值为500毫秒。
     * `period_milliseconds` 频率。
     * */
    fn set_caret_period(&self, period_milliseconds: i32);

    /**
     * 获取插入点闪烁的速率。
     * */
    fn get_caret_period(&self) -> i32;

    /**
     * 设置插入点的样式。插入模式（低位4位，CARETSTYLE_INS_MASK）、重写模式（位4）和诅咒模式（位5）有不同的样式。
     * `caret_style` Caret样式
     * CARETSTYLE_INVISIBLE | 0 | 根本没有绘制插入点。
     * CARETSTYLE_LINE | 1 | 将插入点绘制为线条。这是默认设置。
     * CARETSTYLE_BLOCK | 2 | 将插入点绘制为块。
     * CARETSTYLE_OVERSTRICE_BAR | 0 | 将加粗插入点绘制为条形。这是默认设置。
     * CARETSTYLE_OVERSTRICE_BLOCK | 16 | 将加粗插入点绘制为块。这应该用前三种风格中的一种来探索。
     * CARETSTYLE_CURSES | 32 | 绘制无法在CURSES（终端）环境中绘制的插入点（如其他插入点），并将其绘制为块。主插入点由终端本身绘制。此设置通常是独立设置。
     * CARETSTYLE_BLOCK_AFTER | 256 | 当范围的插入点末尾在末尾并且选择了块插入点样式时，将块绘制在所选内容的外部而不是内部。这可以用CARETSTYLE_BLOCK或CARETSTYLE_CURSES进行搜索。
     * 块插入点成功地绘制了大多数组合字符和多字节字符序列，尽管当光标位于这些字符时，一些字体（如泰国字体（可能还有其他字体））有时会显得奇怪，这可能导致只绘制光标字符序列的一部分。这在Windows平台上最为显著。
     * */
    fn set_caret_style(&self, caret_style: u32);

    /**
     * 获取插入点样式。
     * */
    fn get_caret_style(&self) -> u32;

    /**
     * 将行插入点的宽度设置为0到20像素之间的值。默认宽度为1像素。宽度为0使插入点不可见，类似于将插入点样式设置为CARETSTYLE_INVISIBLE（尽管不能互换）。当光标样式设置为行插入点模式时，此设置仅影响光标的宽度，而不影响块插入点的宽度。
     * `pixel_width` 像素宽度。
     * */
    fn set_caret_width(&self, pixel_width: i32);

    /**
     * 获取插入点宽度。
     * */
    fn get_caret_width(&self) -> i32;

    /**
     * 设置插入点Sticky设置，该设置控制何时保存行上插入点的最后位置。
     * `use_caret_sticky_behaviour` 使用插入点粘性。
     * */
    fn set_caret_sticky(&self, use_caret_sticky_behaviour: CaretSticky);

    /**
     * 获取插入点Sticky设置。
     * */
    fn get_caret_sticky(&self) -> CaretSticky;

    /**
     * 切换插入点Sticky设置。从SC_CARETSTICKY_ON和SC_CARETSSTICKY_WHITESPACE切换到SC_CARETSCTICKY_OFF，并从SC_CARETSTICKY_OFF切换到SC_CARETSTICKY_ON。
     * */
    fn toggle_caret_sticky(&self);

    /**
     * 设置热点激活前景色。当光标悬停在设置了热点属性的样式中的文本上时，可以修改默认颜色，并使用这些设置绘制下划线。
     * `use_setting` 使用设置。
     * `fore` 前景色。
     * */
    fn set_hotspot_active_fore(&self, use_setting: bool, fore: i32);

    /**
     * 获取热点激活前景色。
     * */
    fn get_hotspot_active_fore(&self) -> i32;

    /**
     * 设置热点激活背景色。当光标悬停在设置了热点属性的样式中的文本上时，可以修改默认颜色，并使用这些设置绘制下划线。
     * `use_setting` 使用设置。
     * `back` 背景色。
     * */
    fn set_hotspot_active_back(&self, use_setting: bool, back: i32);

    /**
     * 获取热点激活背景色。
     * */
    fn get_hotspot_active_back(&self) -> i32;

    /**
     * 设置热点激活下划线。当光标悬停在设置了热点属性的样式中的文本上时，可以修改默认颜色，并使用这些设置绘制下划线。
     * `underline` 下划线。
     * */
    fn set_hotspot_active_underline(&self, underline: bool);

    /**
     * 获取热点激活下划线。
     * */
    fn get_hotspot_active_underline(&self) -> bool;

    /**
     * 设置热点单行模式。单行模式阻止热点换行到下一行。
     * `single_line` 单行模式。
     * */
    fn set_hotspot_single_line(&self, single_line: bool);

    /**
     * 获取热点单行模式。
     * */
    fn get_hotspot_single_line(&self) -> bool;

    /**
     * 设置字符表示。任何字符，包括那些通常显示为助记符的字符，都可以用倒圆矩形的Unicode字符串表示。
     * 例如，欧姆符号Ω U+2126看起来与希腊语Omega字符ΩU+03C9非常相似，因此，对于UTF-8编码，要将欧姆符号区分为"U+2126Ω"可以进行此调用：SCI_SETREPRESENTATION("\xe2\x84\xa6", "U+2126 \xe2\x64\xa6")
     * `encoded_character` 当前编码中一个字符的字符串。这不能用于设置多个字符串的表示形式。
     * `representation` 一个UTF-8字符串，最大长度为200字节。
     * 单字符限制的一个例外是，两个字符序列“\r\n”（回车+换行）可以具有在行结束查看（SCI_SETVIEWEOL）模式下可见的表示形式。如果没有“\r\n”的表示形式，则会看到单独的“\r\n”和“\n”表示形式。
     * NUL（0）字符是一种特殊情况，因为encoded_character参数以NUL结尾，所以NUL字符被指定为空字符串。
     * 对于UTF-8和DBCS代码页，单字节≥128的清晰表示可能会导致意外行为。
     * */
    fn set_representation(&self, encoded_character: String, representation: String);

    /**
     * 获取字符表示。
     * `encoded_character` 当前编码中一个字符的字符串。
     * */
    fn get_representation(&self, encoded_character: String) -> Option<String>;

    /**
     * 清除字符表示。
     * `encoded_character` 当前编码中一个字符的字符串。
     * */
    fn clear_representation(&self, encoded_character: String);

    //noinspection StructuralWrap
    /**
     * 设置控制字符。
     * `symbol` 助记符可以由具有在32到 255 范围内的ASCII码的指定符号代替。如果将符号值设置为小于32，则所有控制字符都显示为助记符。您设置的符号将以字符样式集的字体呈现。默认符号值为0。
     * */
    fn set_control_char_symbol(&self, symbol: i32);

    /**
     * 获取控制字符。
     * */
    fn get_control_char_symbol(&self) -> i32;

    /**
     * 分配页边距数量。
     * `margins` 边距数量。
     * */
    fn set_margins(&self, margins: i32);

    /**
     * 查找当前分配的页边距数量。
     * */
    fn get_margins(&self) -> i32;

    /**
     * 设置边距的类型。
     * `margin` 应为0、1、2、3或4。可以使用预定义的常数SC_MARGIN_SYMBOL（0）和SC_MARTIN_NUMBER（1）将边距设置为行号或符号边距。具有应用程序定义的文本的边距可以使用SC_MARGIN_TEXT（4）或SC_MARGIN_RTEXT（5）来右对齐文本。按照惯例，边距0用于行号，后面两个用于符号。您也可以使用常数SC_MARGIN_BACK（2）、SC_MARGIN_FORE（3）和SC_MARGIN_COLOUR（6）作为符号边距，将其背景颜色设置为与STYLE_DEFAULT背景和前景色或指定的颜色相匹配。
     * `margin_type` 边距类型。
     * */
    fn set_margin_type_n(&self, margin: u32, margin_type: i32);

    /**
     * 获取边距的类型。
     * `margin` 边距序号。
     * */
    fn get_margin_type_n(&self, margin: u32) -> i32;

    //noinspection StructuralWrap
    /**
     * 设置以像素为单位的边距宽度。宽度为零的边距是不可见的。默认情况下，Scintilla为宽度为16像素的符号设置边距1，所以如果您不确定什么是合适的，这是一个合理的猜测。行号边距宽度应考虑文档中的行数和行号样式。您可以使用SCI_TEXTWIDTH(STYLE_LINENUMBER,"_99999")来获得合适的宽度。
     * `margin` 边距序号。
     * `pixel_width` 像素宽度。
     * */
    fn set_margin_width_n(&self, margin: u32, pixel_width: i32);

    //noinspection StructuralWrap
    /**
     * 获取以像素为单位的边距宽度。
     * `margin` 边距序号。
     * */
    fn get_margin_width_n(&self, margin: u32) -> i32;

    /**
     * 设置边距标志。掩码为32位值。每个比特对应于32个逻辑符号中的一个，这些逻辑符号可以显示在为符号启用的空白中。有一个有用的常数SC_MASK_FOLDERS（0xFE000000或-3354432），它是用于表示折叠的7个逻辑符号的掩码。您可以为32个逻辑符号中的每一个分配广泛的符号和颜色，有关详细信息，请参阅标记。如果（mask&SC_MASK_FOLDERS）==0，页边空白背景颜色由样式33（STYLE_LINENUMBER）控制。
     * 您可以使用SCI_MARKERADD将逻辑标记添加到一行。如果一条线的相关标记没有出现在任何宽度为非零的边距的掩码中，则该标记会更改该线的背景色。例如，假设您决定使用逻辑标记10来标记有语法错误的行，并且您希望通过更改背景色来显示这些行。此标记的掩码向左移动1次（1<<10），即0x400。如果确保掩码中没有包含0x400的符号边距，则任何带有标记的行都会更改背景颜色。
     * 要设置非折叠边距1，请使用Scintilla默认设置的SCI_SETMARGINMASKN(1, ~SC_MASK_FOLDERS)。要设置折叠边距2，请使用SCI_SETMARGINMASKN(2, SC_MASK_FOLDERS)~SC_MASK_FOLDERS是十六进制的0x1FFFFFF或十进制的33554431。当然，您可能需要在空白处显示所有32个符号，在这种情况下，请使用SCI_SETMARGINMASKN(margin, -1)。
     * `margin` 边距序号。
     * `mask` 位标志。
     * */
    fn set_margin_mask_n(&self, margin: u32, mask: i32);

    /**
     * 获取边距标志。
     * `margin` 边距序号。
     * */
    fn get_margin_mask_n(&self, margin: u32) -> i32;

    /**
     * 设置边距鼠标单击敏感性。五个边距中的每一个都可以设置为对鼠标单击敏感或不敏感。在敏感页边距中单击会向容器发送SCN_MARGINCLICK或SCN_MARGINRIGHTCLICK通知。不敏感的页边距用作选择页边距，这样可以很容易地选择行的范围。默认情况下，所有边距都不敏感。
     * `margin` 边距序号。
     * `sensitive` 是否敏感。
     * */
    fn set_margin_sensitive_n(&self, margin: u32, sensitive: bool);

    //noinspection StructuralWrap
    /**
     * 获取边距鼠标单击敏感性。
     * `margin` 边距序号。
     * */
    fn get_margin_sensitive_n(&self, margin: u32) -> bool;

    /**
     * 设置边距光标。反向箭头光标通常显示在所有页边距上。这可以用SCI_SETMARGINCURSORN(margin, SC_CURSORARROW)更改为正常箭头，或用SCI_SETMARGINCURSORN(margin, SC _CURSORREVERSERROW)恢复为反向箭头。
     * `margin` 边距序号。
     * `cursor` 光标。
     * */
    fn set_margin_cursor_n(&self, margin: u32, cursor: u32);

    /**
     * 获取边距光标。
     * `margin` 边距序号。
     * */
    fn get_margin_cursor_n(&self, margin: u32) -> u32;

    /**
     * 设置边距背景色。SC_MARGIN_COLOUR类型的边距的颜色可以设置为SCI_SETMARGINBACKN。
     * `margin` 边距序号。
     * `back` 背景色。
     * */
    fn set_margin_back_n(&self, margin: u32, back: i32);

    /**
     * 获取边距背景色。
     * `margin` 边距序号。
     * */
    fn get_margin_back_n(&self, margin: u32) -> i32;

    //noinspection StructuralWrap
    /**
     * 设置文本左侧空白页边距的宽度（以像素为单位）。默认为一个像素。
     * `pixel_width` 像素宽度。
     * */
    fn set_margin_left(&self, pixel_width: i32);

    /**
     * 获取文本左侧空白页边距的宽度（以像素为单位）。
     * */
    fn get_margin_left(&self) -> i32;

    //noinspection StructuralWrap
    /**
     * 设置文本右侧空白页边距的宽度（以像素为单位）。默认为一个像素。
     * `pixel_width` 像素宽度。
     * */
    fn set_margin_right(&self, pixel_width: i32);

    /**
     * 获取文本右侧空白页边距的宽度（以像素为单位）。
     * */
    fn get_margin_right(&self) -> i32;

    /**
     * 更改折叠页边距显示的颜色。在Windows上，折叠边距颜色默认为：GetSysColor(COLOR_3DFACE)。
     * `use_setting` 使用设置。
     * `back` 背景色。
     * */
    fn set_fold_margin_colour(&self, use_setting: bool, back: i32);

    /**
     * 更改折叠页边距高亮显示的颜色。在Windows上，折叠边距高亮显示颜色为：GetSysColor(COLOR_3DHIGHLIGHT)。
     * `use_setting` 使用设置。
     * `fore` 前景色。
     * */
    fn set_fold_margin_hi_colour(&self, use_setting: bool, fore: i32);

    /**
     * 设置边距文字。文本边距是使用类型SC_MARGIN_TEXT或SC_MARTIN_RTEXT创建的。可以使用SCI_MARGINSETTEXT为每一行设置不同的字符串。设置文本边距将导致发送SC_MOD_CHANGEMARGIN通知。
     * 只有一些样式属性在文本页边距中处于活动状态：字体、大小分数、粗体、斜体、前、后和字符集。
     * `line` 行号。
     * `text` 要设置的文字。
     * */
    fn margin_set_text(&self, line: usize, text: String);

    /**
     * 获取边距文字。
     * `line` 行号。
     * */
    fn margin_get_text(&self, line: usize) -> Option<String>;

    /**
     * 设置边距样式。文本边距是使用类型SC_MARGIN_TEXT或SC_MARTIN_RTEXT创建的。可以使用 SCI_MARGINSETSTYLE 以特定样式显示一行上的整个文本边距，或者可以使用 SCI_MARGINSETSTYLES 单独设置每个字符的样式，它使用字节数组，每个字节设置相应文本字节的样式，类似于 SCI_SETSTYLINGEX。设置文本边距将导致发送SC_MOD_CHANGEMARGIN通知。
     * 只有一些样式属性在文本页边距中处于活动状态：字体、大小分数、粗体、斜体、前、后和字符集。
     * `line` 行号。
     * `style` 要设置的样式。
     * */
    fn margin_set_style(&self, line: usize, style: u32);

    /**
     * 获取边距样式。
     * `line` 行号。
     * */
    fn margin_get_style(&self, line: usize) -> u32;

    /**
     * 设置边距样式。文本边距是使用类型SC_MARGIN_TEXT或SC_MARTIN_RTEXT创建的。设置文本边距将导致发送SC_MOD_CHANGEMARGIN通知。
     * 只有一些样式属性在文本页边距中处于活动状态：字体、大小分数、粗体、斜体、前、后和字符集。
     * `line` 行号。
     * `styles` 要设置的样式。
     * */
    fn margin_set_styles(&self, line: usize, styles: &[u8]);

    /**
     * 获取边距样式。
     * `line` 行号。
     * */
    fn margin_get_styles(&self, line: usize) -> Vec<u8>;

    /**
     * 清除所有边距文字。
     * */
    fn margin_text_clear_all(&self);

    /**
     * 设置边距样式偏移量。通过设置样式偏移量，页边距样式可以与标准文本样式完全分离。例如，SCI_MARGINSETSTYLEOFFSET(256)将允许将裕度样式从256编号到511，这样它们就不会与lexer设置的样式重叠。在查找样式之前，使用SCI_MARGINSETSTYLE或SCI_MARGINSETSTYLES设置的每个样式编号都会添加偏移量。
     * 始终在SCI_MARGINSETSTYLEOFFSET之前调用SCI_ALLOCATEEXTENDEDSTYLES，并将结果用作SCI_MARGINSETSTYLEOFSET的参数。
     * `style` 样式。
     * */
    fn margin_set_style_offset(&self, style: i32);

    /**
     * 获取边距样式偏移量。
     * */
    fn margin_get_style_offset(&self) -> i32;

    /**
     * 设置边距选项。
     * `margin_options` 边距选项。
     * */
    fn set_margin_options(&self, margin_options: MarginOptions);

    /**
     * 获取边距选项。
     * */
    fn get_margin_options(&self) -> MarginOptions;

    /**
     * 设置批注文字。可以使用 SCI_ANNOTATIONSETTEXT 为每一行设置不同的字符串。要清除批注，请使用 NULL 指针调用 SCI_ANNOTATIONSETTEXT。设置批注将导致发送 SC_MOD_CHANGEANNOTATION 通知。
     * 批注中仅某些样式属性处于活动状态：font、size/sizeFractional、bold/weight、italics、fore、back 和 characterSet。
     * `line` 行号。
     * `text` 要设置的文字。
     * */
    fn annotation_set_text(&self, line: usize, text: Option<String>);

    /**
     * 获取批注文字。
     * `line` 行号。
     * */
    fn annotation_get_text(&self, line: usize) -> Option<String>;

    /**
     * 设置批注样式。可以使用 SCI_ANNOTATIONSETSTYLE 以特定样式显示一行上的整个文本 ANNOTATION，或者可以使用 SCI_ANNOTATIONSETSTYLES 单独设置每个字符的样式，它使用一个字节数组，每个字节设置相应文本字节的样式，类似于 SCI_SETSTYLINGEX。必须先设置文本，因为它指定批注的长度，从而要读取多少字节的样式。设置批注将导致发送 SC_MOD_CHANGEANNOTATION 通知。
     * 批注中仅某些样式属性处于活动状态：font、size/sizeFractional、bold/weight、italics、fore、back 和 characterSet。
     * `line` 行号。
     * `style` 要设置的样式。
     * */
    fn annotation_set_style(&self, line: usize, style: u32);

    /**
     * 获取批注样式。
     * `line` 行号。
     * */
    fn annotation_get_style(&self, line: usize) -> u32;

    /**
     * 设置批注样式。单独设置每个字符的样式，这使用一个字节数组，每个字节设置相应文本字节的样式，类似于 SCI_SETSTYLINGEX。必须先设置文本，因为它指定批注的长度，从而要读取多少字节的样式。设置批注将导致发送 SC_MOD_CHANGEANNOTATION 通知。
     * 批注中仅某些样式属性处于活动状态：font、size/sizeFractional、bold/weight、italics、fore、back 和 characterSet。
     * `line` 行号。
     * `styles` 要设置的样式。
     * */
    fn annotation_set_styles(&self, line: usize, styles: &[u8]);

    /**
     * 获取批注样式。
     * `line` 行号。
     * */
    fn annotation_get_styles(&self, line: usize) -> Vec<u8>;

    /**
     * 查询批注一行的行数。
     * 批注中仅某些样式属性处于活动状态：font、size/sizeFractional、bold/weight、italics、fore、back 和 characterSet。
     * `line` 行号。
     * */
    fn annotation_get_lines(&self, line: usize) -> i32;

    /**
     * 清除所有行的批注，这相当于清除每一行（设置为 0），然后删除用于此功能的其他内存。
     * 批注中仅某些样式属性处于活动状态：font、size/sizeFractional、bold/weight、italics、fore、back 和 characterSet。
     * */
    fn annotation_clear_all(&self);

    /**
     * 设置批注显示模式。
     * `visible` 显示模式。
     * */
    fn annotation_set_visible(&self, visible: Annotation);

    /**
     * 获取批注显示模式。
     * */
    fn annotation_get_visible(&self) -> Annotation;

    /**
     * 设置批注样式偏移量。通过设置样式偏移量，批注样式可以与标准文本样式完全分开。例如，SCI_ANNOTATIONSETSTYLEOFFSET(512) 允许批注样式的编号从 512 到 767，这样它们就不会与词法分析器设置的样式重叠（如果边距偏移量为 256，则不会与边距重叠）。使用 SCI_ANNOTATIONSETSTYLE 或 SCI_ANNOTATIONSETSTYLES 设置的每个样式编号在查找样式之前都会添加偏移量。
     * 始终在 SCI_ANNOTATIONSETSTYLEOFFSET 之前调用 SCI_ALLOCATEEXTENDEDSTYLES，并将结果用作 SCI_ANNOTATIONSETSTYLEOFFSET 的参数。
     * `style` 样式。
     * */
    fn annotation_set_style_offset(&self, style: i32);

    /**
     * 获取批注样式偏移量。
     * */
    fn annotation_get_style_offset(&self) -> i32;

    /**
     * 打开或关闭缓冲绘图。缓冲绘图将每条线绘制到位图中，而不是直接绘制到屏幕上，然后将位图复制到屏幕上。这样可以避免闪烁，尽管它确实需要更长的时间。默认情况下，绘图在Win32和GTK上缓冲，而在Cocoa和Qt上不缓冲。Cocoa不支持缓冲绘图。
     * 当前平台执行窗口缓冲，因此关闭此选项几乎总是更好的。对于Win32和GTK，客户端代码应该在初始化时关闭缓冲。在一些较旧的平台和不寻常的模式中，缓冲可能仍然有用。
     * `buffered` 是否缓冲。
     * */
    fn set_buffered_draw(&self, buffered: bool);

    /**
     * 查询缓冲绘图的状态。
     * */
    fn get_buffered_draw(&self) -> bool;

    /**
     * 设置绘图阶段。
     * `phases` 阶段。
     * */
    fn set_phases_draw(&self, phases: Phases);

    /**
     * 获取绘图阶段。
     * */
    fn get_phases_draw(&self) -> Phases;

    /**
     * 设置绘图技术。
     * `technology` 绘图技术。
     * */
    fn set_technology(&self, technology: Technology);

    /**
     * 获取绘图技术。
     * */
    fn get_technology(&self) -> Technology;

    /**
     * 设置字体质量（抗锯齿方法）。目前，以下值在Windows上可用：SC_EFF_QUALITY_DEFAULT（向后兼容）、SC_EFF_QUALITY_NON_ANTIALIASED、SC_EFF_QUALITY_ANTIALIASED和SC_EFF_QUALITY_LCD_OPTIMIZED。
     * 如果需要将更多选项压缩到该属性中，则只有SC_EFF_QUALITY_MASK（0xF）定义的有限数量的位将用于质量。
     * */
    fn set_font_quality(&self, font_quality: u32);

    /**
     * 获取字体质量。
     * */
    fn get_font_quality(&self) -> u32;

    /**
     * 设置代码页。Scintilla支持UTF-8、日语、中文和韩语DBCS以及Latin-1等单字节编码。UTF-8（SC_CP_UTF8）是默认值。使用此消息时，将codePage设置为代码页码，将Scintilla设置为使用代码页信息，以确保将多个字节字符视为一个字符而不是多个字符。这也会阻止插入符号在多字节字符的字节之间移动。不要使用此消息在不同的单字节字符集之间进行选择-请使用SCI_STYLESETCHARACTERSET。在codePage设置为零的情况下调用以禁用多字节支持。
     * 代码页SC_CP_UTF8（65001）将闪烁体设置为Unicode模式，文档被视为以UTF-8表示的字符序列。文本在被操作系统绘制之前被转换为平台的正常Unicode编码，因此可以显示希伯来语、阿拉伯语、西里尔语和汉族字符。可以在一个水平空间中使用两个垂直堆叠的字符的语言，如泰语，基本上可以使用，但也存在一些问题，即字符是分开绘制的，这会导致视觉问题。不支持双向文本。
     * 代码页可以设置为65001（UTF-8）、932（日语Shift-JIS）、936（简体中文GBK）、949（朝鲜统一韩文代码）、950（繁体中文Big5）或1361（朝鲜语Johab）。
     * `code_page` 代码页。
     * */
    fn set_code_page(&self, code_page: u32);

    /**
     * 获取代码页。
     * */
    fn get_code_page(&self) -> u32;

    /**
     * 设置输入法交互。
     * IME 交互 | Windows | GTK | Qt | macOS
     * 检索周围 | ✓ | ✓ | ✓ | ✓
     * 重新转换 | ✓ | ✓ | ✓ | ✓
     * 删除周围 | ✓ | ✓ | ✓ | ✓
     * `ime_interaction` 输入法交互。
     * */
    fn set_ime_interaction(&self, ime_interaction: Ime);

    /**
     * 获取输入法交互。
     * */
    fn get_ime_interaction(&self) -> Ime;

    /**
     * 设置文字方向。
     * `bidirectional` 双向模式。
     * */
    fn set_bidirectional(&self, bidirectional: Bidirectional);

    /**
     * 获取文字方向。
     * */
    fn get_bidirectional(&self) -> Bidirectional;

    /**
     * 抓住焦点。这在GTK上更需要，因为它的焦点处理比在Windows上更复杂。
     * */
    fn grab_focus(&self);

    /**
     * 设置内部焦点标志。这是由具有复杂焦点要求的客户端使用的，例如有自己的窗口来获得真正的焦点，但需要指示Scintilla具有逻辑焦点。
     * `focus` 焦点标志。
     * */
    fn set_focus(&self, focus: bool);

    /**
     * 获取内部焦点。
     * */
    fn get_focus(&self) -> bool;

    /**
     * 在“大括号突出显示样式”中最多可以突出显示两个字符，该样式定义为样式号STYLE_BRACELIGHT（34）。如果已启用缩进辅助线，则可能还希望突出显示与大括号对应的缩进。可以使用SCI_GETCOLUMN查找列，并使用SCI_SETHIGHLIGUITE高亮显示缩进。
     * `pos_a` 位置a。
     * `pos_b` 位置b。
     * */
    fn brace_highlight(&self, pos_a: usize, pos_b: usize);

    /**
     * 如果没有匹配的支撑，则可以使用支撑徽章样式STYLE_BRACEBAD（35）来显示不匹配的支撑。使用INVALID_POSITION（-1）的位置可移除高亮显示。
     * */
    fn brace_badlight(&self, pos: usize);

    /**
     * 使用指定的指示器高亮显示匹配的大括号，而不是更改其样式。
     * `use_setting` 使用设置。
     * `indicator` 指示器。
     * */
    fn brace_highlight_indicator(&self, use_setting: bool, indicator: i32);

    /**
     * 使用指定的指示器高亮显示不匹配的大括号，而不是更改其样式。
     * `use_setting` 使用设置。
     * `indicator` 指示器。
     * */
    fn brace_badlight_indicator(&self, use_setting: bool, indicator: i32);

    /**
     * 在给定位置（一个括号的位置）找到相应的匹配括号。所处理的括号字符为“(”，“)”，“[”，“]”，“{”，“}”、“<”和“>”。搜索从左括号向前，从右括号向后。如果位置处的字符不是括号字符，或者找不到匹配的括号，则返回值为-1。否则，返回值为匹配括号的位置。
     * 只有当匹配括号的样式与起始括号相同或匹配括号超出样式末尾时，才会发生匹配。正确处理嵌套括号。
     * `pos` 位置。
     * max_re_style` 当前必须为0，将来可能会用于限制括号搜索的长度。
     * */
    fn brace_match(&self, pos: usize, max_re_style: i32) -> usize;

    /**
     * 将制表符的大小设置为STYLE_DEFAULT中空格字符大小的倍数。默认制表符宽度为 8 个字符。制表符大小没有限制，但小于 1 的值或过大的值可能会产生不良影响。
     * `tab_width` 制表符宽度。
     * */
    fn set_tab_width(&self, tab_width: i32);

    /**
     * 获取制表符宽度。
     * */
    fn get_tab_width(&self) -> i32;

    /**
     * 清除行上的显式制表位。更改制表位会产生 SC_MOD_CHANGETABSTOPS 通知。
     * `line` 行号。
     * */
    fn clear_tabstops(&self, line: usize);

    /**
     * 在距左侧指定距离（以像素为单位）处添加显式制表位。更改制表位会产生 SC_MOD_CHANGETABSTOPS 通知。
     * `line` 行号。
     * `x` x坐标。
     * */
    fn add_tabstop(&self, line: usize, x: i32);

    /**
     * 获取在给定 x 位置之后设置的下一个显式制表位位置，如果没有，则获取零。更改制表位会产生 SC_MOD_CHANGETABSTOPS 通知。
     * `line` 行号。
     * `x` x坐标。
     * */
    fn get_next_tabstop(&self, line: usize, x: i32) -> i32;

    /**
     * 确定是否应从制表符和空格的混合物中创建缩进，还是纯粹基于空格。将use_tabs设置为false（0），以在空格中创建所有制表符和缩进。默认值为true。您可以使用SCI_GETCOLUMN来获取位置的列，将制表符的宽度考虑在内。
     * `use_tabs` 是否使用制表符。
     * */
    fn set_use_tabs(&self, use_tabs: bool);

    /**
     * 获取是否使用制表符。
     * */
    fn get_use_tabs(&self) -> bool;

    /**
     * 根据STYLE_DEFAULT中的空格宽度设置缩进的大小。如果将宽度设置为 0，则缩进大小与制表符大小相同。缩进大小没有限制，但小于 0 的值或较大的值可能会产生不良影响。
     * `indent_size` 缩进大小。
     * */
    fn set_indent(&self, indent_size: i32);

    /**
     * 获取缩进大小。
     * */
    fn get_indent(&self) -> i32;

    //noinspection StructuralWrap
    /**
     * 在缩进空白处，可以使制表符缩进，而不是插入制表符。
     * `tab_indents` 制表符缩进。
     * */
    fn set_tab_indents(&self, tab_indents: bool);

    /**
     * 获取制表符缩进。
     * */
    fn get_tab_indents(&self) -> bool;

    //noinspection StructuralWrap
    /**
     * 在缩进空白处，可以使退格键取消缩进，而不是删除字符。
     * `bs_un_indents` 退格取消缩进。
     * */
    fn set_backspace_un_indents(&self, bs_un_indents: bool);

    /**
     * 获取退格取消缩进。
     * */
    fn get_backspace_un_indents(&self) -> bool;

    //noinspection StructuralWrap
    /**
     * 设置行上的缩进量。缩进以字符列为单位进行测量，这对应于空格字符的宽度。
     * `line` 行号。
     * `indentation` 缩进。
     * */
    fn set_line_indentation(&self, line: usize, indentation: i32);

    //noinspection StructuralWrap
    /**
     * 获取行上的缩进量。缩进以字符列为单位进行测量，这对应于空格字符的宽度。
     * `line` 行号。
     * */
    fn get_line_indentation(&self, line: usize) -> i32;

    //noinspection StructuralWrap
    /**
     * 这将返回行缩进末尾的位置。
     * `line` 行号。
     * */
    fn get_line_indent_position(&self, line: usize) -> usize;

    /**
     * 设置缩进指南。
     * `indent_view` 缩进显示。
     * */
    fn set_indentation_guides(&self, indent_view: IndentView);

    /**
     * 获取缩进指南。
     * */
    fn get_indentation_guides(&self) -> IndentView;

    /**
     * 设置高亮指南。当出现括号高亮时，括号对应的缩进指南可能会使用括号高亮样式 STYLE_BRACELIGHT (34) 进行高亮。将 column 设置为 0 可取消此高亮。
     * `column` 位置。
     * */
    fn set_highlight_guide(&self, column: usize);

    /**
     * 获取高亮指南。
     * */
    fn get_highlight_guide(&self) -> usize;

    /**
     * 此消息将 0 到 31 范围内的标记号与其中一个标记符号或 ASCII 字符关联。
     * `marker_number` 标记序号。
     * `marker_symbol` 标记符号。
     * */
    fn marker_define(&self, marker_number: u32, marker_symbol: Mark);

    /**
     * 将标记设置为像素图。像素图使用 XPM 格式。像素图使用 SC_MARK_PIXMAP 标记符号。
     * `marker_number` 标记序号。
     * `pixmap` 标记图像。
     * */
    fn marker_define_pixmap(&self, marker_number: u32, pixmap: &[&str]);

    /**
     * 设置RGBA图像宽度。
     * `width` 宽度。
     * */
    fn rgba_image_set_width(&self, width: i32);

    /**
     * 设置RGBA图像高度。
     * `height` 高度。
     * */
    fn rgba_image_set_height(&self, height: i32);

    //noinspection StructuralWrap
    /**
     * 设置RGBA图像百分比比例因子。这在具有视网膜显示屏的macOS上很有用，其中每个显示单元为2个像素：使用200的系数，以便每个图像像素都使用屏​​幕像素显示。默认比例100将拉伸每个图像像素以覆盖视网膜显示屏上的4个屏幕像素。
     * `scale_percent` 百分比系数。
     * */
    fn rgba_image_set_scale(&self, scale_percent: i32);

    /**
     * 将标记设置为半透明像素图。像素图使用 RGBA 格式。必须先使用 SCI_RGBAIMAGESETWIDTH 和 SCI_RGBAIMAGESETHEIGHT 消息设置宽度和高度。像素图使用 SC_MARK_RGBAIMAGE 标记符号。
     * `marker_number` 标记序号。
     * `pixels` 像素。
     * */
    fn marker_define_rgba_image(&self, marker_number: u32, pixels: &[u8]);

    /**
     * 返回使用 SCI_MARKERDEFINE 或 SC_MARK_PIXMAP（如果使用 SCI_MARKERDEFINEPIXMAP 定义）或 SC_MARK_RGBAIMAGE（如果使用 SCI_MARKERDEFINERGBAIMAGE 定义）为 marker_number 定义的符号。
     * `marker_number` 标记序号。
     * */
    fn marker_symbol_defined(&self, marker_number: u32) -> Mark;

    /**
     * 设置标记序号的前景色。
     * `marker_number` 标记序号。
     * `fore` 前景色。
     * */
    fn marker_set_fore(&self, marker_number: u32, fore: i32);

    /**
     * 设置标记序号的背景色。
     * `marker_number` 标记序号。
     * `back` 背景色。
     * */
    fn marker_set_back(&self, marker_number: u32, back: i32);

    //noinspection StructuralWrap
    /**
     * 设置标记编号在选定其折叠块时的高亮背景颜色。默认颜色为#FF0000。
     * `marker_number` 标记序号。
     * */
    fn marker_set_back_selected(&self, marker_number: u32, back: i32);

    //noinspection StructuralWrap
    /**
     * 允许在选择高亮折叠块时启用/禁用它。（即包含插入符号的块）
     * `enabled` 是否启用。
     * */
    fn marker_enable_highlight(&self, enabled: bool);

    /**
     * 设置标记序号的半透明度。
     * `marker_number` 标记序号。
     * `alpha` 透明度。
     * */
    fn marker_set_alpha(&self, marker_number: u32, alpha: i32);

    /**
     * 将标记号 marker_number 添加到一行。如果此操作失败（非法行号、内存不足），则该消息返回-1，否则将返回标识已添加标记的标记句柄号。您可以将此返回的句柄与 SCI_MARKERLINEFROMHANDLE一起使用，以在移动或合并行后查找标记的位置，并与 SCI_MARKERDELETEHANDLE一起使用，以根据其句柄删除标记。该消息不会检查 marker_number 的值，也不会检查该行是否已包含标记。
     * `line` 行号。
     * `marker_number` 标记序号。
     * */
    fn marker_add(&self, line: usize, marker_number: u32) -> i32;

    /**
     * 可通过一次调用将一个或多个标记添加到一行，并以 SCI_MARKERGET 返回的相同“每个标记一位”32 位整数格式指定（并由基于掩码的标记搜索函数 SCI_MARKERNEXT 和 SCI_MARKERPREVIOUS 使用）。与 SCI_MARKERADD 一样，不会检查目标行上是否已存在任何标记。
     * `line` 行号。
     * `marker_set` 标记集合。
     * */
    fn marker_add_set(&self, line: usize, marker_set: i32);

    //noinspection StructuralWrap
    /**
     * 这将在给定的行号中搜索给定的标记号，如果存在则删除它。如果您多次将同一个标记添加到该行，则每次使用时都会删除一个副本。如果您传入的标记号为-1，则所有标记都会从该行中删除。
     * `line` 行号。
     * `marker_number` 标记序号。
     * */
    fn marker_delete(&self, line: usize, marker_number: u32);

    //noinspection StructuralWrap
    /**
     * 这将返回一个32位整数，指示该行上存在哪些标记。如果存在标记0，则设置位0；如果存在标记1，则设置位1，依此类推。
     * `line` 行号。
     * */
    fn marker_get(&self, line: usize) -> i32;

    /**
     * 这将从所有行中删除指定序号的标记。如果 marker_number 为 -1，它将从所有行中删除所有标记。
     * */
    fn marker_delete_all(&self, marker_number: u32);

    /**
     * 返回包含marker_mask中标记之一的第一行的行号，如果未找到标记，则返回 -1。可高效搜索包含给定标记集的行。搜索从行号 line_start 开始，向前继续到文件末尾(SCI_MARKERNEXT)。
     * `line_start` 开始行号。
     * `marker_mask` 应为要查找的每个标记设置一个位。设置位 0 可查找标记 0，设置位 1 可查找标记 1，依此类推。
     * */
    fn marker_next(&self, line_start: usize, marker_mask: i32) -> usize;

    /**
     * 返回包含marker_mask 中标记之一的第一行的行号，如果未找到标记，则返回 -1。可高效搜索包含给定标记集的行。搜索从行号 line_start 开始，向后继续到文件开头 (SCI_MARKERPREVIOUS)。
     * `line_start` 开始行号。
     * `marker_mask` 应为要查找的每个标记设置一个位。设置位 0 可查找标记 0，设置位 1 可查找标记 1，依此类推。
     * */
    fn marker_previous(&self, line_start: usize, marker_mask: i32) -> usize;

    //noinspection StructuralWrap
    /**
     * 此函数在文档中搜索具有句柄的标记，并返回包含该标记的行号，如果未找到则返回-1。
     * `marker_handle` 由SCI_MARKERADD 返回的标记的标识符。
     * */
    fn marker_line_from_handle(&self, marker_handle: i32) -> usize;
    /**
     * 此函数在文档中搜索具有句柄的标记，如果找到则删除该标记。
     * `marker_handle` 由SCI_MARKERADD 返回的标记的标识符。
     * */
    fn marker_delete_handle(&self, marker_handle: i32);

    /**
     * 设置指示符的样式。
     * `indicator` 指示符。
     * `indicator_style` 指示符样式。
     * */
    fn indic_set_style(&self, indicator: i32, indicator_style: Indicator);

    /**
     * 获取指示符的样式。
     * `indicator` 指示符。
     * */
    fn indic_get_style(&self, indicator: i32) -> Indicator;

    /**
     * 设置用于绘制指示符的颜色。默认指示器颜色相当于：
     * SCI_INDICSETFORE(0, 0x007f00); (dark green)
     * SCI_INDICSETFORE(1, 0xff0000); (light blue)
     * SCI_INDICSETFORE(2, 0x0000ff); (light red)
     * `indicator` 指示符。
     * `fore` 前景色。
     * */
    fn indic_set_fore(&self, indicator: i32, fore: i32);

    /**
     * 获取用于绘制指示符的颜色。默认指示器颜色相当于：
     * SCI_INDICSETFORE(0, 0x007f00); (dark green)
     * SCI_INDICSETFORE(1, 0xff0000); (light blue)
     * SCI_INDICSETFORE(2, 0x0000ff); (light red)
     * `indicator` 指示符。
     * */
    fn indic_get_fore(&self, indicator: i32) -> i32;

    /**
     * 设置用于绘制 INDIC_ROUNDBOX 和 INDIC_STRAIGHTBOX 矩形的填充颜色的 alpha 透明度。
     * `indicator` 指示符。
     * `alpha` 透明度，范围可以从 0（完全透明）到 255（不透明）。
     * */
    fn indic_set_alpha(&self, indicator: i32, alpha: i32);

    /**
     * 获取用于绘制 INDIC_ROUNDBOX 和 INDIC_STRAIGHTBOX 矩形的填充颜色的 alpha 透明度。
     * `indicator` 指示符。
     * */
    fn indic_get_alpha(&self, indicator: i32) -> i32;

    /**
     * 设置用于绘制 INDIC_ROUNDBOX 和 INDIC_STRAIGHTBOX 矩形轮廓颜色的 alpha 透明度。
     * `indicator` 指示符。
     * `alpha` 透明度，范围可以从 0（完全透明）到 255（不透明）。
     * */
    fn indic_set_outline_alpha(&self, indicator: i32, alpha: i32);

    /**
     * 获取用于绘制 INDIC_ROUNDBOX 和 INDIC_STRAIGHTBOX 矩形轮廓颜色的 alpha 透明度。
     * `indicator` 指示符。
     * */
    fn indic_get_outline_alpha(&self, indicator: i32) -> i32;

    //noinspection StructuralWrap
    /**
     * 设置指示符是绘制在文本下方还是上方（默认）。在文本下方绘制不适用于已弃用的单相绘制模式。
     * `indicator` 指示符。
     * `under` 下方。
     * */
    fn indic_set_under(&self, indicator: i32, under: bool);

    //noinspection StructuralWrap
    /**
     * 获取指示符是绘制在文本下方还是上方（默认）。在文本下方绘制不适用于已弃用的单相绘制模式。
     * `indicator` 指示符。
     * */
    fn indic_get_under(&self, indicator: i32) -> bool;

    /**
     * 设置当鼠标悬停在指示符上方或插入符号移入指示符时用于绘制指示符的样式。指示符以悬停样式绘制时，鼠标光标也会发生变化。默认情况下，悬停外观与正常外观相同，调用SCI_INDICSETFORE或SCI_INDICSETSTYLE 也会重置悬停属性。
     * `indicator` 指示符。
     * `indicator_style` 指示符样式。
     * */
    fn indic_set_hover_style(&self, indicator: i32, indicator_style: Indicator);

    /**
     * 获取当鼠标悬停在指示符上方或插入符号移入指示符时用于绘制指示符的样式。当指示符以悬停样式绘制时，鼠标光标也会发生变化。默认情况下，悬停外观与正常外观相同，调用SCI_INDICSETFORE或SCI_INDICSETSTYLE也会重置悬停属性。
     * `indicator` 指示符。
     * */
    fn indic_get_hover_style(&self, indicator: i32) -> Indicator;

    //noinspection StructuralWrap
    /**
     * 设置当鼠标悬停在指示符上方或插入符号移入指示符时用于绘制指示符的颜色。
     * `indicator` 指示符。
     * `fore` 前景色。
     * */
    fn indic_set_hover_fore(&self, indicator: i32, fore: i32);

    //noinspection StructuralWrap
    /**
     * 获取当鼠标悬停在指示符上方或插入符号移入指示符时用于绘制指示符的颜色。
     * `indicator` 指示符。
     * */
    fn indic_get_hover_fore(&self, indicator: i32) -> i32;

    /**
     * 设置与指示符关联的标志。当前定义了一个标志，SC_INDICFLAG_VALUEFORE：设置此标志时，指示符使用的颜色不是来自指示符的前端设置，而是来自文件中该点的指示符值。SC_INDICFLAG_NONE 是默认值。这允许为单个指示符显示多种颜色。该值是调用 SCI_SETINDICATORVALUE 时已与 SC_INDICVALUEBIT(0x1000000) 进行或运算的 RGB 整数颜色。要从值中找到颜色，请使用 SC_INDICVALUEMASK(0xFFFFFF) 的值。
     * `indicator` 指示符。
     * `flags` 位标志。
     * */
    fn indic_set_flags(&self, indicator: i32, flags: u32);

    /**
     * 获取与指示符关联的标志。
     * `indicator` 指示符。
     * */
    fn indic_get_flags(&self, indicator: i32) -> u32;

    /**
     * 设置将受 SCI_INDICATORFILLRANGE(position start,position length_fill)和 SCI_INDICATORCLEARRANGE(position start,position length_clear) 调用影响的指示符。
     * `indicator` 指示符。
     * */
    fn set_indicator_current(&self, indicator: i32);

    /**
     * 获取将受 SCI_INDICATORFILLRANGE(position start,position length_fill)和 SCI_INDICATORCLEARRANGE(position start,position length_clear) 调用影响的指示符。
     * */
    fn get_indicator_current(&self) -> i32;

    /**
     * 设置通过调用 SCI_INDICATORFILLRANGE 设置的值。
     * `value` 指示符的值。
     * */
    fn set_indicator_value(&self, value: i32);

    /**
     * 获取通过调用 SCI_INDICATORFILLRANGE 设置的值。
     * */
    fn get_indicator_value(&self) -> i32;

    /**
     * 填充当前指示符的范围。SCI_INDICATORFILLRANGE 用当前值填充。
     * `start` 开始点。
     * `length_fill` 要填充的长度。
     * */
    fn indicator_fill_range(&self, start: usize, length_fill: usize);

    /**
     * 清除当前指示符的范围。
     * `start` 开始点。
     * `length_clear` 要清除的长度。
     * */
    fn indicator_clear_range(&self, start: usize, length_clear: usize);

    //noinspection StructuralWrap
    /**
     * 查询表示某个位置上哪些指示符非零的位图值。结果中仅表示前32个指示符，因此不包括IME指示符。
     * `pos` 位置。
     * */
    fn indicator_all_on_for(&self, pos: usize) -> i32;

    /**
     * 查询某个位置的特定指示符的值。
     * `indicator` 指示符。
     * `pos` 位置。
     * */
    fn indicator_value_at(&self, indicator: i32, pos: usize) -> i32;

    //noinspection StructuralWrap
    /**
     * 从范围内的某个位置查找具有一个值的范围的起点。可用于遍历文档以发现所有指示符位置。
     * `indicator` 指示符。
     * `pos` 位置。
     * */
    fn indicator_start(&self, indicator: i32, pos: usize) -> usize;

    //noinspection StructuralWrap
    /**
     * 从范围内的某个位置查找具有一个值的范围的结束点。可用于遍历文档以发现所有指示符位置。
     * `indicator` 指示符。
     * `pos` 位置。
     * */
    fn indicator_end(&self, indicator: i32, pos: usize) -> usize;

    /**
     * 显示查找指示符。指示符在 SCI_FINDINDICATORSHOW 下保持可见。SCI_FINDINDICATORSHOW 的行为类似于 macOS TextEdit 和 Safari 应用程序，最适合编辑搜索目标通常是单词的文档。
     * `start` 开始点。
     * `end` 结束点。
     * */
    fn find_indicator_show(&self, start: usize, end: usize);

    /**
     * 动画显示查找指示符。指示符在 SCI_FINDINDICATORFLASH 下显示半秒后淡出。SCI_FINDINDICATORFLASH 与 Xcode 类似，适合编辑源代码，其中匹配通常位于运算符旁边，否则运算符会隐藏在指示符的填充下。
     * `start` 开始点。
     * `end` 结束点。
     * */
    fn find_indicator_flash(&self, start: usize, end: usize);

    /**
     * 隐藏了查找指示符。 Scintilla 的早期版本允许在样式编号和指示符之间划分样式字节，并提供了用于设置和查询此功能的 API。
     * */
    fn find_indicator_hide(&self);

    /**
     * 显示一个列表。初始分隔符是空格，但可以使用 SCI_AUTOCSETSEPARATOR 和 SCI_AUTOCGETSEPARATOR 设置或获取。使用默认设置，单词列表应按排序顺序排列。如果使用 SCI_AUTOCSETIGNORECASE 设置为忽略大小写模式，则字符串在转换为大写后进行匹配。这样做的一个结果是列表应按字母后的标点符号 '['、'\'、']'、'^'、'_' 和 '`' 排序。可以使用 SCI_AUTOCSETORDER 指定列表顺序的替代处理
     * `item_list` 用分隔符分隔的单词列表。
     * */
    fn autoc_show(&self, item_list: String);

    /**
     * 取消显示的任何自动完成列表。在自动完成模式下，当用户输入不能成为自动完成一部分的字符时，列表应该消失，例如在输入标识符时输入“.”、“(”或“[”。可以使用SCI_AUTOCSTOPS指定一组将取消自动完成的字符。
     * */
    fn autoc_cancel(&self);

    /**
     * 如果存在活动的自动完成列表，则此消息返回true，如果没有，则返回false。
     * */
    fn autoc_active(&self) -> bool;

    /**
     * 返回 SCI_AUTOCSHOW 开始显示列表时的当前位置的值。
     * */
    fn autoc_pos_start(&self) -> usize;

    /**
     * 触发自动完成。其效果与 Tab 键相同。
     * */
    fn autoc_complete(&self);

    //noinspection StructuralWrap
    /**
     * 设置停止的字符。
     * `character_set` 一个字符串，其中包含将自动取消自动完成列表的字符列表。启动编辑器时，此列表为空。
     * */
    fn autoc_stops(&self, character_set: String);

    /**
     * 设置 SCI_AUTOCSHOW 列表中用于分隔单词的分隔符。默认为空格字符。
     * `separator_character` 分隔字符。
     * */
    fn autoc_set_separator(&self, separator_character: u32);

    /**
     * 获取 SCI_AUTOCSHOW 列表中用于分隔单词的分隔符。
     * */
    fn autoc_get_separator(&self) -> u32;

    /**
     * 选择自动完成列表中的一项。它在单词列表中搜索第一个与select 匹配的单词。默认情况下，比较区分大小写，但您可以使用 SCI_AUTOCSETIGNORECASE 更改此设置。匹配是按 select 字符串的长度逐个字符进行的。也就是说，如果 select 是“Fred”，则如果这是列表中以“Fred”开头的第一项，它将匹配“Frederick”。如果找到一项，则选择它。如果未找到该项，则自动完成列表将关闭（如果自动隐藏为真）（请参阅 SCI_AUTOCSETAUTOHIDE）。
     * `select` 要选择项目的前缀文本。
     * */
    fn autoc_select(&self, select: String);

    /**
     * 查询当前选择索引。
     * */
    fn autoc_get_current(&self) -> i32;

    /**
     * 查询自动完成列表中当前选定的文本。通常使用 SCN_AUTOCSELECTION 通知。
     * */
    fn autoc_get_current_text(&self) -> Option<String>;

    //noinspection StructuralWrap
    /**
     * 默认行为是，如果插入点移动到列表显示时的位置，列表将被取消。通过以false参数调用此消息，列表不会被取消，直到插入点移动到单词完成前至少一个字符。
     * `cancel` 是否取消。
     * */
    fn autoc_set_cancel_at_start(&self, cancel: bool);

    /**
     * 获取插入点移动到列表开始处时是否取消。
     * */
    fn autoc_get_cancel_at_start(&self) -> bool;

    //noinspection StructuralWrap
    /**
     * 如果在自动完成列表处于活动状态时输入填充字符，则列表中当前选定的项目将添加到文档中，然后添加填充字符。常见的填充字符是“(”、“[”和“。”，但根据语言的不同，其他填充字符也可能存在。默认情况下，不设置填充字符。
     * `character_set` 字符集。
     * */
    fn autoc_set_fillups(&self, character_set: String);

    /**
     * 如果您使用SCI_AUTOCSETCHOOSESINGLE(1)并且列表只有一个项目，则会自动添加该项目并且不显示任何列表。默认情况下，即使只有一个项目也会显示列表。
     * `choose_single` 选择单项。
     * */
    fn autoc_set_choose_single(&self, choose_single: bool);

    /**
     * 获取只有一项时是否显示列表。
     * */
    fn autoc_get_choose_single(&self) -> bool;

    //noinspection StructuralWrap
    /**
     * 设置区分大小写。默认情况下，列表成员的字符匹配区分大小写。
     * `ignore_case` 是否区分大小写。
     * */
    fn autoc_set_ignore_case(&self, ignore_case: bool);

    /**
     * 获取区分大小写。
     * */
    fn autoc_get_ignore_case(&self) -> bool;

    /**
     * 当自动完成设置为忽略大小写 (SCI_AUTOCSETIGNORECASE) 时，默认情况下，它仍会选择第一个以区分大小写的方式与输入字符匹配的列表成员。这对应于行为属性 SC_CASEINSENSITIVEBEHAVIOUR_RESPECTCASE (0)。如果您希望自动完成完全忽略大小写，请选择 SC_CASEINSENSITIVEBEHAVIOUR_IGNORECASE (1)。
     * `behaviour` 大小写行为。
     * */
    fn autoc_set_case_in_sensitive_behaviour(&self, behaviour: u32);

    /**
     * 获取大小写敏感行为。
     * */
    fn autoc_get_case_in_sensitive_behaviour(&self) -> u32;

    /**
     * 当自动完成多个选择时，自动完成的文本可以仅进入主选择（SC_MULTIAUTOC_ONCE (0)）或进入每个选择（SC_MULTIAUTOC_EACH (1)。默认值为 SC_MULTIAUTOC_ONCE。
     * `multi` 多选模式。
     * */
    fn autoc_set_multi(&self, multi: MultiAutoc);

    /**
     * 获取多选模式。
     * */
    fn autoc_get_multi(&self) -> MultiAutoc;

    /**
     * 自动完成设置排序。默认设置 SC_ORDER_PRESORTED (0) 要求列表按字母顺序排列。 Scintilla 可以对列表进行排序，而不是使用 SC_ORDER_PERFORMSORT (1) 的应用程序。这将花费更多时间。希望优先考虑某些值并按优先级而不是字母顺序显示列表的应用程序可以使用 SC_ORDER_CUSTOM (2)。这需要在 SCI_AUTOCSHOW 中进行额外处理以创建排序索引。应在调用 SCI_AUTOCSHOW 之前设置顺序。
     * `order` 排序方式。
     * */
    fn autoc_set_order(&self, order: Order);

    /**
     * 自动完成获取排序。
     * */
    fn autoc_get_order(&self) -> Order;

    /**
     * 默认情况下，如果没有可行匹配项（用户输入的字符不再与列表条目匹配），列表将被取消。如果您想继续显示原始列表，请将auto_hide设置为 false。这也会影响 SCI_AUTOCSELECT。
     * `auto_hide` 是否自动隐藏。
     * */
    fn autoc_set_auto_hide(&self, auto_hide: bool);

    /**
     * 获取自动完成列表是否自动隐藏。
     * */
    fn autoc_get_auto_hide(&self) -> bool;

    /**
     * 当选择某项时，如果 drop_rest_of_word 设置为 true，则首先删除插入符号后面的所有单词字符。默认值为 false。
     * `drop_rest_of_word` 是否删除单词的剩余部分。
     * */
    fn autoc_set_drop_rest_of_word(&self, drop_rest_of_word: bool);

    /**
     * 获取是否删除单词的剩余部分。
     * */
    fn autoc_get_drop_rest_of_word(&self) -> bool;

    /**
     * 自动完成列表项可以显示图像和文本。每个图像首先用整数类型注册。然后，这个整数包含在列表的文本中，并用“?”与文本分隔。例如，“fclose?2 fopen”在字符串“fclose”之前显示图像 2，在“fopen”之前不显示图像。
     * 图像采用 XPM 格式 (SCI_REGISTERIMAGE)。
     * `type` 类型。
     * `xpm_data` 图像数据。
     * */
    fn register_image(&self, r#type: i32, xpm_data: &[&str]);

    /**
     * 自动完成列表项可以显示图像和文本。每个图像首先用整数类型注册。然后，这个整数包含在列表的文本中，并用“?”与文本分隔。例如，“fclose?2 fopen”在字符串“fclose”之前显示图像 2，在“fopen”之前不显示图像。
     * 图像采用RGBA 格式 (SCI_REGISTERRGBAIMAGE)。必须先使用 SCI_RGBAIMAGESETWIDTH 和 SCI_RGBAIMAGESETHEIGHT 消息设置宽度和高度。
     * `type` 类型。
     * `pixels` 像素数据。
     * */
    fn register_rgba_image(&self, r#type: i32, pixels: &[u8]);

    /**
     * 清除已注册图像集。
     * */
    fn clear_registered_images(&self);

    /**
     * 更改“?”分隔符。
     * `separator_character` 分隔字符。
     * */
    fn autoc_set_type_separator(&self, separator_character: u32);

    /**
     * 获取分隔字符。
     * */
    fn autoc_get_type_separator(&self) -> u32;

    //noinspection StructuralWrap
    /**
     * 设置自动完成列表中可见的最大行数。如果列表中有更多行，则会显示垂直滚动条。默认值为5。
     * `row_count` 行数。
     * */
    fn autoc_set_max_height(&self, row_count: i32);

    /**
     * 获取自动完成列表中可见的最大行数。如果列表中有更多行，则会显示垂直滚动条。
     * */
    fn autoc_get_max_height(&self) -> i32;

    //noinspection StructuralWrap
    /**
     * 设置自动完成列表的最大宽度，以完全可见的最长项目中的字符数表示。如果为零（默认值），则列表的宽度将计算为适合字符最多的项目。任何无法在可用宽度内完全显示的项目都以省略号表示。
     * `character_count` 字符数。
     * */
    fn autoc_set_max_width(&self, character_count: i32);

    /**
     * 获取自动完成列表的最大宽度，以完全可见的最长项目中的字符数表示。如果为零（默认值），则列表的宽度将计算为适合字符最多的项目。任何无法在可用宽度内完全显示的项目都以省略号表示。
     * */
    fn autoc_get_max_width(&self) -> i32;

    /**
     * 用户列表使用与自动完成列表相同的内部机制，并且列出的所有自动完成调用都适用于它们；您不能在自动完成列表处于活动状态时显示用户列表。它们在以下方面有所不同：
     * - SCI_AUTOCSETCHOOSESINGLE 消息无效。
     * - 当用户进行选择时，您会收到 SCN_USERLISTSELECTION 通知消息，而不是 SCN_AUTOCSELECTION。
     * 注意：如果您设置了填充字符或停止字符，这些字符仍将与用户列表一起处于活动状态，并且可能导致由于用户在编辑器中输入而选择项目或取消用户列表。
     * `list_type` 作为SCNotification结构的wParam字段返回到容器。它必须大于0，因为这是Scintilla区分自动完成列表和用户列表的方式。如果您有不同类型的列表，例如缓冲区列表和宏列表，则可以使用list_type来判断哪一个返回了选择。
     * `item_list` 用分隔符分隔的单词列表。
     * */
    fn user_list_show(&self, list_type: i32, item_list: String);

    //noinspection StructuralWrap
    /**
     * 显示调用提示窗口来启动该过程。如果调用提示已处于活动状态，则此消息不起作用。这里会记住插入点的位置，以便如果后续删除操作将插入点移动到该位置之前，则可以自动取消调用提示。
     * `pos` 是文档中对齐调用提示的位置。除非您在调用提示文本中包含向上和/或向下箭头，否则调用提示文本将对齐到此字符下方 1 行的开始位置，在这种情况下，提示将与最右侧箭头的右边缘对齐。假设您将以类似“\001 1 of 3 \002”的内容开始文本。
     * `definition` 调用提示文本。这可以包含由“\n”（换行符，ASCII代码10）字符分隔的多行。不要包含“\r”（回车符，ASCII代码13），因为这很可能会打印为空框。如果您使用SCI_CALLTIPUSESTYLE设置制表符大小，则支持“\t”（制表符，ASCII代码9）。
     * */
    fn call_tip_show(&self, pos: usize, definition: String);

    /**
     * 将取消显示的任何调用提示。如果您使用任何与编辑函数参数列表不兼容的键盘命令，Scintilla 也会为您取消调用提示。如果您删除触发提示时插入点所在的位置，调用提示将被取消。
     * */
    fn call_tip_cancel(&self);

    /**
     * 如果调用提示处于活动状态则返回true，如果未处于活动状态则返回false。
     * */
    fn call_tip_active(&self) -> bool;

    /**
     * 返回开始显示调用提示时的当前位置的值。
     * */
    fn call_tip_pos_start(&self) -> usize;

    //noinspection StructuralWrap
    /**
     * 返回开始显示调用提示时的当前位置的值。
     * `pos_start` 开始位置。
     * */
    fn call_tip_set_pos_start(&self, pos_start: usize);

    /**
     * 这将设置要以高亮样式显示的调用提示文本区域。如果需要，高亮可以延伸到行尾。未高亮的文本以中灰色绘制。选定的文本以深蓝色绘制。背景为白色。可以使用 SCI_CALLTIPSETBACK、SCI_CALLTIPSETFORE 和 SCI_CALLTIPSETFOREHLT 更改这些。
     * `highlight_start` 要高亮的第一个字符的字符串中的从零开始的索引。
     * `highlight_end` 高亮后的第一个字符的索引。highlight_end必须大于highlight_start；highlight_end-highlight_start是要高亮的字符数。
     * */
    fn call_tip_set_hlt(&self, highlight_start: usize, highlight_end: usize);

    //noinspection StructuralWrap
    /**
     * 设置调用提示的背景颜色；默认颜色为白色。将深色设置为背景不是一个好主意，因为普通调用提示文本的默认颜色是中灰色，而高亮文本的默认颜色是深蓝色。这还会设置STYLE_CALLTIP的背景颜色。
     * `back` 背景色。
     * */
    fn call_tip_set_back(&self, back: i32);

    /**
     * 设置调用提示文本的颜色；默认颜色是中灰的。这也设置了style_calltip的前景颜色。
     * `fore` 前景色。
     * */
    fn call_tip_set_fore(&self, fore: i32);

    //noinspection StructuralWrap
    /**
     * 设置高亮的调用提示文本的颜色；默认颜色为深蓝色。
     * `fore` 前景色。
     * */
    fn call_tip_set_fore_hlt(&self, fore: i32);

    /**
     * 将用于调用提示的样式从STYLE_DEFAULT更改为 STYLE_CALLTIP，并设置屏幕像素中的制表符大小。如果tab_size 小于 1，则不会对制表符进行特殊处理。一旦使用此调用，调用提示的前景色和背景色也将从样式中获取。
     * `tab_size` 制表符大小。
     * */
    fn call_tip_use_style(&self, tab_size: i32);

    /**
     * 默认情况下，calltip显示在文本下方，将 above 设置为 true (1) 将使其显示在文本上方。
     * `above` 是否显示在文本上方。
     * */
    fn call_tip_set_position(&self, above: bool);

    /**
     * 下一行。
     * */
    fn line_down(&self);

    /**
     * 下一行扩展选区。
     * */
    fn line_down_extend(&self);

    /**
     * 下一行扩展矩形选区。
     * */
    fn line_down_rect_extend(&self);

    /**
     * 向下滚动一行。
     * */
    fn line_scroll_down(&self);

    /**
     * 上一行。
     * */
    fn line_up(&self);

    /**
     * 上一行扩展选区。
     * */
    fn line_up_extend(&self);

    /**
     * 上一行扩展矩形选区。
     * */
    fn line_up_rect_extend(&self);

    /**
     * 向上滚动一行。
     * */
    fn line_scroll_up(&self);

    /**
     * 下一个段落。
     * */
    fn para_down(&self);

    /**
     * 下一个段落扩展选区。
     * */
    fn para_down_extend(&self);

    /**
     * 上一个段落。
     * */
    fn para_up(&self);

    /**
     * 上一个段落扩展选区。
     * */
    fn para_up_extend(&self);

    /**
     * 向左移动字符。
     * */
    fn char_left(&self);

    /**
     * 向左移动字符扩展选区。
     * */
    fn char_left_extend(&self);

    /**
     * 向左移动字符扩展矩形选区。
     * */
    fn char_left_rect_extend(&self);

    /**
     * 向右移动字符。
     * */
    fn char_right(&self);

    /**
     * 向右移动字符扩展选区。
     * */
    fn char_right_extend(&self);

    /**
     * 向右移动字符扩展矩形选区。
     * */
    fn char_right_rect_extend(&self);

    /**
     * 将插入点移动至行首。
     * */
    fn home(&self);

    /**
     * 将插入点移动至行首并扩展选区。
     * */
    fn home_extend(&self);

    /**
     * 将插入点移动至行首并扩展矩形选区。
     * */
    fn home_rect_extend(&self);

    /**
     * 在换行模式下移动到显示行的开头，这与普通的 SCI_HOME 命令移动到文档行的开头不同。
     * */
    fn home_display(&self);

    /**
     * 在换行模式下移动到显示行的开头并扩展选区，这与普通的 SCI_HOME 命令移动到文档行的开头不同。
     * */
    fn home_display_extend(&self);

    /**
     * 与其同名的SCI_HOME*类似，只是在启用自动换行时它们的行为有所不同：它们首先转到显示行的开始，就像SCI_HOMEDISPLAY*一样，但如果光标已经在该点，它会继续转到文档行的开始，以适合SCI_HOME*。
     * */
    fn home_wrap(&self);

    /**
     * 与其同名的SCI_HOME*类似，只是在启用自动换行时它们的行为有所不同：它们首先转到显示行的开始，就像SCI_HOMEDISPLAY*一样，但如果光标已经在该点，它会继续转到文档行的开始，以适合SCI_HOME*，此命令会扩展选区。
     * */
    fn home_wrap_extend(&self);

    /**
     * 将插入点移动至行的第一个非空白字符（即缩进之后），除非它已经在那里；在这种情况下，它充当 SCI_HOME*。
     * */
    fn vc_home(&self);

    /**
     * 将插入点移动至行的第一个非空白字符（即缩进之后）并扩展选区，除非它已经在那里；在这种情况下，它充当 SCI_HOME*。
     * */
    fn vc_home_extend(&self);

    /**
     * 将插入点移动至行的第一个非空白字符（即缩进之后）并扩展矩形选区，除非它已经在那里；在这种情况下，它充当 SCI_HOME*。
     * */
    fn vc_home_rectextend(&self);

    /**
     * 与其同名的SCI_VCHOME*类似，只是在启用自动换行时它们的行为有所不同：它们首先转到显示行的开始，就像SCI_HOMEDISPLAY*一样，但如果光标已经在该点，它会继续转到文档行的开始，以适合SCI_VCHOME*。
     * */
    fn vc_home_wrap(&self);

    /**
     * 与其同名的SCI_VCHOME*类似，只是在启用自动换行时它们的行为有所不同：它们首先转到显示行的开始，就像SCI_HOMEDISPLAY*一样，但如果光标已经在该点，它会继续转到文档行的开始，以适合SCI_VCHOME*，此命令会扩展选区。
     * */
    fn vc_home_wrap_extend(&self);

    /**
     * 在换行模式下移动到显示行的开头，这与普通的 SCI_VCHOME 命令移动到文档行的开头不同。
     * */
    fn vc_home_display(&self);

    /**
     * 在换行模式下移动到显示行的开头并扩展选区，这与普通的 SCI_VCHOME 命令移动到文档行的开头不同。
     * */
    fn vc_home_display_extend(&self);

    /**
     * 将插入点移动至行尾。
     * */
    fn line_end(&self);

    /**
     * 将插入点移动至行尾扩展选区。
     * */
    fn line_end_extend(&self);

    /**
     * 将插入点移动至行尾扩展矩形选区。
     * */
    fn line_end_rect_extend(&self);

    /**
     * 在换行模式下移动到显示行的结尾，这与普通的 SCI_LINEEND 命令移动到文档行的结尾不同。
     * */
    fn line_end_display(&self);

    /**
     * 在换行模式下移动到显示行的结尾并扩展选区，这与普通的 SCI_LINEEND 命令移动到文档行的结尾不同。
     * */
    fn line_end_display_extend(&self);

    /**
     * 与其同名的SCI_LINEEND*类似，只是在启用自动换行时它们的行为有所不同：它们首先转到显示行的结束，就像SCI_LINEENDDISPLAY*一样，但如果光标已经在该点，它会继续转到文档行的结束，以适合 SCI_LINEEND*。
     * */
    fn line_end_wrap(&self);

    /**
     * 与其同名的SCI_LINEEND*类似，只是在启用自动换行时它们的行为有所不同：它们首先转到显示行的结束，就像SCI_LINEENDDISPLAY*一样，但如果光标已经在该点，它会继续转到文档行的结束，以适合SCI_LINEEND*，此命令会扩展选区。
     * */
    fn line_end_wrap_extend(&self);

    /**
     * 移动到文档开始。
     * */
    fn document_start(&self);

    /**
     * 移动到文档开始扩展选区。
     * */
    fn document_start_extend(&self);

    /**
     * 移动到文档结束。
     * */
    fn document_end(&self);

    /**
     * 移动到文档结束扩展选区。
     * */
    fn document_end_extend(&self);

    /**
     * 向上翻页。
     * */
    fn page_up(&self);

    /**
     * 向上翻页扩展选区。
     * */
    fn page_up_extend(&self);

    /**
     * 向上翻页扩展矩形选区。
     * */
    fn page_up_rect_extend(&self);

    /**
     * 向下翻页。
     * */
    fn page_down(&self);

    /**
     * 向下翻页扩展选区。
     * */
    fn page_down_extend(&self);

    /**
     * 向下翻页扩展矩形选区。
     * */
    fn page_down_rect_extend(&self);

    /**
     * 缓慢向上翻页。
     * */
    fn stuttered_page_up(&self);

    /**
     * 缓慢向上翻页扩展选区。
     * */
    fn stuttered_page_up_extend(&self);

    /**
     * 缓慢向下翻页。
     * */
    fn stuttered_page_down(&self);

    /**
     * 缓慢向下翻页扩展选区。
     * */
    fn stuttered_page_down_extend(&self);

    /**
     * 向后删除。
     * */
    fn delete_back(&self);

    /**
     * 向后删除非空行。
     * */
    fn delete_back_not_line(&self);

    /**
     * 删除行左侧。
     * */
    fn del_line_left(&self);

    /**
     * 删除行右侧。
     * */
    fn del_line_right(&self);

    /**
     * 行删除。
     * */
    fn line_delete(&self);

    /**
     * 行剪切。
     * */
    fn line_cut(&self);

    /**
     * 行复制。
     * */
    fn line_copy(&self);

    /**
     * 行逆序。
     * */
    fn line_transpose(&self);

    /**
     * 行颠倒。
     * */
    fn line_reverse(&self);

    /**
     * 行副本。
     * */
    fn line_duplicate(&self);

    /**
     * 小写。
     * */
    fn lower_case(&self);

    /**
     * 大写。
     * */
    fn upper_case(&self);

    /**
     * 取消自动完成和调用提示显示并删除任何其他选择。
     * */
    fn cancel(&self);

    /**
     * 编辑切换改写。
     * */
    fn edit_toggle_overtype(&self);

    /**
     * 新行。
     * */
    fn new_line(&self);

    /**
     * 表单填充。
     * */
    fn form_feed(&self);

    /**
     * tab。
     * */
    fn tab(&self);

    /**
     * 反向tab。
     * */
    fn back_tab(&self);

    /**
     * 选区副本。
     * */
    fn selection_duplicate(&self);

    /**
     * 垂直中心插入点。
     * */
    fn vertical_centre_caret(&self);

    /**
     * 将文档滚动到开始，而不会更改选择。命令与 macOS 平台的 home 键行为惯例相符。通过将 home 键绑定到这些命令，可以使 Scintilla 与 macOS 应用程序相匹配。
     * */
    fn scroll_tostart(&self);

    /**
     * 将文档滚动到结束，而不会更改选择。这些命令与 macOS 平台的 end 键行为惯例相符。通过将 end 键绑定到这些命令，可以使 Scintilla 与 macOS 应用程序相匹配。
     * */
    fn scroll_to_end(&self);

    /**
     * 这将给定的键定义分配给由 sci_command 标识的 Scintilla 命令。sci_command 可以是任何没有参数的 SCI_* 命令。
     * 在 macOS 上，Command 键映射到 SCMOD_CTRL，Control 键映射到 SCMOD_META。SCMOD_SUPER 仅在 GTK 上可用，通常是 Windows 键。如果您正在构建表格，则可能需要使用 SCMOD_NORM（其值为 0），表示没有修饰键。在 Win32 上，按下 Alt 时的数字键盘可用于按数字输入字符。当分配功能键时，这可能会在非数字锁定模式下产生意外结果，因此可能会忽略有问题的键。例如，设置 SCMOD_ALT、SCK_UP 只会对主光标键上的向上键有效，而不是数字键盘。
     * `modifier` SCMOD_ALT、SCMOD_CTRL、SCMOD_SHIFT、SCMOD_META 和 SCMOD_SUPER 中的零个或多个的组合。
     * `key_code` 键代码。
     * `sci_command` SCI_*无参数的命令消息。SCI_NULL 不执行任何操作，是分配给不执行任何操作的键的值。SCI_NULL 确保键不会传播到父窗口，因为这可能会导致焦点移动。如果您想要标准平台行为，请使用常量 0。
     * */
    fn assign_cmd_key(&self, modifier: u32, key_code: KeyCode, sci_command: u32);

    /**
     * 通过为给定的键定义分配操作 SCI_NULL，这会使给定的键定义不执行任何操作。
     * `modifier` SCMOD_ALT、SCMOD_CTRL、SCMOD_SHIFT、SCMOD_META 和 SCMOD_SUPER 中的零个或多个的组合。
     * `key_code` 键代码。
     * */
    fn clear_cmd_key(&self, modifier: u32, key_code: KeyCode);

    /**
     * 该命令通过设置一个 空的 映射表来删除所有键盘命令映射。
     * */
    fn clear_all_cmd_keys(&self);

    /**
     * 单击鼠标上的错误按钮会弹出一个简短的默认编辑菜单。可以使用 SCI_USEPOPUP(SC_POPUP_NEVER) 关闭此功能。如果将其关闭，Scintilla 将不会处理上下文菜单命令（在 Windows 中为 WM_CONTEXTMENU），因此 Scintilla 窗口的父窗口将有机会处理该消息。
     * `pop_up_mode` 弹出模式。
     * */
    fn use_pop_up(&self, pop_up_mode: PopUpMode);

    /**
     * 打开宏录制。
     * */
    fn start_record(&self);

    /**
     * 关闭宏录制。
     * */
    fn stop_record(&self);

    /**
     * 此调用将一系列文本渲染到设备上下文中。如果您将其用于打印，则可能需要安排页眉和页脚；Scintilla不会为您执行此操作。有关示例，请参阅SciTEWinDlg.cxx 中的 SciTEWin::Print()。
     * 每次使用此消息都会将一系列文本渲染到矩形区域中，并返回下一个要打印的字符在文档中的位置。min 和 max 定义要输出的字符的起始位置和最大位置。此字符范围内的每行都会被绘制。在 Cocoa 上，当调用视图的 drawRect 方法时，用于打印的表面 ID（draw=1）应该是当前上下文的图形端口 ((CGContextRef) [[NSGraphicsContext currentContext] graphicsPort])。表面 ID 并不真正用于测量（draw=0），但可以设置为位图上下文（使用 CGBitmapContextCreate 创建）以避免运行时警告。在 GTK 上，可以使用 gtk_print_context_get_cairo_context(context) 从打印上下文中找到要使用的表面 ID。 打印时，最繁琐的部分始终是确定页边距应为多少以留出纸张的不可打印区域并打印页眉和页脚。如果您查看 SciTE 中的打印代码，您会发现大部分内容都与此有关。如果您删除所有页边距、不可打印区域、页眉和页脚代码，则导致 Scintilla 渲染文本的循环非常简单。
     * `draw` 控制是否完成任何输出。如果您要分页，请将其设置为 false（例如，如果您将其与 MFC 一起使用，则需要在输出每页之前在 OnBeginPrinting() 中进行分页。
     * `hdc` 我们打印到的表面 ID。
     * `hdc_target` 我们用于测量的表面 ID（可能与 hdc 相同）。在 Windows 上，hdc 和 hdc_target 都应设置为输出设备（通常是打印机）的设备上下文句柄。如果打印到图元文件，它们将与 Windows 图元文件不同（与扩展图元文件不同），不实现用于返回信息的完整 API。在这种情况下，将 hdc_target 设置为屏幕 DC。
     * `rc` 打印的矩形。rc 是用于呈现文本的矩形（当然，它将适合 rc_page 定义的矩形）。
     * `rc_page` 物理上可打印的页面大小。rc_page 是矩形 {0, 0, max_x, max_y}，其中 max_x+1 和 max_y+1 是 x 和 y 中物理上可打印的像素数。
     * `min` 要打印的开始字符位置。
     * `max` 要打印的结束字符位置。
     * */
    fn format_range(
        &self,
        draw: bool,
        hdc: SurfaceId,
        hdc_target: SurfaceId,
        rc: Rectangle,
        rc_page: Rectangle,
        min: usize,
        max: usize,
    ) -> usize;

    //noinspection StructuralWrap
    /**
     * 允许您以不同于屏幕字体的大小进行打印。放大倍数是添加到每个屏幕字体大小的点数。值为-3或-4时，打印尺寸会比较小。
     * `magnification` 放大倍数。
     * */
    fn set_print_magnification(&self, magnification: i32);

    /**
     * 获取打印的放大倍数。
     * */
    fn get_print_magnification(&self) -> i32;

    //noinspection StructuralWrap
    /**
     * 设置在可能使用白纸的打印机上呈现彩色文本的方法。如果您使用深色或黑色屏幕背景，则考虑颜色处理尤其重要。在黑色上打印白色会比相反的方式消耗碳粉和墨水快很多倍。
     * `mode` 打印模式。
     * */
    fn set_print_colour_mode(&self, mode: PrintMode);

    /**
     * 获取在可能使用白纸的打印机上呈现彩色文本的方法。
     * */
    fn get_print_colour_mode(&self) -> PrintMode;

    /**
     * 设置打印机换行模式。
     * `wrap_mode` 换行模式。
     * */
    fn set_print_wrap_mode(&self, wrap_mode: WrapMode);

    /**
     * 获取打印机换行模式。
     * */
    fn get_print_wrap_mode(&self) -> WrapMode;

    /**
     * 此消息返回要调用的函数的地址，以处理 Scintilla 消息，而无需通过 Windows 消息系统的开销。无论您创建了多少个 Scintilla 窗口，都只需调用一次。
     * */
    fn get_direct_function(&self) -> fn(isize, u32, usize, isize) -> isize;

    /**
     * 这将返回一个指向数据的指针，该数据标识正在使用的 Scintilla 窗口。您必须为您创建的每个 Scintilla 窗口调用一次此函数。当您调用直接函数时，您必须传入与目标窗口关联的直接指针。
     * */
    fn get_direct_pointer(&self) -> isize;

    /**
     * 授予对Scintilla用于存储文档的内存的临时直接只读访问权限。
     * 移动Scintilla内的间隙，以便连续存储文档的文本并确保文本后有一个NUL字符，然后返回指向第一个字符的指针。然后，应用程序可以将其传递给接受字符指针的函数，例如正则表达式搜索或解析器。不应写入指针，因为这可能会使Scintilla的内部状态不同步。由于Scintilla中的任何操作都可能改变其内部状态，因此此指针在任何调用或允许用户界面活动后变为无效。应用程序应在对Scintilla进行任何调用或执行任何用户界面调用（例如修改进度指示器）后重新获取指针。此调用所需的时间与在文档末尾插入字符的时间相似，这可能包括移动文档内容。
     * 具体而言，文档间隙后的所有字符都移动到间隙之前。这种压缩状态应在不会更改文档内容的调用和用户界面操作中持续存在，因此之后重新获取指针非常快。如果此调用用于实现全局替换操作，则每次替换都会移动间隙，因此如果在每次替换后调用SCI_GETCHARACTERPOINTER，则操作将变为O(n^2)而不是O(n)。相反，应该找到并记住所有匹配项，然后执行所有替换。
     * */
    fn get_character_pointer(&self) -> isize;

    /**
     * 提供对请求范围的直接访问。除非间隙在请求的范围内，否则不会移动间隙，因此此调用可以比SCI_GETCHARACTERPOINTER更快。这可以由能够对文本块或行范围进行操作的应用程序代码使用。
     * `start` 开始点。
     * `length_range` 长度范围。
     * */
    fn get_range_pointer(&self, start: usize, length_range: usize) -> isize;

    /**
     * 返回当前间隙位置。这是一个提示，应用程序可以使用它来避免使用包含间隙的范围调用 SCI_GETRANGEPOINTER 以及移动间隙的后续成本。
     * */
    fn get_gap_position(&self) -> usize;

    /**
     * 这将返回窗口当前正在使用的文档的指针。它没有其他作用。
     * */
    fn get_doc_pointer(&self) -> isize;

    //noinspection StructuralWrap
    /**
     * 执行以下操作：
     * 1. 从当前文档持有的列表中删除当前窗口。
     * 2. 将当前文档的引用计数减少1。
     * 3. 如果引用计数达到0，则删除该文档。
     * 4. 将doc设置为窗口的新文档。
     * 5. 如果doc为0，则创建一个新的空文档并将其附加到窗口。
     * 6. 如果doc不为0，则将其引用计数增加1。
     * `doc` 文档指针。
     * */
    fn set_doc_pointer(&self, doc: isize);

    /**
     * 创建一个新的空文档并返回指向它的指针。此文档未被选入编辑器，并以1的引用计数开始。这意味着您拥有它的所有权，并且必须在使用SCI_SETDOCPOINTER后将其引用计数减少 1，以便 Scintilla 窗口拥有它，或者您必须确保在关闭应用程序之前使用 SCI_RELEASEDOCUMENT 将引用计数减少 1，以避免内存泄漏。
     * `bytes` 确定文档的初始内存分配，因为分配一次比依靠随着数据添加而增长的缓冲区更有效。如果 SCI_CREATEDOCUMENT 失败，则返回 0。
     * `document_options` 在影响内存分配和性能的不同文档功能之间进行选择，其中 SC_DOCUMENTOPTION_DEFAULT (0) 选择标准选项。 SC_DOCUMENTOPTION_STYLES_NONE (0x1) 停止为样式字符分配内存，从而节省大量内存，通常可节省 40%，整个文档被视为样式 0。词法分析器仍可使用指示器生成视觉样式。SC_DOCUMENTOPTION_TEXT_LARGE (0x100) 可在 64 位可执行文件中容纳大于 2 GB 的文档。使用 SC_DOCUMENTOPTION_STYLES_NONE，词法分析器仍处于活动状态并可显示指示器。有些词法分析器可能生成折叠信息，但大多数词法分析器需要词汇样式才能正确确定折叠。将空词法分析器设置为 NULL 以便不运行词法分析器通常更有效。对于许多应用程序来说，对大于 4GB 的文档进行词法分析会过于缓慢，因此可以使用 SC_DOCUMENTOPTION_STYLES_NONE 和空词法分析器“null”。另一种方法是使用 SCI_SETIDLESTYLING 打开空闲样式。
     * */
    fn create_document(&self, bytes: usize, document_options: u32) -> isize;

    /**
     * 这会使文档的引用计数增加 1。如果您想要替换 Scintilla 窗口中的当前文档并取得当前文档的所有权，例如，如果您在一个窗口中编辑多个文档，请执行以下操作：
     * 1. 使用 SCI_GETDOCPOINTER 获取指向文档 doc 的指针。
     * 2. 使用 SCI_ADDREFDOCUMENT(0, doc) 增加引用计数。
     * 3. 使用 SCI_SETDOCPOINTER(0, docNew) 设置其他文档或使用 SCI_SETDOCPOINTER(0, 0) 设置新的空文档。
     * `doc` 文档指针。
     * */
    fn add_ref_document(&self, doc: isize);

    /**
     * 减少了由 doc 标识的文档的引用计数。
     * `doc` 必须是 SCI_GETDOCPOINTER 或 SCI_CREATEDOCUMENT 的结果，并且必须指向仍然存在的文档。如果您对引用计数为 1 且仍附加到 Scintilla 窗口的文档调用此消息，则会发生糟糕的事情。为了让世界保持运转，您必须平衡对 SCI_CREATEDOCUMENT 或 SCI_ADDREFDOCUMENT 的每次调用与对 SCI_RELEASEDOCUMENT 的调用。
     * */
    fn release_document(&self, doc: isize);

    /**
     * 为了确保用户界面的响应，应用程序可能会决定使用与用户界面不同的线程来加载和保存文档。
     * 应用程序可以将整个文件加载到它在后台线程上分配的缓冲区中，然后将该缓冲区中的数据添加到用户界面线程上的Scintilla 文档中。该技术使用额外的内存来存储文件的完整副本，也意味着 Scintilla 执行初始行尾发现所需的时间会阻塞用户界面。为了避免这些问题，可以创建一个加载器对象并使用它加载文件。加载器对象支持 ILoader 接口。
     * 创建一个支持 ILoader 接口的对象，该对象可用于加载数据，然后转换为 Scintilla 文档对象以附加到视图对象。
     * ```cpp
     * class ILoader {
     * public:
     *        virtual int SCI_METHOD Release() = 0;
     *        // Returns a status code from SC_STATUS_*
     *        virtual int SCI_METHOD AddData(const char *data, Sci_Position length) = 0;
     *        virtual void * SCI_METHOD ConvertToDocument() = 0;
     * };
     * ```
     * 应用程序应该对从文件读取的每个数据块调用 AddData 方法。除非发生故障（例如内存耗尽），否则 AddData 将返回 SC_STATUS_OK。如果在 AddData 或文件读取调用中发生故障，则可以放弃加载并使用 Release 调用释放加载器。读取整个文件后，应调用 ConvertToDocument 以生成 Scintilla 文档指针。可以将此指针视为 void* cookie 以传递给其他 API 或转换为 IDocumentEditable* 指针。新创建的文档将具有 1 个引用计数，与从 SCI_CREATEDOCUMENT 返回的文档指针相同。在 ConvertToDocument 之后无需调用 Release。
     * `bytes` 确定文档的初始内存分配，因为一次性分配比依靠随着数据添加而增长的缓冲区更有效。如果 SCI_CREATELOADER 失败，则返回 0。
     * `document_options` 在 SCI_CREATEDOCUMENT 部分中描述。
     * */
    fn create_loader(&self, bytes: usize, document_options: u32) -> isize;

    //noinspection StructuralWrap
    /**
     * 当某些行被隐藏和/或注释被显示时，文档中的特定行可能会显示在与其文档位置不同的位置。如果没有行被隐藏且没有注释，则此消息返回doc_line。否则，这将返回显示行（将第一个可见行计为0）。不可见行的显示行与前一个可见行相同。文档中第一行的显示行号为0。如果行被隐藏且doc_line超出文档中的行范围，则返回值为-1。如果行换行，则它们可以占据多个显示行。
     * `doc_line` 文档行号。
     * */
    fn visible_from_doc_line(&self, doc_line: usize) -> usize;

    //noinspection StructuralWrap
    /**
     * 将一系列线条标记为可见，然后重新绘制显示。对折叠级别或折叠标志没有影响。
     * `line_start` 开始行号。
     * `line_end` 结束行号。
     * */
    fn show_lines(&self, line_start: usize, line_end: usize);

    //noinspection StructuralWrap
    /**
     * 将一系列线条标记为不可见，然后重新绘制显示。对折叠级别或折叠标志没有影响。
     * `line_start` 开始行号。
     * `line_end` 结束行号。
     * */
    fn hide_lines(&self, line_start: usize, line_end: usize);

    /**
     * 报告线条的可见状态，如果可见则返回true，如果不可见则返回false。对折叠级别或折叠标志没有影响。
     * `line` 行号。
     * */
    fn get_line_visible(&self, line: usize) -> bool;

    /**
     * 如果所有线条都可见，返回true，如果某些线条隐藏，则返回false。对折叠级别或折叠标志没有影响。
     * */
    fn get_all_lines_visible(&self) -> bool;

    /**
     * 设置一个 32 位值，该值包含行的折叠级别和一些与折叠相关的标志。折叠级别是 0 到 SC_FOLDLEVELNUMBERMASK(0x0FFF) 范围内的数字。但是，初始折叠级别设置为 SC_FOLDLEVELBASE(0x400)，以允许对折叠级别进行无符号算术运算。有两个附加标志位。SC_FOLDLEVELWHITEFLAG表示该行为空白，并允许将其处理方式与其级别可能指示的方式略有不同。例如，空白行通常不应是折叠点，并且将被视为前一节的一部分，即使它们的折叠级别可能较低。SC_FOLDLEVELHEADERFLAG表示该行是标题（折叠点）。SC_FOLDLEVELNONE 是折叠前可能出现的默认级别。同样，要设置折叠级别，您必须在相关标志中。例如，要将级别设置为this_level 并将某行标记为折叠点，请使用：SCI_SETFOLDLEVEL(line, this_level | SC_FOLDLEVELHEADERFLAG)。如果您使用词法分析器，则不需要使用 SCI_SETFOLDLEVEL，因为词法分析器可以更好地处理这个问题。如果您确实更改了折叠级别，折叠边距将更新以匹配您的更改。
     * `line` 行号。
     * */
    fn set_fold_level(&self, line: usize, level: u32);

    /**
     * 获取一个 32 位值，该值包含行的折叠级别和一些与折叠相关的标志。使用 SCI_GETFOLDLEVEL(line) & SC_FOLDLEVELNUMBERMASK 获取行的折叠级别。同样，使用 SCI_GETFOLDLEVEL(line) & SC_FOLDLEVEL*FLAG 获取标志的状态。您需要使用 SCI_GETFOLDLEVEL 来决定如何处理用户折叠请求。如果您确实更改了折叠级别，折叠边距将更新以匹配您的更改。
     * `line` 行号。
     * */
    fn get_fold_level(&self, line: usize) -> u32;

    /**
     * 设置折叠标志，此消息导致显示重新绘制。除了在折叠边距中显示标记外，您还可以通过在文本区域中绘制线条来向用户指示折叠。如果设置了 SC_ELEMENT_FOLD_LINE 颜色，则线条将以该颜色绘制。如果未设置，则使用为 STYLE_DEFAULT 设置的前景颜色。
     * `flags` 在标志中设置的位决定折叠线的绘制位置：
     * 折叠标志符号 | 值 | 效果
     * SC_FOLDFLAG_NONE | 0 | 默认值。
     * | 1 | 已删除的实验功能。
     * SC_FOLDFLAG_LINEBEFORE_EXPANDED | 2 | 如果展开，则绘制在上方
     * SC_FOLDFLAG_LINEBEFORE_CONTRACTED | 4 | 如果未展开，则绘制在上方
     * SC_FOLDFLAG_LINEAFTER_EXPANDED | 8 | 如果展开，则绘制在下方
     * SC_FOLDFLAG_LINEAFTER_CONTRACTED | 16 | 如果未展开，则绘制在下方
     * SC_FOLDFLAG_LEVELNUMBERS | 64 |在行边距中显示十六进制折叠级别以帮助调试折叠。此功能的外观将来可能会发生变化。
     * SC_FOLDFLAG_LINESTATE | 128 | 在行边距中显示十六进制行状态以帮助调试词法分析和折叠。不能与 SC_FOLDFLAG_LEVELNUMBERS 同时使用。
     * */
    fn set_fold_flags(&self, flags: u32);

    /**
     * 逐行搜索折叠级别小于或等于 level 的下一行，然后返回前一行的行号。如果将 level 设置为 -1，level 将设置为行 line 的折叠级别。如果 from 是折叠点，SCI_GETLASTCHILD(from, -1) 将返回通过切换折叠状态可显示或隐藏的最后一行。
     * `line` 行号。
     * `level` 折叠级别。
     * */
    fn get_last_child(&self, line: usize, level: u32) -> usize;

    /**
     * 将返回line前的第一行的行号，该行被标记为带有SC_FOLDLEVELHEADERFLAG的折叠点，并且折叠级别小于line。如果找不到行，或者标头标志和折叠级别不一致，则返回值为-1。
     * `line` 行号。
     * */
    fn get_fold_parent(&self, line: usize) -> usize;

    //noinspection StructuralWrap
    /**
     * 每个折叠点可以展开，显示其所有子行，也可以收缩，隐藏所有子行。只要设置了SC_FOLDLEVELHEADERFLAG，这些消息就会切换给定行的折叠状态。这些消息负责折叠或展开依赖于该行的所有行。显示在此消息后更新。
     * `line` 行号。
     * */
    fn toggle_fold(&self, line: usize);

    /**
     * 在折叠文本的右侧显示可选文本标记。可以使用 SCI_SETDEFAULTFOLDDISPLAYTEXT 设置所有标题行的默认文本。文本使用 STYLE_FOLDDISPLAYTEXT 样式绘制。
     * `line` 行号。
     * `text` 文字。
     * */
    fn toggle_fold_show_text(&self, line: usize, text: String);

    //noinspection StructuralWrap
    /**
     * 改变了折叠文本标签的外观。
     * `style` 样式。
     * */
    fn fold_display_text_set_style(&self, style: FoldDisplayText);

    /**
     * 设置单行的展开状态。设置消息对行或任何依赖它的行的可见状态没有影响。它确实会更改折叠边距中的标记。如果您请求文档外行的展开状态，则结果为false(0)。如果您只想切换一行的折叠状态并处理所有依赖它的行，使用SCI_TOGGLEFOLD会更容易。您可以使用 SCI_SETFOLDEXPANDED 消息来处理许多折叠，而无需在完成之前更新显示。有关使用这些消息的示例，请参阅 SciTEBase::FoldAll() 和 SciTEBase::Expand()。
     * `line` 行号。
     * `expanded` 展开状态。
     * */
    fn set_fold_expanded(&self, line: usize, expanded: bool);

    //noinspection StructuralWrap
    /**
     * 获取单行的展开状态。
     * `line` 行号。
     * */
    fn get_fold_expanded(&self, line: usize) -> bool;

    /**
     * 提供了一种更高级别的折叠方法，而不是设置扩展标志并显示或隐藏单个行。可以使用SCI_FOLDLINE收缩/扩展/切换单个折叠。使用 SC_FOLDACTION_TOGGLE 检查文档中的第一个折叠标题以决定是扩展还是收缩。
     * `line` 行号。
     * `action` 动作。
     * */
    fn fold_line(&self, line: usize, action: FoldAction);

    /**
     * 提供了一种更高级别的折叠方法，而不是设置扩展标志并显示或隐藏单个行。要影响所有子折叠，请调用SCI_FOLDCHILDREN。使用 SC_FOLDACTION_TOGGLE 检查文档中的第一个折叠标题以决定是扩展还是收缩。
     * `line` 行号。
     * `action` 动作。
     * */
    fn fold_children(&self, line: usize, action: FoldAction);

    /**
     * 提供了一种更高级别的折叠方法，而不是设置扩展标志并显示或隐藏单个行。要影响整个文档，请调用SCI_FOLDALL。使用SC_FOLDACTION_TOGGLE检查文档中的第一个折叠标题以决定是扩展还是收缩。
     * `action` 动作。
     * */
    fn fold_all(&self, action: FoldAction);

    //noinspection StructuralWrap
    /**
     * 这用于响应行的更改导致其折叠级别或是否是标题更改，可能是在添加或删除“{”时。当容器收到行已更改的通知时，折叠级别已经设置，因此容器必须在这次调用中使用前一个级别，以便可以显示隐藏在此行下方的任何范围。
     * `line` 行号。
     * `level` 折叠级别。
     * */
    fn expand_children(&self, line: usize, level: u32);

    /**
     * 设置自动折叠位标志。Scintilla无需在容器中实现处理折叠的所有逻辑，而是可以提供适用于许多应用程序的行为。
     * `automatic_fold` 折叠标志，一个位集，定义应启用 3 个折叠实现中的哪一个。大多数应用程序应该能够使用 SC_AUTOMATICFOLD_SHOW 和 SC_AUTOMATICFOLD_CHANGE 标志，除非它们希望实现完全不同的行为（例如定义自己的折叠结构）。当应用程序想要添加或更改点击行为（例如仅在 Shift+Alt 与点击结合使用时显示方法标题）时，更有可能触发 SC_AUTOMATICFOLD_CLICK。
     * 符号 | 值 | 效果
     * SC_AUTOMATICFOLD_NONE | 0 | 无自动行为的值。
     * SC_AUTOMATICFOLD_SHOW | 1 | 根据需要自动显示线条。这样可以避免发送 SCN_NEEDSHOWN 通知。
     * SC_AUTOMATICFOLD_CLICK | 2 | 自动处理折叠边距中的点击。这样可以避免发送折叠边距的 SCN_MARGINCLICK 通知。
     * SC_AUTOMATICFOLD_CHANGE | 4 | 折叠结构发生变化时根据需要显示线条。除非容器禁用 SCN_MODIFIED 通知，否则仍会发送该通知。
     * */
    fn set_automatic_fold(&self, automatic_fold: u32);

    /**
     * 获取自动折叠位标志。
     * */
    fn get_automatic_fold(&self) -> u32;

    //noinspection StructuralWrap
    /**
     * 高效搜索收缩折叠标题行。这在切换文档时保存用户的折叠或保存文件的折叠时非常有用。搜索从行号line_start开始，一直到文件末尾。如果是收缩折叠标题，则返回line_start，否则返回下一个收缩折叠标题。如果没有其他收缩折叠标题，则返回-1。
     * `line_start` 开始行号。
     * */
    fn contracted_fold_next(&self, line_start: usize) -> usize;

    //noinspection StructuralWrap
    /**
     * 一行可能因为其父行中有不止一行被收缩而被隐藏。这会沿着折叠层次向上传递，展开任何收缩的折叠，直到它们到达顶层。然后该行将可见。
     * `line` 行号。
     * */
    fn ensure_visible(&self, line: usize);

    /**
     * 一行可能因为其父行中有不止一行被收缩而被隐藏。这会沿着折叠层次向上传递，展开任何收缩的折叠，直到它们到达顶层。然后该行将可见。这将应用由SCI_SETVISIBLEPOLICY设置的垂直插入点策略。
     * `line` 行号。
     * */
    fn ensure_visible_enforce_policy(&self, line: usize);

    /**
     * 设置换行模式。
     * `wrap_mode` 换行模式。
     * */
    fn set_wrap_mode(&self, wrap_mode: WrapMode);

    /**
     * 获取换行模式。
     * */
    fn get_wrap_mode(&self) -> WrapMode;

    //noinspection StructuralWrap
    /**
     * 您可以启用视觉标志的绘制来指示行已换行。
     * `wrap_visual_flags` 设置的位决定绘制哪些视觉标志。
     * */
    fn set_wrap_visual_flags(&self, wrap_visual_flags: u32);

    /**
     * 获取换行视觉标志。
     * */
    fn get_wrap_visual_flags(&self) -> u32;

    //noinspection StructuralWrap
    /**
     * 您可以设置指示换行的视觉标志是在边框附近还是在文本附近绘制。
     * `wrap_visual_flags_location` 设置的位将相应视觉标志的位置设置为靠近文本。
     * */
    fn set_wrap_visual_flags_location(&self, wrap_visual_flags_location: u32);

    /**
     * 获取换行视觉标志位置。
     * */
    fn get_wrap_visual_flags_location(&self) -> u32;

    /**
     * 换行的子行可以缩进到其第一个子行的位置或再缩进一个级别。默认值为 SC_WRAPINDENT_FIXED。
     * `wrap_indent_mode` 换行缩进模式。
     * */
    fn set_wrap_indent_mode(&self, wrap_indent_mode: WrapIndent);

    /**
     * 获取换行缩进模式。
     * */
    fn get_wrap_indent_mode(&self) -> WrapIndent;

    /**
     * 根据STYLE_DEFAULT中的平均字符宽度设置换行的子行缩进大小。缩进大小没有限制，但小于 0 的值或较大的值可能会产生不良影响。子行的缩进与视觉标志无关，但如果设置了 SC_WRAPVISUALFLAG_START，则使用至少 1 的缩进。
     * `indent` 缩进。
     * */
    fn set_wrap_start_indent(&self, indent: i32);

    /**
     * 获取换行子行缩进大小。
     * */
    fn get_wrap_start_indent(&self) -> i32;

    /**
     * 设置布局缓存模式。
     * `cache_mode` 缓存模式。
     * */
    fn set_layout_cache(&self, cache_mode: CacheMode);

    /**
     * 获取布局缓存模式。
     * */
    fn get_layout_cache(&self) -> CacheMode;

    //noinspection StructuralWrap
    /**
     * 位置缓存存储短文本的位置信息，以便在文本重复时可以更快地确定其布局。
     * `size` 缓存的条目大小。
     * */
    fn set_position_cache(&self, size: i32);

    /**
     * 获取位置缓存。
     * */
    fn get_position_cache(&self) -> i32;

    /**
     * 将目标指示的行范围拆分为宽度最多为pixel_width的行。拆分尽可能发生在单词边界上，方式与换行类似。当 pixel_width 为 0 时，则使用窗口的宽度。
     * `pixel_width` 像素宽度。
     * */
    fn lines_split(&self, pixel_width: i32);

    /**
     * 通过删除行尾字符将目标指示的行范围合并为一行。如果这会导致单词之间没有空格，则会插入一个额外的空格。
     * */
    fn lines_join(&self);

    /**
     * 如果doc_line换行，则它们可以占用多个显示行，这将返回一个doc_line换行所需的显示行数。
     * `doc_line` 文档行号。
     * */
    fn wrap_count(&self, doc_line: usize) -> usize;

    /**
     * 如果当前缩放系数小于 20 点，则 SCI_ZOOMIN 将缩放系数增加 1 点。
     * */
    fn zoom_in(&self);

    /**
     * 如果当前缩放系数大于 -10 点，则 SCI_ZOOMOUT 将缩放系数减少 1 点。
     * */
    fn zoom_out(&self);

    //noinspection StructuralWrap
    /**
     * 设置缩放系数。您可以设置的系数没有限制，因此将自己限制在-10到+20以匹配增量缩放功能是一个好主意。
     * `zoom_in_points` 缩放系数。
     * */
    fn set_zoom(&self, zoom_in_points: i32);

    /**
     * 获取缩放系数。
     * */
    fn get_zoom(&self) -> i32;

    /**
     * 设置用于显示长行的模式。
     * `edge_mode` 边缘模式。
     * */
    fn set_edge_mode(&self, edge_mode: EdgeMode);

    /**
     * 获取用于显示长行的模式。
     * */
    fn get_edge_mode(&self) -> EdgeMode;

    /**
     * 设置显示长行标记的列号。绘制线条时，列以 STYLE_DEFAULT 中空格字符的宽度为单位设置位置。设置背景颜色时，列是行中的字符数（允许制表符）。
     * `column` 列号。
     * */
    fn set_edge_column(&self, column: usize);

    /**
     * 获取显示长行标记的列号。
     * */
    fn get_edge_column(&self) -> usize;

    /**
     * 设置用于显示行已超出 SCI_SETEDGECOLUMN 设置的长度的标记的颜色。
     * `edge_colour` 边缘颜色。
     * */
    fn set_edge_colour(&self, edge_colour: i32);

    /**
     * 获取用于显示行已超出 SCI_SETEDGECOLUMN 设置的长度的标记的颜色。
     * */
    fn get_edge_colour(&self) -> i32;

    /**
     * 向视图添加新的垂直边。该边将显示在给定的列号处。结果边的位置取决于 STYLE_DEFAULT 中空格字符的度量。
     * `column` 列号。
     * `edge_colour` 边缘颜色。
     * */
    fn multi_edge_add_line(&self, column: usize, edge_colour: i32);

    /**
     * 清除所有边。
     * */
    fn multi_edge_clear_all(&self);

    /**
     * 启用或禁用无障碍。在大多数平台上，可访问性要么已实现，要么未实现，这可以通过 SCI_GETACCESSIBILITY 发现，而 SCI_SETACCESSIBILITY 不执行任何操作。在 GTK 上，可访问性会消耗存储和性能，因此可以通过调用 SCI_SETACCESSIBILITY 来禁用它。
     * `accessibility` 无障碍状态。
     * */
    fn set_accessibility(&self, accessibility: Accessibility);

    /**
     * 获取当前无障碍启用状态。
     * */
    fn get_accessibility(&self) -> Accessibility;

    /**
     * 您可以从 SciLexer.h 中的 SCLEX_* 枚举中检索当前词法分析器的整数词法分析器 ID。某些词法分析器可能没有词法分析器 ID，只有词法分析器名称，在这种情况下会返回 0。
     * */
    fn get_lexer(&self) -> u32;

    /**
     * SCI_GETLEXERLANGUAGE 返回当前词法分析器的名称，该名称必须使用 SCI_SETILEXER 进行设置。要找到词法分析器的名称，请打开相关的 lexilla/lexers/Lex*.cxx 文件并搜索 LexerModule。LexerModule 构造函数中的第三个参数是要使用的名称。
     * */
    fn get_lexer_language(&self) -> Option<String>;

    /**
     * 这将请求当前词法分析器或容器（如果词法分析器设置为 NULL）在开始和结束之间设置文档样式。如果结束为 -1，则文档从开始到结束都设置样式。如果“折叠”属性设置为“1”，并且您的词法分析器或容器支持折叠，则还会设置折叠级别。此消息会导致重绘。
     * `start` 开始点。
     * `end` 结束点。
     * */
    fn colourise(&self, start: usize, end: usize);

    /**
     * 表明词法分析器的内部状态在一定范围内发生了变化，因此可能需要重新绘制。
     * `start` 开始点。
     * `end` 结束点。
     * */
    fn change_lexer_state(&self, start: usize, end: usize) -> i32;

    /**
     * 可以检索有关可以为当前词法分析器设置的属性的信息。此信息仅适用于较新的词法分析器。这返回一个字符串，其中所有有效属性都以“\n”分隔。如果词法分析器不支持此调用，则返回一个空字符串。
     * */
    fn property_names(&self) -> Option<String>;

    /**
     * 获取属性的类型。
     * `name` 属性名。
     * */
    fn property_type(&self, name: String) -> PropertyType;

    /**
     * 返回属性的英文描述。
     * `name` 属性名。
     * */
    fn describe_property(&self, name: String) -> Option<String>;

    /**
     * 您可以使用键：值字符串对将设置传达给词法分析器。除了可用内存之外，您可以设置的关键字对数量没有限制。如果已经有与关键字关联的值字符串，则会替换该值字符串。如果您传递的是零长度字符串，则消息不执行任何操作。key 和 value 均无需修改即可使用；key 开头或结尾的额外空格很重要。 value 字符串不再能引用其他关键字，而 Scintilla 的旧版本中却可以这样做。词法分析器只能存储它们支持的关键字的值。目前，大多数词法分析器都定义了“fold”属性，如果设置为“1”，则设置折叠结构。
     * SCLEX_PYTHON 将“tab.timmy.whinge.level”理解为确定如何指示错误缩进的设置。大多数关键字的值都被解释为整数。在词法分析器源中搜索 GetPropertyInt 以了解如何使用属性。词法分析器使用的属性命名有一个约定，以便脚本可以找到属性集。当属性名称适用于一个词法分析器时，应以“lexer..”或“fold..”开头，如果适用于多个词法分析器，则应以“lexer.”或“fold.”开头。
     * 应用程序可以通过在词法分析器的源代码中搜索包含 GetProperty 或 DefineProperty 和双引号字符串的行来发现所使用的属性集，并提取双引号字符串的值作为属性名称。lexilla/scripts/LexillaData.py 脚本执行此操作，可以用作示例。
     * `key` 区分大小写的关键字。
     * `value` 与关键字关联的字符串。
     * */
    fn set_property(&self, key: String, value: String);

    /**
     * 现在，这与 SCI_GETPROPERTY 相同 - 不执行扩展。使用指定的键查找键：值对。
     * `key` 区分大小写的关键字。
     * */
    fn get_property_expanded(&self, key: String) -> Option<String>;

    /**
     * 使用指定键查找键：值对；如果找到，则将值解释为整数并返回。如果未找到（或值为空字符串），则返回提供的默认值。如果找到关键字：值对但不是数字，则返回 0。
     * `key` 区分大小写的关键字。
     * `default_value` 默认值。
     * */
    fn get_property_int(&self, key: String, default_value: u32) -> u32;

    /**
     * 使用指定键查找键：值对。
     * */
    fn get_property(&self, key: String) -> Option<String>;

    /**
     * 您最多可以设置 9 个关键字列表供当前词法分析器使用。如何使用这些关键字完全取决于词法分析器。某些语言（例如 HTML）可能包含嵌入语言，VBScript 和 JavaScript 对 HTML 很常见。对于 HTML，关键字集 0 代表 HTML，1 代表 JavaScript，2 代表 VBScript，3 代表 Python，4 代表 PHP，5 代表 SGML 和 DTD 关键字。查看词法分析器代码以查看关键字列表的示例。完全符合规范的词法分析器将 LexerModule 构造函数的第四个参数设置为描述关键字列表用途的字符串列表。或者，您可以将集合 0 用于一般关键字，将集合 1 用于导致缩进的关键字，将集合 2 用于导致取消缩进的关键字。同样，您可能有一个为关键字着色的简单词法分析器，并且您可以通过更改集合 0 中的关键字来更改语言。没有什么可以阻止您在词法分析器中构建自己的关键字列表，但这意味着如果添加更多关键字，则必须重建词法分析器。
     * `key_word_set` 可以是 0 到 8（实际上是 0 到 KEYWORDSET_MAX），并选择要替换的关键字列表。
     * `key_words` 是由空格、制表符、“\n”或“\r”或这些的任意组合分隔的关键字列表。预计关键字将由标准 ASCII 打印字符组成，但没有什么可以阻止您使用 1 到 255 之间的任何非分隔符字符代码（常识除外）。
     * */
    fn set_keywords(&self, key_word_set: u32, key_words: String);

    /**
     * 返回以“\n”分隔的所有关键字集的描述。
     * */
    fn describe_keyword_sets(&self) -> Option<String>;

    /*
     * 用一个字节填充每个可拆分为子样式的样式。
     * */
    fn get_substyle_bases(&self) -> Option<Vec<u8>>;

    /**
     * 返回主要样式与其对应的次要样式之间的距离。
     * */
    fn distance_to_secondary_styles(&self) -> i32;

    //noinspection StructuralWrap
    /**
     * 为特定基本样式分配一定数量的子样式，并返回分配的第一个子样式编号。如果失败（例如请求的子样式多于可用子样式），则返回负数。支持子样式的词法分析器通常允许分配64个子样式。子样式是连续分配的。
     * `style_base` 基本样式。
     * `number_styles` 样式数量。
     * */
    fn allocate_substyles(&self, style_base: i32, number_styles: i32) -> i32;

    /**
     * 释放所有分配的子样式。
     * */
    fn free_substyles(&self);

    //noinspection StructuralWrap
    /**
     * 返回分配给基本样式的子样式的开始。
     * `style_base` 基本样式。
     * */
    fn get_substyles_start(&self, style_base: i32) -> i32;

    //noinspection StructuralWrap
    /**
     * 返回分配给基本样式的子样式的长度。
     * `style_base` 基本样式。
     * */
    fn get_substyles_length(&self, style_base: i32) -> i32;

    //noinspection StructuralWrap
    /**
     * 对于子样式，返回基本样式，否则返回参数。
     * `sub_style` 子样式。
     * */
    fn get_style_from_substyle(&self, sub_style: i32) -> i32;

    //noinspection StructuralWrap
    /**
     * 对于次要样式，返回主要样式，否则返回参数。
     * `style` 样式。
     * */
    fn get_primary_style_from_style(&self, style: i32) -> i32;

    /**
     * 与 SCI_SETKEYWORDS 类似，但适用于子样式。SCI_SETKEYWORDS 提供的前缀功能未在 SCI_SETIDENTIFIERS 中实现。
     * `style` 样式。
     * `identifiers` 标识符。
     * */
    fn set_identifiers(&self, style: i32, identifiers: String);

    /**
     * 以 Scintilla 无法理解的方式调用词法分析器。
     * `operation` 操作。
     * `pointer` 指针。
     * */
    fn private_lexer_call(&self, operation: i32, pointer: isize) -> isize;

    /**
     * 检索词法分析器的命名样式的数量。
     * */
    fn get_named_styles(&self) -> i32;

    /**
     * 查询样式的名称。这是一个 C 预处理器符号，如“SCE_C_COMMENTDOC”。
     * `style` 样式。
     * */
    fn name_of_style(&self, style: i32) -> Option<String>;

    /**
     * 检索样式的标签。这是一组以空格分隔的单词，例如“注释文档”。
     * `style` 样式。
     * */
    fn tags_of_style(&self, style: i32) -> Option<String>;

    /**
     * 查询适合在用户界面中显示的样式的英文描述。这看起来像“文档注释：以 /\*\* 或 /\* 开头的块注释！”。
     * */
    fn description_of_style(&self, style: i32) -> Option<String>;
}

#[cfg(test)]
mod test_scintilla {
    use win_wrap::{
        common::{find_window_ex, HWND},
        control::WindowControl,
    };

    use crate::scintilla::{
        accessibility::Accessibility,
        annotation::Annotation,
        autoc::MultiAutoc,
        bidirectional::Bidirectional,
        cache::CacheMode,
        caret::CaretSticky,
        character::CharacterSet,
        edge::EdgeMode,
        eol::EolMode,
        folding::{FoldAction, FoldDisplayText},
        ime::Ime,
        indentation::IndentView,
        indicator::Indicator,
        key::KeyCode,
        margin::MarginOptions,
        marker::{Mark, MarkerNumber},
        order::Order,
        phases::Phases,
        popup::PopUpMode,
        print::PrintMode,
        selection::SelectionMode,
        space::{TabDrawMode, WhiteSpace},
        status::Status,
        style::{Case, IdleStyling, STYLE_BRACEBAD},
        technology::Technology,
        wrap::{
            WrapIndent, WrapMode, SC_WRAPVISUALFLAGLOC_START_BY_TEXT, SC_WRAPVISUALFLAG_MARGIN,
        },
        Rectangle, Scintilla, CARETSTYLE_LINE, CARET_JUMPS, SCFIND_MATCHCASE, SCI_COPYTEXT,
        SCMOD_META, SCMOD_SUPER, SCVS_USERACCESSIBLE, SC_AUTOMATICFOLD_CLICK,
        SC_CASEINSENSITIVEBEHAVIOUR_IGNORECASE, SC_CP_UTF8, SC_CURSORREVERSEARROW, SC_CURSORWAIT,
        SC_DOCUMENTOPTION_DEFAULT, SC_EFF_QUALITY_ANTIALIASED, SC_FOLDFLAG_LINEBEFORE_CONTRACTED,
        SC_FOLDLEVELHEADERFLAG, SC_INDICFLAG_VALUEFORE, SC_LINE_END_TYPE_UNICODE, SC_MARGIN_NUMBER,
        UNDO_MAY_COALESCE, VISIBLE_STRICT,
    };

    //noinspection GrazieInspection
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
        control.undo();
        dbg!(control.can_undo());
        control.redo();
        dbg!(control.can_redo());
        control.empty_undo_buffer();
        control.set_undo_collection(false);
        assert_eq!(false, control.get_undo_collection());
        control.begin_undo_action();
        control.end_undo_action();
        control.add_undo_action(0, UNDO_MAY_COALESCE);
        control.set_first_visible_line(0);
        assert_eq!(0, control.get_first_visible_line());
        control.set_x_offset(0);
        assert_eq!(0, control.get_x_offset());
        control.line_scroll(1, 1);
        control.scroll_caret();
        control.scroll_range(0, 1);
        control.set_x_caret_policy(CARET_JUMPS, 0);
        control.set_y_caret_policy(CARET_JUMPS, 0);
        control.set_visible_policy(VISIBLE_STRICT, 0);
        control.set_h_scroll_bar(true);
        assert_eq!(true, control.get_h_scroll_bar());
        control.set_v_scroll_bar(true);
        assert_eq!(true, control.get_v_scroll_bar());
        control.set_scroll_width(3000);
        dbg!(control.get_scroll_width());
        control.set_scroll_width_tracking(true);
        assert_eq!(true, control.get_scroll_width_tracking());
        control.set_end_at_last_line(true);
        assert_eq!(true, control.get_end_at_last_line());
        control.set_view_ws(WhiteSpace::VisibleAways);
        assert_eq!(WhiteSpace::VisibleAways, control.get_view_ws());
        control.set_white_space_back(true, 0x0000ff);
        control.set_white_space_fore(true, 0x000000);
        control.set_white_space_size(16);
        assert_eq!(16, control.get_white_space_size());
        control.set_tab_draw_mode(TabDrawMode::LongArrow);
        assert_eq!(TabDrawMode::LongArrow, control.get_tab_draw_mode());
        control.set_extra_ascent(5);
        assert_eq!(5, control.get_extra_ascent());
        control.set_extra_descent(5);
        assert_eq!(5, control.get_extra_descent());
        control.set_cursor(SC_CURSORWAIT);
        assert_eq!(SC_CURSORWAIT, control.get_cursor());
        control.set_mouse_down_captures(false);
        assert_eq!(false, control.get_mouse_down_captures());
        control.set_mouse_wheel_captures(false);
        assert_eq!(false, control.get_mouse_wheel_captures());
        control.set_eol_mode(EolMode::Lf);
        assert_eq!(EolMode::Lf, control.get_eol_mode());
        control.convert_eols(EolMode::Cr);
        control.set_view_eol(true);
        assert_eq!(true, control.get_view_eol());
        dbg!(control.get_line_end_types_supported());
        control.set_line_end_types_allowed(SC_LINE_END_TYPE_UNICODE);
        dbg!(control.get_line_end_types_allowed());
        dbg!(control.get_line_end_types_active());
        dbg!(control.word_start_position(4, false));
        dbg!(control.word_end_position(4, false));
        dbg!(control.is_range_word(3, 7));
        control.set_word_chars("abcdefg".to_string());
        assert_eq!(Some("gfedcba".to_string()), control.get_word_chars());
        control.set_white_space_chars("h".to_string());
        dbg!(control.get_white_space_chars());
        control.set_punctuation_chars(".,:".to_string());
        dbg!(control.get_punctuation_chars());
        control.set_chars_default();
        control.word_left();
        control.word_left_extend();
        control.word_right();
        control.word_right_extend();
        control.word_left_end();
        control.word_left_end_extend();
        control.word_right_end();
        control.word_right_end_extend();
        control.word_part_left();
        control.word_part_left_extend();
        control.word_part_right();
        control.word_part_right_extend();
        control.del_word_left();
        control.del_word_right();
        control.del_word_right_end();
        dbg!(control.get_end_styled());
        control.start_styling(2);
        control.set_styling(1, 0);
        control.set_styling_ex(&[0u8, 2, 3, 4]);
        control.set_idle_styling(IdleStyling::All);
        assert_eq!(IdleStyling::All, control.get_idle_styling());
        control.set_line_state(0, 4);
        assert_eq!(4, control.get_line_state(0));
        dbg!(control.get_max_line_state());
        control.style_reset_default();
        control.style_clear_all();
        control.style_set_font(0, "Verdana".to_string());
        assert_eq!(Some("Verdana".to_string()), control.style_get_font(0));
        control.style_set_size(0, 22);
        assert_eq!(22, control.style_get_size(0));
        control.style_set_size_fractional(0, 220);
        assert_eq!(220, control.style_get_size_fractional(0));
        control.style_set_bold(0, true);
        assert_eq!(true, control.style_get_bold(0));
        control.style_set_italic(0, true);
        assert_eq!(true, control.style_get_italic(0));
        control.style_set_weight(0, 333);
        assert_eq!(333, control.style_get_weight(0));
        control.style_set_back(0, 0xffffff);
        assert_eq!(0xffffff, control.style_get_back(0));
        control.style_set_fore(0, 0xff0000);
        assert_eq!(0xff0000, control.style_get_fore(0));
        control.style_set_eol_filled(0, true);
        assert_eq!(true, control.style_get_eol_filled(0));
        control.style_set_character_set(0, CharacterSet::Default);
        assert_eq!(CharacterSet::Default, control.style_get_character_set(0));
        control.style_set_case(0, Case::Upper);
        assert_eq!(Case::Upper, control.style_get_case(0));
        control.style_set_visible(0, false);
        assert_eq!(false, control.style_get_visible(0));
        control.style_set_changeable(0, false);
        assert_eq!(false, control.style_get_changeable(0));
        control.style_set_hotspot(0, false);
        assert_eq!(false, control.style_get_hotspot(0));
        control.set_sel_fore(true, 0xff0000);
        control.set_sel_back(true, 0x00ff00);
        control.set_sel_alpha(0x00);
        assert_eq!(0x00, control.get_sel_alpha());
        control.set_sel_eol_filled(true);
        assert_eq!(true, control.get_sel_eol_filled());
        control.set_caret_fore(0xff0000);
        assert_eq!(0xff0000, control.get_caret_fore());
        control.set_caret_line_back(0xff0000);
        assert_eq!(0xff0000, control.get_caret_line_back());
        control.set_caret_line_back_alpha(0xff);
        assert_eq!(0xff, control.get_caret_line_back_alpha());
        control.set_caret_line_visible(true);
        assert_eq!(true, control.get_caret_line_visible());
        control.set_caret_line_frame(1);
        assert_eq!(1, control.get_caret_line_frame());
        control.set_caret_line_visible_always(true);
        assert_eq!(true, control.get_caret_line_visible_always());
        control.set_caret_period(2000);
        assert_eq!(2000, control.get_caret_period());
        control.set_caret_style(CARETSTYLE_LINE);
        assert_eq!(CARETSTYLE_LINE, control.get_caret_style());
        control.set_caret_width(4);
        assert_eq!(4, control.get_caret_width());
        control.set_caret_sticky(CaretSticky::On);
        assert_eq!(CaretSticky::On, control.get_caret_sticky());
        control.toggle_caret_sticky();
        control.set_hotspot_active_fore(true, 0xff0000);
        assert_eq!(0xff0000, control.get_hotspot_active_fore());
        control.set_hotspot_active_back(true, 0x00ff00);
        assert_eq!(0x00ff00, control.get_hotspot_active_back());
        control.set_hotspot_active_underline(true);
        assert_eq!(true, control.get_hotspot_active_underline());
        control.set_hotspot_single_line(true);
        assert_eq!(true, control.get_hotspot_single_line());
        control.set_representation("Ω".to_string(), "U+2126 Ω".to_string());
        assert_eq!(
            Some("U+2126 Ω".to_string()),
            control.get_representation("Ω".to_string())
        );
        control.clear_representation("Ω".to_string());
        control.set_control_char_symbol(88);
        assert_eq!(88, control.get_control_char_symbol());
        control.set_margins(8);
        assert_eq!(8, control.get_margins());
        control.set_margin_type_n(SC_MARGIN_NUMBER, 0);
        assert_eq!(0, control.get_margin_type_n(SC_MARGIN_NUMBER));
        control.set_margin_width_n(SC_MARGIN_NUMBER, 20);
        assert_eq!(20, control.get_margin_width_n(SC_MARGIN_NUMBER));
        control.set_margin_mask_n(SC_MARGIN_NUMBER, 0x40);
        assert_eq!(0x40, control.get_margin_mask_n(SC_MARGIN_NUMBER));
        control.set_margin_sensitive_n(SC_MARGIN_NUMBER, true);
        assert_eq!(true, control.get_margin_sensitive_n(SC_MARGIN_NUMBER));
        control.set_margin_cursor_n(SC_MARGIN_NUMBER, SC_CURSORREVERSEARROW);
        assert_eq!(
            SC_CURSORREVERSEARROW,
            control.get_margin_cursor_n(SC_MARGIN_NUMBER)
        );
        control.set_margin_back_n(SC_MARGIN_NUMBER, 0xff0000);
        assert_eq!(0xff0000, control.get_margin_back_n(SC_MARGIN_NUMBER));
        control.set_margin_left(14);
        assert_eq!(14, control.get_margin_left());
        control.set_margin_right(15);
        assert_eq!(15, control.get_margin_right());
        control.set_fold_margin_colour(true, 0x000000);
        control.set_fold_margin_hi_colour(true, 0xffffff);
        control.margin_set_text(0, "测试".to_string());
        assert_eq!(Some("测试".to_string()), control.margin_get_text(0));
        control.margin_set_style(0, STYLE_BRACEBAD);
        assert_eq!(STYLE_BRACEBAD, control.margin_get_style(0));
        control.margin_set_styles(0, &[0, 1, 2]);
        dbg!(control.margin_get_styles(0));
        control.margin_text_clear_all();
        control.margin_set_style_offset(255);
        assert_eq!(255, control.margin_get_style_offset());
        control.set_margin_options(MarginOptions::SubLineSelect);
        assert_eq!(MarginOptions::SubLineSelect, control.get_margin_options());
        control.annotation_set_text(0, Some("测试批注".to_string()));
        assert_eq!(Some("测试批注".to_string()), control.annotation_get_text(0));
        control.annotation_set_style(0, STYLE_BRACEBAD);
        assert_eq!(STYLE_BRACEBAD, control.annotation_get_style(0));
        control.annotation_set_styles(0, &[0, 1, 2]);
        dbg!(control.annotation_get_styles(0));
        dbg!(control.annotation_get_lines(0));
        control.annotation_clear_all();
        control.annotation_set_visible(Annotation::Standard);
        assert_eq!(Annotation::Standard, control.annotation_get_visible());
        control.annotation_set_style_offset(512);
        assert_eq!(512, control.annotation_get_style_offset());
        control.set_buffered_draw(true);
        assert_eq!(true, control.get_buffered_draw());
        control.set_phases_draw(Phases::Two);
        assert_eq!(Phases::Two, control.get_phases_draw());
        control.set_technology(Technology::DirectWrite);
        assert_eq!(Technology::DirectWrite, control.get_technology());
        control.set_font_quality(SC_EFF_QUALITY_ANTIALIASED);
        assert_eq!(SC_EFF_QUALITY_ANTIALIASED, control.get_font_quality());
        control.set_code_page(SC_CP_UTF8);
        assert_eq!(SC_CP_UTF8, control.get_code_page());
        control.set_ime_interaction(Ime::Inline);
        assert_eq!(Ime::Inline, control.get_ime_interaction());
        control.set_bidirectional(Bidirectional::R2L);
        assert_eq!(Bidirectional::R2L, control.get_bidirectional());
        control.grab_focus();
        control.set_focus(true);
        assert_eq!(true, control.get_focus());
        control.brace_highlight(3, 6);
        control.brace_badlight(3);
        control.brace_highlight_indicator(true, 0xff0000);
        control.brace_badlight_indicator(true, 0xff0000);
        dbg!(control.brace_match(3, 0));
        control.set_tab_width(16);
        assert_eq!(16, control.get_tab_width());
        control.clear_tabstops(1);
        control.add_tabstop(1, 5);
        dbg!(control.get_next_tabstop(1, 5));
        control.set_use_tabs(false);
        assert_eq!(false, control.get_use_tabs());
        control.set_indent(6);
        assert_eq!(6, control.get_indent());
        control.set_tab_indents(true);
        assert_eq!(true, control.get_tab_indents());
        control.set_backspace_un_indents(true);
        assert_eq!(true, control.get_backspace_un_indents());
        control.set_line_indentation(0, 10);
        assert_eq!(10, control.get_line_indentation(0));
        dbg!(control.get_line_indent_position(1));
        control.set_indentation_guides(IndentView::Real);
        assert_eq!(IndentView::Real, control.get_indentation_guides());
        control.set_highlight_guide(5);
        assert_eq!(5, control.get_highlight_guide());
        control.marker_define(u32::FOLDER_MID_TAIL, Mark::Bookmark);
        /* has bugs
        const IMAGE_XPM: [&str; 4] = [
            /* columns rows colors chars-per-pixel */
            "2 2 1 1 ",
            "  c white",
            /* pixels */
            "  ",
            "  "
        ];
        control.marker_define_pixmap(1, &IMAGE_XPM);
        */
        control.rgba_image_set_width(2);
        control.rgba_image_set_height(2);
        control.rgba_image_set_scale(200);
        control.marker_define_rgba_image(1, &[0, 0, 0, 0]);
        dbg!(control.marker_symbol_defined(1));
        control.marker_set_fore(1, 0xff0000);
        control.marker_set_back(1, 0x0000ff);
        control.marker_set_back_selected(1, 0x000ccc);
        control.marker_enable_highlight(true);
        control.marker_set_alpha(1, 0x0);
        let handle = control.marker_add(1, 1);
        control.marker_add_set(1, 0b1);
        control.marker_delete(1, 1);
        control.marker_delete_all(1);
        dbg!(control.marker_get(1));
        dbg!(control.marker_next(0, 0b1));
        dbg!(control.marker_previous(2, 0b1));
        dbg!(control.marker_line_from_handle(handle));
        control.marker_delete_handle(handle);
        control.indic_set_style(0, Indicator::Plain);
        assert_eq!(Indicator::Plain, control.indic_get_style(0));
        control.indic_set_fore(0, 0xffccdd);
        assert_eq!(0xffccdd, control.indic_get_fore(0));
        control.indic_set_alpha(0, 32);
        assert_eq!(32, control.indic_get_alpha(0));
        control.indic_set_outline_alpha(0, 32);
        assert_eq!(32, control.indic_get_outline_alpha(0));
        control.indic_set_under(0, true);
        assert_eq!(true, control.indic_get_under(0));
        control.indic_set_hover_style(0, Indicator::Point);
        assert_eq!(Indicator::Point, control.indic_get_hover_style(0));
        control.indic_set_hover_fore(0, 0x00ccdd);
        assert_eq!(0x00ccdd, control.indic_get_hover_fore(0));
        control.indic_set_flags(0, SC_INDICFLAG_VALUEFORE);
        assert_eq!(SC_INDICFLAG_VALUEFORE, control.indic_get_flags(0));
        control.set_indicator_current(0);
        assert_eq!(0, control.get_indicator_current());
        control.set_indicator_value(0);
        dbg!(control.get_indicator_value());
        control.indicator_fill_range(2, 10);
        control.indicator_clear_range(2, 10);
        dbg!(control.indicator_all_on_for(4));
        dbg!(control.indicator_value_at(0, 8));
        dbg!(control.indicator_start(0, 4));
        dbg!(control.indicator_end(0, 4));
        control.find_indicator_show(4, 8);
        control.find_indicator_flash(4, 8);
        control.find_indicator_hide();
        control.autoc_show("ab cd".to_string());
        control.autoc_cancel();
        dbg!(control.autoc_active());
        dbg!(control.autoc_pos_start());
        control.autoc_complete();
        control.autoc_stops("efg".to_string());
        control.autoc_set_separator(44);
        assert_eq!(44, control.autoc_get_separator());
        control.autoc_select("a".to_string());
        dbg!(control.autoc_get_current());
        dbg!(control.autoc_get_current_text());
        control.autoc_set_cancel_at_start(true);
        assert_eq!(true, control.autoc_get_cancel_at_start());
        control.autoc_set_fillups("([{".to_string());
        control.autoc_set_choose_single(true);
        assert_eq!(true, control.autoc_get_choose_single());
        control.autoc_set_ignore_case(true);
        assert_eq!(true, control.autoc_get_ignore_case());
        control.autoc_set_case_in_sensitive_behaviour(SC_CASEINSENSITIVEBEHAVIOUR_IGNORECASE);
        assert_eq!(
            SC_CASEINSENSITIVEBEHAVIOUR_IGNORECASE,
            control.autoc_get_case_in_sensitive_behaviour()
        );
        control.autoc_set_multi(MultiAutoc::Once);
        assert_eq!(MultiAutoc::Once, control.autoc_get_multi());
        control.autoc_set_order(Order::Presorted);
        assert_eq!(Order::Presorted, control.autoc_get_order());
        control.autoc_set_auto_hide(true);
        assert_eq!(true, control.autoc_get_auto_hide());
        control.autoc_set_drop_rest_of_word(true);
        assert_eq!(true, control.autoc_get_drop_rest_of_word());
        /* has bugs
        const IMAGE_XPM: [&str; 4] = [
            /* columns rows colors chars-per-pixel */
            "2 2 1 1 ",
            "  c white",
            /* pixels */
            "  ",
            "  "
        ];
        control.register_image(1, &IMAGE_XPM);
        */
        control.register_rgba_image(1, &[0, 0, 0, 0]);
        control.clear_registered_images();
        control.autoc_set_type_separator(59);
        assert_eq!(59, control.autoc_get_type_separator());
        control.autoc_set_max_height(5);
        assert_eq!(5, control.autoc_get_max_height());
        control.autoc_set_max_width(10);
        assert_eq!(10, control.autoc_get_max_width());
        control.user_list_show(1, "ab cd ef gh".to_string());
        control.call_tip_show(7, "qqq\nwww".to_string());
        control.call_tip_cancel();
        dbg!(control.call_tip_active());
        control.call_tip_set_pos_start(12);
        assert_eq!(12, control.call_tip_pos_start());
        control.call_tip_set_hlt(10, 14);
        control.call_tip_set_back(0xfefefe);
        control.call_tip_set_fore(0x1d1d1d);
        control.call_tip_set_fore_hlt(0x1d1dff);
        control.call_tip_use_style(5);
        control.call_tip_set_position(true);
        control.line_down();
        control.line_down_extend();
        control.line_down_rect_extend();
        control.line_scroll_down();
        control.line_up();
        control.line_up_extend();
        control.line_up_rect_extend();
        control.line_scroll_up();
        control.para_down();
        control.para_down_extend();
        control.para_up();
        control.para_up_extend();
        control.char_left();
        control.char_left_extend();
        control.char_left_rect_extend();
        control.char_right();
        control.char_right_extend();
        control.char_right_rect_extend();
        control.home();
        control.home_extend();
        control.home_rect_extend();
        control.home_display();
        control.home_display_extend();
        control.home_wrap();
        control.home_wrap_extend();
        control.vc_home();
        control.vc_home_extend();
        control.vc_home_rectextend();
        control.vc_home_wrap();
        control.vc_home_wrap_extend();
        control.vc_home_display();
        control.vc_home_display_extend();
        control.line_end();
        control.line_end_extend();
        control.line_end_rect_extend();
        control.line_end_display();
        control.line_end_display_extend();
        control.line_end_wrap();
        control.line_end_wrap_extend();
        control.document_start();
        control.document_start_extend();
        control.document_end();
        control.document_end_extend();
        control.page_up();
        control.page_up_extend();
        control.page_up_rect_extend();
        control.page_down();
        control.page_down_extend();
        control.page_down_rect_extend();
        control.stuttered_page_up();
        control.stuttered_page_up_extend();
        control.stuttered_page_down();
        control.stuttered_page_down_extend();
        control.delete_back();
        control.delete_back_not_line();
        control.del_line_left();
        control.del_line_right();
        control.line_delete();
        control.line_cut();
        control.line_copy();
        control.line_transpose();
        control.line_reverse();
        control.line_duplicate();
        control.lower_case();
        control.upper_case();
        control.cancel();
        control.edit_toggle_overtype();
        control.new_line();
        control.form_feed();
        control.tab();
        control.back_tab();
        control.selection_duplicate();
        control.vertical_centre_caret();
        control.scroll_tostart();
        control.scroll_to_end();
        control.assign_cmd_key(SCMOD_SUPER, KeyCode::Escape, SCI_COPYTEXT);
        control.clear_cmd_key(SCMOD_SUPER, KeyCode::Escape);
        control.clear_all_cmd_keys();
        control.use_pop_up(PopUpMode::All);
        control.start_record();
        control.stop_record();
        dbg!(control.format_range(
            false,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            Rectangle {
                left: 0,
                top: 0,
                right: 100,
                bottom: 100,
            },
            Rectangle {
                left: 0,
                top: 0,
                right: 100,
                bottom: 100,
            },
            0,
            20,
        ));
        control.set_print_magnification(2);
        assert_eq!(2, control.get_print_magnification());
        control.set_print_colour_mode(PrintMode::BlackOnWhite);
        assert_eq!(PrintMode::BlackOnWhite, control.get_print_colour_mode());
        control.set_print_wrap_mode(WrapMode::Word);
        assert_eq!(WrapMode::Word, control.get_print_wrap_mode());
        dbg!(control.get_direct_function());
        dbg!(control.get_direct_pointer());
        dbg!(control.get_character_pointer());
        dbg!(control.get_range_pointer(5, 8));
        dbg!(control.get_gap_position());
        dbg!(control.get_doc_pointer());
        control.set_doc_pointer(0);
        dbg!(control.create_document(20, SC_DOCUMENTOPTION_DEFAULT));
        // has bugs
        // control.add_ref_document(0);
        // control.release_document(0);
        dbg!(control.create_loader(20, SC_DOCUMENTOPTION_DEFAULT));
        dbg!(control.visible_from_doc_line(0));
        control.show_lines(0, 1);
        control.hide_lines(0, 1);
        dbg!(control.get_line_visible(0));
        dbg!(control.get_all_lines_visible());
        control.set_fold_level(0, SC_FOLDLEVELHEADERFLAG);
        assert_eq!(
            SC_FOLDLEVELHEADERFLAG,
            control.get_fold_level(0) & SC_FOLDLEVELHEADERFLAG
        );
        control.set_fold_flags(SC_FOLDFLAG_LINEBEFORE_CONTRACTED);
        dbg!(control.get_last_child(1, SC_FOLDLEVELHEADERFLAG));
        dbg!(control.get_fold_parent(0));
        control.toggle_fold(0);
        control.toggle_fold_show_text(0, "折叠".to_string());
        control.fold_display_text_set_style(FoldDisplayText::Boxed);
        control.set_fold_expanded(0, true);
        assert_eq!(true, control.get_fold_expanded(0));
        control.fold_line(0, FoldAction::Toggle);
        control.fold_children(0, FoldAction::Toggle);
        control.fold_all(FoldAction::Toggle);
        control.expand_children(0, SC_FOLDLEVELHEADERFLAG);
        control.set_automatic_fold(SC_AUTOMATICFOLD_CLICK);
        assert_eq!(SC_AUTOMATICFOLD_CLICK, control.get_automatic_fold());
        dbg!(control.contracted_fold_next(0));
        control.ensure_visible(0);
        control.ensure_visible_enforce_policy(0);
        control.set_wrap_mode(WrapMode::Char);
        assert_eq!(WrapMode::Char, control.get_wrap_mode());
        control.set_wrap_visual_flags(SC_WRAPVISUALFLAG_MARGIN);
        assert_eq!(
            SC_WRAPVISUALFLAG_MARGIN,
            control.get_wrap_visual_flags() & SC_WRAPVISUALFLAG_MARGIN
        );
        control.set_wrap_visual_flags_location(SC_WRAPVISUALFLAGLOC_START_BY_TEXT);
        assert_eq!(
            SC_WRAPVISUALFLAGLOC_START_BY_TEXT,
            control.get_wrap_visual_flags_location() & SC_WRAPVISUALFLAGLOC_START_BY_TEXT
        );
        control.set_wrap_indent_mode(WrapIndent::DeepIndent);
        assert_eq!(WrapIndent::DeepIndent, control.get_wrap_indent_mode());
        control.set_wrap_start_indent(12);
        assert_eq!(12, control.get_wrap_start_indent());
        control.set_layout_cache(CacheMode::Document);
        assert_eq!(CacheMode::Document, control.get_layout_cache());
        control.set_position_cache(30);
        assert_eq!(30, control.get_position_cache());
        control.lines_split(50);
        control.lines_join();
        dbg!(control.wrap_count(0));
        control.zoom_in();
        control.zoom_out();
        control.set_zoom(5);
        assert_eq!(5, control.get_zoom());
        control.set_edge_mode(EdgeMode::Line);
        assert_eq!(EdgeMode::Line, control.get_edge_mode());
        control.set_edge_column(4);
        assert_eq!(4, control.get_edge_column());
        control.set_edge_colour(0xccefef);
        assert_eq!(0xccefef, control.get_edge_colour());
        control.multi_edge_add_line(3, 0xc2c2ef);
        control.multi_edge_clear_all();
        control.set_accessibility(Accessibility::Enabled);
        dbg!(control.get_accessibility());
        dbg!(control.get_lexer());
        dbg!(control.get_lexer_language());
        control.colourise(3, 6);
        dbg!(control.change_lexer_state(4, 7));
        dbg!(control.property_names());
        dbg!(control.property_type("fold".to_string()));
        dbg!(control.describe_property("fold".to_string()));
        control.set_property("fold".to_string(), "1".to_string());
        dbg!(control.get_property_expanded("fold".to_string()));
        dbg!(control.get_property_int("fold".to_string(), 0));
        dbg!(control.get_property("fold".to_string()));
        control.set_keywords(0, "pub unsafe".to_string());
        dbg!(control.describe_keyword_sets());
        dbg!(control.get_substyle_bases());
        dbg!(control.distance_to_secondary_styles());
        dbg!(control.allocate_substyles(1024, 2));
        control.free_substyles();
        dbg!(control.get_substyles_start(1024));
        dbg!(control.get_substyles_length(1024));
        dbg!(control.get_style_from_substyle(2));
        dbg!(control.get_primary_style_from_style(2));
        control.set_identifiers(2, "x y z".to_string());
        dbg!(control.private_lexer_call(0, 0));
        dbg!(control.get_named_styles());
        dbg!(control.name_of_style(2));
        dbg!(control.tags_of_style(2));
        dbg!(control.description_of_style(2));
        dbg!(control);
    }
}
