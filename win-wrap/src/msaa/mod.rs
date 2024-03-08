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

pub mod event;
pub mod object;

use crate::{
    common::{attach_thread_input, get_desktop_window, get_foreground_window, Result, FALSE, TRUE},
    input::get_focus,
    msaa::{
        event::{WinEventHook, WinEventSource},
        object::AccessibleObject,
    },
    threading::{get_current_thread_id, get_window_thread_process_id},
};
use std::sync::RwLock;
use windows::Win32::UI::WindowsAndMessaging::{
    EVENT_OBJECT_ACCELERATORCHANGE, EVENT_OBJECT_CLOAKED, EVENT_OBJECT_CONTENTSCROLLED,
    EVENT_OBJECT_CREATE, EVENT_OBJECT_DEFACTIONCHANGE, EVENT_OBJECT_DESCRIPTIONCHANGE,
    EVENT_OBJECT_DESTROY, EVENT_OBJECT_DRAGCANCEL, EVENT_OBJECT_DRAGCOMPLETE,
    EVENT_OBJECT_DRAGDROPPED, EVENT_OBJECT_DRAGENTER, EVENT_OBJECT_DRAGLEAVE,
    EVENT_OBJECT_DRAGSTART, EVENT_OBJECT_END, EVENT_OBJECT_FOCUS, EVENT_OBJECT_HELPCHANGE,
    EVENT_OBJECT_HIDE, EVENT_OBJECT_HOSTEDOBJECTSINVALIDATED, EVENT_OBJECT_IME_CHANGE,
    EVENT_OBJECT_IME_HIDE, EVENT_OBJECT_IME_SHOW, EVENT_OBJECT_INVOKED,
    EVENT_OBJECT_LIVEREGIONCHANGED, EVENT_OBJECT_LOCATIONCHANGE, EVENT_OBJECT_NAMECHANGE,
    EVENT_OBJECT_PARENTCHANGE, EVENT_OBJECT_REORDER, EVENT_OBJECT_SELECTION,
    EVENT_OBJECT_SELECTIONADD, EVENT_OBJECT_SELECTIONREMOVE, EVENT_OBJECT_SELECTIONWITHIN,
    EVENT_OBJECT_SHOW, EVENT_OBJECT_STATECHANGE, EVENT_OBJECT_TEXTEDIT_CONVERSIONTARGETCHANGED,
    EVENT_OBJECT_TEXTSELECTIONCHANGED, EVENT_OBJECT_UNCLOAKED, EVENT_OBJECT_VALUECHANGE,
    EVENT_OEM_DEFINED_END, EVENT_OEM_DEFINED_START, EVENT_SYSTEM_ALERT,
    EVENT_SYSTEM_ARRANGMENTPREVIEW, EVENT_SYSTEM_CAPTUREEND, EVENT_SYSTEM_CAPTURESTART,
    EVENT_SYSTEM_CONTEXTHELPEND, EVENT_SYSTEM_CONTEXTHELPSTART, EVENT_SYSTEM_DESKTOPSWITCH,
    EVENT_SYSTEM_DIALOGEND, EVENT_SYSTEM_DIALOGSTART, EVENT_SYSTEM_DRAGDROPEND,
    EVENT_SYSTEM_DRAGDROPSTART, EVENT_SYSTEM_END, EVENT_SYSTEM_FOREGROUND, EVENT_SYSTEM_MENUEND,
    EVENT_SYSTEM_MENUPOPUPEND, EVENT_SYSTEM_MENUPOPUPSTART, EVENT_SYSTEM_MENUSTART,
    EVENT_SYSTEM_MINIMIZEEND, EVENT_SYSTEM_MINIMIZESTART, EVENT_SYSTEM_MOVESIZEEND,
    EVENT_SYSTEM_MOVESIZESTART, EVENT_SYSTEM_SCROLLINGEND, EVENT_SYSTEM_SCROLLINGSTART,
    EVENT_SYSTEM_SOUND, EVENT_SYSTEM_SWITCHEND, EVENT_SYSTEM_SWITCHSTART,
};

#[derive(Debug)]
pub struct Msaa {
    events: RwLock<Vec<WinEventHook>>,
}

