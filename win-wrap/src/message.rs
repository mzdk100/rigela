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

use crate::common::{BOOL, FALSE, HWND, LPARAM, LRESULT, WPARAM};
pub use windows::Win32::UI::WindowsAndMessaging::{
    HWND_BROADCAST, MSG, SEND_MESSAGE_TIMEOUT_FLAGS, SMTO_ABORTIFHUNG, SMTO_BLOCK,
    SMTO_ERRORONEXIT, SMTO_NORMAL, SMTO_NOTIMEOUTIFNOTHUNG, WM_QUIT,
};
use windows::{
    core::HSTRING,
    Win32::UI::WindowsAndMessaging::{
        DispatchMessageW, GetMessageW, PostThreadMessageW, RegisterWindowMessageW,
        SendMessageTimeoutW, SendMessageW, TranslateMessage,
    },
};

/**
 * 将消息调度到窗口过程。它通常用于调度 get_message 函数检索到的消息。
 * `msg` 消息结构。
 * */
pub fn dispatch_message(msg: &mut MSG) -> LRESULT {
    unsafe { DispatchMessageW(msg) }
}

/**
 * 从调用线程的消息队列中查询消息。函数调度传入的已发送消息，直到已发布的消息可供查询。
 * 与 get_message 不同， peek_message 函数在返回之前不会等待消息发布。
 * `msg` 消息结构，该结构从线程的消息队列接收消息信息。
 * `h_wnd` 要检索其消息的窗口的句柄。窗口必须属于当前线程。如果 h_wnd 为 NULL， get_message 将检索属于当前线程的任何窗口的消息，以及当前线程的消息队列中 h_wnd 值为 NULL 的任何消息。因此，如果 h_wnd 为 NULL，则同时处理窗口消息和线程消息。如果 h_wnd 为 -1，则 get_message 仅检索当前线程的消息队列中 h_wnd 值为 NULL 的消息，即当 h_wnd 参数为 NULL) 或 post_thread_message 时，post_message (发布的线程消息。
 * `msg_filter_min` 要检索的最低消息值的整数值。使用 WM_KEYFIRST (0x0100) 指定第一条键盘消息， 或使用WM_MOUSEFIRST (0x0200) 指定第一条鼠标消息。在此处和 msg_filter_max 中使用WM_INPUT仅指定WM_INPUT消息。如果 msg_filter_min 和 msg_filter_max 均为零， 则 get_message 将返回所有可用消息 (即不) 执行范围筛选。
 * `msg_filter_max` 要检索的最高消息值的整数值。使用 WM_KEYLAST 指定最后一条键盘消息， WM_MOUSELAST 指定最后一条鼠标消息。在此处和 msg_filter_min 中使用WM_INPUT，仅指定WM_INPUT消息。如果 msg_filter_min 和 msg_filter_max 均为零， 则 get_message 将返回所有可用消息 (即不) 执行范围筛选。
 * */
pub fn get_message(msg: &mut MSG, h_wnd: HWND, msg_filter_min: u32, msg_filter_max: u32) -> BOOL {
    unsafe { GetMessageW(msg, h_wnd, msg_filter_min, msg_filter_max) }
}

/**
 * 将消息发布到指定会话的消息队列。返回而不等待线程处理消息。
 * `id_thread` 将消息发布到的线程的标识符。如果指定线程上没有消息队列，则函数将失败。系统在首次调用 User 函数或 GDI 函数之一时为线程创建消息队列。发布消息受 UIPI 的约束。进程中的线程只能将消息发布到进程中具有较低或相同完整性级别的线程的已发布消息队列。此线程必须具有SE_TCB_NAME权限才能将消息发布到属于具有相同本地唯一标识符 （LUID） 但位于不同桌面上的进程的线程。否则，该函数将失败并返回ERROR_INVALID_THREAD_ID。此线程必须与调用线程属于同一桌面，或者必须属于具有相同 LUID 的进程。否则，该函数将失败并返回ERROR_INVALID_THREAD_ID。
 * `msg` 消息类型。
 * `w_param` 其他特定于消息的信息。
 * `l_param` 其他特定于消息的信息。
 * */
pub fn post_thread_message(id_thread: u32, msg: u32, w_param: WPARAM, l_param: LPARAM) {
    unsafe { PostThreadMessageW(id_thread, msg, w_param, l_param) }
        .expect("Can't post the message to the thread.")
}

