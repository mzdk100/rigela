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

use super::AccessibleText::{IAccessibleText, IAccessibleText_Impl, IAccessibleText_Vtbl};
use windows::core::interface;
use windows::core::BSTR;
use windows::core::HRESULT;

/**
 * This interface gives read-only access to text.
 * The %IAccessibleText2 interface extends the functionality of the
 * %IAccessibleText interface.
 * */
#[interface("9690A9CC-5C80-4DF5-852E-2D5AE4189A54")]
pub(crate) unsafe trait IAccessibleText2: IAccessibleText {
    //noinspection SpellCheckingInspection
    /**
     * Returns the range and of the specified set of attributes.
     * Return the range (start and end offsets) and text attributes that correspond to the given attributes filter at the given offset.
     * `offset` The offset at which to search for the attributes specified in the filter.
     * `filter` The requested attribute names.  The filter format is "attribute1, attribute2".
     * `startOffset` The starting (0-based) offset of the text containing the specified attributes.
     * `endOffset` The (0-based) offset one past the last character of the text containing the specified attributes.
     * `attributeValues` The values of the requested attributes.
     * retrieval S_FALSE if nothing to return, [out] values are -1, NULL respectively.
     * retrieval E_INVALIDARG if bad [in] passed.
     * */
    pub(crate) fn attributeRange(
        &self,
        offset: i32,
        filter: BSTR,
        startOffset: *mut i32,
        endOffset: *mut i32,
        attributeValues: *mut BSTR,
    ) -> HRESULT;
}

/**
 * Idl file copyright information:
 *  File Name (AccessibleText2.idl)
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
 *  <http://www.opensource.org/licenses/bsd-license.php>
 *   
 *  IAccessible2 is a trademark of the Linux Foundation. The IAccessible2
 *  mark may be used in accordance with the Linux Foundation Trademark
 *  Policy to indicate compliance with the IAccessible2 specification.
 * */
trait IdlCopyright {}
