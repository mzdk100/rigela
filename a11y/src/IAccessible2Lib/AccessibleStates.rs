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

pub type AccessibleStates = i32;

//noinspection SpellCheckingInspection
/**
 * %IAccessible2 specific state bit constants
 * This enum defines the state bits returned by IAccessible2::states.
 * The %IAccessible2 state bits are in addition to those returned by MSAA.
 * */
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub(crate) enum IA2States {
    /**
     * Indicates a window is currently the active window, or is an active supplement
     * within a container or table.
     * This state can be used to indicate the current active item in a container, even
     * if the container itself is not currently active.
     * In other words, this would indicate
     * the item that will get focus if you tab to the container.
     * This information is important for knowing what to report for trees and potentially
     * other containers in a virtual buffer.
     * Also, see: IA2_STATE_MANAGES_DESCENDANTS for more information.
     * */
    IA2_STATE_ACTIVE = 0x1,

    /**
     * Indicates that the object is armed.
     * Used to indicate that the control is "pressed"
     * and will be invoked when the actuator, e.g., a mouse button, is "released".
     * An AT which either monitors the mouse or synthesizes mouse events might need to know that,
     * and possibly a talking interface would even let the user know about it.
     * It could also potentially be useful to on-screen keyboards or test tools
     * since the information does indicate something about the state of the interface.
     * For example,
     * code operating asynchronously might need
     * to wait for the armed state to change before doing something else.
     * */
    IA2_STATE_ARMED = 0x2,

    /** Indicates the user interface object corresponding to this object no longer exists. */
    IA2_STATE_DEFUNCT = 0x4,

    /**
     * An object with this state has a caret and implements the IAccessibleText interface.
     * Such fields may be read-only,
     * so STATE_SYSTEM_READONLY is valid in combination with IA2_STATE_EDITABLE.
     * */
    IA2_STATE_EDITABLE = 0x8,

    /** Indicates the orientation of this object is horizontal. */
    IA2_STATE_HORIZONTAL = 0x10,

    /**
     * Indicates this object is minimized and is represented only by an icon.
     * */
    IA2_STATE_ICONIFIED = 0x20,

    /** Indicates an input validation failure. */
    IA2_STATE_INVALID_ENTRY = 0x40,

    /**
     * Indicates that this object manages its children.
     * Note: Due to the fact that MSAA's WinEvents don't allow the active child index
     * to be passed on the IA2_EVENT_ACTIVE_DESCENDANT_CHANGED event, the manager
     * descendants scheme can't be used.
     * Instead, the active child object has to fire
     * MSAA's EVENT_OBJECT_FOCUS.
     * In a future release, a new event mechanism may be
     * added to provide for event-specific data to be passed with the event.
     * At that time, the IA2_EVENT_ACTIVE_DECENDENT_CHANGED event and
     * IA2_STATE_MANAGES_DESCENDANTS state would be useful.
     * */
    IA2_STATE_MANAGES_DESCENDANTS = 0x80,

    /**
     * Indicates that an object is modal.
     * Modal objects have the behavior that something must be done with the object
     * before the user can interact with an object in a different window.
     * */
    IA2_STATE_MODAL = 0x100,

    /** Indicates this text object can contain multiple lines of text. */
    IA2_STATE_MULTI_LINE = 0x200,

    /** Indicates this object paints every pixel within its rectangular region. */
    IA2_STATE_OPAQUE = 0x400,

    /**
     * Indicates that user interaction is required.
     * An example of when this state is used is when a field in a form must be filled
     * before a form can be processed.
     * */
    IA2_STATE_REQUIRED = 0x800,

    /**
     * Indicates an object which supports text selection.
     * Note: This is different from MSAA STATE_SYSTEM_SELECTABLE.
     * */
    IA2_STATE_SELECTABLE_TEXT = 0x1000,

    /** Indicates that this text object can contain only a single line of text. */
    IA2_STATE_SINGLE_LINE = 0x2000,

    /**
     * Indicates that the accessible object is stale.
     * This state is used when the accessible object no longer accurately
     * represents the state of the object which it is representing.
     * Such as when an
     * object is transient or when an object has been or is in the process of being
     * destroyed or when the object's index in its parent has changed.
     * */
    IA2_STATE_STALE = 0x4000,

    /**
     * Indicates that the object implements autocompletion.
     * This state indicates that a text control will respond to the input of
     * one or more characters and cause a sub-item to become selected.
     * The selection may also result in events fired on the parent object.
     * */
    IA2_STATE_SUPPORTS_AUTOCOMPLETION = 0x8000,

    /**
     * Indicates this object is transient.
     * An object has this state when its parent object has the state: IA2_STATE_MANAGES_DESCENDANTS.
     * For example, a list item object may be managed by its parent list object and may only
     * exist as long as the object is actually rendered.
     * Similarly, a table cell's accessible
     * object may exist only while the cell has focus.
     * However, from the perspective of an
     * assistive technology, a transient object behaves like a non-transient object.
     * As a result, it is likely that this state is not of use to an assistive technology.
     * However,
     * it is provided in case an assistive technology determines
     * that knowledge of its transient nature is useful and also for harmony with the Linux accessibility API.
     * Also, see: IA2_STATE_MANAGES_DESCENDANTS for more information.
     * */
    IA2_STATE_TRANSIENT = 0x10000,

    /** Indicates the orientation of this object is vertical. */
    IA2_STATE_VERTICAL = 0x20000,

    /**
     * Indicates this object is checkable.
     * The standard checkable objects are check boxes, radio buttons, check box menu
     * items, radio menu items, and toggle buttons.
     * Since assistive technology will
     * determine that these objects are checkable via the object's role, the checkable
     * state is not required.
     * However, this state is necessary in those cases where
     * an object has a role which is not one of the previously mentioned roles.
     * An example is a table cell that indicates whether an email has an attachment,
     * whether mail is considered spam, and whether an email has been read.
     * */
    IA2_STATE_CHECKABLE = 0x40000,

    /**
     * Indicates this object is pinned.
     * This state indicates that an object is fixed at a certain location.
     * One example is a browser tab that when pinned cannot be moved until unpinned.
     * Another example is a movable or floating object that when pinned remains in its pinned location
     * until being unpinned.
     * */
    IA2_STATE_PINNED = 0x80000,
}

/*
 * Idl file copyright information:
 *  File Name (AccessibleStates.idl)
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
