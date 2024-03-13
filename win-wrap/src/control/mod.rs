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

pub mod edit;

use std::{
    ffi::CString,
    fmt::{Debug, Formatter},
};

use crate::{
    common::{HWND, LPARAM, WPARAM},
    ext::StringExt,
    graphic::HDC,
    message::{send_message_timeout, SMTO_ABORTIFHUNG},
};
use windows::core::imp::{heap_alloc, heap_free};
pub use windows::Win32::UI::WindowsAndMessaging::{
    PRF_CHECKVISIBLE, PRF_CHILDREN, PRF_CLIENT, PRF_ERASEBKGND, PRF_NONCLIENT, PRF_OWNED, WM_CLEAR,
    WM_CLOSE, WM_COPY, WM_CUT, WM_GETTEXT, WM_GETTEXTLENGTH, WM_PAINT, WM_PASTE, WM_PRINT,
    WM_SETREDRAW, WM_SETTEXT, WM_UNDO,
};

#[macro_export]
macro_rules! sm {
    ($self:expr,$msg:expr,$wp:expr,$lp:expr) => {
        send_message_timeout($self.0, $msg, $wp, $lp, SMTO_ABORTIFHUNG, 500)
    };
}

pub struct WindowControl(HWND);

impl From<HWND> for WindowControl {
    fn from(value: HWND) -> Self {
        Self(value)
    }
}

impl Debug for WindowControl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(text) = self.get_text() {
            write!(f, "WindowControl(text:{})", text)
        } else {
            write!(f, "WindowControl()")
        }
    }
}

impl WindowControl {
    /**
     * def_window_proc 函数将与窗口关联的文本复制到指定的缓冲区中，并返回复制的字符数。
     * 请注意，对于非文本静态控件，这会提供最初创建控件时使用的文本，即 ID 号。
     * 但是，它提供最初创建的非文本静态控件的 ID。 也就是说，如果随后使用 STM_SETIMAGE 对其进行更改，则仍会返回原始 ID。
     * 对于编辑控件，要复制的文本是编辑控件的内容。
     * 对于组合框，文本是编辑控件 (或静态文本) 组合框部分的内容。
     * 对于按钮，文本是按钮名称。
     * 对于其他窗口，文本是窗口标题。
     * 若要复制列表框中项的文本，应用程序可以使用 LB_GETTEXT 消息。
     * 将 WM_GETTEXT 消息发送到具有 SS_ICON 样式的静态控件时， 将在 lParam 指向的缓冲区前四个字节中返回图标的句柄。
     * 仅当 WM_SETTEXT 消息已用于设置图标时，才如此。
     * 富编辑： 如果要复制的文本超过 64K，请使用 EM_STREAMOUT 或 EM_GETSELTEXT 消息。
     * 向非文本静态控件（如静态位图或静态图标控件）发送 WM_GETTEXT 消息不会返回字符串值。相反，它返回零。
     * 此外，在早期版本的 Windows 中，应用程序可以将 WM_GETTEXT 消息发送到非文本静态控件，以检索控件的 ID。
     * 若要检索控件的 ID，应用程序可以使用 get_window_long 传递GWL_ID作为索引值，或使用 GWLP_ID传递 get_window_long_ptr。
     * */
    pub fn get_text(&self) -> Option<String> {
        let (_, len) = sm!(self, WM_GETTEXTLENGTH, WPARAM::default(), LPARAM::default());
        let Ok(ptr) = heap_alloc(len * 2 + 1) else {
            return None;
        };
        unsafe { ptr.write_bytes(b'\0', len * 2 + 1) };

        sm!(self, WM_GETTEXT, WPARAM(len + 1), LPARAM(ptr as isize));
        let text = (ptr as *const u16).to_string_utf16();
        unsafe {
            heap_free(ptr);
        }
        Some(text)
    }

