/*
 * Copyright (c) 2023. The RigelA open source project team and
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
    common::{
        call_next_hook_ex, set_windows_hook_ex, unhook_windows_hook_ex, HINSTANCE, LPARAM, LRESULT,
        WINDOWS_HOOK_ID, WPARAM,
    },
    message::message_loop,
    threading::{get_current_thread_id, ThreadNotify},
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread::{self, sleep},
    time::{Duration, SystemTime},
};
use windows::Win32::UI::WindowsAndMessaging::{
    CWPRETSTRUCT, CWPSTRUCT, KBDLLHOOKSTRUCT, KBDLLHOOKSTRUCT_FLAGS, MSLLHOOKSTRUCT,
    WH_CALLWNDPROC, WH_CALLWNDPROCRET, WH_GETMESSAGE, WH_KEYBOARD_LL, WH_MOUSE_LL,
};
pub use windows::Win32::UI::WindowsAndMessaging::{
    LLKHF_ALTDOWN, LLKHF_EXTENDED, LLKHF_INJECTED, LLKHF_LOWER_IL_INJECTED, LLKHF_UP,
};

/* 钩子类型。 */
pub type HookType = WINDOWS_HOOK_ID;

/* 低级键盘钩子。 */
pub const HOOK_TYPE_KEYBOARD_LL: HookType = WH_KEYBOARD_LL;

/* 低级鼠标钩子。 */
pub const HOOK_TYPE_MOUSE_LL: HookType = WH_MOUSE_LL;

/* 当SendMessage()把消息交给WndProc时,在WndProc尚未执行前,系统调用CallWndProc钩子函数,钩子函数执行后才执行窗口过程。 */
pub const HOOK_TYPE_CALL_WND_PROC: WINDOWS_HOOK_ID = WH_CALLWNDPROC;

/* 当SendMessage()把消息交给WndProc时,在WndProc执行完毕,系统调用CallWndProcRet钩子函数,从而可以拦截窗口过程函数的结果。 */
pub const HOOK_TYPE_CALL_WND_PROC_RET: WINDOWS_HOOK_ID = WH_CALLWNDPROCRET;

/* 拦截队列消息（拦截由get_message或者post_message或者peek_message的队列消息）。 */
pub const HOOK_TYPE_GET_MESSAGE: WINDOWS_HOOK_ID = WH_GETMESSAGE;

/* 低级键盘钩子信息结构。 */
pub type KbdLlHookStruct = KBDLLHOOKSTRUCT;
pub type KbdLlHookFlags = KBDLLHOOKSTRUCT_FLAGS;

/* 低级鼠标钩子信息结构。 */
pub type MsLlHookStruct = MSLLHOOKSTRUCT;

/* 窗口过程函数之前的钩子信息结构。 */
pub type CwpStruct = CWPSTRUCT;

/* 窗口过程函数之后的钩子信息结构。 */
pub type CwpRStruct = CWPRETSTRUCT;

type NextHookFunc = dyn Fn() -> LRESULT;
type HookCbFunc = Arc<dyn Fn(&WPARAM, &LPARAM, &NextHookFunc) -> LRESULT + Sync + Send + 'static>;
static H_HOOK: RwLock<Option<HashMap<i32, ThreadNotify>>> = RwLock::new(None);
static HOOK_MAP: RwLock<Option<HashMap<i32, Vec<WindowsHook>>>> = RwLock::new(None);

/* Windows 的钩子。 */
#[derive(Clone)]
pub struct WindowsHook(WINDOWS_HOOK_ID, HookCbFunc, SystemTime);
impl WindowsHook {
    fn call(
        &self,
        w_param: &WPARAM,
        l_param: &LPARAM,
        next: impl Fn() -> LRESULT + 'static,
    ) -> LRESULT {
        (&*self.1)(w_param, l_param, &next)
    }

    /**
     * 创建一个钩子对象并开始监听事件。
     * `hook_type` 钩子类型，使用“HOOK_TYPE_”开头的常亮。
     * `cb` 用于接收事件的处理函数，需要实现三个参数：
     * fn cb(w_param: &WPARAM, l_param: &LPARAM, next: &NextHookFunc) {next()}
     * 也可以使用闭包
     * |w_param, l_param, next| {next()}
     * 处理函数运行在单独的子线程中，使用时可以通过next()调用钩子链中的下一个函数，也可以直接拦截。
     * */
    pub fn new(
        hook_type: HookType,
        cb: impl Fn(&WPARAM, &LPARAM, &NextHookFunc) -> LRESULT + Sync + Send + 'static,
    ) -> Self {
        let info = Self(hook_type, Arc::new(cb), SystemTime::now());
        let mut lock = HOOK_MAP.write().unwrap();
        let mut map = match lock.as_ref() {
            None => {
                install(hook_type);
                HashMap::<i32, Vec<WindowsHook>>::new()
            }
            Some(x) => x.clone(),
        };
        let mut vec = match map.get(&hook_type.0) {
            None => Vec::new(),
            Some(x) => x.clone(),
        };
        vec.insert(0, info.clone());
        map.insert(hook_type.0, vec);
        *lock = Some(map);
        info
    }

