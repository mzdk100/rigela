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

use super::IA2CommonTypes::IA2TableModelChange;
use windows::core::interface;
use windows::core::BSTR;
use windows::core::HRESULT;
use windows::core::{IUnknown, IUnknown_Vtbl};

/**
 * This interface gives access to a two-dimensional table.
 * Please also refer to the IAccessibleTableCell interface.
 * If you want to support older applications, you should also support the IAccessibleTable interface.
 * */
#[interface("6167f295-06f0-4cdd-a1fa-02e25153d869")]
pub(crate) unsafe trait IAccessibleTable2: IUnknown {
    //noinspection SpellCheckingInspection
    /**
     * Returns the accessible object at the specified row and column in the table.  This object could be an IAccessible or an IAccessible2.
     * `row` The 0 based row index for which to retrieve the cell.
     * `column` The 0 based column index for which to retrieve the cell.
     * `cell` If both row and column index are valid, then the corresponding accessible object is returned that represents the requested cell regardless of whether the cell is currently visible (on the screen).
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn cellAt(&self, row: i32, column: i32, cell: *mut *mut IUnknown) -> HRESULT;

    /**
     * Returns the caption for the table.  The returned object could be an IAccessible or an IAccessible2.
     * `accessible` If the table has a caption, then a reference to it is returned, else a NULL pointer is returned.
     * retrieval S_FALSE if there is nothing to return, [out] value is NULL
     * @deprecated use a describedBy relation
     * */
    fn caption(&self, accessible: *mut *mut IUnknown) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns the description text of the specified column in the table.
     * `column` The 0 based index of the column for which to retrieve the description.
     * `description` Returns the description text of the specified column in the table if such a description exists.
     * Otherwise, a NULL pointer is returned.
     * retrieval S_FALSE if there is nothing to return, [out] value is NULL
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn columnDescription(&self, column: i32, description: *mut BSTR) -> HRESULT;

    /**
     * Returns the total number of columns in table
     * `columnCount` Number of columns in table (including columns outside the current viewport)
     * */
    fn nColumns(&self, columnCount: *mut i32) -> HRESULT;

    /**
     * Returns the total number of rows in table
     * `rowCount` Number of rows in table (including rows outside the current viewport)
     * */
    fn nRows(&self, rowCount: *mut i32) -> HRESULT;

    /**
     * Returns the selected cell's total number.
     * `cellCount` Numbers of cells currently selected.
     * */
    fn nSelectedCells(&self, cellCount: *mut i32) -> HRESULT;

    /**
     * Returns the selected column's total number.
     * `columnCount` Numbers of columns currently selected.
     * */
    fn nSelectedColumns(&self, columnCount: *mut i32) -> HRESULT;

