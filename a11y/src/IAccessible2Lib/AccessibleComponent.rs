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
 * A value specifying a color in ARGB format, where each 8-bit color component specifies alpha, red, green, and blue respectively.
 * The alpha value is optional.
 * */
pub(crate) type IA2Color = i32;

//noinspection SpellCheckingInspection
/**
 * This interface is implemented by any object that can be rendered on the screen.
 * This interface provides the standard mechanism for an assistive technology to retrieve information concerning the graphical representation of an object.
 * Coordinates used by the functions of this interface are specified in different coordinate systems.
 * Their scale is the same and is equal to that of the screen coordinate system.  In other words, all coordinates are measured in pixels.  They differ in their respective origin:
 * <ul>
 *  <li>The screen coordinate system has its origin in the upper left corner of the current screen.</li>
 *  <li>The origin of the parent coordinate system is the upper-left corner of the parent's bounding box.  With no parent, the screen coordinate system is used instead.</li>
 * </ul>
 * */
#[interface("1546D4B0-4C98-4bda-89AE-9A64748BDDE4")]
pub(crate) unsafe trait IAccessibleComponent: IUnknown {
    /**
     * Returns the upper-left corner its location of the object's bounding box relative to the immediate parent object.
     * The coordinates of the bounding box are given relative to the parent's coordinate system. The coordinates of the returned position are relative to this object's parent or relative to the screen on which this object is rendered if it has no parent. If the object is not on any screen, the returned position is (0,0).
     * `x` `y`
     * */
    fn locationInParent(&self, x: *mut i32, y: *mut i32) -> HRESULT;

    /**
     * Returns the foreground color of this object.
     * `foreground` The returned color is the foreground color of this object or, if that is not supported, the default foreground color.
     * */
    fn foreground(&self, foreground: *mut IA2Color) -> HRESULT;

    /**
     * Returns the background color of this object.
     * `background` The returned color is the background color of this object, or, if that is not supported, the default background color.
     * */
    fn background(&self, background: *mut IA2Color) -> HRESULT;
}

/**
 * Idl file copyright information:
 *  File Name (AccessibleComponent.idl)
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