    /**
     * 卸载钩子。
     * */
    pub fn unhook(&self) {
        let mut lock = HOOK_MAP.write().unwrap();
        let mut map = match lock.as_ref() {
            None => {
                return;
            }
            Some(x) => x.clone(),
        };
        match map.get(&self.0 .0) {
            None => {
                uninstall(self.0);
            }
            Some(x) => {
                let mut x = x.clone();
                for i in 0..x.len() {
                    let j = x.get(i).unwrap();
                    if j == self {
                        x.remove(i);
                        break;
                    }
                }
                if x.len() < 1 {
                    uninstall(self.0);
                    map.remove(&self.0 .0);
                } else {
                    map.insert(self.0 .0, x);
                }
                if map.is_empty() {
                    *lock = None;
                } else {
                    *lock = Some(map);
                }
            }
        }
    }
}
impl PartialEq for WindowsHook {
    fn eq(&self, other: &Self) -> bool {
        self.2 == other.2
    }
}
fn next_hook(
    vec: Vec<WindowsHook>,
    index: usize,
    code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if index >= vec.len() {
        return call_next_hook_ex(code, w_param, l_param);
    }
    match vec.get(index) {
        None => call_next_hook_ex(code, w_param, l_param),
        Some(x) => x.clone().call(&w_param, &l_param, move || {
            next_hook(vec.clone(), index + 1, code, w_param, l_param)
        }),
    }
}

macro_rules! define_hook_proc {
    ($name:tt, $id:tt) => {
        unsafe extern "system" fn $name(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
            if code < 0 {
                return call_next_hook_ex(code, w_param, l_param);
            }
            let lock = HOOK_MAP.read().unwrap();
            let vec = match lock.as_ref() {
                None => {
                    drop(lock);
                    return call_next_hook_ex(code, w_param, l_param);
                }
                Some(x) => x.get(&$id.0),
            };
            let vec = match vec {
                None => {
                    drop(lock);
                    return call_next_hook_ex(code, w_param, l_param);
                }
                Some(x) => x.clone(),
            };
            drop(lock);
            next_hook(vec, 0, code, w_param, l_param)
        }
    };
}
define_hook_proc!(proc_keyboard_ll, HOOK_TYPE_KEYBOARD_LL);
define_hook_proc!(proc_mouse_ll, HOOK_TYPE_MOUSE_LL);
define_hook_proc!(proc_call_wnd_proc, HOOK_TYPE_CALL_WND_PROC);
define_hook_proc!(proc_call_wnd_proc_ret, HOOK_TYPE_CALL_WND_PROC_RET);
define_hook_proc!(proc_get_message, HOOK_TYPE_GET_MESSAGE);

fn install(hook_type: HookType) {
    let lock = H_HOOK.read().unwrap();
    if let Some(x) = lock.as_ref() {
        if x.contains_key(&hook_type.0) {
            return;
        }
    }
    drop(lock);
    thread::spawn(move || {
        let proc = match hook_type {
            HOOK_TYPE_KEYBOARD_LL => proc_keyboard_ll,
            HOOK_TYPE_MOUSE_LL => proc_mouse_ll,
            HOOK_TYPE_CALL_WND_PROC => proc_call_wnd_proc,
            HOOK_TYPE_CALL_WND_PROC_RET => proc_call_wnd_proc_ret,
            HOOK_TYPE_GET_MESSAGE => proc_get_message,
            x => panic!("Unsupported hook type: {}.", x.0),
        };
        let mut retry = 0;
        let h_hook = loop {
            let h = set_windows_hook_ex(hook_type, Some(proc), HINSTANCE::default(), 0);
            if h.is_ok() {
                break h.unwrap();
            }
            if retry > 5 {
                panic!(
                    "Can't set the windows hook ({}), and retrying it.",
                    hook_type.0
                );
            }
            retry += 1;
            sleep(Duration::from_millis(1000));
        };
        let notify = ThreadNotify::new(get_current_thread_id());
        let mut lock = H_HOOK.write().unwrap();
        let mut map = match lock.as_ref() {
            None => HashMap::new(),
            Some(x) => x.clone(),
        };
        map.insert(hook_type.0, notify.clone());
        *lock = Some(map);
        drop(lock);
        message_loop();
        unhook_windows_hook_ex(h_hook).unwrap_or(());
        notify.finish();
    });
}

fn uninstall(hook_type: HookType) {
    let lock = H_HOOK.read().unwrap();
    match lock.as_ref() {
        None => {
            return;
        }
        Some(x) => {
            if let Some(x) = x.get(&hook_type.0) {
                x.quit();
                x.join(5000);
            }
        }
    }
}
