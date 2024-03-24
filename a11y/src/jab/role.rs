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

use crate::JabLib::packages::{
    ACCESSIBLE_ALERT, ACCESSIBLE_AWT_COMPONENT, ACCESSIBLE_CANVAS, ACCESSIBLE_CHECK_BOX,
    ACCESSIBLE_COLOR_CHOOSER, ACCESSIBLE_COLUMN_HEADER, ACCESSIBLE_COMBO_BOX,
    ACCESSIBLE_DATE_EDITOR, ACCESSIBLE_DESKTOP_ICON, ACCESSIBLE_DESKTOP_PANE, ACCESSIBLE_DIALOG,
    ACCESSIBLE_DIRECTORY_PANE, ACCESSIBLE_EDITBAR, ACCESSIBLE_FILE_CHOOSER, ACCESSIBLE_FILLER,
    ACCESSIBLE_FONT_CHOOSER, ACCESSIBLE_FOOTER, ACCESSIBLE_FRAME, ACCESSIBLE_GLASS_PANE,
    ACCESSIBLE_GROUP_BOX, ACCESSIBLE_HEADER, ACCESSIBLE_HYPERLINK, ACCESSIBLE_ICON,
    ACCESSIBLE_INTERNAL_FRAME, ACCESSIBLE_LABEL, ACCESSIBLE_LAYERED_PANE, ACCESSIBLE_LIST,
    ACCESSIBLE_LIST_ITEM, ACCESSIBLE_MENU, ACCESSIBLE_MENU_BAR, ACCESSIBLE_MENU_ITEM,
    ACCESSIBLE_OPTION_PANE, ACCESSIBLE_PAGE_TAB, ACCESSIBLE_PAGE_TAB_LIST, ACCESSIBLE_PANEL,
    ACCESSIBLE_PARAGRAPH, ACCESSIBLE_PASSWORD_TEXT, ACCESSIBLE_POPUP_MENU, ACCESSIBLE_PROGRESS_BAR,
    ACCESSIBLE_PUSH_BUTTON, ACCESSIBLE_RADIO_BUTTON, ACCESSIBLE_ROOT_PANE, ACCESSIBLE_ROW_HEADER,
    ACCESSIBLE_RULER, ACCESSIBLE_SCROLL_BAR, ACCESSIBLE_SCROLL_PANE, ACCESSIBLE_SEPARATOR,
    ACCESSIBLE_SLIDER, ACCESSIBLE_SPIN_BOX, ACCESSIBLE_SPLIT_PANE, ACCESSIBLE_STATUS_BAR,
    ACCESSIBLE_SWING_COMPONENT, ACCESSIBLE_TABLE, ACCESSIBLE_TEXT, ACCESSIBLE_TOGGLE_BUTTON,
    ACCESSIBLE_TOOL_BAR, ACCESSIBLE_TOOL_TIP, ACCESSIBLE_TREE, ACCESSIBLE_UNKNOWN,
    ACCESSIBLE_VIEWPORT, ACCESSIBLE_WINDOW, PROGRESS_MONITOR,
};

/**
 ******************************************************
 *  Accessible Roles
 *      Defines all AccessibleRoles in Local.US
 ******************************************************
 * */
#[derive(Debug)]
#[allow(dead_code)]
pub enum AccessibleRole {
    /**
     * Object is used to alert the user about something.
     * */
    Alert,

    /**
     * The header for a column of data.
     * */
    ColumnHeader,

    /**
     * Object that can be drawn into and is used to trap
     * events.
     * see ACCESSIBLE_FRAME
     * see ACCESSIBLE_GLASS_PANE
     * see ACCESSIBLE_LAYERED_PANE
     * */
    Canvas,

    /**
     * A list of choices the user can select from.
     * Also, optionally allows the user to enter a choice of their own.
     * */
    ComboBox,

    /**
     * An iconified internal frame in a DesktopPane.
     * See ACCESSIBLE_DESKTOP_PANE
     * see ACCESSIBLE_INTERNAL_FRAME
     * */
    DesktopIcon,

