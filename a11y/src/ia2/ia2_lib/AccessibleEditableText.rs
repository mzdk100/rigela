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

use windows::core::interface;
use windows::core::BSTR;
use windows::core::HRESULT;
use windows::core::{IUnknown, IUnknown_Vtbl};

/**
 * This interface provides clipboard capability to text objects.
 * This interface is typically used in conjunction with the IAccessibleText interface and complements that interface with the additional capability of clipboard operations.  Note that even a read-only text object can support the copy capability, so this interface is not limited to editable objects.
 * The substrings used with this interface are specified as follows:
 * If startOffset is less than endOffset, the substring starts with the character at startOffset and ends with the character just before endOffset.
 * If endOffset is lower than startOffset, the result is the same as a call with the two arguments exchanged. The whole text can be defined by passing the indices zero and IAccessibleText::nCharacters. If both indices have the same value, an empty string is defined.
 * Refer to the @ref _specialOffsets "Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods" for information about a special offset constant that can be used in %IAccessibleEditableText methods.
 * */
#[interface("A59AA09A-7011-4b65-939D-32B1FB5547E3")]
pub(crate) unsafe trait IAccessibleEditableText: IUnknown {
    //noinspection SpellCheckingInspection
    /**
     * Copies the text range into the clipboard.
     * The selection is set to the specified offsets, and then selection is copied into the system clipboard.
     * `startOffset` Start index of the text to move into the clipboard. The valid range is from zero to length.
     * `endOffset` End index of the text to move into the clipboard. The valid range is from zero to length.
     * retrieval E_INVALIDARG if bad [in] passed
     * @note Refer to @ref _specialOffsets "Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods" for information about special offsets that can be used in %IAccessibleEditableText methods.
     * @deprecated This function is available via the application's GUI.
     * */
    fn copyText(&self, startOffset: i32, endOffset: i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Deletes a range of text.
     * The text between and including the two given indices is deleted from the text represented by this object.
     * `startOffset` Start index of the text to be deleted. The valid range is from zero to length.
     * `endOffset` End index of the text to be deleted. The valid range is from zero to length.
     * retrieval E_INVALIDARG if bad [in] passed
     * @note Refer to @ref _specialOffsets "Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods" for information about special offsets that can be used in %IAccessibleEditableText methods.
     * */
    fn deleteText(&self, startOffset: i32, endOffset: i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Inserts text at the specified position.
     * The specified string is inserted at the given index into the text represented by this object.
     * `offset` Index at which to insert the text.
     * The valid range is from zero to length.
     * Refer to @ref _specialOffsets "Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods" for information about special offsets that can be used in %IAccessibleEditableText methods.
     * `text` Text that is inserted.
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn insertText(&self, offset: i32, text: *mut BSTR) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Deletes a range of text and copies it to the clipboard.
     * The selection is set to the specified offsets, the selection is then copied into the system clipboard, and then the selection is deleted.
     * `startOffset` Start index of the text to be deleted. The valid range is from zero to length.
     * `endOffset` End index of the text to be deleted. The valid range is from zero to length.
     * retrieval E_INVALIDARG if bad [in] passed
     * @note Refer to @ref _specialOffsets "Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods" for information about special offsets that can be used in %IAccessibleEditableText methods.
     * @deprecated This function is available via the application's GUI.
     * */
    fn cutText(&self, startOffset: i32, endOffset: i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Pastes content from the clipboard.
     * Any existing selection is removed, the clipboard content is then pasted into this object's text at the given offset.  This method is similar to the insertText method.  If the index is not valid, the system clipboard content is not inserted. The behavior is the same as when Ctrl+V is used, i.e., the pasted contents are not necessarily plain text.
     * `offset` Index at which to insert the content from the system clipboard into the text represented by this object. The valid range is from zero to length.
     * Refer to @ref _specialOffsets "Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods" for information about special offsets that can be used in %IAccessibleEditableText methods.
     * retrieval E_INVALIDARG if bad [in] passed
     * @deprecated This function is available via the application's GUI.
     * */
    fn pasteText(&self, offset: i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Replaces text.
     * The text between the two given indices is replaced by the specified replacement string. This method is equivalent to calling first
     * IAccessibleEditableText::deleteText with the two indices and then calling IAccessibleEditableText::insertText with the replacement text at the start index.
     * `startOffset` Start index of the text to be replaced. The valid range is from zero to length.
     * `endOffset` End index of the text to be replaced. The valid range is from zero to length.
     * `text` The Text that replaces the text between the given indices.
     * retrieval E_INVALIDARG if bad [in] passed
     * @note Refer to @ref _specialOffsets "Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods" for information about special offsets that can be used in %IAccessibleEditableText methods.
     * */
    fn replaceText(&self, startOffset: i32, endOffset: i32, text: *mut BSTR) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Replaces the attributes of a text range by the given set of attributes.
     * Sets the attributes for the text between the two given indices. The old attributes are replaced by the new list of attributes.
     * `startOffset` Start index of the text whose attributes are modified. The valid range is from zero to length.
     * `endOffset` End index of the text whose attributes are modified. The valid range is from zero to length.
     * `attributes` Set of attributes that completely replaces the previous attributes assigned to the specified text portion.
     * retrieval E_INVALIDARG if bad [in] passed
     * @note Refer to @ref _specialOffsets "Special Offsets for use in the IAccessibleText and IAccessibleEditableText Methods" for information about special offsets that can be used in %IAccessibleEditableText methods.
     * */
    fn setAttributes(&self, startOffset: i32, endOffset: i32, attributes: *mut BSTR) -> HRESULT;
}

/*
 * Idl file copyright information:
 *  File Name (AccessibleEditableText.idl)
 *
 *  IAccessible2 IDL Specification
 *
 *  Copyright (c) 2007, 2012 Linux Foundation
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
