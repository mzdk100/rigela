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

use std::{
    fmt::{Debug, Display, Formatter},
    sync::Weak,
};

use windows::{
    core::BSTR,
    Win32::{
        Foundation::{HWND, RECT},
        UI::Accessibility::{
            IUIAutomation6, IUIAutomationElement, IUIAutomationElementArray, TreeScope_Children,
            UIA_AppBarControlTypeId, UIA_ButtonControlTypeId, UIA_CalendarControlTypeId,
            UIA_CheckBoxControlTypeId, UIA_ComboBoxControlTypeId, UIA_CustomControlTypeId,
            UIA_DataGridControlTypeId, UIA_DataItemControlTypeId, UIA_DocumentControlTypeId,
            UIA_EditControlTypeId, UIA_GroupControlTypeId, UIA_HeaderControlTypeId,
            UIA_HeaderItemControlTypeId, UIA_HyperlinkControlTypeId, UIA_ImageControlTypeId,
            UIA_ListControlTypeId, UIA_ListItemControlTypeId, UIA_MenuBarControlTypeId,
            UIA_MenuControlTypeId, UIA_MenuItemControlTypeId, UIA_PaneControlTypeId,
            UIA_ProgressBarControlTypeId, UIA_RadioButtonControlTypeId, UIA_ScrollBarControlTypeId,
            UIA_SemanticZoomControlTypeId, UIA_SeparatorControlTypeId, UIA_SliderControlTypeId,
            UIA_SpinnerControlTypeId, UIA_SplitButtonControlTypeId, UIA_StatusBarControlTypeId,
            UIA_TabControlTypeId, UIA_TabItemControlTypeId, UIA_TableControlTypeId,
            UIA_TextControlTypeId, UIA_ThumbControlTypeId, UIA_TitleBarControlTypeId,
            UIA_ToolBarControlTypeId, UIA_ToolTipControlTypeId, UIA_TreeControlTypeId,
            UIA_TreeItemControlTypeId, UIA_WindowControlTypeId, UIA_CONTROLTYPE_ID,
        },
    },
};

use crate::{common::Result, ext::VecExt};

/// UiAutomationElement 的本地封装
#[derive(Clone)]
pub struct UiAutomationElement {
    _automation: Weak<IUIAutomation6>,
    _current: IUIAutomationElement,
}

impl UiAutomationElement {
    /**
     * 获取原始的元素引用（不对外暴露）。
     * */
    pub(crate) fn get_raw(&self) -> &IUIAutomationElement {
        &self._current
    }
    pub fn get_aut(&self) -> Weak<IUIAutomation6> {
        self._automation.clone()
    }

    pub(crate) fn get_aut_ref(&self) -> &IUIAutomation6 {
        unsafe { &*self._automation.as_ptr() }
    }

    pub(crate) fn obtain(automation: Weak<IUIAutomation6>, element: IUIAutomationElement) -> Self {
        Self {
            _automation: automation,
            _current: element,
        }
    }

    /**
     * 获取元素的当前名称。
     * */
    pub fn get_name(&self) -> String {
        unsafe { self._current.CurrentName() }
            // 不需要手动释放BSTR类型的指针，windows-rs已经对BSTR类型实现drop特征
            .unwrap_or(BSTR::new())
            .to_string()
    }

    /**
     * 获取本土化的控件类型描述。
     * */
    pub fn get_localized_control_type(&self) -> String {
        unsafe { self._current.CurrentLocalizedControlType() }
            .unwrap_or(BSTR::new())
            .to_string()
    }

    /**
     * 获取窗口句柄。
     * */
    pub fn native_window_handle(&self) -> HWND {
        unsafe { self._current.CurrentNativeWindowHandle() }.unwrap_or(HWND::default())
    }

    /**
     * 获取父对象。
     * */
    pub fn get_parent(&self) -> Result<UiAutomationElement> {
        match unsafe { self._current.GetCachedParent() } {
            Ok(p) => Ok(UiAutomationElement::obtain(self._automation.clone(), p)),
            Err(e) => Err(e),
        }
    }

    /**
     * 获取控件类型。
     * */
    pub fn get_control_type(&self) -> ControlType {
        unsafe {
            match self._current.CurrentControlType() {
                Ok(x) => ControlType::from(x),
                Err(_) => ControlType::Custom,
            }
        }
    }

    /**
     * 获取元素支持的模式列表(ids,names)。
     * */
    pub fn get_supported_patterns(&self) -> Result<(Vec<i32>, Vec<String>)> {
        unsafe {
            let (mut ids, mut names) = std::mem::zeroed();
            if let Err(e) = self.get_aut_ref().PollForPotentialSupportedPatterns(
                &self._current,
                &mut ids,
                &mut names,
            ) {
                return Err(e);
            }
            let names: Vec<BSTR> = names.to_vec();
            Ok((ids.to_vec(), names.iter().map(|i| i.to_string()).collect()))
        }
    }

    /**
     * 获取提供程序描述。
     * */
    pub fn get_provider_description(&self) -> String {
        unsafe { self._current.CurrentProviderDescription() }
            .unwrap_or(BSTR::new())
            .to_string()
    }

    /**
     * 获取子元素数量。
     * */
    pub fn get_child_count(&self) -> i32 {
        if let Ok(children) = unsafe {
            self._current.FindAll(
                TreeScope_Children,
                &self.get_aut_ref().CreateTrueCondition().unwrap(),
            )
        } {
            return unsafe { children.Length() }.unwrap();
        }
        0
    }

    /**
     * 获取子元素。
     * `index` 序号。
     * */
    pub fn get_child(&self, index: i32) -> Option<UiAutomationElement> {
        if let Ok(children) = unsafe {
            self._current.FindAll(
                TreeScope_Children,
                &self.get_aut_ref().CreateTrueCondition().unwrap(),
            )
        } {
            if let Ok(el) = unsafe { children.GetElement(index) } {
                return Some(UiAutomationElement::obtain(self._automation.clone(), el));
            }
        }
        None
    }

