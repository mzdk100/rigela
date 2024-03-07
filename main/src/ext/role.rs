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

use a11y::jab::{context::AccessibleContext, role::AccessibleRole};
use win_wrap::msaa::object::{
    AccessibleObject, ROLE_SYSTEM_ALERT, ROLE_SYSTEM_ANIMATION, ROLE_SYSTEM_APPLICATION,
    ROLE_SYSTEM_BORDER, ROLE_SYSTEM_BUTTONDROPDOWN, ROLE_SYSTEM_BUTTONDROPDOWNGRID,
    ROLE_SYSTEM_BUTTONMENU, ROLE_SYSTEM_CARET, ROLE_SYSTEM_CELL, ROLE_SYSTEM_CHARACTER,
    ROLE_SYSTEM_CHART, ROLE_SYSTEM_CHECKBUTTON, ROLE_SYSTEM_CLIENT, ROLE_SYSTEM_CLOCK,
    ROLE_SYSTEM_COLUMN, ROLE_SYSTEM_COLUMNHEADER, ROLE_SYSTEM_COMBOBOX, ROLE_SYSTEM_CURSOR,
    ROLE_SYSTEM_DIAGRAM, ROLE_SYSTEM_DIAL, ROLE_SYSTEM_DIALOG, ROLE_SYSTEM_DOCUMENT,
    ROLE_SYSTEM_DROPLIST, ROLE_SYSTEM_EQUATION, ROLE_SYSTEM_GRAPHIC, ROLE_SYSTEM_GRIP,
    ROLE_SYSTEM_GROUPING, ROLE_SYSTEM_HELPBALLOON, ROLE_SYSTEM_HOTKEYFIELD, ROLE_SYSTEM_INDICATOR,
    ROLE_SYSTEM_IPADDRESS, ROLE_SYSTEM_LINK, ROLE_SYSTEM_LIST, ROLE_SYSTEM_LISTITEM,
    ROLE_SYSTEM_MENUBAR, ROLE_SYSTEM_MENUITEM, ROLE_SYSTEM_MENUPOPUP, ROLE_SYSTEM_OUTLINE,
    ROLE_SYSTEM_OUTLINEBUTTON, ROLE_SYSTEM_OUTLINEITEM, ROLE_SYSTEM_PAGETAB,
    ROLE_SYSTEM_PAGETABLIST, ROLE_SYSTEM_PANE, ROLE_SYSTEM_PROGRESSBAR, ROLE_SYSTEM_PROPERTYPAGE,
    ROLE_SYSTEM_PUSHBUTTON, ROLE_SYSTEM_RADIOBUTTON, ROLE_SYSTEM_ROW, ROLE_SYSTEM_ROWHEADER,
    ROLE_SYSTEM_SCROLLBAR, ROLE_SYSTEM_SEPARATOR, ROLE_SYSTEM_SLIDER, ROLE_SYSTEM_SOUND,
    ROLE_SYSTEM_SPINBUTTON, ROLE_SYSTEM_SPLITBUTTON, ROLE_SYSTEM_STATICTEXT, ROLE_SYSTEM_STATUSBAR,
    ROLE_SYSTEM_TABLE, ROLE_SYSTEM_TEXT, ROLE_SYSTEM_TITLEBAR, ROLE_SYSTEM_TOOLBAR,
    ROLE_SYSTEM_TOOLTIP, ROLE_SYSTEM_WHITESPACE, ROLE_SYSTEM_WINDOW,
};
use win_wrap::uia::element::{ControlType, UiAutomationElement};

pub(crate) trait AccessibleRoleExt {
    /**
     * 获取角色名称。
     * */
    fn get_role_name(&self) -> String;
}

