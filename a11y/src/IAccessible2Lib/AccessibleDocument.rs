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
use windows::core::HRESULT;
use windows::core::{IUnknown, IUnknown_Vtbl};

/**
 * This interface represents documents.
 * This interface is used for a representation of documents.
 * */
#[interface("C48C7FCF-4AB5-4056-AFA6-902D6E1D1149")]
pub(crate) unsafe trait IAccessibleDocument: IUnknown {
    /**
     * Returns the most recently used anchor target within a document.
     * A document's most recently targeted in-page anchor is returned.  A typical use of this method is to fetch the anchor target within an HTML document.  In this case, anchor targets are those that have been defined with the <a> tag.  
     * `accessible` retrieval S_FALSE if there are no existing valid anchor targets, [out] value is NULL.
     * */
    fn anchorTarget(&self, accessible: *mut *mut IUnknown) -> HRESULT;
}

/**
 * Idl file copyright information:
 *  File Name (AccessibleDocument.idl)
 *
 *  IAccessible2 IDL Specification
 *
 *  Copyright (c) 2013 Linux Foundation
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
