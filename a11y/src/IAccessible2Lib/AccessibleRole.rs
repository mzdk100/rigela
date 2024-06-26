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

//noinspection SpellCheckingInspection
/**
 * Collection of roles
 * This enumerator defines an extended set of accessible roles of objects implementing the %IAccessible2 interface. These roles are in addition to the MSAA roles obtained through the MSAA get_accRole method.  Examples are 'footnote', 'heading', and 'label'.
 * You obtain an object's %IAccessible2 roles by calling IAccessible2::role.
 * */
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum IA2Role {
    /**
     * Unknown role. The object contains some Accessible information, but its role is not known.
     * */
    IA2_ROLE_UNKNOWN = 0,

    /**
     * An object that can be drawn into and to manage events from the objects drawn into it.  Also refer to IA2_ROLE_FRAME, IA2_ROLE_GLASS_PANE, and IA2_ROLE_LAYERED_PANE.
     * */
    IA2_ROLE_CANVAS = 0x401,

    /// A caption describing another object.
    IA2_ROLE_CAPTION,

    /// Used for check buttons that are menu items.
    IA2_ROLE_CHECK_MENU_ITEM,

    /// A specialized dialog that lets the user choose a color.
    IA2_ROLE_COLOR_CHOOSER,

    /// A date editor.
    IA2_ROLE_DATE_EDITOR,

    /**
     * An iconified internal frame in an IA2_ROLE_DESKTOP_PANE.
     * Also refer to IA2_ROLE_INTERNAL_FRAME.
     * */
    IA2_ROLE_DESKTOP_ICON,

    /**
     * A desktop pane. A pane that supports internal frames and iconified versions of those internal frames.  Also refer to IA2_ROLE_INTERNAL_FRAME.
     * */
    IA2_ROLE_DESKTOP_PANE,

    /**
     * A directory pane. A pane that allows the user to navigate through and select the contents of a directory. May be used by a file chooser.
     * Also refer to IA2_ROLE_FILE_CHOOSER.
     * */
    IA2_ROLE_DIRECTORY_PANE,

    /**
     * An editable text object in a toolbar.
     * <b>Deprecated.</b>
     * The edit bar role was meant for a text area in a toolbar. However, to detect a text area in a toolbar, the AT can query the parent.
     * */
    IA2_ROLE_EDITBAR,

    /// Embedded (OLE) object.
    IA2_ROLE_EMBEDDED_OBJECT,

    /// Text used as an endnote (footnote at the end of a chapter or section).
    IA2_ROLE_ENDNOTE,

    /**
     * A file chooser. A specialized dialog that displays the files in the directory and lets the user select a file, browse a different directory, or specify a filename. May use the directory pane to show the contents of a directory.
     * Also refer to IA2_ROLE_DIRECTORY_PANE.
     * */
    IA2_ROLE_FILE_CHOOSER,

    /**
     * A font chooser. A font chooser is a component that lets the user pick various attributes for fonts.
     * */
    IA2_ROLE_FONT_CHOOSER,

    /**
     * Footer of a document page.
     * Also refer to IA2_ROLE_HEADER.
     * */
    IA2_ROLE_FOOTER,

    /// Text that is used as a footnote.  Also refer to IA2_ROLE_ENDNOTE.
    IA2_ROLE_FOOTNOTE,

    /**
     * A container of form controls.
     * The use's example of this role is to represent an HTML FORM tag.
     * */
    IA2_ROLE_FORM,

    /**
     * Frame role. A top level window with a title bar, border, menu bar, etc.
     * It is often used as the primary window for an application.  Also refer to IA2_ROLE_CANVAS and the MSAA roles of dialog and window.
     * */
    IA2_ROLE_FRAME,

    /**
     * A glass pane. A pane that is guaranteed to be painted on top of all panes beneath it.  Also refer to IA2_ROLE_CANVAS, IA2_ROLE_INTERNAL_FRAME, and IA2_ROLE_ROOT_PANE.
     * */
    IA2_ROLE_GLASS_PANE,

    /**
     * Header of a document page.
     * Also refer to IA2_ROLE_FOOTER.
     * */
    IA2_ROLE_HEADER,

    /// Heading.  Use the IAccessible2::attributes level attribute to determine the heading level.
    IA2_ROLE_HEADING,

    /// A small fixed size picture, typically used to decorate components.
    IA2_ROLE_ICON,

    /**
     * An image map object.  Usually a graphic with multiple hotspots, where each hotspot can be activated resulting in the loading of another document or section of a document.
     * */
    IA2_ROLE_IMAGE_MAP,

    /**
     * An object which is used to allow input of characters not found on a keyboard, such as the input of Chinese characters on a Western keyboard.
     * */
    IA2_ROLE_INPUT_METHOD_WINDOW,

    /**
     * An internal frame. A frame-like object that is clipped by a desktop pane.
     * The desktop pane, internal frame, and desktop icon objects are often used to create multiple document interfaces within an application.
     * Also refer to IA2_ROLE_DESKTOP_ICON, IA2_ROLE_DESKTOP_PANE, and IA2_ROLE_FRAME.
     * */
    IA2_ROLE_INTERNAL_FRAME,

    /// An object used to present an icon or short string in an interface.
    IA2_ROLE_LABEL,

    /**
     * A layered pane. A specialized pane that allows its children to be drawn in layers, providing a form of stacking order. This is usually the pane that holds the menu bar as well as the pane that contains most of the visual components in a window.
     * Also refer to IA2_ROLE_CANVAS, IA2_ROLE_GLASS_PANE, and IA2_ROLE_ROOT_PANE.
     * */
    IA2_ROLE_LAYERED_PANE,

    /**
     * A section whose content is parenthetic or ancillary to the main content of the resource.
     * */
    IA2_ROLE_NOTE,

    /**
     * A specialized pane whose primary use is inside a dialog.
     * Also refer to MSAA's dialog role.
     * */
    IA2_ROLE_OPTION_PANE,

    /**
     * An object representing a page of document content.  It is used in documents which are accessed by the user on a page-by-page basis.
     * */
    IA2_ROLE_PAGE,

    /// A paragraph of text.
    IA2_ROLE_PARAGRAPH,

    /**
     * A radio button that is a menu item.
     * Also refer to MSAA's button and menu item roles.
     * */
    IA2_ROLE_RADIO_MENU_ITEM,

    /**
     * An object which is redundant with another object in the accessible hierarchy.
     * ATs typically ignore objects with this role.
     * */
    IA2_ROLE_REDUNDANT_OBJECT,

    /**
     * A root pane. A specialized pane that has a glass pane and a layered pane as its children.
     * Also refer to IA2_ROLE_GLASS_PANE and IA2_ROLE_LAYERED_PANE
     * */
    IA2_ROLE_ROOT_PANE,

    /**
     * A ruler such as those used in word processors.
     * */
    IA2_ROLE_RULER,

    /**
     * A scroll pane. An object that allows a user to incrementally view a large amount of information.  Its children can include scroll bars and a viewport.
     * Also refer to IA2_ROLE_VIEW_PORT and MSAA's scroll bar role.
     * */
    IA2_ROLE_SCROLL_PANE,

    /**
     * The document content's container.
     * The use's example of this role is to represent an HTML DIV tag.
     * A section may be used as a region.
     * A region is a group of elements that together form a perceivable unit.
     * A region does not necessarily follow the logical structure of the content, but follows the perceivable structure of the page.
     * A region may have an attribute in the set of IAccessible2::attributes which indicates that it is "live".
     * A live region is content that is likely to change in response to a timed change, a user event, or some other programmed logic or event.
     * */
    IA2_ROLE_SECTION,

    /// Object with graphical representation used to represent content on draw pages.
    IA2_ROLE_SHAPE,

    /**
     * A split pane.
     * A specialized panel that presents two other panels at the same time.
     * Between the two panels is a divider the user can manipulate to make one panel larger and the other panel smaller.
     * */
    IA2_ROLE_SPLIT_PANE,

    /**
     * An object that forms part of a menu system but which can be "undocked" from or "torn off" the menu system to exist as a separate window.
     * */
    IA2_ROLE_TEAR_OFF_MENU,

    /// An object used as a terminal emulator.
    IA2_ROLE_TERMINAL,

    /// Collection of objects that constitute a logical text entity.
    IA2_ROLE_TEXT_FRAME,

    /**
     * A toggle button. A specialized push button that can be checked or unchecked, but does not provide a separate indicator for the current state.
     * Also refer to MSAA's roles of push button, check box, and radio button.
     * <BR><B>Note:</B> IA2_ROLE_TOGGLE_BUTTON should not be used.  Instead, use MSAA's
     * ROLE_SYSTEM_PUSHBUTTON and STATE_SYSTEM_PRESSED.
     * */
    IA2_ROLE_TOGGLE_BUTTON,

    /**
     * A viewport. An object usually used in a scroll pane. It represents the portion of the entire data that the user can see. As the user manipulates the scroll bars, the contents of the viewport can change.
     * Also refer to IA2_ROLE_SCROLL_PANE.
     * */
    IA2_ROLE_VIEW_PORT,

    /**
     * An object containing content which is complementary to the main content of a document, but remains meaningful when separated from the main content.
     * There are various types of content that would appropriately have this role.
     * For example, in the case where content is delivered via a web portal to a web browser, this may include but not be limited to show times, current weather, related articles, or stocks to watch.
     * The complementary role indicates that contained content is relevant to the main content.
     * If the complementary content is completely separable main content, it may be appropriate to use a more general role.
     * */
    IA2_ROLE_COMPLEMENTARY_CONTENT,

    /**
     * An object representing a navigational landmark, a region on a page to which the user may want quick access, such as a navigation area, a search facility or the main content of a page.
     * */
    IA2_ROLE_LANDMARK,

    /**
     * A bar that serves as a level indicator to, for instance, shows
     * the strength of a password or the charge of a battery.
     * */
    IA2_ROLE_LEVEL_BAR,

    /**
     * Content previously deleted or proposed for deletion, e.g., in revision history or a content view providing suggestions from reviewers.
     * */
    IA2_ROLE_CONTENT_DELETION,

    /**
     * Content previously inserted or proposed for insertion, e.g., in revision history or a content view providing suggestions from reviewers.
     * */
    IA2_ROLE_CONTENT_INSERTION,

    /// A section of content that is quoted from another source.
    IA2_ROLE_BLOCK_QUOTE,

    /**
     * A run of content that is marked or highlighted, such as for reference purposes, or to call it out as having a special purpose that is clear from context.
     * If the mark is used in conjuction with a related content section in the document, then IA2_RELATION_DETAILS should be used to link the related content (and the reverse relation IA2_RELATION_DETAILS_FOR should link back to the IA2_ROLE_MARK object).
     * If the mark has related information in a tooltip, or as hidden text, then accDescription should be used to provide this information.
     * */
    IA2_ROLE_MARK,

    /**
     * A grouping for content that is called out as a proposed change from the current version of the document, such as by a reviewer of the content.
     * Should include as children one or both of: IA2_ROLE_CONTENT_DELETION and IA2_ROLE_CONTENT_INSERTION, in any order, to indicate what the actual change is.
     * If the suggestion is accepted, the implementation should change the role to a generic one such as IA2_ROLE_SECTION or IA2_ROLE_TEXT_FRAME.
     * */
    IA2_ROLE_SUGGESTION,

    /**
     * A single comment, typically user-generated content.
     * Supports reply hierarchies via descendant structure, e.g., a child comment is a reply to the parent comment.
     * Supports groupPosition() method to determine reply level (top comment is 1), as well as set size and position in a set within that level.
     * */
    IA2_ROLE_COMMENT,
}