impl AccessibleRoleExt for UiAutomationElement {
    fn get_role_name(&self) -> String {
        match self.get_control_type() {
            ControlType::AppBar => t!("role.app_bar"),
            ControlType::Button => t!("role.button"),
            ControlType::Calendar => t!("role.calendar"),
            ControlType::CheckBox => t!("role.check_box"),
            ControlType::ComboBox => t!("role.combo_box"),
            ControlType::Custom => t!("role.custom"),
            ControlType::DataGrid => t!("role.data_grid"),
            ControlType::DataItem => t!("role.data_item"),
            ControlType::Document => t!("role.document"),
            ControlType::Edit => t!("role.edit"),
            ControlType::Group => t!("role.group"),
            ControlType::Header => t!("role.header"),
            ControlType::HeaderItem => t!("role.header_item"),
            ControlType::Hyperlink => t!("role.hyperlink"),
            ControlType::Image => t!("role.image"),
            ControlType::List => t!("role.list"),
            ControlType::ListItem => t!("role.list_item"),
            ControlType::MenuBar => t!("role.menu_bar"),
            ControlType::Menu => t!("role.menu"),
            ControlType::MenuItem => t!("role.menu_item"),
            ControlType::Pane => t!("role.pane"),
            ControlType::ProgressBar => t!("role.progress_bar"),
            ControlType::RadioButton => t!("role.radio_button"),
            ControlType::ScrollBar => t!("role.scroll_bar"),
            ControlType::SemanticZoom => t!("role.semantic_zoom"),
            ControlType::Separator => t!("role.separator"),
            ControlType::Slider => t!("role.slider"),
            ControlType::Spinner => t!("role.spinner"),
            ControlType::SplitButton => t!("role.split_button"),
            ControlType::StatusBar => t!("role.status_bar"),
            ControlType::Tab => t!("role.tab"),
            ControlType::TabItem => t!("role.tab_item"),
            ControlType::Table => t!("role.table"),
            ControlType::Text => t!("role.text"),
            ControlType::Thumb => t!("role.thumb"),
            ControlType::TitleBar => t!("role.title_bar"),
            ControlType::ToolBar => t!("role.tool_bar"),
            ControlType::ToolTip => t!("role.tool_tip"),
            ControlType::Tree => t!("role.tree"),
            ControlType::TreeItem => t!("role.tree_item"),
            ControlType::Window => t!("role.window"),
        }
            .parse()
            .unwrap()
    }
}