    /**
     * 将 WM_SETREDRAW 消息发送到窗口，以允许重绘该窗口中的更改，或防止重绘该窗口中的更改。
     * 如果应用程序处理此消息，则它应返回 0。
     * 如果应用程序必须向列表框添加多个项，则此消息可能很有用。
     * 应用程序可以在 `enabled` 设置为 false 的情况下调用此消息，添加项，然后在 `enabled` 设置为 true 的情况下再次调用该消息。
     * 最后，应用程序可以调用 redraw_window (hWnd、 NULL、 NULL RDW_ERASE |RDW_FRAME |RDW_INVALIDATE |RDW_ALLCHILDREN) 会导致重新绘制列表框。
     * 应将 redraw_window 与指定的标志一起使用，而不是 invalidate_rect，因为前者对于某些控件而言是必需的，这些控件具有自己的非工作区，或者具有导致它们获得非工作区 (（如 WS_THICKFRAME、 WS_BORDER 或 WS_EX_CLIENTEDGE) ）。
     * 如果控件没有非工作区，则带这些标志的 redraw_window 将只执行 与 invalidate_rect 一样多的失效。
     * 当 `enabled` 设置为 false 时，将WM_SETREDRAW消息传递给 def_window_proc 函数会从窗口中删除WS_VISIBLE样式。
     * 尽管窗口内容在屏幕上保持可见，但在此状态下的窗口上调用 is_window_visible 函数时返回 FALSE 。
     * 当 `enabled` 设置为 true 时，将WM_SETREDRAW消息传递给 def_window_proc 函数会将WS_VISIBLE样式添加到窗口（如果未设置）。
     * 如果应用程序将 `enabled` 设置为 true的WM_SETREDRAW消息发送到隐藏窗口，则该窗口将变为可见。
     * Windows 10 及更高版本;Windows Server 2016 及更高版本。
     * 系统在窗口上设置名为 SysSetRedraw 的属性，该窗口的窗口过程将 WM_SETREDRAW 消息传递给 def_window_proc。
     * 可以使用 get_prop 函数获取属性值（如果可用）。
     * 禁用重绘时，get_prop 返回非零值。
     * 启用重绘或窗口属性不存在时，get_prop 将返回零。
     * `enabled` 重绘状态。 如果此参数为 true，则可以在更改后重绘内容。 如果此参数为 false，则更改后无法重绘内容。
     * */
    pub fn set_redraw(&self, enabled: bool) -> usize {
        let enabled = if enabled { WPARAM(1) } else { WPARAM(0) };
        let (_, res) = sm!(self, WM_SETREDRAW, enabled, LPARAM::default());
        res
    }

    /**
     * 设置窗口的文本。
     * 如果设置了文本，则返回值为 true 。如果没有足够的空间来设置编辑控件中的文本，则为 FALSE (编辑控件) ， LB_ERRSPACE (列表框);CB_ERRSPACE (组合框)。 如果此消息发送到没有编辑控件的组合框，则会 CB_ERR 。
     * def_window_proc 函数设置并显示窗口文本。
     * 对于编辑控件，文本是编辑控件的内容。
     * 对于组合框，文本是组合框编辑控件部分的内容。
     * 对于按钮，文本是按钮名称。
     * 对于其他窗口，文本是窗口标题。
     * 此消息不会更改组合框的列表框中的当前选择。
     * 应用程序应使用 CB_SELECTSTRING 消息在列表框中选择与编辑控件中的文本匹配的项。
     * */
    pub fn set_text(&self, text: &str) -> bool {
        let text = CString::new(text).unwrap();
        let (_, res) = sm!(
            self,
            WM_SETTEXT,
            WPARAM::default(),
            LPARAM(text.as_ptr() as isize)
        );
        res != 0
    }

    /**
     * 当系统或其他应用程序请求绘制应用程序窗口的一部分时，将发送 WM_PAINT 消息。
     * 调用 update_window 或 redraw_window 函数时发送消息，当应用程序使用 get_message 或 peek_message 函数获取WM_PAINT消息时，将发送该消息。
     * 窗口通过其 WindowProc 函数接收此消息。
     * 如果应用程序处理此消息，则返回零。
     * WM_PAINT消息由系统生成，不应由应用程序发送。
     * 若要强制窗口绘制到特定设备上下文中，请使用 WM_PRINT 或 WM_PRINTCLIENT 消息。
     * 请注意，这需要目标窗口支持 WM_PRINTCLIENT 消息。
     * 大多数常用控件支持 WM_PRINTCLIENT 消息。
     * def_window_proc 函数验证更新区域。
     * 如果必须绘制窗口框架，函数还可以将 WM_NCPAINT 消息发送到窗口过程，如果必须擦除窗口背景，则发送 WM_ERASEBKGND 消息。
     * 当应用程序的消息队列中没有其他消息时，系统会发送此消息。
     * dispatch_message 确定消息的发送位置;
     * get_message 确定要调度的消息。
     * 当应用程序的消息队列中没有其他消息时，get_message 将返回WM_PAINT消息，并且 dispatch_message 会将消息发送到相应的窗口过程。
     * 由于调用 redraw_window 并设置了RDW_INTERNALPAINT标志，窗口可能会收到内部绘制消息。
     * 在这种情况下，窗口可能不包含更新区域。
     * 应用程序可以调用 get_update_rect 函数来确定窗口是否具有更新区域。
     * 如果 get_update_rect 返回零，则应用程序无需调用 begin_paint 和 end_paint 函数。
     * 应用程序必须通过查看每个WM_PAINT消息的内部数据结构来检查任何必要的内部绘制，因为WM_PAINT消息可能是由非 NULL 更新区域和调用 redraw_window 以及设置了RDW_INTERNALPAINT标志的 redraw_window 引起的。
     * 系统仅发送一次内部 WM_PAINT 消息。
     * 从 get_message 或 peek_message 返回内部WM_PAINT消息或由 update_window 发送到窗口后，系统不会发布或发送进一步WM_PAINT消息，直到窗口失效或重新调用 redraw_window 并设置RDW_INTERNALPAINT标志。
     * 对于某些常见控件，默认 WM_PAINT 消息处理会检查 wParam 参数。
     * 如果 wParam 为非 NULL，则控件假定该值为 HDC，并使用该设备上下文进行绘制。
     * */
    pub fn paint(&self) -> usize {
        let (_, res) = sm!(self, WM_PAINT, WPARAM::default(), LPARAM::default());
        res
    }

