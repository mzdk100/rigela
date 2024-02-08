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

use crate::common::{Result, HWND};
use std::fmt::{Debug, Display, Formatter};
use windows::Win32::System::Variant::VARIANT;
use windows::Win32::UI::Accessibility::AccessibleChildren;
pub use windows::Win32::UI::Accessibility::{
    ROLE_SYSTEM_ALERT, ROLE_SYSTEM_ANIMATION, ROLE_SYSTEM_APPLICATION, ROLE_SYSTEM_BORDER,
    ROLE_SYSTEM_BUTTONDROPDOWN, ROLE_SYSTEM_BUTTONDROPDOWNGRID, ROLE_SYSTEM_BUTTONMENU,
    ROLE_SYSTEM_CARET, ROLE_SYSTEM_CELL, ROLE_SYSTEM_CHARACTER, ROLE_SYSTEM_CHART,
    ROLE_SYSTEM_CHECKBUTTON, ROLE_SYSTEM_CLIENT, ROLE_SYSTEM_CLOCK, ROLE_SYSTEM_COLUMN,
    ROLE_SYSTEM_COLUMNHEADER, ROLE_SYSTEM_COMBOBOX, ROLE_SYSTEM_CURSOR, ROLE_SYSTEM_DIAGRAM,
    ROLE_SYSTEM_DIAL, ROLE_SYSTEM_DIALOG, ROLE_SYSTEM_DOCUMENT, ROLE_SYSTEM_DROPLIST,
    ROLE_SYSTEM_EQUATION, ROLE_SYSTEM_GRAPHIC, ROLE_SYSTEM_GRIP, ROLE_SYSTEM_GROUPING,
    ROLE_SYSTEM_HELPBALLOON, ROLE_SYSTEM_HOTKEYFIELD, ROLE_SYSTEM_INDICATOR, ROLE_SYSTEM_IPADDRESS,
    ROLE_SYSTEM_LINK, ROLE_SYSTEM_LIST, ROLE_SYSTEM_LISTITEM, ROLE_SYSTEM_MENUBAR,
    ROLE_SYSTEM_MENUITEM, ROLE_SYSTEM_MENUPOPUP, ROLE_SYSTEM_OUTLINE, ROLE_SYSTEM_OUTLINEBUTTON,
    ROLE_SYSTEM_OUTLINEITEM, ROLE_SYSTEM_PAGETAB, ROLE_SYSTEM_PAGETABLIST, ROLE_SYSTEM_PANE,
    ROLE_SYSTEM_PROGRESSBAR, ROLE_SYSTEM_PROPERTYPAGE, ROLE_SYSTEM_PUSHBUTTON,
    ROLE_SYSTEM_RADIOBUTTON, ROLE_SYSTEM_ROW, ROLE_SYSTEM_ROWHEADER, ROLE_SYSTEM_SCROLLBAR,
    ROLE_SYSTEM_SEPARATOR, ROLE_SYSTEM_SLIDER, ROLE_SYSTEM_SOUND, ROLE_SYSTEM_SPINBUTTON,
    ROLE_SYSTEM_SPLITBUTTON, ROLE_SYSTEM_STATICTEXT, ROLE_SYSTEM_STATUSBAR, ROLE_SYSTEM_TABLE,
    ROLE_SYSTEM_TEXT, ROLE_SYSTEM_TITLEBAR, ROLE_SYSTEM_TOOLBAR, ROLE_SYSTEM_TOOLTIP,
    ROLE_SYSTEM_WHITESPACE, ROLE_SYSTEM_WINDOW, STATE_SYSTEM_HASPOPUP, STATE_SYSTEM_NORMAL,
};
use windows::{
    core::{ComInterface, Error, Type, BSTR, HSTRING},
    Win32::{
        Foundation::{POINT, S_FALSE},
        System::{
            Com::IDispatch,
            Variant::{VariantClear, VariantInit, VT_DISPATCH, VT_EMPTY, VT_I4},
        },
        UI::{
            Accessibility::{
                AccessibleObjectFromEvent, AccessibleObjectFromPoint, AccessibleObjectFromWindow,
                GetRoleTextW, GetStateTextW, IAccessible, WindowFromAccessibleObject,
            },
            WindowsAndMessaging::{OBJID_CARET, OBJID_WINDOW},
        },
    },
};