impl AccessibleRoleExt for (AccessibleObject, i32) {
    fn get_role_name(&self) -> String {
        match self.0.get_role(self.1) {
            ROLE_SYSTEM_ALERT => t!("role.alert"),
            ROLE_SYSTEM_ANIMATION => t!("role.animation"),
            ROLE_SYSTEM_APPLICATION => t!("role.application"),
            ROLE_SYSTEM_BORDER => t!("role.border"),
            ROLE_SYSTEM_BUTTONDROPDOWN => t!("role.button_dropdown"),
            ROLE_SYSTEM_BUTTONDROPDOWNGRID => t!("role.button_dropdown_grid"),
            ROLE_SYSTEM_BUTTONMENU => t!("role.button_menu"),
            ROLE_SYSTEM_CARET => t!("role.caret"),
            ROLE_SYSTEM_CELL => t!("role.cell"),
            ROLE_SYSTEM_CHARACTER => t!("role.character"),
            ROLE_SYSTEM_CHART => t!("role.chart"),
            ROLE_SYSTEM_CHECKBUTTON => t!("role.check_button"),
            ROLE_SYSTEM_CLIENT => t!("role.client"),
            ROLE_SYSTEM_CLOCK => t!("role.clock"),
            ROLE_SYSTEM_COLUMN => t!("role.column"),
            ROLE_SYSTEM_COLUMNHEADER => t!("role.column_header"),
            ROLE_SYSTEM_COMBOBOX => t!("role.combo_box"),
            ROLE_SYSTEM_CURSOR => t!("role.cursor"),
            ROLE_SYSTEM_DIAGRAM => t!("role.diagram"),
            ROLE_SYSTEM_DIAL => t!("role.dial"),
            ROLE_SYSTEM_DIALOG => t!("role.dialog"),
            ROLE_SYSTEM_DOCUMENT => t!("role.document"),
            ROLE_SYSTEM_DROPLIST => t!("role.drop_list"),
            ROLE_SYSTEM_EQUATION => t!("role.equation"),
            ROLE_SYSTEM_GRAPHIC => t!("role.graphic"),
            ROLE_SYSTEM_GRIP => t!("role.grip"),
            ROLE_SYSTEM_GROUPING => t!("role.grouping"),
            ROLE_SYSTEM_HELPBALLOON => t!("role.help_balloon"),
            ROLE_SYSTEM_HOTKEYFIELD => t!("role.hotkey_field"),
            ROLE_SYSTEM_INDICATOR => t!("role.indicator"),
            ROLE_SYSTEM_IPADDRESS => t!("role.ip_address"),
            ROLE_SYSTEM_LINK => t!("role.link"),
            ROLE_SYSTEM_LIST => t!("role.list"),
            ROLE_SYSTEM_LISTITEM => t!("role.list_item"),
            ROLE_SYSTEM_MENUBAR => t!("role.menu_bar"),
            ROLE_SYSTEM_MENUITEM => t!("role.menu_item"),
            ROLE_SYSTEM_MENUPOPUP => t!("role.menu_popup"),
            ROLE_SYSTEM_OUTLINE => t!("role.outline"),
            ROLE_SYSTEM_OUTLINEBUTTON => t!("role.outline_button"),
            ROLE_SYSTEM_OUTLINEITEM => t!("role.outline_item"),
            ROLE_SYSTEM_PAGETAB => t!("role.page_tab"),
            ROLE_SYSTEM_PAGETABLIST => t!("role.page_tab_list"),
            ROLE_SYSTEM_PANE => t!("role.pane"),
            ROLE_SYSTEM_PROGRESSBAR => t!("role.progress_bar"),
            ROLE_SYSTEM_PROPERTYPAGE => t!("role.property_page"),
            ROLE_SYSTEM_PUSHBUTTON => t!("role.push_button"),
            ROLE_SYSTEM_RADIOBUTTON => t!("role.radio_button"),
            ROLE_SYSTEM_ROW => t!("role.row"),
            ROLE_SYSTEM_ROWHEADER => t!("role.row_header"),
            ROLE_SYSTEM_SCROLLBAR => t!("role.scroll_bar"),
            ROLE_SYSTEM_SEPARATOR => t!("role.separator"),
            ROLE_SYSTEM_SLIDER => t!("role.slider"),
            ROLE_SYSTEM_SOUND => t!("role.sound"),
            ROLE_SYSTEM_SPINBUTTON => t!("role.spin_button"),
            ROLE_SYSTEM_SPLITBUTTON => t!("role.split_button"),
            ROLE_SYSTEM_STATICTEXT => t!("role.static_text"),
            ROLE_SYSTEM_STATUSBAR => t!("role.status_bar"),
            ROLE_SYSTEM_TABLE => t!("role.table"),
            ROLE_SYSTEM_TEXT => t!("role.text"),
            ROLE_SYSTEM_TITLEBAR => t!("role.title_bar"),
            ROLE_SYSTEM_TOOLBAR => t!("role.tool_bar"),
            ROLE_SYSTEM_TOOLTIP => t!("role.tool_tip"),
            ROLE_SYSTEM_WHITESPACE => t!("role.white_space"),
            ROLE_SYSTEM_WINDOW => t!("role.window"),
            _ => t!("role.unknown"),
        }
            .parse()
            .unwrap()
    }
}

