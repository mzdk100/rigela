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
 * grpRelations Relations
 * Use the following constants to compare against the BSTRs returned by
 * IAccessibleRelation::relationType.
 * */

/** The target object is the containing application object. */
#[allow(unused)]
const IA2_RELATION_CONTAINING_APPLICATION: &str = "containingApplication";

/**
 * The target object is the containing document object.
 * The target object implements the IAccessibleDocument interface.
 * */
#[allow(unused)]
const IA2_RELATION_CONTAINING_DOCUMENT: &str = "containingDocument";

/** The target object is the containing tab pane object. */
#[allow(unused)]
const IA2_RELATION_CONTAINING_TAB_PANE: &str = "containingTabPane";

/** The target object is the containing window object. */
#[allow(unused)]
const IA2_RELATION_CONTAINING_WINDOW: &str = "containingWindow";

/**
 * A target object affects some attribute of this object.
 * */
#[allow(unused)]
const IA2_RELATION_CONTROLLED_BY: &str = "controlledBy";

/** This object is interactive and controls some attribute of a target object. */
#[allow(unused)]
const IA2_RELATION_CONTROLLER_FOR: &str = "controllerFor";

/** This object is described by the target object. */
#[allow(unused)]
const IA2_RELATION_DESCRIBED_BY: &str = "describedBy";

/** This object is describing the target object. */
#[allow(unused)]
const IA2_RELATION_DESCRIPTION_FOR: &str = "descriptionFor";

/** This object is embedded by a target object. */
#[allow(unused)]
const IA2_RELATION_EMBEDDED_BY: &str = "embeddedBy";

//noinspection SpellCheckingInspection
/**
 * This object embeds a target object.
 * This relation can be used on the OBJID_CLIENT accessible for a top level window
 * to show where the content areas are.
 * */
#[allow(unused)]
const IA2_RELATION_EMBEDS: &str = "embeds";

/**
 * Content flows to this object from a target object.
 * This relation and IA2_RELATION_FLOWS_TO are useful to tie text and non-text objects together
 * to allow assistive technology to follow the intended reading order.
 * */
#[allow(unused)]
const IA2_RELATION_FLOWS_FROM: &str = "flowsFrom";

/** Content flows from this object to a target object. */
#[allow(unused)]
const IA2_RELATION_FLOWS_TO: &str = "flowsTo";

/** This object is label for a target object. */
#[allow(unused)]
const IA2_RELATION_LABEL_FOR: &str = "labelFor";

/**
 * This object is labeled by a target object.
 * Note that the double L spelling which follows is preferred.
 * Please use it instead.
 * This single L version may be removed in a later version.
 * */
#[allow(unused)]
const IA2_RELATION_LABELED_BY: &str = "labelledBy";

/** This object is labeled by a target object. */
#[allow(unused)]
const IA2_RELATION_LABELLED_BY: &str = "labelledBy";

/**
 * This object is a member of a group of one or more objects.
 * When there is more than one object in the group,
 * each member may have one and the same target, e.g., a grouping object.
 * It is also possible that each member has multiple additional targets,
 * e.g., one for every other member in the group.
 * */
#[allow(unused)]
const IA2_RELATION_MEMBER_OF: &str = "memberOf";

/** The target object is the next object in the tab order. */
#[allow(unused)]
const IA2_RELATION_NEXT_TABBABLE: &str = "nextTabbable";

/**
 * This object is a logical child of a target object.
 * This relation is the reciprocal of the IA2_RELATION_NODE_PARENT_OF relation.
 * In some cases,
 * an application's accessible tree is such
 * that objects can be in a logical parent-child relationship
 * that is different from the hierarchy of the accessible tree.
 * */
#[allow(unused)]
const IA2_RELATION_NODE_CHILD_OF: &str = "nodeChildOf";

/**
 * This object is a logical parent of a target object.
 * This relation is the reciprocal of the IA2_RELATION_NODE_CHILD_OF relation.
 * In some cases,
 * an application's accessible tree is such
 * that objects can be in a logical parent-child relationship
 * that is different from the hierarchy of the accessible tree.
 * */