pub struct AccessibleObject(IAccessible, i32);

impl AccessibleObject {
    pub(crate) fn from_raw(acc: IAccessible, child: i32) -> Self {
        Self(acc, child)
    }
    pub fn get_raw(&self) -> &IAccessible {
        &self.0
    }

    /**
     * 从窗口获取对象。
     * `h_wnd` 窗口句柄。
     * */
    pub fn from_window(h_wnd: HWND) -> Result<Self> {
        // https://learn.microsoft.com/zh-cn/windows/win32/api/oleacc/nf-oleacc-accessibleobjectfromwindow
        let acc = unsafe {
            let mut p_acc = std::mem::zeroed();
            if let Err(e) = AccessibleObjectFromWindow(
                h_wnd,
                OBJID_WINDOW.0 as u32,
                &IAccessible::IID,
                &mut p_acc,
            ) {
                return Err(e);
            }
            match IAccessible::from_abi(p_acc) {
                Err(e) => return Err(e),
                Ok(r) => r,
            }
        };
        Ok(Self(acc, 0))
    }

    /**
     * 从插入点获取对象。
     * */
    pub fn from_caret() -> Result<Self> {
        // https://learn.microsoft.com/zh-cn/windows/win32/api/oleacc/nf-oleacc-accessibleobjectfromwindow
        let acc = unsafe {
            let mut p_acc = std::mem::zeroed();
            if let Err(e) = AccessibleObjectFromWindow(
                None,
                OBJID_CARET.0 as u32,
                &IAccessible::IID,
                &mut p_acc,
            ) {
                return Err(e);
            }
            match IAccessible::from_abi(p_acc) {
                Err(e) => return Err(e),
                Ok(r) => r,
            }
        };
        Ok(Self(acc, 0))
    }

    /**
     * 从屏幕坐标获取对象。
     * `x` 横坐标。
     * `y` 纵坐标。
     * */
    pub fn from_point(x: i32, y: i32) -> Result<(Self, i32)> {
        // https://learn.microsoft.com/zh-cn/previous-versions/ms696163(v=vs.85)
        let acc = unsafe {
            let mut p_acc: Option<IAccessible> = None;
            let point = POINT { x, y };
            let mut var = VariantInit();
            if let Err(e) = AccessibleObjectFromPoint(point, &mut p_acc, &mut var) {
                VariantClear(&mut var).unwrap_or(());
                return Err(e);
            }
            let val = match p_acc {
                None => {
                    VariantClear(&mut var).unwrap_or(());
                    return Err(Error::new(
                        S_FALSE,
                        HSTRING::from(format!(
                            "Can't obtain the accessible object at ({}, {}).",
                            x, y
                        )),
                    ));
                }
                Some(r) => (r, var.Anonymous.Anonymous.Anonymous.intVal),
            };
            VariantClear(&mut var).unwrap_or(());
            val
        };
        Ok((Self(acc.0, acc.1), acc.1))
    }

    //noinspection SpellCheckingInspection
    /**
     * 从事件获取对象。
     * `h_wnd` 指定生成事件的窗口的窗口句柄。此值必须是发送到事件挂钩函数的窗口句柄。
     * `id` 指定生成事件的 对象的对象 ID。 此值必须是发送到事件挂钩函数的对象 ID。
     * `child_id` 指定事件是由对象还是由其子元素之一触发。如果对象触发了事件，则 child_id = CHILDID_SELF。如果子元素触发了事件， 则 child_id 是元素的子 ID。此值必须是发送到事件挂钩函数的子 ID。
     * */
    pub fn from_event(h_wnd: HWND, id: i32, child_id: i32) -> Result<(Self, i32)> {
        // https://learn.microsoft.com/zh-cn/windows/win32/api/oleacc/nf-oleacc-accessibleobjectfromevent
        let acc = unsafe {
            let mut p_acc = std::mem::zeroed();
            let mut var = std::mem::zeroed();
            if let Err(e) =
                AccessibleObjectFromEvent(h_wnd, id as u32, child_id as u32, &mut p_acc, &mut var)
            {
                return Err(e);
            }
            let val = match p_acc {
                None => {
                    return Err(Error::new(
                        S_FALSE,
                        HSTRING::from(format!(
                            "Can't obtain the accessible object, the h_wnd is {}.",
                            h_wnd.0
                        )),
                    ));
                }
                Some(r) => (r, var.Anonymous.Anonymous.Anonymous.intVal),
            };
            val
        };
        Ok((Self(acc.0, acc.1), acc.1))
    }

