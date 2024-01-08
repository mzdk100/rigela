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

use windows::Win32::UI::WindowsAndMessaging::{CallNextHookEx, GetForegroundWindow, HOOKPROC, SetWindowsHookExW, UnhookWindowsHookEx};
pub use windows::Win32::UI::WindowsAndMessaging::{HHOOK, WINDOWS_HOOK_ID};
pub use windows::core::{PCWSTR, Result, h, w};
pub use windows::Win32::Foundation::{BOOL, FALSE, TRUE, HANDLE, HMODULE, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM, WAIT_EVENT};
pub use windows::Win32::System::SystemServices::{DLL_PROCESS_DETACH, DLL_PROCESS_ATTACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH};

use windows::Win32::System::Diagnostics::Debug::Beep;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

/**
 * 播放一个声音。
 * `freq` 声音频率（Hz）
 * `duration` 持续时间（毫秒）
 * */
pub fn beep(freq: u32, duration: u32) {
    unsafe { Beep(freq, duration) }.unwrap();
}

/// 获取当前前台窗口句柄。
pub fn get_foreground_window() -> HWND {
    unsafe { GetForegroundWindow() }
}

/**
 * 调用下一个钩子函数。
 * `code` 传递给当前挂钩过程的挂钩代码。 下一个挂钩过程使用此代码来确定如何处理挂钩信息。
 * `w_param` 传递给当前挂钩过程的 wParam 值。 此参数的含义取决于与当前挂钩链关联的挂钩类型。
 * `l_param` 传递给当前挂钩过程的 lParam 值。 此参数的含义取决于与当前挂钩链关联的挂钩类型。
 * */
pub fn call_next_hook_ex(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    unsafe {
        CallNextHookEx(
            HHOOK::default(),  // 此参数应该忽略： https://learn.microsoft.com/zh-cn/windows/win32/api/winuser/nf-winuser-callnexthookex?redirectedfrom=MSDN
            code,
            w_param,
            l_param
        )
    }
}

/**
 * 将应用程序定义的拦截程序安装到钩子链中。您将安装拦截程序，以监视系统是否有特定类型的事件。这些事件与特定线程或与呼叫线程位于相同桌面中的所有线程相关联。
 * `id_hook` 要安装的拦截程序类型。
 * `func` 钩子程序的地址。如果 id_thread 参数为零，或指定不同进程所建立线程的识别码，func参数必须指向 DLL 中的拦截程序。否则，func可以在与目前进程相关联的代码中指向拦截程序。
 * `h_mod` DLL 的实例句柄，其中包含 func参数所指向的拦截程序。如果id_thread参数指定目前进程所建立的执行绪，而且拦截程序位于与目前进程相关联的代码内，则必须将h_mod参数设定为Null。
 * */
pub fn set_windows_hook_ex(id_hook: WINDOWS_HOOK_ID, func: HOOKPROC, h_mod: HINSTANCE, id_thread: u32) -> HHOOK {
    unsafe { SetWindowsHookExW(id_hook, func, h_mod, id_thread) }
        .expect(format!("Can't set the {} hook.", id_hook.0).as_str())
}

/**
 * 删除 SetWindowsHookEx 函数安装在钩子链中的挂钩过程。
 * `h_hook` 要移除的钩子的句柄。此参数是由先前调用 set_windows_hook_ex 获取的钩子句柄。
 * */
pub fn unhook_windows_hook_ex(h_hook: HHOOK) {
    unsafe { UnhookWindowsHookEx(h_hook) }
        .expect("Can't remove the hook.")
}

/**
 * 关闭打开的对象句柄。
 * `h_object` 打开对象的有效句柄。
 * */
pub fn close_handle(h_object: HANDLE) {
    unsafe { CloseHandle(h_object) }
        .expect("Can't close the object handle.")
}

/**
 * 提取指定模块的模块句柄。调用进程必须已加载模块。
 * 若要避免一节中所述的竞争条件，请使用 get_module_handle_ex 函数。
 * `module_name` 装入的模块名（.dll或.exe文件）。如果省略扩展名，则会附加预设库副文件名.dll。文件名串可以包含尾端点字符（.），表示模块名称没有扩展名。字符串不需要指定路径。指定路径时，请务必使用反斜线（\），而不是正斜线（/）。名称会独立比较（大小写）目前对应至呼叫进程的地址空间的模块名称。如果此参数为 Null， get_module_handle 将返回用来创建调用进程（.exe文件的文件的句柄）。get_module_handle函数不会撷取使用LOAD_LIBRARY_AS_DATAFILE旗标加载之模组的句柄。
 * */
pub fn get_module_handle<T>(module_name: T) -> HMODULE
where T: windows::core::IntoParam<PCWSTR> {
    unsafe { GetModuleHandleW(module_name) }
        .expect("Can't get the module handle.")
}