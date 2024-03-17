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
    common::{get_class_name, Result, HMODULE, HWND},
    message::message_loop,
    msaa::object::AccessibleObject,
};
use std::{
    fmt::{Debug, Formatter},
    sync::{Arc, RwLock},
    thread,
    time::SystemTime,
};
pub use windows::Win32::UI::Accessibility::{HWINEVENTHOOK, WINEVENTPROC};
use windows::Win32::UI::{
    Accessibility::{SetWinEventHook, UnhookWinEvent},
    WindowsAndMessaging::{EVENT_MAX, EVENT_MIN, WINEVENT_OUTOFCONTEXT},
};

//noinspection StructuralWrap
//noinspection SpellCheckingInspection
/**
 * 为一系列事件设置事件挂钩函数。
 * `event_min` 指定挂钩函数处理的事件范围中最低事件值的事件常量。此参数可以设置为EVENT_MIN，以指示可能的最低事件值。
 * `event_max` 指定由挂钩函数处理的事件范围中最高事件值的事件常量。此参数可以设置为EVENT_MAX，以指示可能的最高事件值。
 * `h_mod_win_event_proc` 如果在flags参数中指定了WINEVENT_INCONTEXT标志，则为包含fn_win_event_proc 中的挂钩函数的DLL的句柄。如果挂钩函数不位于DLL中，或者指定了WINEVENT_OUTOFCONTEXT标志，则此参数为NULL。
 * `fn_win_event_proc` 指向事件挂钩函数的指针。有关此函数的详细信息，请参阅WinEventProc。
 * `id_process` 指定挂钩函数从中接收事件的进程的ID。指定零 (0) 从当前桌面上的所有进程接收事件。
 * `id_thread` 指定挂钩函数从中接收事件的线程的ID。如果此参数为零，则挂钩函数与当前桌面上的所有现有线程相关联。
 * `flags` 标记值，用于指定要跳过的挂钩函数和事件的位置。
 * 以下标志有效：
 * - WINEVENT_INCONTEXT 包含回调函数的DLL映射到生成事件的进程的地址空间中。使用此标志，系统会在事件通知发生时向回调函数发送事件通知。指定此标志时，挂钩函数必须位于DLL中。当调用进程和生成进程都不是32位或64位进程，或者生成进程是控制台应用程序时，此标志不起作用。有关详细信息，请参阅上下文中挂钩函数。
 * - WINEVENT_OUTOFCONTEXT 回调函数不会映射到生成事件的进程的地址空间中。由于挂钩函数是跨进程边界调用的，因此系统必须对事件进行排队。虽然此方法是异步的，但事件保证按顺序排列。有关详细信息，请参阅上下文外挂钩函数。
 * - WINEVENT_SKIPOWNPROCESS 防止挂钩的此实例接收此进程中线程生成的事件。此标志不会阻止线程生成事件。
 * - WINEVENT_SKIPOWNTHREAD 防止此挂钩实例接收注册此挂钩的线程生成的事件。
 * 以下标志组合有效：
 * • WINEVENT_INCONTEXT |WINEVENT_SKIPOWNPROCESS
 * • WINEVENT_INCONTEXT |WINEVENT_SKIPOWNTHREAD
 * • WINEVENT_OUTOFCONTEXT |WINEVENT_SKIPOWNPROCESS
 * • WINEVENT_OUTOFCONTEXT |WINEVENT_SKIPOWNTHREAD
 * 此外，客户端应用程序可以指定WINEVENT_INCONTEXT或单独WINEVENT_OUTOFCONTEXT。
 * */
pub fn set_win_event_hook(
    event_min: u32,
    event_max: u32,
    h_mod_win_event_proc: HMODULE,
    fn_win_event_proc: WINEVENTPROC,
    id_process: u32,
    id_thread: u32,
    flags: u32,
) -> HWINEVENTHOOK {
    unsafe {
        SetWinEventHook(
            event_min,
            event_max,
            h_mod_win_event_proc,
            fn_win_event_proc,
            id_process,
            id_thread,
            flags,
        )
    }
}