    /**
     * A frame-like object that is clipped by a desktop pane.
     * The desktop pane, internal frame, and desktop icon objects are
     * often used to create multiple document interfaces within an
     * application.
     * see ACCESSIBLE_DESKTOP_ICON
     * see ACCESSIBLE_DESKTOP_PANE
     * see ACCESSIBLE_FRAME
     * */
    InternalFrame,

    /**
     * A pane that supports internal frames and
     * iconified versions of those internal frames.
     * see ACCESSIBLE_DESKTOP_ICON
     * see ACCESSIBLE_INTERNAL_FRAME
     * */
    DesktopPane,

    /**
     * A specialized pane whose primary use is inside a DIALOG
     * see ACCESSIBLE_DIALOG
     * */
    OptionPane,

    /**
     * A top level window with no title or border.
     * see ACCESSIBLE_FRAME
     * see ACCESSIBLE_DIALOG
     * */
    Window,

    /**
     * A top level window with a title bar, border, menu bar, etc.
     * It is often used as the primary window for an application.
     * see ACCESSIBLE_DIALOG
     * see ACCESSIBLE_CANVAS
     * see ACCESSIBLE_WINDOW
     * */
    Frame,

    /**
     * A top level window with title bar and a border.
     * A dialog is similar to a frame, but it has fewer properties and is often used as a
     * secondary window for an application.
     * see ACCESSIBLE_FRAME
     * see ACCESSIBLE_WINDOW
     * */
    Dialog,

    /**
     * A specialized dialog that lets the user choose a color.
     * */
    ColorChooser,

    /**
     * A pane that allows the user to navigate through
     * and select the contents of a directory.
     * May be used by a file chooser.
     * see ACCESSIBLE_FILE_CHOOSER
     * */
    DirectoryPane,

    /**
     * A specialized dialog that displays the files in the directory
     * and lets the user select a file, browse a different directory,
     * or specify a filename.
     * May use the directory pane to show the contents of a directory.
     * see ACCESSIBLE_DIRECTORY_PANE
     * */
    FileChooser,

    /**
     * An object that fills up space in a user interface.
     * It is often used in interfaces to tweak the spacing between components,
     * but serves no other purpose.
     * */
    Filler,

    /**
     * A hypertext anchor
     * */
    Hyperlink,

    /**
     * A small fixed size picture, typically used to decorate components.
     * */
    Icon,

    /**
     * An object used to present an icon or short string in an interface.
     * */
    Label,

    /**
     * A specialized pane that has a glass pane and a layered pane as its
     * children.
     * see ACCESSIBLE_GLASS_PANE
     * see ACCESSIBLE_LAYERED_PANE
     * */
    RootPane,

    /**
     * A pane that is guaranteed to be painted on top
     * of all panes beneath it.
     * see ACCESSIBLE_ROOT_PANE
     * see ACCESSIBLE_CANVAS
     * */
    GlassPane,

    /**
     * A specialized pane that allows its children to be drawn in layers,
     * providing a form of stacking order.
     * This is usually the pane that holds the menu bar as well as the pane that contains most of the
     * visual components in a window.
     * see ACCESSIBLE_GLASS_PANE
     * see ACCESSIBLE_ROOT_PANE
     * */
    LayeredPane,

    /**
     * An object that presents a list of objects to the user and allows the
     * user to select one or more of them.
     * A list is usually contained within a scroll pane.
     * see ACCESSIBLE_SCROLL_PANE
     * see ACCESSIBLE_LIST_ITEM
     * */
    List,

    /**
     * An object that presents an element in a list.
     * A list is usually contained within a scroll pane.
     * see ACCESSIBLE_SCROLL_PANE
     * see ACCESSIBLE_LIST
     * */
    ListItem,

    /**
     * An object usually drawn at the primary dialog box's top of
     * an application that contains a list of menus the user can choose from.
     * For example, a menu bar might contain menus for "File," "Edit," and "Help."
     * see ACCESSIBLE_MENU
     * see ACCESSIBLE_POPUP_MENU
     * see ACCESSIBLE_LAYERED_PANE
     * */
    MenuBar,

    /**
     * A temporary window that is usually used to offer the user a
     * list of choices, and then hides when the user selects one of
     * those choices.
     * see ACCESSIBLE_MENU
     * see ACCESSIBLE_MENU_ITEM
     * */
    PopupMenu,

