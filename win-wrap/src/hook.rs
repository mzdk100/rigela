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

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::SystemTime;
use windows::Win32::Foundation::{CloseHandle, FALSE, HANDLE, HINSTANCE, HWND, LPARAM, LRESULT, TRUE, WPARAM};
use windows::Win32::System::Threading::{GetCurrentThreadId, CreateEventW, SetEvent, WaitForSingleObject};
use windows::Win32::UI::WindowsAndMessaging::{CallNextHookEx, CWPRETSTRUCT, CWPSTRUCT, DispatchMessageW, GetMessageW, HHOOK, KBDLLHOOKSTRUCT, MSG, MSLLHOOKSTRUCT, PostThreadMessageW, SetWindowsHookExW, UnhookWindowsHookEx, WH_CALLWNDPROC, WH_CALLWNDPROCRET, WH_KEYBOARD_LL, WH_MOUSE_LL, WINDOWS_HOOK_ID, WM_QUIT};

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

/* 低级键盘钩子信息结构。 */
pub type KbdLlHookStruct = KBDLLHOOKSTRUCT;

/* 低级鼠标钩子信息结构。 */
pub type MsLlHookStruct = MSLLHOOKSTRUCT;

/* 窗口过程函数之前的钩子信息结构。 */
pub type CwpStruct = CWPSTRUCT;

/* 窗口过程函数之后的钩子信息结构。 */
pub type CwpRStruct = CWPRETSTRUCT;

pub trait ConvertLParam {
    /**
     * 把LPARAM转换成另一种类型的方法。
     * */
    fn to<T>(&self) -> &T;
}
impl ConvertLParam for LPARAM{
    fn to<T>(&self) -> &T {
        let ptr = self.0 as *const T;
        unsafe { &*ptr }
    }
}
type NextHookFunc = dyn Fn() -> LRESULT;
type HookCbFunc = Arc<dyn Fn(&WPARAM, &LPARAM, &NextHookFunc) -> LRESULT + Sync + Send + 'static>;
#[derive(Clone)]
struct HookInfo(u32, HHOOK, HANDLE);
impl HookInfo {
    pub(crate) fn new(thread_id: u32, h_hook: HHOOK, event: HANDLE) -> Self {
        Self(thread_id, h_hook,     event)
    }
    fn hook(&self) -> HHOOK {
        self.1
    }
    fn quit(&self) {
        unsafe { PostThreadMessageW(self.0, WM_QUIT, WPARAM::default(), LPARAM::default()) }
            .unwrap();
    }
    fn join(&self, millis: u32) {
        unsafe { WaitForSingleObject(self.2, millis); }
        unsafe { CloseHandle(self.2) }
            .unwrap();
    }
    fn finish(&self) {
        unsafe { SetEvent(self.2) }
            .unwrap();
    }
}
static H_HOOK: RwLock<Option<HashMap<i32, HookInfo>>> = RwLock::new(None);

