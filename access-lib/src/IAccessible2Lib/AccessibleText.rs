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

use super::IA2CommonTypes::{IA2CoordinateType, IA2ScrollType};
use windows::core::BSTR;
use windows::core::HRESULT;
use windows::core::{IUnknown, IUnknown_Vtbl};
use windows_interface::interface;

/**
 * A structure containing a substring and the start and end offsets in the enclosing string.
 * IAccessibleText::newText and IAccessibleText::oldText return this struct.
 * */
#[allow(dead_code)]
struct IA2TextSegment {
    text: BSTR, // A segment's copy of a text taken from an enclosing paragraph.
    start: i32, // The first character's index of the segment in the enclosing text.
    end: i32, // Index of the character following the last character of the segment in the enclosing text.
}

/**
 * This enum defines values which specify a text boundary type.
 * IA2_TEXT_BOUNDARY_SENTENCE is optional.  When a method doesn't implement this method, it must return S_FALSE.
 * Typically, this feature would not be implemented by an application.
 * If the application developer is not satisfied with how screen readers handle sentence reading, they can implement this boundary type, allowing screen readers to use the application's version of a sentence instead of their own.
 * The rest of the boundary types must be supported.
 * This enum is used in IAccessibleText::textBeforeOffset, IAccessibleText::textAtOffset, and IAccessibleText::textAfterOffset.
 * */
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum IA2TextBoundaryType {
    /**
     * Typically, a single character is returned. In some cases, more than one character is returned, for example, when a document contains field data such as a field containing a date, time, or footnote reference. In this case, the caret can move over several characters in one movement of the caret.
     * Note that after the caret moves, the caret offset changes by the number of characters in the field, e.g., by eight characters in the following date: 03/26/07.
     * */
    IA2_TEXT_BOUNDARY_CHAR,

    /**
     * The range provided matches the range observed when the application processes the Ctrl + left arrow and Ctrl + right arrow key sequences.
     * Typically, this is from the start of one word to the start of the next, but various applications are inconsistent in the handling of a line's end.
     * */
    IA2_TEXT_BOUNDARY_WORD,

    /**
     * Range is from the start of one sentence to the start of another sentence.
     * */
    IA2_TEXT_BOUNDARY_SENTENCE,

    /**
     * Range is from the start of one paragraph to the start of another paragraph.
     * */
    IA2_TEXT_BOUNDARY_PARAGRAPH,

    /**
     * Range is from the start of one line to the start of another line. This often means that an end-of-line character will appear at the end of the range.
     * However, in the case of some applications, an end-of-line character indicates the end of a paragraph and the lines composing the paragraph, other than the last line, do not contain an end of line character.
     * */
    IA2_TEXT_BOUNDARY_LINE,

    /**
     * <b>Deprecated.</b> Using this value will cause all text to be returned.
     * Note: IAccessibleText::text should be used instead.
     * */
    IA2_TEXT_BOUNDARY_ALL,
}

/**
 * This interface gives read-only access to text.
 * The %IAccessibleText interface should be implemented by all components that present textual information on the display like buttons, text entry fields, or text portions of the document window.  The interface provides access to the text's content, attributes, and spatial location.
 * However, a text cannot be modified with this interface.  That is the task of the IAccessibleEditableText interface.
 * The text length, i.e., the number of characters in the text, is returned by IAccessibleText::nCharacters. All methods that operate on particular characters (e.g., IAccessibleText::textAtOffset) use character indices from 0 to length-1. All methods that operate on character positions (e.g., IAccessibleText::text) use indices from 0 to length.
 * Please note that accessible text does not necessarily support selection.
 * In this case, it should behave as if there were no selection.  An empty selection is used, for example, to express the current cursor position.
 * Refer to @ref _specialOffsets "Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods" for information about special offsets that can be used in %IAccessibleText methods.
 * E_FAIL is returned in the following cases
 * @li endOffset < startOffset
 * @li end_offset > length
 * */