    /**
     * Returns the selected row's total number.
     * `rowCount` Numbers of rows currently selected.
     * */
    fn nSelectedRows(&self, rowCount: *mut i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns the description text of the specified row in the table.
     * `row` The 0 based index of the row for which to retrieve the description.
     * `description` Returns the description text of the specified row in the table if such a description exists.
     * Otherwise, a NULL pointer is returned.
     * retrieval S_FALSE if there is nothing to return, [out] value is NULL
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn rowDescription(&self, row: i32, description: *mut BSTR) -> HRESULT;

    /**
     * Returns a list of accessibles currently selected.
     * `cells` Pointer to an array of references to selected accessibles.  The array is allocated by the server with CoTaskMemAlloc and freed by the client with CoTaskMemFree.
     * `nSelectedCells` The number of accessibles returned; the size of the returned array.
     * retrieval S_FALSE if there are none, [out] values are NULL and 0 respectively
     * */
    fn selectedCells(&self, cells: *mut *mut *mut IUnknown, nSelectedCells: *mut i32) -> HRESULT;

    /**
     * Returns a list of column indexes currently selected (zero-based).
     * `selectedColumns` A pointer to column index's array of selected columns (each index is zero based).  The array is allocated by the server with CoTaskMemAlloc and freed by the client with CoTaskMemFree.
     * `nColumns` The number of column indexes returned; the size of the returned array.
     * retrieval S_FALSE if there are none, [out] values are NULL and 0 respectively
     * */
    fn selectedColumns(&self, selectedColumns: *mut *mut i32, nColumns: *mut i32) -> HRESULT;

    /**
     * Returns a list of row indexes currently selected (zero-based).
     * `selectedRows` Row index's array of selected rows (each index is zero-based).  The array is allocated by the server with CoTaskMemAlloc and freed by the client with CoTaskMemFree.
     * `nRows` The number of row indexes returned; the size of the returned array.
     * retrieval S_FALSE if there are none, [out] values are NULL and 0 respectively
     * */
    fn selectedRows(&self, selectedRows: *mut *mut i32, nRows: *mut i32) -> HRESULT;

    /**
     * Returns the summary description of the table.  The returned object could be an IAccessible or an IAccessible2.
     * `accessible` Returns a reference to an implementation-dependent accessible object representing the table's summary or a NULL pointer if the table does not support a summary.
     * retrieval S_FALSE if there is nothing to return, [out] value is NULL
     * @deprecated Use the labeledBy relation
     * */
    fn summary(&self, accessible: *mut *mut IUnknown) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns a boolean value indicating whether the specified column is completely selected.
     * `column` zero-based index of the column for which to determine whether it is selected.
     * `isSelected` Returns TRUE if the specified column is selected completely and FALSE otherwise.
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn isColumnSelected(&self, column: i32, isSelected: *mut bool) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns a boolean value indicating whether the specified row is completely selected.
     * `row` zero-based index of the row for which to determine whether it is selected.
     * `isSelected` Returns TRUE if the specified row is selected completely and FALSE otherwise.
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn isRowSelected(&self, row: i32, isSelected: *mut bool) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Selects a row and unselects all previously selected rows.
     * The behavior should mimic that of the application, but for those applications which do not have a means in the GUI to select a full row of cells, the behavior should be as follows:
     * First any selected rows in the table are unselected.  Then the entire row of cells for the specified row is selected.  If any of the cells in the selected row span additional rows, the cells in those rows are also selected.
     * `row` zero-based index of the row to be selected.
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn selectRow(&self, row: i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Selects a column and unselects all previously selected columns.
     * The behavior should mimic that of the application, but for those applications which do not have a means in the GUI to select a full column of cells, the behavior should be as follows:
     * First any selected columns in the table are unselected.  Then the entire column of cells for the specified column is selected.  If any of the cells in the selected column span additional columns, the cells in those columns are also selected.
     * `column` zero-based index of the column to be selected.
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn selectColumn(&self, column: i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Unselects one row, leaving other selected rows selected (if any).
     * The behavior should mimic that of the application, but for those applications which do not have a means in the GUI to unselect a full row of cells, the behavior should be as follows:
     * The entire row of cells for the specified row is unselected.  If any of the cells in the selected row span additional rows, the cells in those rows are also unselected.
     * `row` zero-based index of the row to be unselected.
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn unselectRow(&self, row: i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Unselects one column, leaving other selected columns selected (if any).
     * The behavior should mimic that of the application, but for those applications which do not have a means in the GUI to unselect a full column of cells, the behavior should be as follows:
     * The entire column of cells for the specified column is unselected.  If any of the cells in the selected column span additional columns, the cells in those columns are also unselected.
     * `column` zero-based index of the column to be unselected.
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn unselectColumn(&self, column: i32) -> HRESULT;

    /**
     * Returns the type and extents describing how a table changed.
     * Provided for use by the IA2_EVENT_TABLE_MODEL_CHANGED event handler.
     * This data is only guaranteed to be valid while the thread notifying the event continues. Once the handler has returned, the validity of the data depends on how the server manages the life cycle of its objects. Also, note that the server may have different life cycle management strategies for controls depending on whether a control manages its children. Lists, trees, and tables can have a large number of children, and thus it's possible that the child objects for those controls would only be created as needed. Servers should document their life cycle strategy as this will be of interest to assistive technology or script engines accessing data out of process or from other threads. Servers only need to save the most recent row and column values associated with the change, and the scope of the entire application is adequate.
     * `modelChange` A struct of (type (insert, delete, update), firstRow, lastRow, firstColumn, lastColumn).
     * retrieval S_FALSE if there is nothing to return, [out] value is NULL
     * */
    fn modelChange(&self, modelChange: *mut IA2TableModelChange) -> HRESULT;
}

/*
 * Idl file copyright information:
 *  File Name (AccessibleTable2.idl)
 *
 *  IAccessible2 IDL Specification
 *
 *  Copyright (c) 2007, 2012 Linux Foundation
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
