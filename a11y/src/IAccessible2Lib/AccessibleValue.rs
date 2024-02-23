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

use windows::core::{
    HRESULT,
    IUnknown,
    IUnknown_Vtbl,
    VARIANT,
};
use windows_interface::interface;

/**
 * This interface gives access to a single numerical value.
 * The %IAccessibleValue interface represents a single numerical value and should be implemented by any class that supports numerical value like progress bars and spin boxes.  This interface lets you access the value and its upper and lower bounds.
 * */
#[interface("35855B5B-C566-4fd0-A7B1-E65465600394")]
pub(crate) unsafe trait IAccessibleValue: IUnknown {
    //noinspection SpellCheckingInspection
    /**
     * Returns the value of this object as a number.
     * The exact return type is implementation-dependent.  Typical types are long and double.
     * `currentValue` Returns the current value represented by this object.  See the section about
     * @ref _variants "VARIANTs" for additional information.
     * retrieval S_FALSE if there is nothing to return, [out] value is a VARIANT with vt = VT_EMPTY
     * */
    fn currentValue(&self, currentValue: *mut VARIANT) -> HRESULT;

    /**
     * Sets the value of this object to the given number.
     * The argument is clipped to the valid interval whose upper and lower bounds are returned by the methods IAccessibleValue::maximumValue and IAccessibleValue::minimumValue, i.e., if it is lower than the minimum value, the new value will be the minimum, and if it is greater than the maximum, then the new value will be the maximum.
     * `value` The new value represented by this object.  The set of admissible types for this argument is implementation-dependent.
     * */
    fn setCurrentValue(&self, value: VARIANT) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns the maximal value that can be represented by this object.
     * The type of the returned value is implementation-dependent.  It does not have to be the same type as that returned by method IAccessibleValue::currentValue.
     * `maximumValue` Returns the maximal value in an implementation-dependent type. If this object has no upper bound, then an empty object is returned.  See the section about
     * @ref _variants "VARIANTs" for additional information.
     * retrieval S_FALSE if there is nothing to return, [out] value is a VARIANT with vt = VT_EMPTY
     * */
    fn maximumValue(&self, maximumValue: *mut VARIANT) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns the minimal value that can be represented by this object.
     * The type of the returned value is implementation-dependent.  It does not have to be the same type as that returned by method IAccessibleValue::currentValue.
     * `minimumValue` Returns the minimal value in an implementation-dependent type. If this object has no lower bound, then an empty object is returned.  See the section about
     * @ref _variants "VARIANTs" for additional information.
     * retrieval S_FALSE if there is nothing to return, [out] value is a VARIANT with vt = VT_EMPTY
     * */
    fn minimumValue(&self, minimumValue: *mut VARIANT) -> HRESULT;
}

/**
 * Idl file copyright information:
 *  File Name (AccessibleValue.idl)
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