impl Msaa {
    /**
     * 创建一个MSAA实例。
     * */
    pub fn new() -> Self {
        Self {
            events: vec![].into(),
        }
    }

    /**
     * 获取桌面窗口的可访问性对象。
     * */
    pub fn get_desktop_object(&self) -> Result<AccessibleObject> {
        AccessibleObject::from_window(get_desktop_window())
    }

    /**
     * 获取输入焦点的可访问性对象。
     * */
    pub fn get_focus_object(&self) -> Result<AccessibleObject> {
        let current_thread_id = get_current_thread_id();
        let h_foreground = get_foreground_window();
        let (remote_thread_id, _) = get_window_thread_process_id(h_foreground);
        attach_thread_input(current_thread_id, remote_thread_id, TRUE);
        let obj = AccessibleObject::from_window(get_focus());
        attach_thread_input(current_thread_id, remote_thread_id, FALSE);
        obj
    }

    /**
     * 对象的 KeyboardShortcut 属性 已更改。 服务器应用程序为它们的辅助性对象发送该事件。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_accelerator_change_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_ACCELERATORCHANGE, func));
    }

    /**
     * 在隐藏窗口时发送。 隐藏的窗口仍然存在，但对用户不可见。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_cloaked_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_CLOAKED, func));
    }

    //noinspection SpellCheckingInspection
    /**
     * 窗口对象的滚动已结束。 与 EVENT_SYSTEM_SCROLLEND不同，此事件与滚动窗口相关联。 无论滚动是水平滚动还是垂直滚动，只要滚动操作完成，都应发送此事件。
     * WinEventProc 回调函数的 hwnd 参数描述滚动窗口;idObject 参数OBJID_CLIENT，idChild 参数CHILDID_SELF。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_content_scrolled_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_CONTENTSCROLLED, func));
    }

    /**
     * 已创建 对象。 系统为以下用户界面元素发送此事件：插入点、 标题控件、 列表视图控件、 选项卡控件、 工具栏控件、 树视图控件和 窗口 对象。 服务器应用程序为它们的辅助性对象发送该事件。
     * 在为父对象发送事件之前，服务器必须为对象的所有子对象发送事件。 服务器必须确保在父对象发送此事件之前，已完全创建所有子对象并准备好接受来自客户端的 IAccessible 调用。
     * 由于父对象在其子对象之后创建，因此客户端必须确保在调用 IAccessible：：get_accParent之前已创建对象的父对象，尤其是在使用上下文中挂钩函数的情况下。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_create_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_CREATE, func));
    }

    /**
     * 对象的 DefaultAction 属性 已更改。 操作系统为对话框发送该事件。 服务器应用程序为它们的辅助性对象发送该事件。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_default_action_change_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_DEFACTIONCHANGE, func));
    }

    /**
     * 对象的 Description 属性 已更改。 服务器应用程序为它们的辅助性对象发送该事件。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_description_change_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_DESCRIPTIONCHANGE, func));
    }

    /**
     * 对象已被销毁。 系统为以下用户界面元素发送此事件：插入点、标题控件、列表视图控件、选项卡控件、工具栏控件、树视图控件和窗口对象。 服务器应用程序为它们的辅助性对象发送该事件。
     * 客户端假定当父对象发送此事件时，对象的所有子级都会被销毁。
     * 收到此事件后，客户端不会调用对象的 IAccessible 属性或方法。 但是，只要由于 COM 规则) ，接口指针 (存在引用计数，接口指针必须保持有效，但 UI 元素可能不再存在。 对接口指针的进一步调用可能会返回失败错误;为了防止这种情况，服务器 会创建代理对象 并监视其生命周期。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_destroy_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_DESTROY, func));
    }

    /**
     * 用户开始拖动元素。 WinEventProc 回调函数的 hwnd、idObject 和 idChild 参数标识要拖动的对象。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_drag_start_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_DRAGSTART, func));
    }

    /**
     * 用户已结束拖动操作，然后再将拖动的元素放在放置目标上。 WinEventProc 回调函数的 hwnd、idObject 和 idChild 参数标识要拖动的对象。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_drag_cancel_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_DRAGCANCEL, func));
    }

    /**
     * 用户删除了放置目标上的元素。 WinEventProc 回调函数的 hwnd、idObject 和 idChild 参数标识要拖动的对象。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_drag_complete_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_DRAGCOMPLETE, func));
    }

    /**
     * 用户将元素拖动到放置目标的边界。 WinEventProc 回调函数的 hwnd、idObject 和 idChild 参数标识放置目标。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_drag_enter_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_DRAGENTER, func));
    }

    /**
     * 用户将元素拖出放置目标的边界。 WinEventProc 回调函数的 hwnd、idObject 和 idChild 参数标识放置目标。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_drag_leave_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_DRAGLEAVE, func));
    }

    /**
     * 用户删除了放置目标上的元素。 WinEventProc 回调函数的 hwnd、idObject 和 idChild 参数标识放置目标。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_drag_dropped_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_DRAGDROPPED, func));
    }

    /**
     * 最高的对象事件值。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_end_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_END, func));
    }

    /**
     * 对象已接收键盘焦点。 系统为以下用户界面元素发送此事件：列表视图控件、菜单栏、弹出菜单、切换窗口、选项卡控件、树视图控件和窗口对象。 服务器应用程序为它们的辅助性对象发送该事件。
     * WinEventProc 回调函数的 hwnd 参数标识接收键盘焦点的窗口。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_focus_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_FOCUS, func));
    }

    /**
     * 对象的 帮助属性 已更改。 服务器应用程序为它们的辅助性对象发送该事件。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_help_change_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_HELPCHANGE, func));
    }

    /**
     * 对象已隐藏。 系统为以下用户界面元素发送此事件：插入点和光标。 服务器应用程序为它们的辅助性对象发送该事件。
     * 为父对象生成此事件时，所有子对象都已隐藏。 服务器应用程序不会为子对象发送此事件。
     * 隐藏的对象包括 STATE_SYSTEM_INVISIBLE 标志;shown 对象不包括此标志。 EVENT_OBJECT_HIDE 事件还指示已设置STATE_SYSTEM_INVISIBLE标志。 因此，在这种情况下，服务器不会发送 EVENT_OBJECT_STATECHANGE 事件。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_hide_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_HIDE, func));
    }

    //noinspection SpellCheckingInspection
    /**
     * 承载其他可访问对象的窗口已更改托管对象。 客户端可能需要查询主机窗口以发现新的托管对象，尤其是在客户端一直在监视窗口中的事件时。 托管对象是与主机不同的辅助功能框架 (MSAA 或 UI 自动化) 的对象。 托管对象中与主机相同的框架中的更改应随结构更改事件一起处理，例如 MSAA 的EVENT_OBJECT_CREATE 。 有关详细信息，请参阅 winuser.h 中的注释。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_hosted_objects_invalidated_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            EVENT_OBJECT_HOSTEDOBJECTSINVALIDATED,
            func,
        ));
    }

    /**
     * IME 窗口已隐藏。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_ime_hide_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_IME_HIDE, func));
    }

    /**
     * IME 窗口已变为可见。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_ime_show_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_IME_SHOW, func));
    }

    /**
     * 输入法窗口的大小或位置已更改。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_ime_change_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_IME_CHANGE, func));
    }

    /**
     * 已调用 对象;例如，用户单击了一个按钮。 此事件受常见控件支持，由 UI 自动化使用。
     * 对于此事件，WinEventProc 回调函数的 hwnd、ID 和 idChild 参数标识所调用的项。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_invoked_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_INVOKED, func));
    }

    /**
     * 属于活动区域的对象已更改。 实时区域是应用程序频繁更改和/或异步更改的区域。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_live_region_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_LIVEREGIONCHANGED, func));
    }

    //noinspection SpellCheckingInspection
    /**
     * 对象已更改位置、形状和大小。 系统为以下用户界面元素发送此事件：插入点和窗口对象。 服务器应用程序为它们的辅助性对象发送该事件。
     * 生成此事件以响应对象层次结构中顶级对象的更改;它不是为对象可能具有的任何子项生成的。 例如，如果用户调整窗口大小，系统会为窗口发送此通知，但不会针对菜单栏、标题栏、滚动条或其他也已更改的对象发送此通知。
     * 当父窗口移动时，系统不会为所有非浮动子窗口发送该事件。 但是，如果应用程序由于调整父窗口的大小而显式调整子窗口的大小，系统将为重设大小的子窗口发送多个事件。
     * 如果对象的 State 属性 设置为 STATE_SYSTEM_FLOATING，则每当对象更改位置时，服务器就会发送 EVENT_OBJECT_LOCATIONCHANGE 。 如果对象不具有此状态，则服务器仅在对象相对于其父级移动时才触发此事件。 对于此事件通知，WinEventProc 回调函数的 idChild 参数标识已更改的子对象。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_location_change_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_LOCATIONCHANGE, func));
    }

    /**
     * 对象的 Name 属性 已更改。 系统为以下用户界面元素发送此事件：检查框、光标、列表视图控件、推送按钮、单选按钮、状态栏控件、树视图控件和窗口对象。 服务器应用程序为它们的辅助性对象发送该事件。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_name_change_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_NAMECHANGE, func));
    }

    /**
     * 对象具有新的父对象。 服务器应用程序为它们的辅助性对象发送该事件。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_parent_change_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_PARENTCHANGE, func));
    }

    /**
     * 容器对象已添加、移除其子对象或对其子对象重新排序。 系统为以下用户界面元素发送此事件：标头控件、列表视图控件、工具栏控件和窗口对象。 服务器应用程序在适当的时候为它们的辅助性对象发送该事件。
     * 例如，当子元素的数量或元素的顺序发生更改时，列表视图对象会生成此事件。 当子窗口的 Z 顺序更改时，父窗口也会发送此事件。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_reorder_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_REORDER, func));
    }

    //noinspection SpellCheckingInspection
    /**
     * 容器对象中的选定内容已更改。 系统为以下用户界面元素发送此事件：列表视图控件、选项卡控件、树视图控件和窗口对象。 服务器应用程序为它们的辅助性对象发送该事件。
     * 此事件指示单个选择：在以前不包含任何选定子项的容器中选择了子级，或者所选内容已从一个子级更改为另一个子级。
     * WinEventProc 回调函数的 hwnd 和 idObject 参数描述容器;idChild 参数标识所选的对象。 如果所选子级是同时包含 对象的窗口，则 idChild 参数 OBJID_WINDOW。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_selection_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_SELECTION, func));
    }

    /**
     * 容器对象中的子项已添加到现有选定内容中。 系统为以下用户界面元素发送此事件：列表框、列表视图控件和树视图控件。 服务器应用程序为它们的辅助性对象发送该事件。
     * WinEventProc 回调函数的 hwnd 和 idObject 参数描述容器。 idChild 参数是添加到所选内容的子级。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_selection_add_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_SELECTIONADD, func));
    }

    /**
     * 容器对象中的项已从所选内容中删除。 系统为以下用户界面元素发送此事件：列表框、列表视图控件和树视图控件。 服务器应用程序为它们的辅助性对象发送该事件。
     * 此事件指示子项已从现有选定内容中删除。
     * WinEventProc 回调函数的 hwnd 和 idObject 参数描述容器;idChild 参数标识已从所选内容中删除的子级。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_selection_remove_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_SELECTIONREMOVE, func));
    }

    //noinspection SpellCheckingInspection
    /**
     * 容器对象中发生了许多选择更改。 系统为列表框发送此事件;服务器应用程序为其可访问的对象发送它。
     * 当控件中的选定项发生重大更改时，将发送此事件。 该事件通知客户端发生了许多选择更改，并且发送该事件而不是多个 EVENT_OBJECT_SELECTIONADD 或 EVENT_OBJECT_SELECTIONREMOVE 事件。 客户端通过调用容器对象的 IAccessible：：get_accSelection 方法并枚举所选项来查询所选项。
     * 对于此事件通知，WinEventProc 回调函数的 hwnd 和 idObject 参数描述发生更改的容器。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_selection_within_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_SELECTIONWITHIN, func));
    }

    /**
     * 显示隐藏的对象。 系统为下列用户界面元素发送此事件：插入符号、光标和窗口对象。 服务器应用程序为它们的辅助性对象发送该事件。
     * 客户端假定当父对象发送此事件时，已显示所有子对象。 因此，服务器应用程序不会为子对象发送此事件。
     * 隐藏的对象包括 STATE_SYSTEM_INVISIBLE 标志;shown 对象不包括此标志。 EVENT_OBJECT_SHOW 事件还指示已清除STATE_SYSTEM_INVISIBLE标志。 因此，在这种情况下，服务器不会发送 EVENT_OBJECT_STATECHANGE 事件。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_show_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_SHOW, func));
    }

    /**
     * 对象的状态已更改。 系统为以下用户界面元素发送此事件：检查框、组合框、标题控件、推送按钮、单选按钮、滚动条、工具栏控件、树视图控件、向上-向下控件和窗口对象。 服务器应用程序为它们的辅助性对象发送该事件。
     * 例如，单击或释放按钮对象时，或者启用或禁用某个对象时，会发生状态更改。
     * 对于此事件通知，WinEventProc 回调函数的 idChild 参数标识其状态已更改的子对象。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_state_change_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_STATECHANGE, func));
    }

    /**
     * IME 组合中的转换目标已更改。 转换目标是 IME 组合的子集，主动选择作为用户发起的转换的目标。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_text_edit_conversion_target_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            EVENT_OBJECT_TEXTEDIT_CONVERSIONTARGETCHANGED,
            func,
        ));
    }

    /**
     * 对象的文本选择已更改。 此事件受常见控件支持，由 UI 自动化使用。
     * WinEventProc 回调函数的 hwnd、ID 和 idChild 参数描述更新的文本选择中包含的项。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_text_selection_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_TEXTSELECTIONCHANGED, func));
    }

    /**
     * 在取消隐藏窗口时发送。 隐藏的窗口仍然存在，但对用户不可见。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_uncloaked_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_UNCLOAKED, func));
    }

    /**
     * 对象的 Value 属性 已更改。 系统为包括滚动条和以下控件的用户界面元素发送此事件：编辑、标头、热键、进度栏、滑块和向上。 服务器应用程序为它们的辅助性对象发送该事件。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_value_change_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OBJECT_VALUECHANGE, func));
    }

    /**
     * 为 OEM 保留的事件常量值的最低范围。 有关详细信息，请参阅 WinEvent ID 的分配。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_oem_defined_start_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OEM_DEFINED_START, func));
    }

    /**
     * 为 OEM 保留的事件常量值的最高范围。 有关详细信息，请参阅 WinEvent ID 的分配。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_oem_defined_end_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_OEM_DEFINED_END, func));
    }

    /**
     * 已生成警报。 服务器应用程序不应发送此事件。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_alert_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_ALERT, func));
    }

    /**
     * 正在显示预览矩形。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_arrangement_preview_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_ARRANGMENTPREVIEW, func));
    }

    /**
     * 窗口已丢失鼠标捕获。 此事件由系统发送，从不由服务器发送。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_capture_end_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_CAPTUREEND, func));
    }

    /**
     * 窗口已收到鼠标捕获。 此事件由系统发送，从不由服务器发送。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_capture_start_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_CAPTURESTART, func));
    }

    /**
     * 窗口已退出上下文相关帮助模式。 系统不一致地发送此事件。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_context_help_end_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_CONTEXTHELPEND, func));
    }

    /**
     * 窗口已进入上下文相关帮助模式。 系统不一致地发送此事件。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_context_help_start_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_CONTEXTHELPSTART, func));
    }

    /**
     * 已切换活动桌面。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_desktop_switch_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_DESKTOPSWITCH, func));
    }

    /**
     * 对话框已关闭。 系统为标准对话框发送此事件;服务器将其发送到自定义对话框。 系统不一致地发送此事件。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_dialog_end_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_DIALOGEND, func));
    }

    /**
     * 已显示一个对话框。 系统为使用资源模板或 Win32 对话框函数创建的标准对话框发送此事件。 服务器为自定义对话框发送此事件，这些对话框是充当对话框但不是以标准方式创建的窗口。
     * 系统不一致地发送此事件。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_dialog_start_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_DIALOGSTART, func));
    }

    /**
     * 应用程序将退出拖放模式。 支持拖放操作的应用程序必须发送此事件;系统不发送此事件。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_drag_drop_end_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_DRAGDROPEND, func));
    }

    /**
     * 应用程序将进入拖放模式。 支持拖放操作的应用程序必须发送此事件，因为系统不会发送它。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_drag_drop_start_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_DRAGDROPSTART, func));
    }

    /**
     * 最高的系统事件值。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_end_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_END, func));
    }

    //noinspection SpellCheckingInspection
    /**
     * 前景窗口已更改。 即使前台窗口已更改为同一线程中的另一个窗口，系统也会发送此事件。 服务器应用程序从不发送该事件。
     * 对于此事件， WinEventProc 回调函数的 hwnd 参数是前台窗口的句柄， idObject 参数 OBJID_WINDOW， idChild 参数 CHILDID_SELF。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_foreground_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_FOREGROUND, func));
    }

    //noinspection SpellCheckingInspection
    /**
     * 弹出菜单已关闭。 系统为标准菜单发送此事件;服务器将其发送到自定义菜单。
     * 当弹出菜单关闭时，客户端将收到此消息，然后 接收EVENT_SYSTEM_MENUEND 事件。
     * 系统不一致地发送此事件。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_menu_popup_end_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_MENUPOPUPEND, func));
    }

    //noinspection SpellCheckingInspection
    /**
     * 已显示弹出菜单。 系统为标准菜单发送此事件，这些菜单由 HMENU 标识，并使用菜单模板资源或 Win32 菜单函数创建。 服务器为自定义菜单发送此事件，这些菜单是充当菜单但不以标准方式创建的用户界面元素。 系统不一致地发送此事件。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_menu_popup_start_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_MENUPOPUPSTART, func));
    }

    //noinspection SpellCheckingInspection
    /**
     * 菜单栏中的菜单已关闭。 系统为标准菜单发送此事件;服务器将其发送到自定义菜单。
     * 对于此事件， WinEventProc 回调函数的 hwnd、 idObject 和 idChild 参数引用包含菜单栏的控件或激活上下文菜单的控件。 hwnd 参数是与事件相关的窗口的句柄。 idObject参数OBJID_MENU或OBJID_SYSMENU菜单，或弹出菜单的OBJID_WINDOW。 idChild参数CHILDID_SELF。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_menu_end_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_MENUEND, func));
    }

    //noinspection SpellCheckingInspection
    /**
     * 已选择菜单栏上的菜单项。 系统为标准菜单发送此事件，这些菜单由 HMENU 标识，使用菜单模板资源或 Win32 菜单 API 元素创建。 服务器为自定义菜单发送此事件，自定义菜单是充当菜单但不以标准方式创建的用户界面元素。
     * 对于此事件， WinEventProc 回调函数的 hwnd、 idObject 和 idChild 参数引用包含菜单栏的控件或激活上下文菜单的控件。 hwnd 参数是与事件相关的窗口的句柄。 idObject 参数是菜单的OBJID_MENU或OBJID_SYSMENU，或弹出菜单的OBJID_WINDOW。 idChild参数CHILDID_SELF。
     * 系统触发多个 EVENT_SYSTEM_MENUSTART 事件，这些事件并不总是与 EVENT_SYSTEM_MENUEND 事件相对应。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_menu_start_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_MENUSTART, func));
    }

    /**
     * 即将还原窗口对象。 此事件由系统发送，从不由服务器发送。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_minimize_end_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_MINIMIZEEND, func));
    }

    /**
     * 窗口对象即将最小化。 此事件由系统发送，从不由服务器发送。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_minimize_start_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_MINIMIZESTART, func));
    }

    /**
     * 窗口的移动或调整大小已完成。 此事件由系统发送，从不由服务器发送。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_move_size_end_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_MOVESIZEEND, func));
    }

    /**
     * 正在移动窗口或调整窗口的大小。 此事件由系统发送，从不由服务器发送。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_move_size_start_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_MOVESIZESTART, func));
    }

    //noinspection SpellCheckingInspection
    /**
     * 滚动条上的滚动已经结束。 此事件由系统为标准滚动条控件和附加到窗口的滚动条发送。 服务器为自定义滚动条发送此事件，这些滚动条是充当滚动条但不是以标准方式创建的用户界面元素。
     * 发送到 WinEventProc 回调函数的 idObject参数OBJID_HSCROLL水平滚动条，垂直滚动条OBJID_VSCROLL。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_scrolling_end_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_SCROLLINGEND, func));
    }

    //noinspection SpellCheckingInspection
    /**
     * 滚动条上的滚动已经开始。 系统为标准滚动条控件和附加到窗口的滚动条发送此事件。 服务器为自定义滚动条发送此事件，这些滚动条是充当滚动条但不是以标准方式创建的用户界面元素。
     * 发送到 WinEventProc 回调函数的 idObject 参数对水平滚动条OBJID_HSCROLL，垂直滚动条OBJID_VSCROLL。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_scrolling_start_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_SCROLLINGSTART, func));
    }

    //noinspection SpellCheckingInspection
    /**
     * 已播放声音。 当系统声音（例如菜单声音）被播放时，系统发送此事件，即使没有声音 (，例如，由于缺少声音文件或声音卡) 。 每当自定义 UI 元素生成声音时，服务器都会发送此事件。
     * 对于此事件， WinEventProc 回调函数接收 OBJID_SOUND 值作为 idObject 参数。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_sound_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_SOUND, func));
    }

    //noinspection SpellCheckingInspection
    /**
     * 用户已释放 Alt+TAB。 此事件由系统发送，从不由服务器发送。 WinEventProc 回调函数的 hwnd 参数标识用户已切换到的窗口。
     * 如果用户按下 Alt+TAB 时只有一个应用程序正在运行，则系统会发送此事件，而不发送相应的 EVENT_SYSTEM_SWITCHSTART 事件。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_switch_end_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_SWITCHEND, func));
    }

    //noinspection SpellCheckingInspection
    /**
     * 用户已按 Alt+TAB，这会激活切换窗口。 此事件由系统发送，从不由服务器发送。 WinEventProc 回调函数的 hwnd 参数标识用户要切换到的窗口。
     * 如果用户按 Alt+TAB 时只有一个应用程序正在运行，则系统会发送 EVENT_SYSTEM_SWITCHEND 事件，而不发送相应的 EVENT_SYSTEM_SWITCHSTART 事件。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_system_switch_start_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(EVENT_SYSTEM_SWITCHSTART, func));
    }

    /**
     * 移除所有注册的监听器。
     * */
    pub fn remove_all_listeners(&self) {
        let mut lock = self.events.write().unwrap();
        for i in lock.iter() {
            i.unhook();
        }
        lock.clear();
    }
}

