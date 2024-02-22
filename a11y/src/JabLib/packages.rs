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

use win_wrap::common::BOOL;

#[allow(unused)]
const MAX_STRING_SIZE: u32 = 1024;
#[allow(unused)]
const SHORT_STRING_SIZE: u32 = 256;

#[allow(unused)]
pub(crate) type JInt = i32;
#[allow(unused)]
pub(crate) type JLong = i64;
#[allow(unused)]
pub(crate) type JObject = *const ();

#[cfg(target_arch = "x86")]
#[allow(unused)]
pub(crate) type JObject64 = JObject;

#[cfg(target_arch = "x86_64")]
#[allow(unused)]
pub(crate) type JObject64 = JLong;

// object types
pub(crate) type AccessibleContext = JObject64;
#[allow(unused)]
pub(crate) type AccessibleText = JObject64;
#[allow(unused)]
pub(crate) type AccessibleValue = JObject64;
#[allow(unused)]
pub(crate) type AccessibleSelection = JObject64;
#[allow(unused)]
pub(crate) type JavaObject = JObject64;
#[allow(unused)]
pub(crate) type PropertyChangeEvent = JObject64;
#[allow(unused)]
pub(crate) type FocusEvent = JObject64;
#[allow(unused)]
pub(crate) type CaretEvent = JObject64;
#[allow(unused)]
pub(crate) type MouseEvent = JObject64;
#[allow(unused)]
pub(crate) type MenuEvent = JObject64;
#[allow(unused)]
pub(crate) type AccessibleTable = JObject64;
#[allow(unused)]
pub(crate) type AccessibleHyperlink = JObject64;
#[allow(unused)]
pub(crate) type AccessibleHypertext = JObject64;

/**
 ******************************************************
 *  Accessible Roles
 *      Defines all AccessibleRoles in Local.US
 ******************************************************
 * */