/**
 * 该函数将指定的消息发送到一个或多个窗口。此函数为指定的窗口调用窗口程序，直到窗口程序处理完消息再返回。
 * 而和函数post_message不同，post_message是将一个消息寄送到一个线程的消息队列后就立即返回。
 * `h_wnd` 指定要接收消息的窗口的句柄。如果此参数为HWND_BROADCAST，则消息将被发送到系统中所有顶层窗口，包括无效或不可见的非自身拥有的窗口、被覆盖的窗口和弹出式窗口，但消息不被发送到子窗口。
 * `msg` 指定被发送的消息。
 * `w_param` 指定附加的消息特定信息。
 * `l_param` 指定附加的消息特定信息。
 * */
pub fn send_message(h_wnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    unsafe { SendMessageW(h_wnd, msg, w_param, l_param) }
}

//noinspection StructuralWrap,SpellCheckingInspection
/**
 * 将指定的消息发送到一个或多个窗口。
 * 此函数为指定的窗口调用窗口程序，并且，如果指定的窗口属于不同的线程，直到窗口程序处理完消息或指定的超时周期结束函数才返回。
 * 如果接收消息的窗口和当前线程属于同一个队列，窗口程序立即调用，超时值无用。
 * `h_wnd` 窗口程序将接收消息的窗口的句柄。如果此参数为HWND_BROADCAST，则消息将被发送到系统中所有顶层窗口，包括无效或不可见的非自身拥有的窗口。
 * `msg` 指定被发送的消息。
 * `w_param` 指定附加的消息指定信息。
 * `l_param` 指定附加的消息指定信息。
 * `flags` 指定如何发送消息。此参数可为下列值的组合：
 * - SMTO_ABORTIFHUNG：如果接收进程处于“hung”状态，不等待超时周期结束就返回。
 * - SMTO_BLOCK：阻止调用线程处理其他任何请求，直到函数返回。
 * - SMTO_NORMAL：调用线程等待函数返回时，不被阻止处理其他请求。
 * - SMTO_NOTIMEOUTIFNOTHUNG：Windows 95及更高版本：如果接收线程没被挂起，当超时周期结束时不返回。
 * `timeout` 为超时周期指定以毫秒为单位的持续时间。如果该消息是一个广播消息，每个窗口可使用全超时周期。例如，如果指定5秒的超时周期，有3个顶层窗口未能处理消息，可以有最多15秒的延迟。
 * */
pub fn send_message_timeout(
    h_wnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
    flags: SEND_MESSAGE_TIMEOUT_FLAGS,
    timeout: u32,
) -> (LRESULT, usize) {
    let mut result: usize = 0;
    (
        unsafe {
            SendMessageTimeoutW(
                h_wnd,
                msg,
                w_param,
                l_param,
                flags,
                timeout,
                Some(&mut result),
            )
        },
        result,
    )
}

/**
 * 将虚拟密钥信息转换为字符信息。字符信息会张贴至调用程序的消息队列，下次程序调用 get_message 或 peek_message 函数时要读取。
 * `msg` MSG结构，其中包含使用get_message或peek_message函数从调用程序消息队列撷取的信息。
 */
pub fn translate_message(msg: &mut MSG) -> BOOL {
    unsafe { TranslateMessage(msg) }
}

/**
 * 注册一个新窗口消息，保证在整个系统中唯一。发送或发布消息时可以使用消息值。
 * register_window_message 函数通常用于注册消息，以便在两个协作应用程序之间进行通信。
 * 如果两个不同的应用程序注册相同的消息字符串，则应用程序将返回相同的消息值。该消息将保持注册状态，直到会话结束。
 * 仅当多个应用程序必须处理同一消息时，才使用此函数。若要在窗口类中发送私人消息，应用程序可以使用 0x7FFF WM_USER 范围内的任何整数。(此范围内的消息是窗口类的专用消息，而不是应用程序。
 * 例如，预定义控件类（如 BUTTON、 EDIT、 LISTBOX 和 COMBOBOX）可能使用此范围中的值。)
 * `string` 消息字符串。
 * */
pub fn register_window_message(string: &str) -> u32 {
    unsafe { RegisterWindowMessageW(&HSTRING::from(string)) }
}
/**
 * 在当前线程上创建一个窗口消息循环，直到接收到WM_QUIT消息为止。
 * */
pub fn message_loop() {
    let mut msg = MSG::default();
    while get_message(&mut msg, HWND::default(), 0, 0) != FALSE {
        if msg.message == WM_QUIT {
            break;
        }
        dispatch_message(&mut msg);
        translate_message(&mut msg);
    }
}