    /**
     * WM_PRINT消息将发送到窗口，请求它在指定的设备上下文（最常见的是打印机设备上下文）中绘制自身。
     * 窗口通过其 WindowProc 函数接收此消息。
     * `h_dc` 要绘制的设备上下文的句柄。
     * `options` 绘图选项。 此参数可使用以下一个或多个值。
     * - PRF_CHECKVISIBLE 仅当窗口可见时，才会绘制该窗口。
     * - PRF_CHILDREN 绘制所有可见子窗口。
     * - PRF_CLIENT 绘制窗口的工作区。
     * - PRF_ERASEBKGND 在绘制窗口之前擦除背景。
     * - PRF_NONCLIENT 绘制窗口的非工作区。
     * - PRF_OWNED 绘制所有拥有的窗口。
     * def_window_proc 函数根据指定的绘图选项处理此消息：如果指定了PRF_CHECKVISIBLE并且窗口不可见，则不执行任何操作，如果指定了PRF_NONCLIENT，则绘制指定设备上下文中的非工作区，如果指定了PRF_ERASEBKGND，则向窗口发送WM_ERASEBKGND消息，如果指定了PRF_CLIENT， 向窗口发送WM_PRINTCLIENT消息，如果设置了PRF_CHILDREN，则向每个可见子窗口发送一条WM_PRINT消息，如果设置了PRF_OWNED，则向每个可见的拥有窗口发送一条WM_PRINT消息。
     * */
    pub fn print(&self, h_dc: HDC, options: i32) -> usize {
        let (_, res) = sm!(
            self,
            WM_PRINT,
            WPARAM(h_dc.0 as usize),
            LPARAM(options as isize)
        );
        res
    }
    /**
     * 作为窗口或应用程序应终止的信号发送。
     * 窗口通过其 WindowProc 函数接收此消息。
     * 如果应用程序处理此消息，则它应返回零。
     * 应用程序可以在销毁窗口之前提示用户进行确认，方法是处理 WM_CLOSE 消息，并仅在用户确认选择时调用 destroy_window 函数。
     * 默认情况下， def_window_proc 函数调用 destroy_window 函数来销毁窗口。
     * */
    pub fn close(&self) -> usize {
        let (_, res) = sm!(self, WM_CLOSE, WPARAM::default(), LPARAM::default());
        res
    }

    /**
     * 应用程序将 WM_CUT 消息发送到编辑控件或组合框，以删除 (剪切) 编辑控件中的当前选定内容（如果有），并将已删除的文本以 CF_TEXT 格式复制到剪贴板。
     * 可以通过向编辑控件发送EM_UNDO消息来撤消WM_CUT消息执行的删除操作。
     * 若要删除当前选定内容而不将已删除的文本放在剪贴板上，请使用 WM_CLEAR 消息。
     * 发送到组合框时， WM_CUT 消息由其编辑控件处理。 发送到具有 CBS_DROPDOWNLIST 样式的组合框时，此消息无效。
     * */
    pub fn cut(&self) {
        sm!(self, WM_CUT, WPARAM::default(), LPARAM::default());
    }

