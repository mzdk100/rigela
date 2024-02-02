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

use std::ffi::CString;
use windows::Win32::UI::Accessibility::{
    SetWinEventHook, UnhookWinEvent, HWINEVENTHOOK, WINEVENTPROC,
};
pub use windows::{
    core::Result,
    Win32::{
        Foundation::{
            BOOL, FALSE, FARPROC, HANDLE, HINSTANCE, HMODULE, HWND, LPARAM, LRESULT, RECT, TRUE,
            WAIT_EVENT, WPARAM,
        },
        Globalization::HIMC,
        System::SystemServices::{
            DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH,
        },
        UI::WindowsAndMessaging::{HHOOK, HOOKPROC, WINDOWS_HOOK_ID},
    },
};
use windows::{
    core::{HSTRING, PCSTR},
    Win32::{
        Foundation::{CloseHandle, FreeLibrary, GetLastError, MAX_PATH},
        Globalization::{GetUserDefaultLocaleName, MAX_LOCALE_NAME},
        System::{
            Diagnostics::Debug::Beep,
            LibraryLoader::{GetModuleFileNameW, GetModuleHandleW, GetProcAddress, LoadLibraryW},
        },
        UI::WindowsAndMessaging::{
            CallNextHookEx, GetDesktopWindow, GetForegroundWindow, SetForegroundWindow,
            SetWindowsHookExW, UnhookWindowsHookEx,
        },
    },
};

/**
 * 播放一个声音。
 * `freq` 声音频率（Hz）
 * `duration` 持续时间（毫秒）
 * */
pub fn beep(freq: u32, duration: u32) {
    unsafe { Beep(freq, duration) }.unwrap();
}

/**
 * 查询用户当前窗口（前台窗口的句柄）。
 * 系统为创建前台窗口的线程分配的优先级略高于其他线程。
 * 返回值是前台窗口的句柄。
 * 在某些情况下，前台窗口可以为
 * NULL
 * ，例如，当窗口失去激活时。
 * */
pub fn get_foreground_window() -> HWND {
    unsafe { GetForegroundWindow() }
}

/**
 * 将创建指定窗口的线程引入前台并激活窗口。
 * 键盘输入将定向到窗口，并为用户更改各种视觉提示。
 * 系统为创建前台窗口的线程分配的优先级略高于其他线程的优先级。
 * `h_wnd`
 * 应激活并带到前台的窗口的句柄。
 * */
pub fn set_foreground_window(h_wnd: HWND) {
    unsafe { SetForegroundWindow(h_wnd) };
}

/**
 * 查询桌面窗口的句柄。
 * 桌面窗口覆盖整个屏幕。
 * 桌面窗口是在上面绘制其他窗口的区域。
 * */
pub fn get_desktop_window() -> HWND {
    unsafe { GetDesktopWindow() }
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
            HHOOK::default(), // 此参数应该忽略： https://learn.microsoft.com/zh-cn/windows/win32/api/winuser/nf-winuser-callnexthookex?redirectedfrom=MSDN
            code,
            w_param,
            l_param,
        )
    }
}

/**
 * 将应用程序定义的拦截程序安装到钩子链中。您将安装拦截程序，以监视系统是否有特定类型的事件。这些事件与特定线程或与呼叫线程位于相同桌面中的所有线程相关联。
 * `id_hook` 要安装的拦截程序类型。
 * `func` 钩子程序的地址。如果 id_thread 参数为零，或指定不同进程所建立线程的识别码，func参数必须指向 DLL 中的拦截程序。否则，func可以在与目前进程相关联的代码中指向拦截程序。
 * `h_mod` DLL 的实例句柄，其中包含 func参数所指向的拦截程序。如果id_thread参数指定目前进程所建立的执行绪，而且拦截程序位于与目前进程相关联的代码内，则必须将h_mod参数设定为Null。
 * */
pub fn set_windows_hook_ex(
    id_hook: WINDOWS_HOOK_ID,
    func: HOOKPROC,
    h_mod: HINSTANCE,
    id_thread: u32,
) -> Result<HHOOK> {
    unsafe { SetWindowsHookExW(id_hook, func, h_mod, id_thread) }
}

/**
 * 删除 SetWindowsHookEx 函数安装在钩子链中的挂钩过程。
 * `h_hook` 要移除的钩子的句柄。此参数是由先前调用 set_windows_hook_ex 获取的钩子句柄。
 * */
pub fn unhook_windows_hook_ex(h_hook: HHOOK) -> Result<()> {
    unsafe { UnhookWindowsHookEx(h_hook) }
}

/**
 * 关闭打开的对象句柄。
 * `h_object` 打开对象的有效句柄。
 * */
pub fn close_handle(h_object: HANDLE) {
    unsafe { CloseHandle(h_object) }.expect("Can't close the object handle.")
}