#[allow(unused)]
const IA2_RELATION_NODE_PARENT_OF: &str = "nodeParentOf";

/** This object is a parent window of the target object. */
#[allow(unused)]
const IA2_RELATION_PARENT_WINDOW_OF: &str = "parentWindowOf";

/**
 * This object is a transient component related to the target object.
 * When this object is activated, the target object doesn't lose focus.
 * */
#[allow(unused)]
const IA2_RELATION_POPUP_FOR: &str = "popupFor";

/** The target object is the previous object in the tab order. */
#[allow(unused)]
const IA2_RELATION_PREVIOUS_TABBABLE: &str = "previousTabbable";

//noinspection SpellCheckingInspection
/** This object is a sub window of a target object. */
#[allow(unused)]
const IA2_RELATION_SUBWINDOW_OF: &str = "subwindowOf";

/**
 * The target object provides a detailed, extended description for this object.
 * It provides more detailed information
 * than would normally be provided using the IA2_RELATION_DESCRIBED_BY relation.
 * A common use for this relation is in digital publishing
 * where an extended description needs
 * to be conveyed in a book
 * that requires structural markup or the embedding of other technology
 * to provide illustrative content.
 * */
#[allow(unused)]
const IA2_RELATION_DETAILS: &str = "details";

/**
 * This object provides a detailed, extended description for the target object.
 * See IA2_RELATION_DETAILS.
 * */
#[allow(unused)]
const IA2_RELATION_DETAILS_FOR: &str = "detailsFor";

/** The target object is the error message for this object. */
#[allow(unused)]
const IA2_RELATION_ERROR: &str = "error";

/** This object is the error message for the target object. */
#[allow(unused)]
const IA2_RELATION_ERROR_FOR: &str = "errorFor";

/**
 * This interface gives access to an object's set of relations.
 * */
#[interface("7CDF86EE-C3DA-496a-BDA4-281B336E1FDC")]
pub(crate) unsafe trait IAccessibleRelation: IUnknown {
    /**
     * Returns the type of the relation.
     * `relationType` The strings returned are defined @ref grpRelations "in this section of the documentation".
     * */
    pub(crate) fn relationType(&self, relationType: *mut BSTR) -> HRESULT;

    /**
     * Returns a localized version of the relation type.
     * `localizedRelationType`
     * */
    pub(crate) fn localizedRelationType(&self, localizedRelationType: *mut BSTR) -> HRESULT;

    /**
     * Returns the number of targets for this relation.
     * `nTargets`
     * */
    pub(crate) fn nTargets(&self, nTargets: *mut i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns one accessible relation target.
     * `targetIndex` zero based index
     * `target` retrieval E_INVALIDARG if bad [in] passed
     * @note Use QueryInterface to get IAccessible2.
     * */
    pub(crate) fn target(&self, targetIndex: i32, target: *mut *mut IUnknown) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns multiple accessible relation targets
     * `maxTargets` maximum size of the array allocated by the client
     * `targets` The array of target objects.
     * Note that this array is to be allocated by the client and freed when no longer needed.
     * Refer to @ref _arrayConsideration "Special Consideration when using Arrays" for more details.
     * You will need to use QueryInterface on the IUnknown to get the IAccessible2.
     * `nTargets` actual number of targets in the returned array (not more than maxTargets)
     * retrieval E_INVALIDARG if bad [in] passed, e.g., a negative value
     * */
    pub(crate) fn targets(
        &self,
        maxTargets: i32,
        targets: *mut *mut IUnknown,
        nTargets: *mut i32,
    ) -> HRESULT;
}

/*
 * Idl file copyright information:
 *  File Name (AccessibleRelation.idl)
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
 *  modification, are permitted if the following conditions are met:
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
 * CONTRIBUTORS "AS ARE" AND ANY EXPRESS OR IMPLIED WARRANTIES,
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