    /**
     * An object usually found inside a menu bar that contains a list
     * of actions the user can choose from.
     * A menu can have any object as its children, but most often they are menu items, other menus,
     * or rudimentary objects such as radio buttons, check boxes, or
     * separators.
     * For example, an application may have an "Edit" menu that contains menu items for "Cut" and "Paste."
     * see ACCESSIBLE_MENU_BAR
     * see ACCESSIBLE_MENU_ITEM
     * see ACCESSIBLE_SEPARATOR
     * see ACCESSIBLE_RADIO_BUTTON
     * see ACCESSIBLE_CHECK_BOX
     * see ACCESSIBLE_POPUP_MENU
     * */
    Menu,

    /**
     * An object usually contained in a menu that presents an action
     * the user can choose.
     * For example, the "Cut" menu item in an "Edit"
     * menu would be an action the user can select to cut the selected area of a text in a document.
     * see ACCESSIBLE_MENU_BAR
     * see ACCESSIBLE_SEPARATOR
     * see ACCESSIBLE_POPUP_MENU
     * */
    MenuItem,

    /**
     * An object usually contained in a menu to provide a visual
     * and logical separation of the contents in a menu.
     * For example, the "File" menu of an application might contain menu items for
     * "Open," "Close," and "Exit," and will place a separator between
     * "Close" and "Exit" menu items.
     * see ACCESSIBLE_MENU
     * see ACCESSIBLE_MENU_ITEM
     * */
    SEPARATOR,

    /**
     * An object that presents a series of panels (or page tabs), one at a
     * time, through some mechanism provided by the object.
     * The most common mechanism is a list of tabs at the top of the panel.
     * The children of a page tab list are all page tabs.
     * see ACCESSIBLE_PAGE_TAB
     * */
    PageTabList,

    /**
     * An object that is a child of a page tab list.
     * Its sole child is the panel that is to be presented to the user when the user
     * selects the page tab from the list of tabs in the page tab list.
     * see ACCESSIBLE_PAGE_TAB_LIST
     * */
    PageTab,

    /**
     * A generic container that is often used to group objects.
     * */
    Panel,

    /**
     * An object used to indicate how much of a task has been completed.
     * */
    ProgressBar,

    /**
     * A text object used for passwords, or other places where the
     * text contents are not shown visibly to the user
     * */
    PasswordText,

    /**
     * An object the user can manipulate to tell the application to do
     * something.
     * see ACCESSIBLE_CHECK_BOX
     * see ACCESSIBLE_TOGGLE_BUTTON
     * see ACCESSIBLE_RADIO_BUTTON
     * */
    PushButton,

    /**
     * A specialized push button that can be checked or unchecked, but
     * does not provide a separate indicator for the current state.
     * see ACCESSIBLE_PUSH_BUTTON
     * see ACCESSIBLE_CHECK_BOX
     * see ACCESSIBLE_RADIO_BUTTON
     * */
    ToggleButton,

    /**
     * A choice that can be checked or unchecked and provides a
     * separate indicator for the current state.
     * see ACCESSIBLE_PUSH_BUTTON
     * see ACCESSIBLE_TOGGLE_BUTTON
     * see ACCESSIBLE_RADIO_BUTTON
     * */
    CheckBox,

    /**
     * A specialized checkbox that will cause other radio buttons in the
     * same group to become unchecked when this one is checked.
     * see ACCESSIBLE_PUSH_BUTTON
     * see ACCESSIBLE_TOGGLE_BUTTON
     * see ACCESSIBLE_CHECK_BOX
     * */
    RadioButton,

    /**
     * The header for a row of data.
     * */
    RowHeader,

    /**
     * An object that allows a user to incrementally view a large amount
     * of information.
     * Its children can include scroll bars and a viewport.
     * see ACCESSIBLE_SCROLL_BAR
     * see ACCESSIBLE_VIEWPORT
     * */
    ScrollPane,

    /**
     * An object usually used to allow a user to incrementally view a
     * large amount of data.
     * Usually used only by a scroll pane.
     * see ACCESSIBLE_SCROLL_PANE
     * */
    ScrollBar,

