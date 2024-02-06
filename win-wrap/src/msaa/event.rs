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

use crate::msaa::object::AccessibleObject;
use crate::{
    common::{set_win_event_hook, unhook_win_event, HMODULE, HWINEVENTHOOK, HWND},
    message::message_loop,
};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::time::SystemTime;
use std::{sync::RwLock, thread};
use windows::Win32::UI::WindowsAndMessaging::{
    EVENT_MAX, EVENT_MIN, WINEVENT_OUTOFCONTEXT, WINEVENT_SKIPOWNPROCESS,
};

static H_WIN_EVENT: RwLock<HWINEVENTHOOK> = RwLock::new(HWINEVENTHOOK(0));
static EVENTS: RwLock<Vec<WinEventHook>> = RwLock::new(vec![]);

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct WinEventSource {
    h_wnd: HWND,
    id_object: i32,
    id_child: i32,
    id_thread: u32,
    ms_time: u32,
}

impl WinEventSource {
    /**
     * 获取事件对应的可访问性对象。
     * */
    pub fn get_object(&self) -> Result<(AccessibleObject, u32), String> {
        AccessibleObject::from_event(self.h_wnd, self.id_object as u32, self.id_child as u32)
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
pub(crate) struct WinEventHook(Arc<dyn Fn(WinEventSource) + Send + Sync>, SystemTime, u32);

impl WinEventHook {
    /**
     * 创建一个事件钩子实例。
     * `event` 事件类型。
     * `func` 接收事件的函数。
     * */
    pub(crate) fn new(event: u32, func: impl Fn(WinEventSource) + Send + Sync + 'static) -> Self {
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
                    WINEVENT_OUTOFCONTEXT | WINEVENT_SKIPOWNPROCESS,
                );
                *lock = handle;
                drop(lock);
                message_loop();
                unhook_win_event(handle);
                *H_WIN_EVENT.write().unwrap() = HWINEVENTHOOK(0);
            });
        }
        let self_ = Self(Arc::new(func), SystemTime::now(), event);
        EVENTS.write().unwrap().push(self_.clone());

        self_
    }
    pub(crate) fn unhook(&self) {
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