/**
 * Object is used to alert the user about something.
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_ALERT: &str = "alert";

/**
 * The header for a column of data.
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_COLUMN_HEADER: &str = "column header";

/**
 * Object that can be drawn into and is used to trap
 * events.
 * see ACCESSIBLE_FRAME
 * see ACCESSIBLE_GLASS_PANE
 * see ACCESSIBLE_LAYERED_PANE
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_CANVAS: &str = "canvas";

/**
 * A list of choices the user can select from.
 * Also, optionally allows the user to enter a choice of their own.
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_COMBO_BOX: &str = "combo box";

/**
 * An iconified internal frame in a DESKTOP_PANE.
 * see ACCESSIBLE_DESKTOP_PANE
 * see ACCESSIBLE_INTERNAL_FRAME
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_DESKTOP_ICON: &str = "desktop icon";

/**
 * A frame-like object that is clipped by a desktop pane.
 * The desktop pane, internal frame, and desktop icon objects are
 * often used to create multiple document interfaces within an
 * application.
 * see ACCESSIBLE_DESKTOP_ICON
 * see ACCESSIBLE_DESKTOP_PANE
 * see ACCESSIBLE_FRAME
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_INTERNAL_FRAME: &str = "internal frame";

/**
 * A pane that supports internal frames and
 * iconified versions of those internal frames.
 * see ACCESSIBLE_DESKTOP_ICON
 * see ACCESSIBLE_INTERNAL_FRAME
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_DESKTOP_PANE: &str = "desktop pane";

/**
 * A specialized pane whose primary use is inside a DIALOG
 * see ACCESSIBLE_DIALOG
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_OPTION_PANE: &str = "option pane";

/**
 * A top level window with no title or border.
 * see ACCESSIBLE_FRAME
 * see ACCESSIBLE_DIALOG
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_WINDOW: &str = "window";

/**
 * A top level window with a title bar, border, menu bar, etc.
 * It is often used as the primary window for an application.
 * see ACCESSIBLE_DIALOG
 * see ACCESSIBLE_CANVAS
 * see ACCESSIBLE_WINDOW
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_FRAME: &str = "frame";

/**
 * A top level window with title bar and a border.
 * A dialog is similar to a frame, but it has fewer properties and is often used as a
 * secondary window for an application.
 * see ACCESSIBLE_FRAME
 * see ACCESSIBLE_WINDOW
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_DIALOG: &str = "dialog";

/**
 * A specialized dialog that lets the user choose a color.
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_COLOR_CHOOSER: &str = "color chooser";

/**
 * A pane that allows the user to navigate through
 * and select the contents of a directory.
 * May be used by a file chooser.
 * see ACCESSIBLE_FILE_CHOOSER
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_DIRECTORY_PANE: &str = "directory pane";

/**
 * A specialized dialog that displays the files in the directory
 * and lets the user select a file, browse a different directory,
 * or specify a filename.
 * May use the directory pane to show the contents of a directory.
 * see ACCESSIBLE_DIRECTORY_PANE
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_FILE_CHOOSER: &str = "file chooser";

/**
 * An object that fills up space in a user interface.
 * It is often used in interfaces to tweak the spacing between components,
 * but serves no other purpose.
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_FILLER: &str = "filler";

/**
 * A hypertext anchor
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_HYPERLINK: &str = "hyperlink";

/**
 * A small fixed size picture, typically used to decorate components.
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_ICON: &str = "icon";

/**
 * An object used to present an icon or short string in an interface.
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_LABEL: &str = "label";

/**
 * A specialized pane that has a glass pane and a layered pane as its
 * children.
 * see ACCESSIBLE_GLASS_PANE
 * see ACCESSIBLE_LAYERED_PANE
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_ROOT_PANE: &str = "root pane";

/**
 * A pane that is guaranteed to be painted on top
 * of all panes beneath it.
 * see ACCESSIBLE_ROOT_PANE
 * see ACCESSIBLE_CANVAS
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_GLASS_PANE: &str = "glass pane";

/**
 * A specialized pane that allows its children to be drawn in layers,
 * providing a form of stacking order.
 * This is usually the pane that holds the menu bar as well as the pane that contains most of the
 * visual components in a window.
 * see ACCESSIBLE_GLASS_PANE
 * see ACCESSIBLE_ROOT_PANE
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_LAYERED_PANE: &str = "layered pane";

/**
 * An object that presents a list of objects to the user and allows the
 * user to select one or more of them.
 * A list is usually contained within a scroll pane.
 * see ACCESSIBLE_SCROLL_PANE
 * see ACCESSIBLE_LIST_ITEM
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_LIST: &str = "list";

/**
 * An object that presents an element in a list.
 * A list is usually contained within a scroll pane.
 * see ACCESSIBLE_SCROLL_PANE
 * see ACCESSIBLE_LIST
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_LIST_ITEM: &str = "list item";

/**
 * An object usually drawn at the primary dialog box's top of
 * an application that contains a list of menus the user can choose from.
 * For example, a menu bar might contain menus for "File," "Edit," and "Help."
 * see ACCESSIBLE_MENU
 * see ACCESSIBLE_POPUP_MENU
 * see ACCESSIBLE_LAYERED_PANE
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_MENU_BAR: &str = "menu bar";

/**
 * A temporary window that is usually used to offer the user a
 * list of choices, and then hides when the user selects one of
 * those choices.
 * see ACCESSIBLE_MENU
 * see ACCESSIBLE_MENU_ITEM
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_POPUP_MENU: &str = "popup menu";

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
#[allow(unused)]
pub(crate) const ACCESSIBLE_MENU: &str = "menu";

/**
 * An object usually contained in a menu that presents an action
 * the user can choose.
 * For example, the "Cut" menu item in an "Edit"
 * menu would be an action the user can select to cut the selected area of a text in a document.
 * see ACCESSIBLE_MENU_BAR
 * see ACCESSIBLE_SEPARATOR
 * see ACCESSIBLE_POPUP_MENU
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_MENU_ITEM: &str = "menu item";

/**
 * An object usually contained in a menu to provide a visual
 * and logical separation of the contents in a menu.
 * For example, the "File" menu of an application might contain menu items for
 * "Open," "Close," and "Exit," and will place a separator between
 * "Close" and "Exit" menu items.
 * see ACCESSIBLE_MENU
 * see ACCESSIBLE_MENU_ITEM
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_SEPARATOR: &str = "separator";

/**
 * An object that presents a series of panels (or page tabs), one at a
 * time, through some mechanism provided by the object.
 * The most common mechanism is a list of tabs at the top of the panel.
 * The children of a page tab list are all page tabs.
 * see ACCESSIBLE_PAGE_TAB
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_PAGE_TAB_LIST: &str = "page tab list";

/**
 * An object that is a child of a page tab list.
 * Its sole child is the panel that is to be presented to the user when the user
 * selects the page tab from the list of tabs in the page tab list.
 * see ACCESSIBLE_PAGE_TAB_LIST
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_PAGE_TAB: &str = "page tab";

/**
 * A generic container that is often used to group objects.
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_PANEL: &str = "panel";

/**
 * An object used to indicate how much of a task has been completed.
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_PROGRESS_BAR: &str = "progress bar";

/**
 * A text object used for passwords, or other places where the
 * text contents are not shown visibly to the user
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_PASSWORD_TEXT: &str = "password text";

/**
 * An object the user can manipulate to tell the application to do
 * something.
 * see ACCESSIBLE_CHECK_BOX
 * see ACCESSIBLE_TOGGLE_BUTTON
 * see ACCESSIBLE_RADIO_BUTTON
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_PUSH_BUTTON: &str = "push button";

/**
 * A specialized push button that can be checked or unchecked, but
 * does not provide a separate indicator for the current state.
 * see ACCESSIBLE_PUSH_BUTTON
 * see ACCESSIBLE_CHECK_BOX
 * see ACCESSIBLE_RADIO_BUTTON
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_TOGGLE_BUTTON: &str = "toggle button";

/**
 * A choice that can be checked or unchecked and provides a
 * separate indicator for the current state.
 * see ACCESSIBLE_PUSH_BUTTON
 * see ACCESSIBLE_TOGGLE_BUTTON
 * see ACCESSIBLE_RADIO_BUTTON
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_CHECK_BOX: &str = "check box";

/**
 * A specialized checkbox that will cause other radio buttons in the
 * same group to become unchecked when this one is checked.
 * see ACCESSIBLE_PUSH_BUTTON
 * see ACCESSIBLE_TOGGLE_BUTTON
 * see ACCESSIBLE_CHECK_BOX
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_RADIO_BUTTON: &str = "radio button";

/**
 * The header for a row of data.
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_ROW_HEADER: &str = "row header";

/**
 * An object that allows a user to incrementally view a large amount
 * of information.
 * Its children can include scroll bars and a viewport.
 * see ACCESSIBLE_SCROLL_BAR
 * see ACCESSIBLE_VIEWPORT
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_SCROLL_PANE: &str = "scroll pane";

/**
 * An object usually used to allow a user to incrementally view a
 * large amount of data.
 * Usually used only by a scroll pane.
 * see ACCESSIBLE_SCROLL_PANE
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_SCROLL_BAR: &str = "scroll bar";

/**
 * An object usually used in a scroll pane.
 * It represents the portion of the entire data that the user can see.
 * As the user manipulates the scroll bars, the contents of the viewport can change.
 * see ACCESSIBLE_SCROLL_PANE
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_VIEWPORT: &str = "viewport";

/**
 * An object that allows the user to select from a bounded range.
 * For example, a slider might be used to select a number between 0 and 100.
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_SLIDER: &str = "slider";

/**
 * A specialized panel that presents two other panels at the same time.
 * Between the two panels is a divider the user can manipulate to make
 * one panel larger and the other panel smaller.
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_SPLIT_PANE: &str = "split pane";

/**
 * An object used to present information in terms of rows and columns.
 * An example might include a spreadsheet application.
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_TABLE: &str = "table";

/**
 * An object that presents text to the user.
 * The text is usually editable by the user as opposed to a label.
 * see ACCESSIBLE_LABEL
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_TEXT: &str = "text";

/**
 * An object used to present hierarchical information to the user.
 * The individual nodes in the tree can be collapsed and expanded
 * to provide selective disclosure of the tree's contents.
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_TREE: &str = "tree";

/**
 * A bar or palette usually composed of push buttons or toggle buttons.
 * It is often used to provide the most frequently used functions for an
 * application.
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_TOOL_BAR: &str = "tool bar";

/**
 * An object that provides information about another object.
 * The accessibleDescription property of the tool tip is often displayed
 * to the user in a small: &str = "help bubble"; when the user causes the
 * mouse to hover over the object associated with the tool tip.
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_TOOL_TIP: &str = "tool tip";

/**
 * An AWT component, but nothing else is known about it.
 * see ACCESSIBLE_SWING_COMPONENT
 * see ACCESSIBLE_UNKNOWN
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_AWT_COMPONENT: &str = "awt component";

/**
 * A Swing component, but nothing else is known about it.
 * see ACCESSIBLE_AWT_COMPONENT
 * see ACCESSIBLE_UNKNOWN
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_SWING_COMPONENT: &str = "swing component";

/**
 * The object contains some Accessible information, but its role is not known.
 * see ACCESSIBLE_AWT_COMPONENT
 * see ACCESSIBLE_SWING_COMPONENT
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_UNKNOWN: &str = "unknown";

/**
 * A STATUS_BAR is a simple component that can contain
 * multiple labels of status information to the user.
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_STATUS_BAR: &str = "status bar";

/**
 * A DATE_EDITOR is a component that allows users to edit
 * java.util.Date and java.util.Time objects
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_DATE_EDITOR: &str = "date editor";

/**
 * A SPIN_BOX is a simple spinner component, and its main use
 * is for simple numbers.
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_SPIN_BOX: &str = "spin box";

/**
 * A FONT_CHOOSER is a component that lets the user pick various
 * attributes for fonts.
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_FONT_CHOOSER: &str = "font chooser";

/**
 * A GROUP_BOX is a simple container that contains a border
 * around it and contains components inside it.
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_GROUP_BOX: &str = "group box";

/**
 * A text header
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_HEADER: &str = "header";

/**
 * A text footer
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_FOOTER: &str = "footer";

/**
 * A text paragraph
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_PARAGRAPH: &str = "paragraph";

/**
 * A ruler is an object used to measure distance
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_RULER: &str = "ruler";

/**
 * A role indicating the object acts as a formula for
 * calculating a value.
 * An example is a formula in a spreadsheet cell.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   
 * */
