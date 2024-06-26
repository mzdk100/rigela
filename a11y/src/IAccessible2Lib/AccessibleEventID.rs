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
 * %IAccessible2 specific event constants
 * This enum defines the event IDs fired by %IAccessible2 objects.  The event IDs are in addition to those used by MSAA.
 * */
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum IA2EventID {
    /**
     * The change of actions its number or attributes of an accessible object is signaled by events of this type.
     * */
    IA2_EVENT_ACTION_CHANGED = 0x101,

    /**
     * The active descendant of a component has changed.
     * The active descendant is used in objects with transient children.
     * Note: Due to the fact that MSAA's WinEvents don't allow the active child index to be passed on the IA2_EVENT_ACTIVE_DESCENDANT_CHANGED event, the manager descendants scheme can't be used.
     * Instead, the active child object has to fire MSAA's EVENT_OBJECT_FOCUS.  In a future release, a new event mechanism may be added to provide for event-specific data to be passed with the event.
     * At that time, the IA2_EVENT_ACTIVE_DESCENDANT_CHANGED event and IA2_STATE_MANAGES_DESCENDANTS state would be useful.
     * */
    IA2_EVENT_ACTIVE_DESCENDANT_CHANGED,

    /**
     * The document wide attributes of the document object have changed.
     * */
    IA2_EVENT_DOCUMENT_ATTRIBUTE_CHANGED,

    /**
     * The contents of the document have changed.
     * */
    IA2_EVENT_DOCUMENT_CONTENT_CHANGED,

    /**
     * The loading of the document has completed.
     * */
    IA2_EVENT_DOCUMENT_LOAD_COMPLETE,

    /**
     * The loading of the document was interrupted.
     * */
    IA2_EVENT_DOCUMENT_LOAD_STOPPED,

    /**
     * The document contents are being reloaded.
     * */
    IA2_EVENT_DOCUMENT_RELOAD,

    /**
     * The ending index of this link within the containing string has changed.
     * */
    IA2_EVENT_HYPERLINK_END_INDEX_CHANGED,

    /**
     * The number of anchors associated with this hyperlink object has changed.
     * */
    IA2_EVENT_HYPERLINK_NUMBER_OF_ANCHORS_CHANGED,

    /**
     * The hyperlink selected state changed from selected to unselected or from unselected to select.
     * */
    IA2_EVENT_HYPERLINK_SELECTED_LINK_CHANGED,

    /**
     * One of the links associated with the hypertext object has been activated.
     * */
    IA2_EVENT_HYPERTEXT_LINK_ACTIVATED,

    /**
     * One of the links associated with the hypertext object has been selected.
     * */
    IA2_EVENT_HYPERTEXT_LINK_SELECTED,

    /**
     * The starting index of this link within the containing string has changed.
     * */
    IA2_EVENT_HYPERLINK_START_INDEX_CHANGED,

    /**
     * Focus has changed from one hypertext object to another, or focus moved from a non-hypertext object to a hypertext object, or focus moved from a hypertext object to a non-hypertext object.
     * */
    IA2_EVENT_HYPERTEXT_CHANGED,

    /**
     * The number of hyperlinks associated with a hypertext object changed
     * */
    IA2_EVENT_HYPERTEXT_NLINKS_CHANGED,

    /**
     * An object's attributes changed.
     * Also see: IA2_EVENT_TEXT_ATTRIBUTE_CHANGED.
     * */
    IA2_EVENT_OBJECT_ATTRIBUTE_CHANGED,

    /**
     * A slide changed in a presentation document or a page boundary was crossed in a word processing document.
     * */
    IA2_EVENT_PAGE_CHANGED,

    /**
     * The caret moved from one section to the next.
     * */
    IA2_EVENT_SECTION_CHANGED,

    /**
     * A table caption changed.
     * */
    IA2_EVENT_TABLE_CAPTION_CHANGED,

    /**
     * A table's column description changed.
     * */
    IA2_EVENT_TABLE_COLUMN_DESCRIPTION_CHANGED,

    /**
     * A table's column header changed.
     * */
    IA2_EVENT_TABLE_COLUMN_HEADER_CHANGED,

    /**
     * A table's data changed.
     * */
    IA2_EVENT_TABLE_MODEL_CHANGED,

    /**
     * A table's row description changed.
     * */
    IA2_EVENT_TABLE_ROW_DESCRIPTION_CHANGED,

    /**
     * A table's row header changed.
     * */
    IA2_EVENT_TABLE_ROW_HEADER_CHANGED,

    /**
     * A table's summary changed.
     * */
    IA2_EVENT_TABLE_SUMMARY_CHANGED,

    /**
     * A text object's attributes changed.
     * Also see: IA2_EVENT_OBJECT_ATTRIBUTE_CHANGED.
     * */
    IA2_EVENT_TEXT_ATTRIBUTE_CHANGED,

    /**
     * The caret has moved to a new position.
     * */
    IA2_EVENT_TEXT_CARET_MOVED,

    /**
     * <b>Deprecated.</b> This event is equivalent to: IA2_EVENT_TEXT_UPDATED.
     * */
    IA2_EVENT_TEXT_CHANGED,

    /**
     * The caret moved from one column to the next.
     * */
    IA2_EVENT_TEXT_COLUMN_CHANGED,

    /**
     * Text was inserted.
     * */
    IA2_EVENT_TEXT_INSERTED,

    /**
     * Text was removed.
     * */
    IA2_EVENT_TEXT_REMOVED,

    /**
     * This event indicates general text changes, i.e., changes to text that are exposed through the IAccessibleText interface.  For compatibility with ATK/AT-SPI which does not have an equivalent event, servers can alternatively fire: IA2_EVENT_TEXT_REMOVED and: IA2_EVENT_TEXT_INSERTED.
     * */
    IA2_EVENT_TEXT_UPDATED,

    /**
     * The text selection changed.  Later versions of Microsoft development environments have an equivalent event identified, EVENT_OBJECT_TEXTSELECTIONCHANGED.  Servers should use that if it is available and use IA2_EVENT_TEXT_SELECTION_CHANGED otherwise.
     * Clients should be prepared to respond to either event.
     * */
    IA2_EVENT_TEXT_SELECTION_CHANGED,

    /**
     * A visible data event indicates the change of an accessible object its visual appearance.
     * This includes, for example, most of the attributes available via the IAccessibleComponent interface.
     * */
    IA2_EVENT_VISIBLE_DATA_CHANGED,

    /**
     * The role changed. This should only be used if the interfaces supported by the object did not also change. If the interfaces need to change, the object should be destroyed and a new object created.
     * */
    IA2_EVENT_ROLE_CHANGED,
}

/*
 * Idl file copyright information:
 *  File Name (AccessibleEventID.idl)
 *
 *  IAccessible2 IDL Specification
 *
 *  Copyright (c) 2007, 2010 Linux Foundation
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