    /**
     * 获取对象名称。
     * `child` 子对象ID，0是对象本身。
     * */
    pub fn get_name(&self, child: i32) -> String {
        unsafe {
            let mut var = VariantInit();
            (*var.Anonymous.Anonymous).vt = VT_I4;
            (*var.Anonymous.Anonymous).Anonymous.intVal = child;
            let val = self.0.get_accName(var.clone()).unwrap_or(BSTR::new());
            VariantClear(&mut var).unwrap_or(());
            val.to_string()
        }
    }

    /**
     * 获取对象描述。
     * `child` 子对象ID，0是对象本身。
     * */
    pub fn get_description(&self, child: i32) -> String {
        unsafe {
            let mut var = VariantInit();
            (*var.Anonymous.Anonymous).vt = VT_I4;
            (*var.Anonymous.Anonymous).Anonymous.intVal = child;
            let val = self
                .0
                .get_accDescription(var.clone())
                .unwrap_or(BSTR::new());
            VariantClear(&mut var).unwrap_or(());
            val.to_string()
        }
    }

    /**
     * 获取对象帮助。
     * `child` 子对象ID，0是对象本身。
     * */
    pub fn get_help(&self, child: i32) -> String {
        unsafe {
            let mut var = VariantInit();
            (*var.Anonymous.Anonymous).vt = VT_I4;
            (*var.Anonymous.Anonymous).Anonymous.intVal = child;
            let val = self.0.get_accHelp(var.clone()).unwrap_or(BSTR::new());
            VariantClear(&mut var).unwrap_or(());
            val.to_string()
        }
    }

    /**
     * 查询与指定对象关联的 WinHelp 文件的完整路径;它还检索该文件中相应主题的标识符。并非所有对象都支持此属性。应用程序很少支持或使用此属性。（已经弃用）
     * `child` 子对象ID，0是对象本身。
     * */
    pub fn get_help_topic(&self, child: i32) -> (String, i32) {
        unsafe {
            let mut var = VariantInit();
            (*var.Anonymous.Anonymous).vt = VT_I4;
            (*var.Anonymous.Anonymous).Anonymous.intVal = child;
            let mut help_file = BSTR::new();
            let mut id_topic = 0i32;
            self.0
                .get_accHelpTopic(&mut help_file, var.clone(), &mut id_topic)
                .unwrap_or(());
            VariantClear(&mut var).unwrap_or(());
            (help_file.to_string(), id_topic)
        }
    }

    /**
     * 获取对象快捷键。
     * `child` 子对象ID，0是对象本身。
     * */
    pub fn get_keyboard_shortcut(&self, child: i32) -> String {
        unsafe {
            let mut var = VariantInit();
            (*var.Anonymous.Anonymous).vt = VT_I4;
            (*var.Anonymous.Anonymous).Anonymous.intVal = child;
            let val = self
                .0
                .get_accKeyboardShortcut(var.clone())
                .unwrap_or(BSTR::new());
            VariantClear(&mut var).unwrap_or(());
            val.to_string()
        }
    }

    /**
     * 获取对象值。
     * `child` 子对象ID，0是对象本身。
     * */
    pub fn get_value(&self, child: i32) -> String {
        unsafe {
            let mut var = VariantInit();
            (*var.Anonymous.Anonymous).vt = VT_I4;
            (*var.Anonymous.Anonymous).Anonymous.intVal = child;
            let val = self.0.get_accValue(var.clone()).unwrap_or(BSTR::new());
            VariantClear(&mut var).unwrap_or(());
            val.to_string()
        }
    }

    /**
     * 获取对象默认动作。
     * `child` 子对象ID，0是对象本身。
     * */
    pub fn get_default_action(&self, child: i32) -> String {
        unsafe {
            let mut var = VariantInit();
            (*var.Anonymous.Anonymous).vt = VT_I4;
            (*var.Anonymous.Anonymous).Anonymous.intVal = child;
            let val = self
                .0
                .get_accDefaultAction(var.clone())
                .unwrap_or(BSTR::new());
            VariantClear(&mut var).unwrap_or(());
            val.to_string()
        }
    }