impl Drop for Msaa {
    fn drop(&mut self) {
        self.remove_all_listeners()
    }
}

#[cfg(test)]
mod test_msaa {
    use super::super::common::get_desktop_window;
    use super::{object::ROLE_SYSTEM_WINDOW, Msaa};

    #[test]
    fn main() {
        let msaa = Msaa::new();
        let obj = msaa.get_desktop_object().unwrap();
        assert_eq!(obj.get_name(0), "桌面");
        assert_eq!(obj.get_description(0), String::new());
        assert_eq!(obj.get_help(0), String::new());
        assert_eq!(obj.get_keyboard_shortcut(0), String::new());
        assert_eq!(obj.get_value(0), String::new());
        assert_eq!(obj.get_default_action(0), String::new());
        assert_eq!(obj.get_role(0), ROLE_SYSTEM_WINDOW);
        assert_eq!(obj.child_count(), 7);
        assert_eq!(obj.get_role_text(0), "窗口");
        assert_eq!(obj.get_state_text(0), "可设定焦点");
        assert_eq!(obj.window(), get_desktop_window());

        let location = obj.location(0);
        dbg!(location);
        msaa.add_on_object_focus_listener(|src| {
            dbg!(src.get_object().unwrap());
        });
        std::thread::sleep(std::time::Duration::from_millis(5000));
        dbg!(msaa);
    }
}
