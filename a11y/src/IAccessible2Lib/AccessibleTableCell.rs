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
 * This interface gives access to the cells of a two-dimensional table.
 * Please also refer to the IAccessibleTable2 interface.
 * */
#[interface("594116B1-C99F-4847-AD06-0A7A86ECE645")]
pub(crate) unsafe trait IAccessibleTableCell: IUnknown {
    /**
     * Returns the number of columns occupied by this cell accessible.
     * The result is greater than 1 if the specified cell spans multiple columns.
     * `nColumnsSpanned` Returns the one-based column extent of the specified cell.
     * */
    fn columnExtent(&self, nColumnsSpanned: *mut i32) -> HRESULT;

    /**
     * Returns the column headers as an array of cell accessibles.
     * `cellAccessibles` Pointer to an array of references to cell accessibles.  The server allocates the array.  The client must free it with CoTaskMemFree.
     * `nColumnHeaderCells` The number of accessibles returned; the size of the returned array.
     * retrieval S_FALSE if there is no header, [out] values are NULL and 0 respectively
     * */
    fn columnHeaderCells(
        &self,
        cellAccessibles: *mut *mut *mut IUnknown,
        nColumnHeaderCells: *mut i32,
    ) -> HRESULT;

    /**
     * Translates this cell accessible into the corresponding column index.
     * `columnIndex` Returns the cell's zero-based column index of the specified cell or the index of the first column if the cell spans multiple columns.
     * */
    fn columnIndex(&self, columnIndex: *mut i32) -> HRESULT;

    /**
     * Returns the number of rows occupied by this cell accessible.
     * `nRowsSpanned` Returns the row extent of the specified cell.
     * */
    fn rowExtent(&self, nRowsSpanned: *mut i32) -> HRESULT;

    /**
     * Returns the row headers as an array of cell accessibles.
     * `cellAccessibles` Pointer to an array of references to cell accessibles.  The server allocates the array.  The client must free it with CoTaskMemFree.
     * `nRowHeaderCells` The number of accessibles returned; the size of the returned array.
     * retrieval S_FALSE if there is no header, [out] values are NULL and 0 respectively
     * */
    fn rowHeaderCells(
        &self,
        cellAccessibles: *mut *mut *mut IUnknown,
        nRowHeaderCells: *mut i32,
    ) -> HRESULT;

    /**
     * Translates this cell accessible into the corresponding row index.
     * `rowIndex` Returns the zero-based row index of the specified cell or the index of the first row if the cell spans multiple rows.
     * */
    fn rowIndex(&self, rowIndex: *mut i32) -> HRESULT;

    /**
     * Returns a boolean value indicating whether this cell is selected.
     * `isSelected` Returns TRUE if the specified cell is selected and FALSE otherwise.
     * */
    fn isSelected(&self, isSelected: *mut bool) -> HRESULT;

    /**
     * Gets the row and column indexes and extents of this cell accessible and whether it is selected.
     * This is a convenience function.  It is not mandatory to implement it.
     * `row` zero-based row index.
     * `column` zero-based column index.
     * `rowExtents` Number of cells spanned by this cell in this row.
     * `columnExtents` Number of cells spanned by this cell in this column.
     * `isSelected` Indicates if the specified cell is selected.
     * */
    fn rowColumnExtents(
        &self,
        row: *mut i32,
        column: *mut i32,
        rowExtents: *mut i32,
        columnExtents: *mut i32,
        isSelected: *mut bool,
    ) -> HRESULT;

    /**
     * Returns a reference to the accessible object of the containing table.
     * `table` Returns a reference to the IUnknown of the containing table.
     * */
    fn table(&self, table: *mut *mut IUnknown) -> HRESULT;
}

/**
 * Idl file copyright information:
 *  File Name (AccessibleTableCell.idl)
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
 * */
trait IdlCopyright {}