    /**
     * 获取对象角色。
     * `child` 子对象ID，0是对象本身。
     * */
    pub fn get_role(&self, child: i32) -> u32 {
        unsafe {
            let mut var = VariantInit();
            (*var.Anonymous.Anonymous).vt = VT_I4;
            (*var.Anonymous.Anonymous).Anonymous.intVal = child;
            let val = if let Ok(mut v) = self.0.get_accRole(var.clone()) {
                let val = match v.Anonymous.Anonymous.vt {
                    VT_I4 => v.Anonymous.Anonymous.Anonymous.uintVal,
                    _ => 0,
                };
                VariantClear(&mut v).unwrap_or(());
                val
            } else {
                0
            };
            VariantClear(&mut var).unwrap_or(());
            val
        }
    }

    /**
     * 查询描述指定角色值的对象角色的本地化字符串。
     * `child` 子对象ID，0是对象本身。
     * */
    pub fn get_role_text(&self, child: i32) -> String {
        let role = self.get_role(child);
        let mut text: [u16; 32] = [0; 32];
        let len = unsafe { GetRoleTextW(role, Some(&mut text)) };
        String::from_utf16_lossy(&text[..len as usize])
    }

    /**
     * 查询描述单个预定义状态位标志的对象状态的本地化字符串。 由于状态值是一个或多个位标志的组合，因此客户端多次调用此函数以检索所有状态字符串。
     * `child` 子对象ID，0是对象本身。
     * */
    pub fn get_state_text(&self, child: i32) -> String {
        let state = self.get_state(child);
        let mut text: [u16; 32] = [0; 32];
        let len = unsafe { GetStateTextW(state, Some(&mut text)) };
        String::from_utf16_lossy(&text[..len as usize])
    }

    /**
     * 获取对象状态。
     * `child` 子对象ID，0是对象本身。
     * */
    pub fn get_state(&self, child: i32) -> u32 {
        unsafe {
            let mut var = VariantInit();
            (*var.Anonymous.Anonymous).vt = VT_I4;
            (*var.Anonymous.Anonymous).Anonymous.intVal = child;
            let val = if let Ok(mut v) = self.0.get_accState(var.clone()) {
                let val = match v.Anonymous.Anonymous.vt {
                    VT_I4 => v.Anonymous.Anonymous.Anonymous.uintVal,
                    _ => 0,
                };
                VariantClear(&mut v).unwrap_or(());
                val
            } else {
                0
            };
            VariantClear(&mut var).unwrap_or(());
            val
        }
    }

    /**
     * 执行默认动作。
     * `child` 子对象ID，0是对象本身。
     * */
    pub fn do_default_action(&self, child: i32) {
        unsafe {
            let mut var = VariantInit();
            (*var.Anonymous.Anonymous).vt = VT_I4;
            (*var.Anonymous.Anonymous).Anonymous.intVal = child;
            self.0.accDoDefaultAction(var.clone()).unwrap_or(());
            VariantClear(&mut var).unwrap_or(());
        }
    }

    //noinspection SpellCheckingInspection
    /**
     * 修改所选内容或移动指定对象的键盘焦点。
     * `flags` 指定要执行哪些选择或焦点操作。此参数必须具有 SELFLAG 常量的组合。
     * `child` 子对象ID，0是对象本身。
     * */
    pub fn select(&self, flags: i32, child: i32) {
        unsafe {
            let mut var = VariantInit();
            (*var.Anonymous.Anonymous).vt = VT_I4;
            (*var.Anonymous.Anonymous).Anonymous.intVal = child;
            self.0.accSelect(flags, var.clone()).unwrap_or(());
            VariantClear(&mut var).unwrap_or(());
        }
    }