#[allow(unused)]
pub(crate) const ACCESSIBLE_EDITBAR: &str = "editbar";

/**
 * A role indicating the object monitors the progress
 * of some operation.
 * */
#[allow(unused)]
pub(crate) const PROGRESS_MONITOR: &str = "progress monitor";

#[derive(Debug)]
#[repr(C)]
pub(crate) struct AccessBridgeVersionInfo {
    /// output of "java -version"
    pub(crate) VMversion: [u16; SHORT_STRING_SIZE as usize],
    /// version of the AccessBridge.class
    pub(crate) bridgeJavaClassVersion: [u16; SHORT_STRING_SIZE as usize],
    /// version of JavaAccessBridge.dll
    pub(crate) bridgeJavaDLLVersion: [u16; SHORT_STRING_SIZE as usize],
    /// version of WindowsAccessBridge.dll
    pub(crate) bridgeWinDLLVersion: [u16; SHORT_STRING_SIZE as usize],
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct AccessibleContextInfo {
    /// the AccessibleName of the object
    pub(crate) name: [u16; MAX_STRING_SIZE as usize],
    /// the AccessibleDescription of the object
    pub(crate) description: [u16; MAX_STRING_SIZE as usize],
    /// localized AccesibleRole string
    pub(crate) role: [u16; SHORT_STRING_SIZE as usize],
    /// AccesibleRole string in the en_US locale
    pub(crate) role_en_US: [u16; SHORT_STRING_SIZE as usize],
    /// localized AccesibleStateSet string (comma separated)
    pub(crate) states: [u16; SHORT_STRING_SIZE as usize],
    /// AccesibleStateSet string in the en_US locale (comma separated)
    pub(crate) states_en_US: [u16; SHORT_STRING_SIZE as usize],
    /// index of an object in parent
    pub(crate) indexInParent: JInt,
    /// # of children, if any
    pub(crate) childrenCount: JInt,
    // screen coords in pixels
    pub(crate) x: JInt,
    pub(crate) y: JInt,
    /// pixel width of an object
    pub(crate) width: JInt,
    /// pixel height of an object
    pub(crate) height: JInt,
    /// flags for various additional
    pub(crate) accessibleComponent: BOOL,
    ///  Java Accessibility interfaces
    pub(crate) accessibleAction: BOOL,
    ///  FALSE if this object doesn't
    pub(crate) accessibleSelection: BOOL,
    ///  implement the additional interface
    pub(crate) accessibleText: BOOL,
    //  in question
    /// new bitfield containing additional interface flags
    pub(crate) accessibleInterfaces: BOOL,
}
