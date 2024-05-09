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

use super::IA2CommonTypes::IA2CoordinateType;
use windows::core::interface;
use windows::core::BSTR;
use windows::core::HRESULT;
use windows::core::{IUnknown, IUnknown_Vtbl};

/**
 * This interface represents images and icons.
 * This interface is used for a representation of images like icons on buttons.
 * %IAccessibleImage only needs to be implemented in certain situations.  Some examples are:
 * <ol>
 * <li>The accessible name and description are not enough to fully describe the image, e.g., when the accessible description is used to define the behavior of an actionable image and the image itself conveys semantically significant information. </li>
 *  <li>The user can edit the content that includes an image and therefore the user needs to be able to review the image's position. </li>
 * </ol>
 * */
#[interface("FE5ABB3D-615E-4f7b-909F-5F0EDA9E8DDE")]
pub(crate) unsafe trait IAccessibleImage: IUnknown {
    /**
     * Returns the localized description of the image.
     * `description` retrieval S_FALSE if there is nothing to return, [out] value is NULL
     * */
    fn description(&self, description: *mut BSTR) -> HRESULT;

    /**
     * Returns the coordinates of the image.
     * `coordinateType` Specifies whether the returned coordinates should be relative to the screen or the parent object.
     * `x` `y` */
    fn imagePosition(&self, coordinateType: IA2CoordinateType, x: *mut i32, y: *mut i32)
                     -> HRESULT;

    /**
     * Returns the size of the image in units specified by parent's coordinate system.
     * `height` `width` */
    fn imageSize(&self, height: *mut i32, width: *mut i32) -> HRESULT;
}

/*
 * Idl file copyright information:
 *  File Name (AccessibleImage.idl)
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
