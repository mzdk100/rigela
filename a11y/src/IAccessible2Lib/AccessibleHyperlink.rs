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

use super::AccessibleAction::{IAccessibleAction, IAccessibleAction_Impl, IAccessibleAction_Vtbl};
use windows::core::interface;
use windows::core::{HRESULT, VARIANT};

//noinspection SpellCheckingInspection
/**
 * This interface represents hyperlinks.
 * This interface represents a hyperlink associated with a single substring of text or single non-text object.  Non-text objects can have either a single link or a collection of links such as when the non-text object is an image map.
 * Linked objects and anchors are implementation-dependent. This interface is derived from IAccessibleAction.  IAccessibleAction::nActions is one greater than the maximum value for the indices used with the methods of this interface.
 * Furthermore, the object that implements this interface has to be connected implicitly or explicitly with an object that implements IAccessibleText.
 * IAccessibleHyperlink::startIndex and IAccessibleHyperlink::endIndex are indices with respect to the text exposed by IAccessibleText.
 * This interface provides access to a single object which can have multiple actions.
 * An example is an image map, which is an image with multiple links, each of which is associated with a separate non-overlapping area of the image.  This interface could also be applied to other kinds of objects with multiple actions such as "smart tags" which are objects, typically strings, which have multiple actions such as "Activate URI", "Bookmark URI", etc.
 * An interesting use case is an image map where each area is associated with multiple actions, e.g. an image map of smart tags.  In this case, you would have to implement two levels of accessible hyperlinks.  The first level of hyperlinks would only implement anchor and anchorTarget.  The anchors would all reference the image object.  The anchorTargets would reference the second level of accessible hyperlink objects.  None of the IAccessibleAction methods would be implemented on the first level of hyperlink objects.
 * The second level hyperlink objects would implement the IAccessibleAction methods.  Their anchors would also reference the image object, and their anchorTargets would reference URLs or the objects that would be activated.
 * This use case demonstrates that in some cases, there is no need for IAccessibleHyperlink to derive from IAccessibleAction.  As a result, it may be removed in a later version of the IDL, and it is suggested that implementations should not rely on the inheritance.
 * */
#[interface("01C20F2B-3DD2-400f-949F-AD00BDAB1D41")]
pub(crate) unsafe trait IAccessibleHyperlink: IAccessibleAction {
    /**
     * Returns an object that represents the link anchor, as appropriate for the link at the specified index.
     * `index` A 0 based index identifies the anchor when, as in the case of an image map, there is more than one link represented by this object.  The valid maximal index is indicated by IAccessibleAction::nActions.
     * `anchor` This is an implementation dependent value.  For example, for a text link this method could return the substring of the containing string where the substring is overridden with link behavior, and for an image link this method could return an IUnknown VARIANT for IAccessibleImage.  See the section about
     * @ref _variants "VARIANTs" for additional information.
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn anchor(&self, index: i32, anchor: *mut VARIANT) -> HRESULT;

    /**
     * Returns an object representing the target of the link, as appropriate for the link at the specified index.
     * `index` A 0 based index identifies the anchor when, as in the case of an image map, there is more than one link represented by this object.  The valid maximal index is indicated by IAccessibleAction::nActions.
     * `anchorTarget` This is an implementation dependent value.  For example, this method could return a BSTR VARIANT of the URI.
     * Alternatively, this method could return an IUnknown VARIANT of a COM interface representing a target object to be activated when the link is activated.  See the section about
     * @ref _variants "VARIANTs" for additional information.
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn anchorTarget(&self, index: i32, anchorTarget: *mut VARIANT) -> HRESULT;

    /**
     * Returns the zero-based character offset at which the textual representation of the hyperlink starts.
     * The returned value is related to the IAccessibleText interface of the object that owns this hyperlink.
     * `index` */
    fn startIndex(&self, index: *mut i32) -> HRESULT;

    /**
     * Returns the zero-based character offset at which the textual representation of the hyperlink ends.
     * The returned value is related to the IAccessibleText interface of the object that owns this hyperlink. The character at the index is not part of the hypertext.
     * `index` */
    fn endIndex(&self, index: *mut i32) -> HRESULT;

    /**
     * Returns whether the target object referenced by this link is still valid.
     * This is a volatile state that may change without sending an appropriate event.
     * Returns TRUE if the referenced target is still valid and FALSE otherwise.
     * This has also been used to indicate whether the URI of the anchorTarget is malformed.
     * `valid` If false, one or more of the object's links are invalid.
     * If true, all the object's links are valid.
     * retrieval S_FALSE if there is nothing to return, [out] value is FALSE
     * @note This method is not being used, is deprecated, and should not be implemented or used.  It is likely that this method will be removed in a later version of the IDL.
     * */
    fn valid(&self, valid: *mut bool) -> HRESULT;
}

/**
 * Idl file copyright information:
 *  File Name (AccessibleHyperlink.idl)
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
 * */
trait IdlCopyright {}