#[interface("24FD2FFB-3AAD-4a08-8335-A3AD89C0FB4B")]
pub(crate) unsafe trait IAccessibleText: IUnknown {
    //noinspection SpellCheckingInspection
    /**
     * Adds a text selection
     * `startOffset` Starting offset (zero-based).
     * `endOffset` Offset of first character after new selection (zero based).
     * retrieval E_INVALIDARG if bad [in] passed
     * @note Refer to @ref _specialOffsets "Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods" for information about special offsets that can be used in %IAccessibleText methods.
     * */
    fn addSelection(&self, startOffset: i32, endOffset: i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns text attributes.
     * `offset` Text offset (zero-based).  Refer to @ref _specialOffsets "Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods" for information about special offsets that can be used in %IAccessibleText methods.
     * `startOffset` The starting offset of the character range over which all text attributes match those of offset. (zero-based)
     * `endOffset` The offset of the first character past the character range over which all text attributes match those of offset. (zero-based)
     * `textAttributes` A string of attributes describing the text.  The attributes are described in the
     * <a href="http://www.linuxfoundation.org/en/Accessibility/IAccessible2/TextAttributes"> text attributes specification</a> on the %IAccessible2 website.
     * retrieval S_FALSE if there is nothing to return, [out] values are 0s and NULL respectively
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn attributes(
        &self,
        offset: i32,
        startOffset: *mut i32,
        endOffset: *mut i32,
        textAttributes: *mut BSTR,
    ) -> HRESULT;

    /**
     * Returns the position of the caret.
     * Returns the 0-based offset of the caret within the text.  If the text is implemented as a tree of text objects with embed characters in higher levels representing substrings of child text objects and the caret is in one of the child text objects, then the offset in the higher level text object would be at the embed character representing child text object that contains the caret.
     * For example, if the string "one two three" is implemented as a two text objects, with a top level text object containing an embed character "one ? three" and a child text object containing "two" and if the caret is in the descendant object just before the 'o' in "two", then:
     * <ul>
     * <li>the caretOffset for the "one ? three" object would be 4, matching the embed character</li>
     * <li>the caretOffset for "two" would be 2, matching the "o"</li>
     * </ul>
     * The caret position/offset is that of the character logically following it, e.g., to the right of it in a left to right language, or to the left of it in a right to left language.
     * `offset` The returned offset is relative to the text represented by this object.
     * retrieval S_FALSE if the caret is not currently active on this object, i.e., the caret is located on some other object.  The returned offset value will be -1.
     * @note S_FALSE (and an offset of -1) will not be returned if the caret is somewhere in the text object or one of its descendants.
     * */
    fn caretOffset(&self, offset: *mut i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns the bounding box of the specified position.
     * The virtual character after the last character of the represented text, i.e., the one at position length is a special case. It represents the current input position and will therefore typically be queried by AT more often than other positions.  Because it does not represent an existing character, its bounding box is defined in relation to preceding characters.  It should be roughly equivalent to the bounding box of some character when inserted at the end of the text.  Its height typically being the maximal height of all the characters in the text or the height of the preceding character, its width being at least one pixel so that the bounding box is not degenerate.
     * Note that the index 'length' is not always valid.  Whether it is or not is implementation dependent.  It typically is when a text is editable or otherwise when on the screen, the caret can be placed behind the text.  You can be sure that the index is valid after you have received a: IA2_EVENT_TEXT_CARET_MOVED event for this index.
     * `offset` Index of the character for which to return its bounding box. The valid range is from zero to length.
     * Refer to @ref _specialOffsets "Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods" for information about special offsets that can be used in %IAccessibleText methods.
     * `coordType` Specifies if the coordinates are relative to the screen or to the parent window.
     * `x` The top left corner's X coordinate for the bounding box of the referenced character.
     * `y` The top left corner's Y coordinate for the bounding box of the referenced character.
     * `width` The bounding box's width of the referenced character.
     * `height` The bounding box's height of the referenced character.
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn characterExtents(
        &self,
        offset: i32,
        coordType: IA2CoordinateType,
        x: *mut i32,
        y: *mut i32,
        width: *mut i32,
        height: *mut i32,
    ) -> HRESULT;

    /**
     * Returns the number of active non-contiguous selections
     * `nSelections` */
    fn nSelections(&self, nSelections: *mut i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns the text position for the specified screen position.
     * Given a point return the zero-based index of the character under that point.  The same functionality could be achieved by using the bounding boxes for each character as returned by IAccessibleText::characterExtents.
     * The method IAccessibleText::offsetAtPoint, however, can be implemented more efficiently.
     * `x` The position's x value for which to look up the index of the character that is rendered on to the display at that point.
     * `y` The position's y value for which to look up the index of the character that is rendered on to the display at that point.
     * `coordType` Screen coordinates or window coordinates.
     * `offset` Index of the character under the given point or -1 if the point is invalid or there is no character under the point.
     * retrieval S_FALSE if nothing to return, [out] value is -1
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn offsetAtPoint(
        &self,
        x: i32,
        y: i32,
        coordType: IA2CoordinateType,
        offset: *mut i32,
    ) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns the character offsets of Nth active text selection
     * Returns the 0-based starting and ending offsets of the Nth selection.  If the text is implemented as a tree of text objects with embed characters in higher levels representing substrings of child text objects, consider the following.
     * If the starting selection offset is in one of the child text objects, the starting offset in the higher-level text object would be at the embed character representing the child text object that contains the starting selection offset.
     * If the ending selection offset is in one of the child text objects, the ending offset in the higher-level text object would be just after the embed character representing the child text object that contains the ending selection offset.
     * For example, if the string "one two three" is implemented as a two text objects, with a top level text object containing an embed character "one ? three" and a child text object containing "two" and if the selection is the string "two" then:
     * <ul>
     * <li>the startOffset for the "one ? three" object would be 4, matching the embed character and the endOffset would be 5.</li>
     * <li>the startOffset for the "two" object would be 0, and the endOffset would be 3</li>
     * </ul>
     * Selection offsets are that of the character logically following it, e.g., to the right of it in a left to right language or to the left of it in a right to left language.
     * `selectionIndex` Index of selection (zero based).
     * `startOffset` 0 based offset of first selected character
     * `endOffset` 0 based offset of one past the last selected character.
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn selection(&self, selectionIndex: i32, startOffset: *mut i32, endOffset: *mut i32)
        -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns the substring between the two given indices.
     * The substring starts with the character at startOffset (inclusive) and up to the character at endOffset (exclusive) if startOffset is less or equal endOffset.
     * If endOffset is lower than startOffset, the result is the same as a call with the two arguments being exchanged.
     * The whole text can be requested by passing the indices zero and IAccessibleText::nCharacters. If both indices have the same value, an empty string is returned.
     * `startOffset` Index of the first character to include in the returned string. The valid range is from zero to length.
     * `endOffset` Index of the last character to exclude in the returned string. The valid range is from zero to length.
     * `text` Returns the substring starting with the character at startOffset (inclusive) and up to the character at endOffset (exclusive) if startOffset is less than or equal to endOffset.
     * retrieval E_INVALIDARG if bad [in] passed
     * @note
     * @li The returned string may be longer than endOffset-startOffset bytes if a text contains multibyte characters.
     * @li Refer to @ref _specialOffsets "Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods" for information about special offsets that can be used in %IAccessibleText methods.
     * */
    fn text(&self, startOffset: i32, endOffset: i32, text: *mut BSTR) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns a text portion before the given position.
     * Returns the substring of the specified text type that is located before the given character and does not include it. The result of this method should be the same as a result for IAccessibleText::textAtOffset with a suitably decreased index value.
     * For example, if text type is: IA2_TEXT_BOUNDARY_WORD, then the complete word that is closest to and located before the offset is returned.
     * If the index is valid, but no text is found, S_FALSE is returned along without values of 0, 0, and a NULL pointer.  This would happen for boundary types other than character when the text consists entirely of whitespace.
     * `offset` Index of the character for which to return the text part before it.  The index character will not be part of the returned string. The valid range is from zero to length.
     * Refer to @ref _specialOffsets "Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods" for information about special offsets that can be used in %IAccessibleText methods.
     * `boundaryType` The type of the text portion to return.
     * See: IA2TextBoundaryType for the complete list.
     * `startOffset` 0 based offset of first character.
     * `endOffset` 0 based offset of one past the last character.
     * `text` Returns the requested text portion.  This portion may be empty or invalid when no appropriate text portion is found or a text type is invalid.
     * retrieval S_FALSE if the requested boundary type is not implemented, such as IA2_TEXT_BOUNDARY_SENTENCE, or if there is nothing to return; [out] values are 0s and NULL respectively
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn textBeforeOffset(
        &self,
        offset: i32,
        boundaryType: IA2TextBoundaryType,
        startOffset: *mut i32,
        endOffset: *mut i32,
        text: *mut BSTR,
    ) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns a text portion after the given position.
     * Returns the substring of the specified text type that is located after the given character and does not include it. The result of this method should be the same as a result for IAccessibleText::textAtOffset with a suitably increased index value.
     * For example, if text type is: IA2_TEXT_BOUNDARY_WORD, then the complete word that is closest to and located after the offset is returned.
     * If the index is valid, but no text is found, S_FALSE is returned along without values of 0, 0, and a NULL pointer.  This would happen for boundary types other than character when the text consists entirely of whitespace.
     * `offset` Index of the character for which to return the text part after it.  The index character will not be part of the returned string. The valid range is from zero to length.
     * Refer to @ref _specialOffsets "Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods" for information about special offsets that can be used in %IAccessibleText methods.
     * `boundaryType` The type of the text portion to return.
     * See: IA2TextBoundaryType for the complete list.
     * `startOffset` 0 based offset of first character.
     * `endOffset` 0 based offset of one past the last character.
     * `text` Returns the requested text portion.  This portion may be empty or invalid when no appropriate text portion is found or a text type is invalid.
     * retrieval S_FALSE if the requested boundary type is not implemented, such as IA2_TEXT_BOUNDARY_SENTENCE, or if there is nothing to return; [out] values are 0s and NULL respectively
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn textAfterOffset(
        &self,
        offset: i32,
        boundaryType: IA2TextBoundaryType,
        startOffset: *mut i32,
        endOffset: *mut i32,
        text: *mut BSTR,
    ) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns a text portion that spans the given position.
     * Returns the substring defined by the specified boundary type at the specified offset.  Refer to IA2TextBoundaryType for more details.
     * For the word boundary type, the returned string will contain the word at the offset if the offset is inside a word and will contain the word before the offset if the offset is not inside a word.  All offsets from the first to the last characters of a word are considered inside the word.  Boundary types of sentence and paragraph should exhibit similar behavior.
     * If the index is valid, but no text is found, S_FALSE is returned along without values of 0, 0, and a NULL pointer.  This would happen for boundary types other than character when the text consists entirely of whitespace.
     * `offset` Index of the character for which to return the text part it belongs to.  The valid range is from zero to length.
     * Refer to @ref _specialOffsets "Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods" for information about special offsets that can be used in %IAccessibleText methods.
     * `boundaryType` The type of the text portion to return.
     * See: IA2TextBoundaryType for the complete list.
     * `startOffset` 0 based offset of first character.
     * `endOffset` 0 based offset of one past the last character.
     * `text` Returns the requested text portion.  This portion may be empty or invalid when no appropriate text portion is found or a text type is invalid.
     * retrieval S_FALSE if the requested boundary type is not implemented, such as IA2_TEXT_BOUNDARY_SENTENCE, or if there is nothing to return; [out] values are 0s and NULL respectively
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn textAtOffset(
        &self,
        offset: i32,
        boundaryType: IA2TextBoundaryType,
        startOffset: *mut i32,
        endOffset: *mut i32,
        text: *mut BSTR,
    ) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Unselects a range of text.
     * `selectionIndex` Index of selection to remove (zero based).
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn removeSelection(&self, selectionIndex: i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Sets the position of the caret.
     * The caret position/offset is that of the character logically following it, e.g., to the right of it in a left to right language.
     * Setting the caret position may or may not alter the current selection.  A change of the selection is notified to the accessibility event listeners with an IA2_EVENT_TEXT_SELECTION_CHANGED event.
     * When the new caret position differs from the old one (which is the standard case), this is notified to the accessibility event listeners with an IA2_EVENT_TEXT_CARET_MOVED event.
     * `offset` The new index of the caret.  This caret is actually placed to the left side of the character with that index.  An index of 0 places the caret so that the next insertion goes before the first character.  An index of IAccessibleText::nCharacters leads to insertion after the last character.  Refer to @ref _specialOffsets "Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods" for information about special offsets that can be used in %IAccessibleText methods.
     * retrieval E_FAIL if the caret cannot be set
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn setCaretOffset(&self, offset: i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Changes the bounds of an existing selection.
     * `selectionIndex` Index of selection to change (zero based)
     * `startOffset` New starting offset (zero based)
     * `endOffset` New ending offset (zero-based) - the offset of the character just past the last character of the selection.
     * retrieval E_INVALIDARG if bad [in] passed
     * @note Refer to @ref _specialOffsets "Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods" for information about special offsets that can be used in %IAccessibleText methods.
     * */
    fn setSelection(&self, selectionIndex: i32, startOffset: i32, endOffset: i32) -> HRESULT;

    /**
     * Returns total number of characters.
     * Note that this may be different from the total number of bytes required to store the text, if the text contains multibyte characters.
     * `nCharacters` */
    fn nCharacters(&self, nCharacters: *mut i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Makes a specific part of string visible on screen.
     * `startIndex` zero-based character offset.
     * `endIndex` zero-based character offset - the offset of the character just past the last character of the string.
     * `scrollType` Defines where the object should be placed on the screen.
     * retrieval E_INVALIDARG if bad [in] passed
     * @note Refer to @ref _specialOffsets "Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods" for information about special offsets that can be used in %IAccessibleText methods.
     * */
    fn scrollSubstringTo(
        &self,
        startIndex: i32,
        endIndex: i32,
        scrollType: IA2ScrollType,
    ) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Moves the top left of a substring to a specified location.
     * `startIndex` zero-based character offset.
     * `endIndex` zero-based character offset - the offset of the character just past the last character of the string.
     * `coordinateType` Specifies whether the coordinates are relative to the screen or the parent object.
     * `x` Defines the x coordinate.
     * `y` Defines the y coordinate.
     * retrieval S_FALSE if the object is already at the specified location.
     * retrieval E_INVALIDARG if bad [in] passed
     * @note Refer to @ref _specialOffsets "Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods" for information about special offsets that can be used in %IAccessibleText methods.
     * */
    fn scrollSubstringToPoint(
        &self,
        startIndex: i32,
        endIndex: i32,
        coordinateType: IA2CoordinateType,
        x: i32,
        y: i32,
    ) -> HRESULT;

    /**
     * Returns any inserted text.
     * Provided for use by the IA2_EVENT_TEXT_INSERTED and IA2_EVENT_TEXT_UPDATED event handlers.
     * This data is only guaranteed to be valid while the thread notifying the event continues. Once the handler has returned, the validity of the data depends on
     * how the server manages the life cycle of its objects. Also, note that the server may have different life cycle management strategies for controls depending on whether a control manages its children. Lists, trees, and tables can have a large number of children, and thus it's possible that the child objects for those controls would only be created as needed. Servers should document their life cycle strategy as this will be of interest to assistive technology or script engines accessing data out of process or from other threads. Servers only need to save the last inserted block of text, and the scope of the entire application is adequate.
     * `newText` The text that was just inserted.
     * retrieval S_FALSE If there is nothing to return, the values of IA2TextSegment struct are set as follows: text = NULL, start = 0, end = 0.
     * */
    fn newText(&self, newText: *mut IA2TextSegment) -> HRESULT;

    /**
     * Returns any removed text.
     * Provided for use by the IA2_EVENT_TEXT_REMOVED/UPDATED event handlers.
     * This data is only guaranteed to be valid while the thread notifying the event continues. Once the handler has returned, the validity of the data depends on how the server manages the life cycle of its objects. Also, note that the server may have different life cycle management strategies for controls depending on whether a control manages its children. Lists, trees, and tables can have a large number of children, and thus it's possible that the child objects for those controls would only be created as needed. Servers should document their life cycle strategy as this will be of interest to assistive technology or script engines accessing data out of process or from other threads. Servers only need to save the last removed block of text, and the scope of the entire application is adequate.
     * `oldText` The text that was just removed.
     * retrieval S_FALSE If there is nothing to return, the values of IA2TextSegment struct are set as follows: text = NULL, start = 0, end = 0.
     * */
    fn oldText(&self, oldText: *mut IA2TextSegment) -> HRESULT;
}

/**
 * Idl file copyright information:
 *  File Name (AccessibleText.idl)
 *
 *  IAccessible2 IDL Specification
 *
 *  Copyright (c) 2007, 2013 Linux Foundation
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