    /**
     * An object usually used in a scroll pane.
     * It represents the portion of the entire data that the user can see.
     * As the user manipulates the scroll bars, the contents of the viewport can change.
     * see ACCESSIBLE_SCROLL_PANE
     * */
    Viewport,

    /**
     * An object that allows the user to select from a bounded range.
     * For example, a slider might be used to select a number between 0 and 100.
     * */
    Slider,

    /**
     * A specialized panel that presents two other panels at the same time.
     * Between the two panels is a divider the user can manipulate to make
     * one panel larger and the other panel smaller.
     * */
    SplitPane,

    /**
     * An object used to present information in terms of rows and columns.
     * An example might include a spreadsheet application.
     * */
    Table,

    /**
     * An object that presents text to the user.
     * The text is usually editable by the user as opposed to a label.
     * see ACCESSIBLE_LABEL
     * */
    Text,

    /**
     * An object used to present hierarchical information to the user.
     * The individual nodes in the tree can be collapsed and expanded
     * to provide selective disclosure of the tree's contents.
     * */
    Tree,

    /**
     * A bar or palette usually composed of push buttons or toggle buttons.
     * It is often used to provide the most frequently used functions for an
     * application.
     * */
    ToolBar,

    /**
     * An object that provides information about another object.
     * The accessibleDescription property of the tool tip is often displayed
     * to the user in a small, when the user causes the
     * mouse to hover over the object associated with the tool tip.
     * */
    ToolTip,

    /**
     * An AWT component, but nothing else is known about it.
     * see ACCESSIBLE_SWING_COMPONENT
     * see ACCESSIBLE_UNKNOWN
     * */
    AwtComponent,

    /**
     * A Swing component, but nothing else is known about it.
     * see ACCESSIBLE_AWT_COMPONENT
     * see ACCESSIBLE_UNKNOWN
     * */
    SwingComponent,

    /**
     * The object contains some Accessible information, but its role is not known.
     * see ACCESSIBLE_AWT_COMPONENT
     * see ACCESSIBLE_SWING_COMPONENT
     * */
    Unknown,

    /**
     * A StatusBar is a simple component that can contain
     * multiple labels of status information to the user.
     * */
    StatusBar,

    /**
     * A DateEditor is a component that allows users to edit
     * java.util.Date and java.util.Time objects
     * */
    DateEditor,

    /**
     * A SpinBox is a simple spinner component, and its main use
     * is for simple numbers.
     * */
    SpinBox,

    /**
     * A FontChooser is a component that lets the user pick various
     * attributes for fonts.
     * */
    FontChooser,

    /**
     * A GroupBox is a simple container that contains a border
     * around it and contains components inside it.
     * */
    GroupBox,

    /**
     * A text header
     * */
    Header,

    /**
     * A text footer
     * */
    Footer,

    /**
     * A text paragraph
     * */
    Paragraph,

    /**
     * A ruler is an object used to measure distance
     * */
    Ruler,

    /**
     * A role indicating the object acts as a formula for
     * calculating a value.
     * An example is a formula in a spreadsheet cell.
     * */
    EditBar,

    /**
     * A role indicating the object monitors the progress
     * of some operation.
     * */
    ProgressMonitor,
}