    /**
     * 遍历到容器中的另一个 UI 元素并查询对象。（已经弃用）
     * `nav_dir` 指定导航方向。此方向按空间顺序（如左或右）或逻辑顺序（例如下一个或上一个）。此值是导航常量之一。
     * `start` 起始子对象ID，0是对象本身。
     * */
    pub fn navigate(&self, nav_dir: i32, start: i32) -> AccessibleResultType {
        unsafe {
            let mut var = VariantInit();
            (*var.Anonymous.Anonymous).vt = VT_I4;
            (*var.Anonymous.Anonymous).Anonymous.intVal = start;
            let val = if let Ok(mut v) = self.0.accNavigate(nav_dir, var.clone()) {
                let val = match v.Anonymous.Anonymous.vt {
                    VT_I4 => AccessibleResultType::ChildId(v.Anonymous.Anonymous.Anonymous.uintVal),
                    VT_EMPTY => AccessibleResultType::None,
                    VT_DISPATCH => AccessibleResultType::Object(
                        (&*v.Anonymous.Anonymous.Anonymous.ppdispVal)
                            .clone()
                            .unwrap(),
                    ),
                    _ => AccessibleResultType::None,
                };
                VariantClear(&mut v).unwrap_or(());
                val
            } else {
                AccessibleResultType::None
            };
            VariantClear(&mut var).unwrap_or(());
            val
        }
    }

    /**
     * 查询在屏幕上特定点显示的子元素或子对象。
     * `left` 指定命中测试点的屏幕坐标。x 坐标从左到右增加。请注意，使用屏幕坐标时，原点是屏幕的左上角。
     * `top` 指定命中测试点的屏幕坐标。y 坐标从上到下增加。请注意，使用屏幕坐标时，原点是屏幕的左上角。
     * */
    pub fn hit_test(&self, left: i32, top: i32) -> AccessibleResultType {
        unsafe {
            if let Ok(mut v) = self.0.accHitTest(left, top) {
                let val = match v.Anonymous.Anonymous.vt {
                    VT_I4 => AccessibleResultType::ChildId(v.Anonymous.Anonymous.Anonymous.uintVal),
                    VT_EMPTY => AccessibleResultType::None,
                    VT_DISPATCH => AccessibleResultType::Object(
                        (&*v.Anonymous.Anonymous.Anonymous.ppdispVal)
                            .clone()
                            .unwrap(),
                    ),
                    _ => AccessibleResultType::None,
                };
                VariantClear(&mut v).unwrap_or(());
                val
            } else {
                AccessibleResultType::None
            }
        }
    }

    /**
     * 获取焦点对象。
     * */
    pub fn focus(&self) -> AccessibleResultType {
        unsafe {
            if let Ok(mut v) = self.0.accFocus() {
                let val = match v.Anonymous.Anonymous.vt {
                    VT_I4 => AccessibleResultType::ChildId(v.Anonymous.Anonymous.Anonymous.uintVal),
                    VT_EMPTY => AccessibleResultType::None,
                    VT_DISPATCH => AccessibleResultType::Object(
                        (&*v.Anonymous.Anonymous.Anonymous.ppdispVal)
                            .clone()
                            .unwrap(),
                    ),
                    _ => AccessibleResultType::None,
                };
                VariantClear(&mut v).unwrap_or(());
                val
            } else {
                AccessibleResultType::None
            }
        }
    }

    /**
     * 获取选中对象。
     * */
    pub fn selection(&self) -> AccessibleResultType {
        unsafe {
            if let Ok(mut v) = self.0.accSelection() {
                let val = match v.Anonymous.Anonymous.vt {
                    VT_I4 => AccessibleResultType::ChildId(v.Anonymous.Anonymous.Anonymous.uintVal),
                    VT_EMPTY => AccessibleResultType::None,
                    VT_DISPATCH => AccessibleResultType::Object(
                        (&*v.Anonymous.Anonymous.Anonymous.ppdispVal)
                            .clone()
                            .unwrap(),
                    ),
                    _ => AccessibleResultType::None,
                };
                VariantClear(&mut v).unwrap_or(());
                val
            } else {
                AccessibleResultType::None
            }
        }
    }

    /**
     * 获取父对象。
     * */
    pub fn parent(&self) -> AccessibleResultType {
        unsafe {
            if let Ok(r) = self.0.accParent() {
                AccessibleResultType::Object(r)
            } else {
                AccessibleResultType::None
            }
        }
    }

    /**
     * 获取子对象数量。
     * */
    pub fn child_count(&self) -> u32 {
        unsafe {
            if let Ok(r) = self.0.accChildCount() {
                return r as u32;
            }
        }
        0
    }

