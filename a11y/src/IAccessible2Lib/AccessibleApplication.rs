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
 * This interface gives access to the application's name and version information.
 * This interface provides the AT with necessary information to distinguish this application from other applications, various versions of this application, or different versions of the application running on different versions of an accessibility bridge or toolkit.
 * Servers implementing IAccessible2 should provide access to the %IAccessibleApplication interface via QueryService from any object so that ATs can easily determine specific information about the application such as its name or version.
 * */
#[interface("D49DED83-5B25-43F4-9B95-93B44595979E")]
pub(crate) unsafe trait IAccessibleApplication: IUnknown {
    /**
     * Returns the application name.
     * `name` retrieval S_FALSE if there is nothing to return, [out] value is NULL
     * */
    fn appName(&self, name: *mut BSTR) -> HRESULT;

    /**
     * Returns the application version.
     * `version` The version string must not contain levels when it is known beforehand that this information will never require a change in a client's behavior.
     * For example, use "3.6.0" rather than "3.6.0.v201005131500".
     * retrieval S_FALSE if there is nothing to return, [out] value is NULL
     * */
    fn appVersion(&self, version: *mut BSTR) -> HRESULT;

    /**
     * Returns the toolkit/bridge name.
     * `name` retrieval S_FALSE if there is nothing to return, [out] value is NULL
     * */
    fn toolkitName(&self, name: *mut BSTR) -> HRESULT;

    /**
     * Returns the toolkit/bridge version.
     * `version` The version string must not contain levels when it is known beforehand that this information will never require a change in a client's behavior.
     * For example, use "3.6.0" rather than "3.6.0.v201005131500".
     * retrieval S_FALSE if there is nothing to return, [out] value is NULL
     * */
    fn toolkitVersion(&self, version: *mut BSTR) -> HRESULT;
}

/*
 * Idl file copyright information:
 *  File Name (AccessibleApplication.idl)
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
 */