impl AccessibleRole {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Alert => ACCESSIBLE_ALERT,
            Self::ColumnHeader => ACCESSIBLE_COLUMN_HEADER,
            Self::Canvas => ACCESSIBLE_CANVAS,
            Self::ComboBox => ACCESSIBLE_COMBO_BOX,
            Self::DesktopIcon => ACCESSIBLE_DESKTOP_ICON,
            Self::InternalFrame => ACCESSIBLE_INTERNAL_FRAME,
            Self::DesktopPane => ACCESSIBLE_DESKTOP_PANE,
            Self::OptionPane => ACCESSIBLE_OPTION_PANE,
            Self::Window => ACCESSIBLE_WINDOW,
            Self::Frame => ACCESSIBLE_FRAME,
            Self::Dialog => ACCESSIBLE_DIALOG,
            Self::ColorChooser => ACCESSIBLE_COLOR_CHOOSER,
            Self::DirectoryPane => ACCESSIBLE_DIRECTORY_PANE,
            Self::FileChooser => ACCESSIBLE_FILE_CHOOSER,
            Self::Filler => ACCESSIBLE_FILLER,
            Self::Hyperlink => ACCESSIBLE_HYPERLINK,
            Self::Icon => ACCESSIBLE_ICON,
            Self::Label => ACCESSIBLE_LABEL,
            Self::RootPane => ACCESSIBLE_ROOT_PANE,
            Self::GlassPane => ACCESSIBLE_GLASS_PANE,
            Self::LayeredPane => ACCESSIBLE_LAYERED_PANE,
            Self::List => ACCESSIBLE_LIST,
            Self::ListItem => ACCESSIBLE_LIST_ITEM,
            Self::MenuBar => ACCESSIBLE_MENU_BAR,
            Self::PopupMenu => ACCESSIBLE_POPUP_MENU,
            Self::Menu => ACCESSIBLE_MENU,
            Self::MenuItem => ACCESSIBLE_MENU_ITEM,
            Self::SEPARATOR => ACCESSIBLE_SEPARATOR,
            Self::PageTabList => ACCESSIBLE_PAGE_TAB_LIST,
            Self::PageTab => ACCESSIBLE_PAGE_TAB,
            Self::Panel => ACCESSIBLE_PANEL,
            Self::ProgressBar => ACCESSIBLE_PROGRESS_BAR,
            Self::PasswordText => ACCESSIBLE_PASSWORD_TEXT,
            Self::PushButton => ACCESSIBLE_PUSH_BUTTON,
            Self::ToggleButton => ACCESSIBLE_TOGGLE_BUTTON,
            Self::CheckBox => ACCESSIBLE_CHECK_BOX,
            Self::RadioButton => ACCESSIBLE_RADIO_BUTTON,
            Self::RowHeader => ACCESSIBLE_ROW_HEADER,
            Self::ScrollPane => ACCESSIBLE_SCROLL_PANE,
            Self::ScrollBar => ACCESSIBLE_SCROLL_BAR,
            Self::Viewport => ACCESSIBLE_VIEWPORT,
            Self::Slider => ACCESSIBLE_SLIDER,
            Self::SplitPane => ACCESSIBLE_SPLIT_PANE,
            Self::Table => ACCESSIBLE_TABLE,
            Self::Text => ACCESSIBLE_TEXT,
            Self::Tree => ACCESSIBLE_TREE,
            Self::ToolBar => ACCESSIBLE_TOOL_BAR,
            Self::ToolTip => ACCESSIBLE_TOOL_TIP,
            Self::AwtComponent => ACCESSIBLE_AWT_COMPONENT,
            Self::SwingComponent => ACCESSIBLE_SWING_COMPONENT,
            Self::Unknown => ACCESSIBLE_UNKNOWN,
            Self::StatusBar => ACCESSIBLE_STATUS_BAR,
            Self::DateEditor => ACCESSIBLE_DATE_EDITOR,
            Self::SpinBox => ACCESSIBLE_SPIN_BOX,
            Self::FontChooser => ACCESSIBLE_FONT_CHOOSER,
            Self::GroupBox => ACCESSIBLE_GROUP_BOX,
            Self::Header => ACCESSIBLE_HEADER,
            Self::Footer => ACCESSIBLE_FOOTER,
            Self::Paragraph => ACCESSIBLE_PARAGRAPH,
            Self::Ruler => ACCESSIBLE_RULER,
            Self::EditBar => ACCESSIBLE_EDITBAR,
            Self::ProgressMonitor => PROGRESS_MONITOR,
        }
    }

    pub fn from_str(role: &str) -> Self {
        match role {
            ACCESSIBLE_ALERT => Self::Alert,
            ACCESSIBLE_COLUMN_HEADER => Self::ColumnHeader,
            ACCESSIBLE_CANVAS => Self::Canvas,
            ACCESSIBLE_COMBO_BOX => Self::ComboBox,
            ACCESSIBLE_DESKTOP_ICON => Self::DesktopIcon,
            ACCESSIBLE_INTERNAL_FRAME => Self::InternalFrame,
            ACCESSIBLE_DESKTOP_PANE => Self::DesktopPane,
            ACCESSIBLE_OPTION_PANE => Self::OptionPane,
            ACCESSIBLE_WINDOW => Self::Window,
            ACCESSIBLE_FRAME => Self::Frame,
            ACCESSIBLE_DIALOG => Self::Dialog,
            ACCESSIBLE_COLOR_CHOOSER => Self::ColorChooser,
            ACCESSIBLE_DIRECTORY_PANE => Self::DirectoryPane,
            ACCESSIBLE_FILE_CHOOSER => Self::FileChooser,
            ACCESSIBLE_FILLER => Self::Filler,
            ACCESSIBLE_HYPERLINK => Self::Hyperlink,
            ACCESSIBLE_ICON => Self::Icon,
            ACCESSIBLE_LABEL => Self::Label,
            ACCESSIBLE_ROOT_PANE => Self::RootPane,
            ACCESSIBLE_GLASS_PANE => Self::GlassPane,
            ACCESSIBLE_LAYERED_PANE => Self::LayeredPane,
            ACCESSIBLE_LIST => Self::List,
            ACCESSIBLE_LIST_ITEM => Self::ListItem,
            ACCESSIBLE_MENU_BAR => Self::MenuBar,
            ACCESSIBLE_POPUP_MENU => Self::PopupMenu,
            ACCESSIBLE_MENU => Self::Menu,
            ACCESSIBLE_MENU_ITEM => Self::MenuItem,
            ACCESSIBLE_SEPARATOR => Self::SEPARATOR,
            ACCESSIBLE_PAGE_TAB_LIST => Self::PageTabList,
            ACCESSIBLE_PAGE_TAB => Self::PageTab,
            ACCESSIBLE_PANEL => Self::Panel,
            ACCESSIBLE_PROGRESS_BAR => Self::ProgressBar,
            ACCESSIBLE_PASSWORD_TEXT => Self::PasswordText,
            ACCESSIBLE_PUSH_BUTTON => Self::PushButton,
            ACCESSIBLE_TOGGLE_BUTTON => Self::ToggleButton,
            ACCESSIBLE_CHECK_BOX => Self::CheckBox,
            ACCESSIBLE_RADIO_BUTTON => Self::RadioButton,
            ACCESSIBLE_ROW_HEADER => Self::RowHeader,
            ACCESSIBLE_SCROLL_PANE => Self::ScrollPane,
            ACCESSIBLE_SCROLL_BAR => Self::ScrollBar,
            ACCESSIBLE_VIEWPORT => Self::Viewport,
            ACCESSIBLE_SLIDER => Self::Slider,
            ACCESSIBLE_SPLIT_PANE => Self::SplitPane,
            ACCESSIBLE_TABLE => Self::Table,
            ACCESSIBLE_TEXT => Self::Text,
            ACCESSIBLE_TREE => Self::Tree,
            ACCESSIBLE_TOOL_BAR => Self::ToolBar,
            ACCESSIBLE_TOOL_TIP => Self::ToolTip,
            ACCESSIBLE_AWT_COMPONENT => Self::AwtComponent,
            ACCESSIBLE_SWING_COMPONENT => Self::SwingComponent,
            ACCESSIBLE_UNKNOWN => Self::Unknown,
            ACCESSIBLE_STATUS_BAR => Self::StatusBar,
            ACCESSIBLE_DATE_EDITOR => Self::DateEditor,
            ACCESSIBLE_SPIN_BOX => Self::SpinBox,
            ACCESSIBLE_FONT_CHOOSER => Self::FontChooser,
            ACCESSIBLE_GROUP_BOX => Self::GroupBox,
            ACCESSIBLE_HEADER => Self::Header,
            ACCESSIBLE_FOOTER => Self::Footer,
            ACCESSIBLE_PARAGRAPH => Self::Paragraph,
            ACCESSIBLE_RULER => Self::Ruler,
            ACCESSIBLE_EDITBAR => Self::EditBar,
            PROGRESS_MONITOR => Self::ProgressMonitor,
            _ => Self::Unknown,
        }
    }
}