/**
 * 提取指定模块的模块句柄。调用进程必须已加载模块。
 * 若要避免一节中所述的竞争条件，请使用 get_module_handle_ex 函数。
 * `module_name` 装入的模块名（.dll或.exe文件）。如果省略扩展名，则会附加预设库副文件名.dll。文件名串可以包含尾端点字符（.），表示模块名称没有扩展名。字符串不需要指定路径。指定路径时，请务必使用反斜线（\），而不是正斜线（/）。名称会独立比较（大小写）目前对应至呼叫进程的地址空间的模块名称。如果此参数为 Null， get_module_handle 将返回用来创建调用进程（.exe文件的文件的句柄）。get_module_handle函数不会撷取使用LOAD_LIBRARY_AS_DATAFILE旗标加载之模组的句柄。
 * */
pub fn get_module_handle(module_name: &str) -> HMODULE {
    unsafe { GetModuleHandleW(&HSTRING::from(module_name)) }.expect("Can't get the module handle.")
}

/**
 * 获取当前进程已加载模块的文件的完整路径，该模块必须由当前进程加载。
 * 如果想要获取另一个已加载模块的文件路径，可以使用get_module_file_name_ex函数。
 * `h_module` 一个模块的句柄。可以是一个DLL模块，或者是一个应用程序的实例句柄。如果该参数为NULL，该函数返回该应用程序全路径。
 */
pub fn get_module_file_name(h_module: HMODULE) -> String {
    unsafe {
        let mut buf: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
        let len = GetModuleFileNameW(h_module, &mut buf);
        String::from_utf16_lossy(&buf[..len as usize])
    }
}

/**
 * 将指定的模块加载到调用进程的地址空间中。指定的模块可能会导致加载其他模块。有关其他加载选项，请使用 load_library_ex 函数。
 * `lib_file_name` 模块的名称。这可以是库模块 (.dll 文件)，也可以是可执行模块 (.exe 文件)。如果指定的模块是可执行模块，则不会加载静态导入;相反，模块就像使用标志的 load_library_ex DONT_RESOLVE_DLL_REFERENCES 加载一样。指定的名称是模块的文件名，与库模块本身中存储的名称无关，该名称由 module-definition (.def) 文件中的 LIBRARY 关键字 (keyword) 指定。如果字符串指定完整路径，则函数仅搜索模块的该路径。如果字符串指定相对路径或模块名称而不指定路径，则函数使用标准搜索策略来查找模块;如果函数找不到模块，该函数将失败。指定路径时，请务必使用反斜杠 (\) ，而不是使用 /) (正斜杠。如果字符串指定模块名称而不指定路径，并且省略文件扩展名，则该函数会将默认库扩展“.DLL”追加到模块名称。若要防止函数将“.DLL”追加到模块名称中，请在模块名称字符串中包含尾随点字符 (.)。
 * */
pub fn load_library(lib_file_name: &str) -> Result<HMODULE> {
    unsafe { LoadLibraryW(&HSTRING::from(lib_file_name)) }
}

/**
 * 释放加载的动态链接库 (DLL) 模块，并在必要时递减其引用计数。
 * 当引用计数达到零时，模块将从调用进程的地址空间中卸载，句柄不再有效。
 * `h_lib_module` 已加载的库模块的句柄。load_library、load_library_ex、get_module_handle 或 get_module_handle_ex 函数返回此句柄。
 * */
pub fn free_library(h_lib_module: HMODULE) {
    unsafe {
        FreeLibrary(h_lib_module).unwrap_or(());
    }
}

/**
 * 从指定的动态链接库 (DLL) 查询导出函数(也称为过程)或变量的地址。
 * `h_module` 包含函数或变量的DLL模块的句柄。load_library、load_library_ex、load_packaged_library 或 get_module_handle函数返回此句柄。get_proc_address函数不会从使用LOAD_LIBRARY_AS_DATAFILE标志加载的模块中检索地址。有关详细信息，请参阅load_library_ex。
 * `proc_name` 函数或变量名称，或函数的序号值。如果此参数是序号值，则它必须在低序位字中；高序位字必须为零。
 * */
pub fn get_proc_address(h_module: HMODULE, proc_name: &str) -> FARPROC {
    let name = CString::new(proc_name).unwrap();
    unsafe { GetProcAddress(h_module, PCSTR::from_raw(name.as_ptr().cast())) }
}

/**
 * 查询调用线程的最后错误代码值。 最后一个错误代码按线程进行维护。多个线程不会覆盖彼此的最后错误代码。
 * */
pub fn get_last_error() -> Result<()> {
    unsafe { GetLastError() }
}

/**
 * 查询用户默认 区域设置名称。
 * 注意 如果设计为仅在 Windows Vista 及更高版本上运行，应用程序应优先调用此函数，而不是 get_user_default_lc_id 。
 */
pub fn get_user_default_locale_name() -> String {
    unsafe {
        let mut name: [u16; MAX_LOCALE_NAME as usize] = [0; MAX_LOCALE_NAME as usize];
        let length = GetUserDefaultLocaleName(&mut name);
        String::from_utf16_lossy(&mut name[..(length - 1) as usize])
    }
}

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
pub fn unhook_win_event(h_win_event_hook: HWINEVENTHOOK) -> BOOL {
    unsafe { UnhookWinEvent(h_win_event_hook) }
}