    /**
     * 获取元素的矩形框
     */
    pub fn get_bounding_rectangle(&self) -> RECT {
        unsafe { self._current.CurrentBoundingRectangle() }
            .expect("Can't get the location of element.")
    }

    /**
     * 获取元素的当前自动化ID。
     * */
    pub fn get_automation_id(&self) -> String {
        unsafe { self._current.CurrentAutomationId() }
            .unwrap_or(BSTR::new())
            .to_string()
    }

    /**
     * 获取元素的当前类名。
     * */
    #[allow(dead_code)]
    pub fn get_class_name(&self) -> String {
        unsafe { self._current.CurrentClassName() }
            .unwrap_or(BSTR::new())
            .to_string()
    }

    /**
     * 获取项目状态。
     * */
    pub fn get_item_status(&self) -> String {
        unsafe { self._current.CurrentItemStatus() }
            .unwrap_or(BSTR::new())
            .to_string()
    }

    /**
     * 获取加速键。
     * */
    pub fn get_accelerator_key(&self) -> String {
        unsafe { self._current.CurrentAcceleratorKey() }
            .unwrap_or(BSTR::new())
            .to_string()
    }

    /**
     * 获取访问键。
     * */
    pub fn get_access_key(&self) -> String {
        unsafe { self._current.CurrentAccessKey() }
            .unwrap_or(BSTR::new())
            .to_string()
    }
}

unsafe impl Send for UiAutomationElement {}

unsafe impl Sync for UiAutomationElement {}

impl Debug for UiAutomationElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for UiAutomationElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UiAutomationElement(name:{})", self.get_name())
    }
}

impl VecExt<IUIAutomationElement> for &IUIAutomationElementArray {
    fn to_vec(self) -> Vec<IUIAutomationElement> {
        unsafe {
            let mut v = vec![];
            for i in 0..self.Length().unwrap() {
                v.push(self.GetElement(i).unwrap());
            }
            v
        }
    }
}

pub enum ControlType {
    AppBar,
    Button,
    Calendar,
    CheckBox,
    ComboBox,
    Custom,
    DataGrid,
    DataItem,
    Document,
    Edit,
    Group,
    Header,
    HeaderItem,
    Hyperlink,
    Image,
    List,
    ListItem,
    MenuBar,
    Menu,
    MenuItem,
    Pane,
    ProgressBar,
    RadioButton,
    ScrollBar,
    SemanticZoom,
    Separator,
    Slider,
    Spinner,
    SplitButton,
    StatusBar,
    Tab,
    TabItem,
    Table,
    Text,
    Thumb,
    TitleBar,
    ToolBar,
    ToolTip,
    Tree,
    TreeItem,
    Window,
}

impl From<UIA_CONTROLTYPE_ID> for ControlType {
    #[allow(non_upper_case_globals)]
    fn from(value: UIA_CONTROLTYPE_ID) -> Self {
        match value {
            UIA_AppBarControlTypeId => Self::AppBar,
            UIA_ButtonControlTypeId => Self::Button,
            UIA_CalendarControlTypeId => Self::Calendar,
            UIA_CheckBoxControlTypeId => Self::CheckBox,
            UIA_ComboBoxControlTypeId => Self::ComboBox,
            UIA_CustomControlTypeId => Self::Custom,
            UIA_DataGridControlTypeId => Self::DataGrid,
            UIA_DataItemControlTypeId => Self::DataItem,
            UIA_DocumentControlTypeId => Self::Document,
            UIA_EditControlTypeId => Self::Edit,
            UIA_GroupControlTypeId => Self::Group,
            UIA_HeaderControlTypeId => Self::Header,
            UIA_HeaderItemControlTypeId => Self::HeaderItem,
            UIA_HyperlinkControlTypeId => Self::Hyperlink,
            UIA_ImageControlTypeId => Self::Image,
            UIA_ListControlTypeId => Self::List,
            UIA_ListItemControlTypeId => Self::ListItem,
            UIA_MenuBarControlTypeId => Self::MenuBar,
            UIA_MenuControlTypeId => Self::Menu,
            UIA_MenuItemControlTypeId => Self::MenuItem,
            UIA_PaneControlTypeId => Self::Pane,
            UIA_ProgressBarControlTypeId => Self::ProgressBar,
            UIA_RadioButtonControlTypeId => Self::RadioButton,
            UIA_ScrollBarControlTypeId => Self::ScrollBar,
            UIA_SemanticZoomControlTypeId => Self::SemanticZoom,
            UIA_SeparatorControlTypeId => Self::Separator,
            UIA_SliderControlTypeId => Self::Slider,
            UIA_SpinnerControlTypeId => Self::Spinner,
            UIA_SplitButtonControlTypeId => Self::SplitButton,
            UIA_StatusBarControlTypeId => Self::StatusBar,
            UIA_TabControlTypeId => Self::Tab,
            UIA_TabItemControlTypeId => Self::TabItem,
            UIA_TableControlTypeId => Self::Table,
            UIA_TextControlTypeId => Self::Text,
            UIA_ThumbControlTypeId => Self::Thumb,
            UIA_TitleBarControlTypeId => Self::TitleBar,
            UIA_ToolBarControlTypeId => Self::ToolBar,
            UIA_ToolTipControlTypeId => Self::ToolTip,
            UIA_TreeControlTypeId => Self::Tree,
            UIA_TreeItemControlTypeId => Self::TreeItem,
            UIA_WindowControlTypeId => Self::Window,
            _ => Self::Custom,
        }
    }
}
