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
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, PostThreadMessageW, TranslateMessage,
};
pub use windows::Win32::UI::WindowsAndMessaging::{MSG, WM_QUIT};

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
 * 将虚拟密钥信息转换为字符信息。字符信息会张贴至调用程序的消息队列，下次程序调用 get_message 或 peek_message 函数时要读取。
 * `msg` MSG结构，其中包含使用get_message或peek_message函数从调用程序消息队列撷取的信息。
 */
pub fn translate_message(msg: &mut MSG) -> BOOL {
    unsafe { TranslateMessage(msg) }
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