    /**
     * 获取子对象。
     * `child` 子对象ID，0是对象本身。
     * */
    pub fn get_child(&self, child: i32) -> Result<AccessibleObject> {
        unsafe {
            let mut var = VariantInit();
            (*var.Anonymous.Anonymous).vt = VT_I4;
            (*var.Anonymous.Anonymous).Anonymous.intVal = child;
            match self.0.get_accChild(var) {
                Err(e) => Err(e),
                Ok(o) => Ok(AccessibleObject::from_raw(o.cast().unwrap(), 0)),
            }
        }
    }

    /**
     * 获取所有子对象。
     * */
    pub fn children(&self, start: u32, count: u32) -> Result<Vec<AccessibleObject>> {
        // https://learn.microsoft.com/zh-cn/windows/win32/api/oleacc/nf-oleacc-accessiblechildren
        unsafe {
            let mut arr = vec![];
            for _ in 0..count {
                arr.push(VARIANT::default());
            }
            let mut cnt = std::mem::zeroed();
            match AccessibleChildren(&self.0, start as i32, &mut arr, &mut cnt) {
                Err(e) => Err(e),
                Ok(_) => {
                    let mut v = vec![];
                    for i in 0..cnt {
                        let child = match arr[i as usize].Anonymous.Anonymous.vt {
                            VT_I4 => AccessibleObject::from_raw(
                                self.0.clone(),
                                arr[i as usize].Anonymous.Anonymous.Anonymous.intVal,
                            ),
                            VT_DISPATCH => AccessibleObject::from_raw(
                                arr[i as usize]
                                    .Anonymous
                                    .Anonymous
                                    .Anonymous
                                    .pdispVal
                                    .as_ref()
                                    .unwrap()
                                    .cast()
                                    .unwrap(),
                                0,
                            ),
                            _ => continue,
                        };
                        v.push(child);
                        VariantClear(&mut arr[i as usize]).unwrap_or(());
                    }
                    Ok(v)
                }
            }
        }
    }

    /**
     * 获取对象位置和大小。
     * `child` 子对象ID，0是对象本身。
     * */
    pub fn location(&self, child: i32) -> (i32, i32, i32, i32) {
        unsafe {
            let (mut left, mut top, mut width, mut height) = (0i32, 0i32, 0i32, 0i32);
            let mut var = VariantInit();
            (*var.Anonymous.Anonymous).vt = VT_I4;
            (*var.Anonymous.Anonymous).Anonymous.intVal = child;
            let val = if let Ok(_) =
                self.0
                    .accLocation(&mut left, &mut top, &mut width, &mut height, var.clone())
            {
                (left, top, width, height)
            } else {
                (0, 0, 0, 0)
            };
            VariantClear(&mut var).unwrap_or(());
            val
        }
    }

    /**
     * 不再支持 put_accName 方法。客户端应用程序应使用特定于控件的解决方法，例如SetWindowText 函数。服务器应返回E_NOTIMPL。
     * */
    #[allow(unused_variables)]
    pub fn put_name(&self, child: i32, name: String) {
        unreachable!()
    }

    /**
     * 设置指定对象的值。并非所有对象都有值。
     * `child` 子对象ID，0是对象本身。
     * `value` 包含对象的值的本地化字符串。
     * */
    pub fn put_value(&self, child: i32, value: String) {
        unsafe {
            let mut var = VariantInit();
            (*var.Anonymous.Anonymous).vt = VT_I4;
            (*var.Anonymous.Anonymous).Anonymous.intVal = child;
            self.0
                .put_accValue(var.clone(), &BSTR::from(value.as_str()))
                .unwrap_or(());
            VariantClear(&mut var).unwrap_or(());
        }
    }

    /* 获取窗口句柄。 */
    pub fn window(&self) -> HWND {
        let mut h_wnd = HWND::default();
        unsafe {
            WindowFromAccessibleObject(&self.0, Some(&mut h_wnd)).unwrap_or(());
        }
        h_wnd
    }
}

pub enum AccessibleResultType {
    None,
    ChildId(u32),
    Object(IDispatch),
}

impl Debug for AccessibleObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for AccessibleObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AccessibleObject(name:{}, description:{}, role:{})",
            self.get_name(self.1),
            self.get_description(self.1),
            self.get_role_text(self.1)
        )
    }
}

unsafe impl Sync for AccessibleObject {}
unsafe impl Send for AccessibleObject {}