/**
 * 删除由上一次调用 set_win_event_hook 创建的事件挂钩函数。
 * `h_win_event_hook` 在上一次调用 set_win_event_hook 时返回的事件挂钩的句柄。
 * */
pub fn unhook_win_event(h_win_event_hook: HWINEVENTHOOK) -> bool {
    unsafe { UnhookWinEvent(h_win_event_hook) }.as_bool()
}

static H_WIN_EVENT: RwLock<HWINEVENTHOOK> = RwLock::new(HWINEVENTHOOK(0));
static EVENTS: RwLock<Vec<WinEventHook>> = RwLock::new(vec![]);

#[derive(Clone)]
#[allow(dead_code)]
pub struct WinEventSource {
    pub h_wnd: HWND,
    pub id_object: i32,
    pub id_child: i32,
    pub id_thread: u32,
    pub ms_time: u32,
}

impl WinEventSource {
    /**
     * 获取事件对应的可访问性对象。
     * */
    pub fn get_object(&self) -> Result<(AccessibleObject, i32)> {
        AccessibleObject::from_event(self.h_wnd, self.id_object, self.id_child)
    }

    /**
     * 获取事件对应窗口的类名。
     * */
    pub fn get_class_name(&self) -> String {
        get_class_name(self.h_wnd)
    }
}

impl Debug for WinEventSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WinEventSource(window:[{}, {}], object:[{}, {}], time:{})",
            self.h_wnd.0,
            self.get_class_name(),
            self.id_object,
            self.id_child,
            self.ms_time
        )
    }
}

unsafe extern "system" fn hook_proc(
    _: HWINEVENTHOOK,
    event: u32,
    h_wnd: HWND,
    id_object: i32,
    id_child: i32,
    id_event_thread: u32,
    ms_event_time: u32,
) {
    let source = WinEventSource {
        h_wnd,
        id_object,
        id_child,
        id_thread: id_event_thread,
        ms_time: ms_event_time,
    };
    let lock = EVENTS.read().unwrap();
    for i in lock.iter() {
        if event == i.2 {
            (&*i.0)(source.clone())
        }
    }
}

#[derive(Clone)]
pub struct WinEventHook(Arc<dyn Fn(WinEventSource) + Send + Sync>, SystemTime, u32);

impl WinEventHook {
    /**
     * 创建一个事件钩子实例。
     * `event` 事件类型。
     * `func` 接收事件的函数。
     * */
    pub fn new(event: u32, func: impl Fn(WinEventSource) + Send + Sync + 'static) -> Self {
        let h_win_event = { *H_WIN_EVENT.read().unwrap() };
        if h_win_event.is_invalid() {
            thread::spawn(|| {
                let mut lock = H_WIN_EVENT.write().unwrap();
                if !lock.is_invalid() {
                    return;
                }
                let handle = set_win_event_hook(
                    EVENT_MIN,
                    EVENT_MAX,
                    HMODULE::default(),
                    Some(hook_proc),
                    0,
                    0,
                    WINEVENT_OUTOFCONTEXT,
                );
                *lock = handle;
                drop(lock);
                message_loop(|_| ());
                unhook_win_event(handle);
                *H_WIN_EVENT.write().unwrap() = HWINEVENTHOOK(0);
            });
        }
        let self_ = Self(Arc::new(func), SystemTime::now(), event);
        EVENTS.write().unwrap().push(self_.clone());

        self_
    }
    pub fn unhook(&self) {
        let mut lock = EVENTS.write().unwrap();
        for i in 0..lock.len() {
            if let Some(x) = lock.get(i) {
                if x == self {
                    lock.remove(i);
                }
            }
        }
        if lock.is_empty() {
            drop(lock);
            let mut lock = H_WIN_EVENT.write().unwrap();
            unhook_win_event(*lock);
            *lock = HWINEVENTHOOK::default();
        }
    }
}

impl Debug for WinEventHook {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "WinEventHook({})", self.2)
    }
}

impl PartialEq for WinEventHook {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}