    /**
     * 此消息由应用程序发送到编辑控件或组合框，以CF_UNICODETEXT格式将当前选定内容复制到剪贴板。
     * 当发送到组合框时，WM_COPY消息由其编辑控件处理。此消息在发送到具有CBS_DROPDOWNLIST样式的组合框时不起作用。
     * */
    pub fn copy(&self) {
        sm!(self, WM_COPY, WPARAM::default(), LPARAM::default());
    }

    /**
     * 应用程序将 WM_PASTE 消息发送到编辑控件或组合框，以将剪贴板的当前内容复制到位于当前插入点位置的编辑控件。 仅当剪贴板包含 CF_TEXT 格式的数据时，才会插入数据。
     * 发送到组合框时， WM_PASTE 消息由其编辑控件处理。 发送到具有 CBS_DROPDOWNLIST 样式的组合框时，此消息无效。
     */
    pub fn paste(&self) {
        sm!(self, WM_PASTE, WPARAM::default(), LPARAM::default());
    }

    /**
     * 应用程序将 WM_CLEAR 消息发送到编辑控件或组合框，以从编辑控件中删除 (清除) 当前选择（如果有）。
     * 可以通过向编辑控件发送EM_UNDO消息来撤消WM_CLEAR消息执行的删除操作。
     * 若要删除当前选定内容并将已删除的内容放在剪贴板上，请使用 WM_CUT 消息。
     * 发送到组合框时， WM_CLEAR 消息由其编辑控件处理。 发送到具有 CBS_DROPDOWNLIST 样式的组合框时，此消息不起作用。
     * */
    pub fn clear(&self) {
        sm!(self, WM_CLEAR, WPARAM::default(), LPARAM::default());
    }

    /**
     * 应用程序将 WM_UNDO 消息发送到编辑控件以撤消最后一个操作。 将此消息发送到编辑控件时，将还原以前删除的文本或删除以前添加的文本。
     * 如果消息成功，则返回值为 true。如果消息失败，则返回值为 false。
     * Rich Edit： 建议使用 EM_UNDO 而不是 WM_UNDO。
     * */
    pub fn undo(&self) -> bool {
        let (_, res) = sm!(self, WM_UNDO, WPARAM::default(), LPARAM::default());
        res != 0
    }
}

#[cfg(test)]
mod test_control {
    use crate::{
        common::{find_window_ex, HWND},
        control::{
            edit::{Edit, EC_LEFTMARGIN, FR_FINDNEXT, SB_LINEDOWN},
            WindowControl,
        },
    };

    #[test]
    fn main() {
        let h_wnd = find_window_ex(HWND::default(), HWND::default(), Some("Notepad++"), None);
        let h_wnd = find_window_ex(h_wnd, HWND::default(), Some("Scintilla"), None);

        let control = WindowControl::from(h_wnd);
        dbg!(control.set_redraw(true));
        assert!(control.set_text("hello你好"));
        for _ in 0..100 {
            assert_eq!(control.get_text(), Some(String::from("hello你好")));
        }
        dbg!(control.paint());
        // dbg!(control.close());
        control.cut();
        control.copy();
        control.clear();
        control.paste();
        dbg!(control.undo());
        test_edit(&control);
        dbg!(control);
    }

    pub fn test_edit(control: &WindowControl) {
        dbg!(control.get_line(0, 2));
        dbg!(control.line_length(0));
        dbg!(control.line_index(0));
        control.replace_sel(true, "123");
        dbg!(control.set_readonly(false));
        // dbg!(control.get_text_range(0, 4));
        control.empty_undo_buffer();
        dbg!(control.can_undo());
        dbg!(control.get_first_visible_line());
        dbg!(control.get_line_count());
        control.set_modify(true);
        dbg!(control.get_modify());
        dbg!(control.get_rect());
        control.set_sel(2, 4);
        // dbg!(control.set_sel_ex(3, 5));
        dbg!(control.get_sel());
        // dbg!(control.get_sel_ex());
        // dbg!(control.get_sel_text(3));
        dbg!(control.line_from_char(3));
        dbg!(control.line_from_char_ex(2));
        dbg!(control.scroll(SB_LINEDOWN));
        dbg!(control.line_scroll(2, 1));
        control.scroll_caret();
        dbg!(control.can_paste());
        dbg!(control.char_from_pos(40, 40));
        dbg!(control.pos_from_char(1));
        dbg!(control.selection_type());
        control.hide_selection(false);
        dbg!(control.find_text(FR_FINDNEXT, "l", 0, -1));
        dbg!(control.find_text_ex(FR_FINDNEXT, "l", 0, -1));
        control.set_margins(EC_LEFTMARGIN, 10);
        dbg!(control.get_margins());
    }
}
