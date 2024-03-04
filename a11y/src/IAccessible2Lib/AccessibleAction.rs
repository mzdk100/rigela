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
 * This enum defines values which are predefined actions for use when implementing support for media.
 * This enum is used when specifying an action for IAccessibleAction::doAction.
 * */
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub(crate) enum IA2Actions {
    /**
     * Used to inform the server that the client will signal via IA2_ACTION_COMPLETE when it has consumed the content provided by the object.  This action allows the object's server to wait for all clients to signal their readiness for additional content.
     * Any form of content generation that requires synchronization with an AT would require use of this action.  One example is the generation of text describing visual content not obvious from a video's soundtrack.
     * In this scenario, the Text to Speech or Braille output may take more time than the related length of silence in the video's soundtrack.
     * */
    IA2_ACTION_OPEN = -1,

    /**
     * Used by the client to inform the server that it has consumed the most recent content provided by this object.
     * */
    IA2_ACTION_COMPLETE = -2,

    /**
     * Used to inform the server that the client no longer requires synchronization.
     * */
    IA2_ACTION_CLOSE = -3,
}

/**
 * This interface gives access to actions that can be executed for accessible objects.
 * Every accessible object that can be manipulated via the native GUI beyond the methods available either in the MSAA IAccessible interface or in the set of
 * IAccessible2 interfaces (other than this IAccessibleAction interface) should support the IAccessibleAction interface in order to provide Assistive Technology access to all the actions that can be performed by the object.  Each action can be performed or queried for a name, description or associated key bindings.  
 * Actions are needed more for ATs that assist the mobility impaired, such as on-screen keyboards and voice command software.  By providing actions directly, the AT can present them to the user without the user having to perform the extra steps to navigate a context menu.
 * The first action should be equivalent to the MSAA default action.  If there is only one action, %IAccessibleAction should also be implemented.
 * */
#[interface("B70D9F59-3B5A-4dba-AB9E-22012F607DF5")]
pub(crate) unsafe trait IAccessibleAction: IUnknown {
    //noinspection SpellCheckingInspection
    /**
     * Returns the number of accessible actions available in this object.
     * If there is more than one, the first one is considered the "default" action of the object.
     * `nActions` The returned value of the number of actions is zero if there are no actions.
     * @note This method is missing a [propget] prefix in the IDL.  The result is the method is named nActions in generated C++ code instead of get_nActions.
     * */
    fn nActions(&self, nActions: *mut i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Performs the specified Action on the object.
     * `actionIndex` zero-based index specifying the action to perform.  If it lies outside the valid range, no action is performed.
     * retrieval S_FALSE if action could not be performed
     * retrieval E_INVALIDARG if bad [in] passed
     * @note If implementing support for media, refer to the predefined constants in the: IA2Actions enum.
     * */
    fn doAction(&self, actionIndex: i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns a description of its specified action.
     * `actionIndex` zero-based index specifying which action's description to return.
     * If it lies outside the valid range, an empty string is returned.
     * `description` The returned value is a localized string of the specified action.
     * retrieval S_FALSE if there is nothing to return, [out] value is NULL
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn description(&self, actionIndex: i32, description: *mut BSTR) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns an array of BSTRs describing one or more key bindings, if there are any, associated with the specified action.
     * The returned strings are the localized human-readable key sequences to be used to activate each action, e.g. "Ctrl+Shift+D".  Since these key sequences are to be used when the object has focus, they are like mnemonics (access keys), and not like shortcut (accelerator) keys.
     * There is no need to implement this method for single action controls since that would be redundant with the standard MSAA programming practice of getting the mnemonic from get_accKeyboardShortcut.
     * An AT such as an On-Screen Keyboard might not expose these bindings but provide alternative means of activation.
     * Note: the client allocates and passes in an array of pointers.  The server allocates the BSTRs and passes back one or more pointers to these BSTRs into the array of pointers allocated by the client.  The client is responsible for deallocating the BSTRs.
     * `actionIndex` zero-based index specifying which action's key bindings should be returned.
     * `nMaxBindings` This parameter is ignored. Refer to @ref _arrayConsideration "Special Consideration when using Arrays" for more details.
     * `keyBindings` An array of BSTRs, allocated by the server, one for each key binding.
     * The client must free it with CoTaskMemFree.
     * `nBindings` The number of key bindings returned; the size of the returned array.
     * retrieval S_FALSE if there are no key bindings, [out] values are NULL and 0 respectively,
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn keyBinding(
        &self,
        actionIndex: i32,
        nMaxBindings: i32,
        keyBindings: *mut *mut BSTR,
        nBindings: *mut i32,
    ) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns the non-localized name of specified action.
     * `actionIndex` zero-based index specifying which action's non-localized name should be returned.
     * `name` retrieval S_FALSE if there is nothing to return, [out] value is NULL
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn name(&self, actionIndex: i32, name: *mut BSTR) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns the localized name of specified action.
     * `actionIndex` zero-based index specifying which action's localized name should be returned.
     * `localizedName` retrieval S_FALSE if there is nothing to return, [out] value is NULL
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn localizedName(&self, actionIndex: i32, localizedName: *mut BSTR) -> HRESULT;
}

/**
 * Idl file copyright information:
 *  File Name (AccessibleAction.idl)
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
