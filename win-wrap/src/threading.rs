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
    common::{close_handle, BOOL, FALSE, HANDLE, LPARAM, TRUE, WAIT_EVENT, WPARAM},
    message::post_thread_message,
};
use windows::{
    core::HSTRING,
    Win32::{
        System::Threading::{
            CreateEventW, GetCurrentThreadId, SetEvent, WaitForSingleObject,
        },
        UI::WindowsAndMessaging::WM_QUIT,
    }
};
pub use windows::Win32::Security::SECURITY_ATTRIBUTES;

/**
 * 查询调用线程的线程标识符。
 * */
pub fn get_current_thread_id() -> u32 {
    unsafe { GetCurrentThreadId() }
}

/**
 * 建立或开启具名或未命名的事件对象。
 * 若要指定对象的访问屏蔽，请使用 create_event_ex 函数。
 * `event_attributes` SECURITY_ATTRIBUTES结构。如果此参数为 Null，子进程就无法继承句柄。结构的 lpSecurityDescriptor 成员会指定新事件的安全性描述符。如果 lpEventAttributes 为 Null，事件会取得默认的安全性描述符。事件的默认安全性描述符中的 ACL 来自创建者的主要或模拟权限。
 * `manual_reset` 如果此参数为TRUE，此函数会建立手动重置事件对象，这需要使用 reset_event 函数将事件状态设定为非signaled。如果此参数为 FALSE，此函数会建立自动重置事件对象，而且系统会在释放单一等候线程之后，自动将事件状态重设为非signaled。
 * `initial_state` 如果此参数为TRUE，则会发出事件对象的初始状态信号;否则，则为非signaled。
 * `name` 事件对象的名称。名称限制为 MAX_PATH个字符。名称比较区分大小写。如果 name 符合现有具名事件对象的名称，此函式会要求 EVENT_ALL_ACCESS 访问权限。在此情况下，系统会忽略 manual_reset 和 initial_state 参数，因为它们已经由建立进程设定。如果 event_attributes 参数不是 Null，它会判断是否可以继承句柄，但会忽略其安全性描述符成员。如果 name 为 Null，则会建立事件对象，而不需要名称。如果 name 符合相同命名空间中另一种对象的名称，（例如现有的号志、mutex、可等候计时器、作业或文件对应对象），则函数会失败，而且 get_last_error 函数会传回 ERROR_INVALID_HANDLE。这是因为这些对象共享相同的命名空间。名称可以有 Global 或 「Local」 前置词，以在全局或会话命名空间中明确建立对象。名称的其余部分可以包含反斜线字符（\）以外的任何字符。使用终端机服务会话实作快速用户切换。核心对象名称必须遵循终端机服务概述的指导方针，让应用程序可以支持多个用户。对象可以在私用命名空间中建立。
 * */
pub fn create_event(
    event_attributes: Option<*const SECURITY_ATTRIBUTES>,
    manual_reset: BOOL,
    initial_state: BOOL,
    name: Option<&str>,
) -> HANDLE {
    unsafe {
        match name {
            None => CreateEventW(event_attributes, manual_reset, initial_state, None),
            Some(x) => CreateEventW(
                event_attributes,
                manual_reset,
                initial_state,
                &HSTRING::from(x),
            ),
        }
    }
    .expect("Can't create the event.")
}

/**
 * 设置事件的状态为signaled，释放任意等待线程。如果事件是手工的，此事件将保持signaled直到调用reset_event，这种情况下将释放多个线程；如果事件是自动的，此事件将保持signaled，直到一个线程被释放，系统将设置事件的状态为非signaled；如果没有线程在等待，则此事件将保持signaled，直到一个线程被释放。
 * `h_event` 事件句柄。
 */
pub fn set_event(h_event: HANDLE) {
    unsafe { SetEvent(h_event) }.expect("Can't set the event.")
}

/**
 * 等待指定的对象处于信号状态或超时间隔已过。若要进入可警报等待状态，请使用 wait_for_single_object_ex 函数。若要等待多个对象，请使用 wait_for_multiple_objects。
 * `h_handle` 对象的句柄。如果在等待仍处于挂起状态时关闭此句柄，则函数的行为未定义。句柄必须具有 SYNCHRONIZE 访问权限。
 * `milliseconds` 超时间隔（以毫秒为单位）。如果指定了非零值，则函数将等待，直到发出对象信号或间隔已过。如果 milliseconds 为零，则如果未向对象发出信号，则函数不会进入等待状态;它始终立即返回。如果 milliseconds 为 INFINITE，则函数仅在发出对象信号时返回。Windows XP、Windows Server 2003、Windows Vista、Windows 7、Windows Server 2008 和 Windows Server 2008 R2：milliseconds 值包括在低功率状态下花费的时间。例如，当计算机处于睡眠状态时，超时也会持续倒计时。Windows 8、Windows Server 2012、Windows 8.1、Windows Server 2012 R2、Windows 10和Windows Server 2016：milliseconds 值不包括在低功率状态下花费的时间。例如，当计算机处于睡眠状态时，超时会暂停倒计时。
 * */
pub fn wait_for_single_object(h_handle: HANDLE, milliseconds: u32) -> WAIT_EVENT {
    unsafe { WaitForSingleObject(h_handle, milliseconds) }
}

/* 在线程之间发送通知事件。 */
#[derive(Clone)]
pub struct ThreadNotify(u32, HANDLE);
impl ThreadNotify {
    /**
     * 创建一个通知对象。
     * `thread_id` 一个线程ID，此线程必须有一个消息循环。
     * */
    pub fn new(thread_id: u32) -> Self {
        let event = create_event(None, TRUE, FALSE, None);
        Self(thread_id, event)
    }

    /**
     * 往线程的消息队列中发送一个退出请求，然后此函数会立即返回。
     */
    pub fn quit(&self) {
        post_thread_message(self.0, WM_QUIT, WPARAM::default(), LPARAM::default());
    }

    /**
     * 等待finish函数被调用，一个应用场景就是主线程需要等待子线程是否接收到退出信号并调用了finish方法，当子线程调用finish方法时，join会返回。
     * `millis` 等待的超时时间（毫秒），如果在指定的时间内子线程没有调用过finish，join也会返回。
     * */
    pub fn join(&self, millis: u32) {
        wait_for_single_object(self.1, millis);
        close_handle(self.1);
    }

    /**
     * 通知join方法返回，这通常放在线程结束时调用。
     * */
    pub fn finish(&self) {
        set_event(self.1);
    }
}