/* Windows 的钩子。 */
#[derive(Clone)]
pub struct WindowsHook(WINDOWS_HOOK_ID, HookCbFunc, SystemTime);
static HOOK_MAP: RwLock<Option<HashMap<i32, Vec<WindowsHook>>>> = RwLock::new(None);
impl WindowsHook{
    fn call(&self, w_param: &WPARAM, l_param: &LPARAM, next: impl Fn() -> LRESULT + 'static) -> LRESULT {
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
    pub fn new(hook_type: HookType, cb: impl Fn(&WPARAM, &LPARAM, &NextHookFunc) -> LRESULT + Sync + Send + 'static) -> Self {
        let info = Self(hook_type, Arc::new(cb), SystemTime::now());
        let mut lock = HOOK_MAP
            .write()
            .unwrap();
        let mut map = match lock.as_ref() {
            None => {
                install(hook_type);
                HashMap::<i32, Vec<WindowsHook>>::new()
            }
            Some(x) => {
                x.clone()
            }
        };
        let mut vec = match map.get(&hook_type.0) {
            None => {
                Vec::new()
            }
            Some(x) => {
                x.clone()
            }
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
        let mut lock = HOOK_MAP
            .write()
            .unwrap();
        let mut map = match lock.as_ref() {
            None => {
                return;
            }
            Some(x) => {
                x.clone()
            }
        };
        match map.get(&self.0.0) {
            None => {
                uninstall(self.0);
            }
            Some(x) => {
                let mut x = x.clone();
                for i in 0..x.len() {
                    let j = x
                        .get(i)
                        .unwrap();
                    if j == self {
                        x.remove(i);
                        break
                    }
                }
                if x.len() < 1 {
                    uninstall(self.0);
                    map.remove(&self.0.0);
                } else {
                    map.insert(self.0.0, x);
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
fn next_hook(vec: Vec<WindowsHook>, index: usize, h_hook: HHOOK, code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if index >= vec.len() {
        unsafe { return CallNextHookEx(h_hook, code, w_param, l_param); }
    }
    match vec.get(index) {
        None => unsafe { CallNextHookEx(h_hook, code, w_param, l_param) },
        Some(x) => x.clone().call(&w_param, &l_param, move || next_hook(vec.clone(), index + 1, h_hook, code, w_param, l_param))
    }
}
macro_rules! define_hook_proc {
    ($name:tt, $id:tt) => {
        unsafe extern "system" fn $name(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
            let lock = H_HOOK
                .read()
                .unwrap();
            let info = match lock.as_ref() {
                None => {
                    drop(lock);
                    return CallNextHookEx(HHOOK::default(), code, w_param, l_param);
                }
                Some(x) => x.get(&$id.0)
            };
            let h_hook = match info {
                None => {
                    drop(lock);
                    return CallNextHookEx(HHOOK::default(), code, w_param, l_param);
                }
                Some(x) => x.hook()
            };
            drop(lock);
            if code < 0 {
                return CallNextHookEx(h_hook, code, w_param, l_param);
            }
            let lock = HOOK_MAP
                .read()
                .unwrap();
            let vec = match lock.as_ref() {
                None => {
                    drop(lock);
                    return CallNextHookEx(h_hook, code, w_param, l_param);
                }
                Some(x) => {
                    x.get(&$id.0)
                }
            };
            let vec = match vec {
                None => {
                    drop(lock);
                    return CallNextHookEx(h_hook, code, w_param, l_param);
                }
                Some(x) => x.clone()
            };
            drop(lock);
            next_hook(vec, 0, h_hook, code, w_param, l_param)
        }
    };
}
define_hook_proc!(proc_keyboard_ll, HOOK_TYPE_KEYBOARD_LL);
define_hook_proc!(proc_mouse_ll, HOOK_TYPE_MOUSE_LL);
define_hook_proc!(proc_call_wnd_proc, HOOK_TYPE_CALL_WND_PROC);
define_hook_proc!(proc_call_wnd_proc_ret, HOOK_TYPE_CALL_WND_PROC_RET);

fn install(hook_type: HookType) {
    let lock = H_HOOK
        .read()
        .unwrap();
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
            x => panic!("Unsupported hook type: {}.", x.0)
        };
        let h_hook = unsafe { SetWindowsHookExW(hook_type, Some(proc), HINSTANCE::default(), 0) }
            .expect(format!("Can't set the {} hook.", hook_type.0).as_str());
        let tid = unsafe { GetCurrentThreadId() };
        let event2 = unsafe { CreateEventW(None, TRUE, FALSE, None) }
            .unwrap();
        let info = HookInfo::new(tid, h_hook, event2);
        let mut lock = H_HOOK
            .write()
            .unwrap();
        let mut map = match lock.as_ref() {
            None => {
                HashMap::new()
            }
            Some(x) => {
                x.clone()
            }
        };
        map.insert(hook_type.0, info.clone());
        *lock = Some(map);
        drop(lock);
        let mut msg = MSG::default();
        while unsafe { GetMessageW(&mut msg, HWND::default(), 0, 0) } != FALSE {
            println!("abc{}", msg.message);
            if msg.message == WM_QUIT {
                break
            }
            unsafe { DispatchMessageW(&mut msg, ); }
        }
        unsafe { UnhookWindowsHookEx(h_hook) }
            .unwrap();
        info.finish();
    });
}
fn uninstall(hook_type: HookType) {
    let lock = H_HOOK
        .read()
        .unwrap();
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