impl<'lib> AccessibleRoleExt for AccessibleContext<'lib> {
    fn get_role_name(&self) -> String {
        match self.get_role() {
            AccessibleRole::Alert => t!("role.alert"),
            AccessibleRole::ColumnHeader => t!("role.column_header"),
            AccessibleRole::Canvas => t!("role.canvas"),
            AccessibleRole::ComboBox => t!("role.combo_box"),
            AccessibleRole::DesktopIcon => t!("role.desktop_icon"),
            AccessibleRole::InternalFrame => t!("role.internal_frame"),
            AccessibleRole::DesktopPane => t!("role.desktop_pane"),
            AccessibleRole::OptionPane => t!("role.option_pane"),
            AccessibleRole::Window => t!("role.window"),
            AccessibleRole::Frame => t!("role.frame"),
            AccessibleRole::Dialog => t!("role.dialog"),
            AccessibleRole::ColorChooser => t!("role.color_chooser"),
            AccessibleRole::DirectoryPane => t!("role.directory_pane"),
            AccessibleRole::FileChooser => t!("role.file_chooser"),
            AccessibleRole::Filler => t!("role.filler"),
            AccessibleRole::Hyperlink => t!("role.hyperlink"),
            AccessibleRole::Icon => t!("role.icon"),
            AccessibleRole::Label => t!("role.label"),
            AccessibleRole::RootPane => t!("role.root_pane"),
            AccessibleRole::GlassPane => t!("role.glass_pane"),
            AccessibleRole::LayeredPane => t!("role.layered_pane"),
            AccessibleRole::List => t!("role.list"),
            AccessibleRole::ListItem => t!("role.list_item"),
            AccessibleRole::MenuBar => t!("role.menu_bar"),
            AccessibleRole::PopupMenu => t!("role.popup_menu"),
            AccessibleRole::Menu => t!("role.menu"),
            AccessibleRole::MenuItem => t!("role.menu_item"),
            AccessibleRole::SEPARATOR => t!("role.separator"),
            AccessibleRole::PageTabList => t!("role.page_tab_list"),
            AccessibleRole::PageTab => t!("role.page_tab"),
            AccessibleRole::Panel => t!("role.panel"),
            AccessibleRole::ProgressBar => t!("role.progress_bar"),
            AccessibleRole::PasswordText => t!("role.password_text"),
            AccessibleRole::PushButton => t!("role.push_button"),
            AccessibleRole::ToggleButton => t!("role.toggle_button"),
            AccessibleRole::CheckBox => t!("role.check_box"),
            AccessibleRole::RadioButton => t!("role.radio_button"),
            AccessibleRole::RowHeader => t!("role.row_header"),
            AccessibleRole::ScrollPane => t!("role.scroll_pane"),
            AccessibleRole::ScrollBar => t!("role.scroll_bar"),
            AccessibleRole::Viewport => t!("role.viewport"),
            AccessibleRole::Slider => t!("role.slider"),
            AccessibleRole::SplitPane => t!("role.split_pane"),
            AccessibleRole::Table => t!("role.table"),
            AccessibleRole::Text => t!("role.text"),
            AccessibleRole::Tree => t!("role.tree"),
            AccessibleRole::ToolBar => t!("role.tool_bar"),
            AccessibleRole::ToolTip => t!("role.tool_tip"),
            AccessibleRole::AwtComponent => t!("role.awt_component"),
            AccessibleRole::SwingComponent => t!("role.swing_component"),
            AccessibleRole::Unknown => t!("role.unknown"),
            AccessibleRole::StatusBar => t!("role.status_bar"),
            AccessibleRole::DateEditor => t!("role.date_editor"),
            AccessibleRole::SpinBox => t!("role.spin_box"),
            AccessibleRole::FontChooser => t!("role.font_chooser"),
            AccessibleRole::GroupBox => t!("role.group_box"),
            AccessibleRole::Header => t!("role.header"),
            AccessibleRole::Footer => t!("role.footer"),
            AccessibleRole::Paragraph => t!("role.paragraph"),
            AccessibleRole::Ruler => t!("role.ruler"),
            AccessibleRole::EditBar => t!("role.edit_bar"),
            AccessibleRole::ProgressMonitor => t!("role.progress_monitor")
        }.parse().unwrap()
    }
}