/*
 * Idl file copyright information:
 *  File Name (AccessibleRole.idl)
 *
 *  IAccessible2 IDL Specification
 *
 *  Copyright (c) 2007-2018 Linux Foundation
 *  Copyright (c) 2006 IBM Corporation
 *  Copyright (c) 2000, 2006 Sun Microsystems, Inc.
 *  All rights reserved.
 *
 *
 *  Redistribution and use in source and binary forms, with or without
 *  modification, are permitted if the following conditions
 *  are met:
 *
 *   1. Redistributions of source code must retain the above copyright
 *      notice, this list of conditions and the following disclaimer.
 *
 *   2. Redistributions in binary form must reproduce the above
 *      copyright notice, this list of conditions and the following
 *      disclaimer in the documentation and/or other materials
 *      provided with the distribution.
 *
 *   3. Neither the name of the Linux Foundation nor the names of its
 *      contributors may be used to endorse or promote products
 *      derived from this software without specific prior written
 *      permission.
 *
 *  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND
 *  CONTRIBUTORS "AS ARE" AND ANY EXPRESS OR IMPLIED WARRANTIES,
 *  INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 *  MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 *  DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR
 *  CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 *  SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 *  NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 *  LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 *  HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 *  CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
 *  OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
 *  EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 *  This BSD License conforms to the Open Source Initiative "Simplified
 *  BSD License" as published at:
 *  <http://www.opensource.org/licenses/bsd-license.php>
 *
 *  IAccessible2 is a trademark of the Linux Foundation. The IAccessible2
 *  mark may be used in accordance with the Linux Foundation Trademark
 *  Policy to indicate compliance with the IAccessible2 specification.
 */
