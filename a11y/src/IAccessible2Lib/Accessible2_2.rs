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

use super::Accessible2::{IAccessible2, IAccessible2_Impl, IAccessible2_Vtbl};
use windows::core::interface;
use windows::core::IUnknown;
use windows::core::BSTR;
use windows::core::{HRESULT, VARIANT};

/**
 * This interface exposes the primary set of information about an IAccessible2 enabled accessible object.
 * This interface must always be provided for objects that support some portion of the collection of the %IAccessible2 interfaces.
 * Please refer to @ref _changingInterfaces "Changing between Accessible Interfaces" for special considerations related to use of the MSAA IAccessible interface and
 * the set of %IAccessible2 interfaces.
 * */
#[interface("6C9430E9-299D-4E6F-BD01-A82A1E88D3FF")]
pub(crate) unsafe trait IAccessible2_2: IAccessible2 {
    //noinspection SpellCheckingInspection
    /**
     * Returns the attribute value of a specified attribute specific to this object.
     * `name`
     * `attribute` retrieval S_FALSE returned if there is nothing to return, [out] value is NULL.
     * retrieval E_INVALIDARG if bad [in] passed.
     * @note The output value is a VARIANT.
     * Typically, it will be a VT_BSTR, but there are some cases where it will be a VT_I4 or VT_BOOL.
     * Refer to the [ Object Attributes specification](http://www.linuxfoundation.org/collaborate/workgroups/accessibility/iaccessible2/objectattributesIAccessible2)
     * for more information.
     * */
    pub(crate) fn attribute(&self, name: BSTR, attribute: *mut VARIANT) -> HRESULT;

    /**
     * Returns the deepest hypertext accessible in the subtree of this object, and the caret offset within it.
     * `accessible` `caretOffset` retrieval S_FALSE returned if there is no caret in any of the objects in the subtree, [out] accessible is NULL and [out] caretOffset is -1.
     * */
    pub(crate) fn accessibleWithCaret(
        &self,
        accessible: *mut *mut IUnknown,
        caretOffset: *mut i32,
    ) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns relation targets for a specified target type.
     * `type` The requested @ref grpRelations "relation type".
     * `maxTargets` The number of targets requested.  Zero indicates that all targets should be returned.
     * `targets` This array is allocated by the server.  The client must free it with CoTaskMemFree.
     * `nTargets` The number of targets returned; the size of the returned array.
     * retrieval S_FALSE if there are no targets, [out] values are NULL and 0 respectively.
     * retrieval E_INVALIDARG if bad [in] passed.
     * */
    pub(crate) fn relationTargetsOfType(
        &self,
        r#type: BSTR,
        maxTargets: i32,
        targets: *mut *mut *mut IUnknown,
        nTargets: *mut i32,
    ) -> HRESULT;
}

/*
 * Idl file copyright information:
 *  File Name (Accessible2_2.idl)
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
 */
