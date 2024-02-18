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

/** These constants control the scrolling of an object or substring into a window.

 This enum is used in IAccessible2::scrollTo and IAccessibleText::scrollSubstringTo.
*/
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum IA2ScrollType {
    /** Scroll the top left corner of the object or substring such that the top left
     corner (and as much as possible of the rest of the object or substring) is within
     the top level window.  In cases where the entire object or substring fits within
     the top level window, the placement of the object or substring is dependent on
     the application.  For example, the object or substring may be scrolled to the
     closest edge, the furthest edge, or midway between those two edges.  In cases
     where there is a hierarchy of nested scrollable controls, more than one control
     may have to be scrolled.
    */
    IA2_SCROLL_TYPE_TOP_LEFT,

    /** Scroll the bottom right corner of the object or substring such that the bottom right
     corner (and as much as possible of the rest of the object or substring) is within
     the top level window.  In cases where the entire object or substring fits within
     the top level window, the placement of the object or substring is dependent on
     the application.  For example, the object or substring may be scrolled to the
     closest edge, the furthest edge, or midway between those two edges.  In cases
     where there is a hierarchy of nested scrollable controls, more than one control
     may have to be scrolled.
    */
    IA2_SCROLL_TYPE_BOTTOM_RIGHT,

    /** Scroll the top edge of the object or substring such that the top edge
     (and as much as possible of the rest of the object or substring) is within the
     top level window.  In cases where the entire object or substring fits within
     the top level window, the placement of the object or substring is dependent on
     the application.  For example, the object or substring may be scrolled to the
     closest edge, the furthest edge, or midway between those two edges.  In cases
     where there is a hierarchy of nested scrollable controls, more than one control
     may have to be scrolled.
    */
    IA2_SCROLL_TYPE_TOP_EDGE,

    /** Scroll the bottom edge of the object or substring such that the bottom edge
     (and as much as possible of the rest of the object or substring) is within the
     top level window.  In cases where the entire object or substring fits within
     the top level window, the placement of the object or substring is dependent on
     the application.  For example, the object or substring may be scrolled to the
     closest edge, the furthest edge, or midway between those two edges.  In cases
     where there is a hierarchy of nested scrollable controls, more than one control
     may have to be scrolled.
    */
    IA2_SCROLL_TYPE_BOTTOM_EDGE,

    /** Scroll the left edge of the object or substring such that the left edge
     (and as much as possible of the rest of the object or substring) is within the
     top level window.  In cases where the entire object or substring fits within
     the top level window, the placement of the object or substring is dependent on
     the application.  For example, the object or substring may be scrolled to the
     closest edge, the furthest edge, or midway between those two edges.  In cases
     where there is a hierarchy of nested scrollable controls, more than one control
     may have to be scrolled.
    */
    IA2_SCROLL_TYPE_LEFT_EDGE,

    /** Scroll the right edge of the object or substring such that the right edge
     (and as much as possible of the rest of the object or substring) is within the
     top level window.  In cases where the entire object or substring fits within
     the top level window, the placement of the object or substring is dependent on
     the application.  For example, the object or substring may be scrolled to the
     closest edge, the furthest edge, or midway between those two edges.  In cases
     where there is a hierarchy of nested scrollable controls, more than one control
     may have to be scrolled.
    */
    IA2_SCROLL_TYPE_RIGHT_EDGE,

    /** Scroll the object or substring such that as much as possible of the
     object or substring is within the top level window.  The placement of
     the object is dependent on the application.  For example, the object or
     substring may be scrolled to the closest edge, the furthest edge, or midway
     between those two edges.
    */
    IA2_SCROLL_TYPE_ANYWHERE,
}

//noinspection SpellCheckingInspection
/** These constants define which coordinate system a point is located in.

 This enum is used in IAccessible2::scrollToPoint, IAccessibleImage::imagePosition,
 IAccessibleText::characterExtents, and IAccessibleText::offsetAtPoint, and
 IAccessibleText::scrollSubstringToPoint.
*/
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum IA2CoordinateType {
    /// The coordinates are relative to the screen.
    IA2_COORDTYPE_SCREEN_RELATIVE,

    /**
     * The coordinates are relative to the upper-left corner of the immediate parent's bounding box.
     * */
    IA2_COORDTYPE_PARENT_RELATIVE,
}

/**
 * Special offsets for use in IAccessibleText and IAccessibleEditableText methods
 * Refer to @ref _specialOffsets "Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods"
 * for more information.
 * */
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum IA2TextSpecialOffsets {
    /**< This offset is equivalent to the length of the string.  It eliminates
    the need to call IAccessibleText::nCharacters. */
    IA2_TEXT_OFFSET_LENGTH = -1,
    /**< This offset signifies that the text related to the physical location
    of the caret should be used. */
    IA2_TEXT_OFFSET_CARET = -2,
}

/**
 * These constants specify the kind of change made to a table.
 * This enum is used in the IA2TableModelChange struct, which in turn is used by
 * IAccessibleTable::modelChange and IAccessibleTable2::modelChange.
 * */
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum IA2TableModelChangeType {
    IA2_TABLE_MODEL_CHANGE_INSERT, // = 0;
    IA2_TABLE_MODEL_CHANGE_DELETE,
    IA2_TABLE_MODEL_CHANGE_UPDATE,
}

/**
 * A structure
 * defining the type of and extents of changes
 * made to a table IAccessibleTable::modelChange and IAccessibleTable2::modelChange return this struct.
 * In the case of an insertion or change,
 * the row and column offsets define the boundaries of the inserted or changed sub-table after the operation.
 * In the case of a deletion,
 * the row and column offsets define the boundaries of the sub-table
 * being removed before the removal.
 * */
#[allow(dead_code)]
pub(crate) struct IA2TableModelChange {
    r#type: IA2TableModelChangeType, // insert, delete, update
    firstRow: i32,                   // 0 based, inclusive
    lastRow: i32,                    // 0 based, inclusive
    firstColumn: i32,                // 0 based, inclusive
    lastColumn: i32,                 // 0 based, inclusive
}

/**
 * Idl file copyright information:
 *  File Name (IA2CommonTypes.idl)
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
 *  http://www.opensource.org/licenses/bsd-license.php
 *   
 *  IAccessible2 is a trademark of the Linux Foundation. The IAccessible2
 *  mark may be used in accordance with the Linux Foundation Trademark
 *  Policy to indicate compliance with the IAccessible2 specification.
 * */
trait IdlCopyright {}
