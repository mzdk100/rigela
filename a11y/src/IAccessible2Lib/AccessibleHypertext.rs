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

use super::{
    AccessibleHyperlink::IAccessibleHyperlink,
    AccessibleText::{IAccessibleText, IAccessibleText_Impl, IAccessibleText_Vtbl},
};
use windows::core::HRESULT;
use windows_interface::interface;

/**
 * This interface exposes information about hypertext in a document.
 * The %IAccessibleHypertext interface is the main interface to expose hyperlinks in a document, typically a text document, that are used to reference other documents.  A typical implementation is to implement this interface on the smallest text object such as a paragraph of text.
 * */
#[interface("6B4F8BBF-F1F2-418a-B35E-A195BC4103B9")]
pub(crate) unsafe trait IAccessibleHypertext: IAccessibleText {
    /**
     * Returns the number of links and link groups contained within this hypertext paragraph.
     * `hyperlinkCount` The number of links and link groups within this hypertext paragraph.
     * Returns 0 if there is no link.
     * */
    fn nHyperlinks(&self, hyperlinkCount: *mut i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns the specified link.
     * The returned IAccessibleHyperlink object encapsulates the hyperlink and provides several kinds of information describing it.
     * `index` This 0 based index specifies the hyperlink to return.
     * `hyperlink` If the given index is valid, i.e., lies in the interval from 0 to the number of links minus one; a reference to the specified hyperlink object is returned.
     * If the index is invalid, then a NULL pointer is returned.
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn hyperlink(&self, index: i32, hyperlink: *mut *mut IAccessibleHyperlink) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns the index of the hyperlink that is associated with this character index.
     * This is the case when a link spans the given character index.
     * `charIndex` A 0 based index of the character for which to return the link index.  If
     * IAccessibleText is used to represent the text containing the link, then the character index is only valid if it is greater than or equal to zero and lower than the number of characters in the text.
     * `hyperlinkIndex` Returns the zero-based index of the hyperlink that is associated with this character index, or -1 if charIndex is not on a link.
     * retrieval S_FALSE if there is nothing to return, [out] value is -1
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn hyperlinkIndex(&self, charIndex: i32, hyperlinkIndex: *mut i32) -> HRESULT;
}

/**
 * Idl file copyright information:
 *  File Name (AccessibleHypertext.idl)
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
