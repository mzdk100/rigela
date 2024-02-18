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

use super::Accessible2_2::{IAccessible2_2, IAccessible2_2_Impl, IAccessible2_2_Vtbl};
use windows::core::IUnknown;
use windows::core::HRESULT;
use windows_interface::interface;

/**
 * This structure represents a directional range of the content. It is defined
 * by two points in the content, where each one is defined by an accessible
 * object and an offset relative to it. A typical case of a range point is
 * a text accessible and text offset within it.
 *
 * The "anchor" is one point of the range and typically remains constant.
 * The other point is the "active" point, which typically corresponds to
 * the user's focus or point of interest. The user moves the active point to
 * expand or collapse the range. In most cases, anchor is the start of the range
 * and active is the end. However, in a case of selection, when selecting
 * backwards (e.g., pressing shift+left arrow in a text field), the start of
 * the range is the active point, as the user moves this to manipulate
 * the selection.
 */
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct IA2Range {
    anchor: *mut IUnknown,
    anchorOffset: i32,
    active: *mut IUnknown,
    activeOffset: i32,
}

/**
 * This interface is an extension of IAccessible2_2 and IAccessible2
 * interfaces.
 */
#[interface("5BE18059-762E-4E73-9476-ABA294FED411")]
pub(crate) unsafe trait IAccessible2_3: IAccessible2_2 {
    /**
     * Returns an array of ranges for selections within the accessible.
     * `ranges` The array of selection ranges, allocated by the server. The client must free it with CoTaskMemFree.
     * `nRanges` the array length
     * retrieval S_FALSE returned if there is no selection within the accessible
     * */
    pub(crate) fn selectionRanges(&self, ranges: *mut *mut IA2Range, nRanges: *mut i32) -> HRESULT;
}
