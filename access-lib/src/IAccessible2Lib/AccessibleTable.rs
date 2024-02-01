use super::IA2CommonTypes::IA2TableModelChange;
use windows::core::BSTR;
use windows::core::HRESULT;
use windows::core::{IUnknown, IUnknown_Vtbl};
use windows_interface::interface;

/**
 * This interface gives access to a two-dimensional table.
 * Typically, all accessible objects that represent cells or cell-clusters of a table will be at the same time children of the table.  In this case, IAccessible2::indexInParent will return the child index which then can be used when calling IAccessibleTable::rowIndex and IAccessibleTable::columnIndex.
 * However, in some cases, that kind of implementation will not be possible.  When the table cells are not direct children of a table, the object representing the cell can define a "table-cell-index" object attribute identifying the zero-based table cell index.  This object attribute is obtained by parsing the attribute string returned by IAccessible2::attributes.  The "table-cell-index" attribute can be used just like a child index of the typical case.  ATs should first test for the presence of the "table-cell-index" attribute, and if it is not present, then IAccessible2::indexInParent can be used as in the typical case where cells are direct children of the table.
 * The range of valid coordinates for this interface is implementation-dependent.  
 * However, that range includes at least the intervals from the first row or column with index 0 up to the last (but not including) used row or column as returned by IAccessibleTable::nRows and IAccessibleTable::nColumns.
 * Note that newer implementations are now using IAccessibleTable2 and IAccessibleTableCell rather than this interface.
 * */
#[interface("35AD8070-C20C-4fb4-B094-F4F7275DD469")]
pub(crate) unsafe trait IAccessibleTable: IUnknown {
    //noinspection SpellCheckingInspection
    /**
     * Returns the accessible object at the specified row and column in the table.  This object could be an IAccessible or an IAccessible2.
     * `row` The 0 based row index for which to retrieve the cell.
     * `column` The 0 based column index for which to retrieve the cell.
     * `accessible` If both row and column index are valid, then the corresponding accessible object is returned that represents the requested cell regardless of whether the cell is currently visible (on the screen).
     * retrieval E_INVALIDARG if bad [in] passed, [out] value is NULL
     * */
    fn accessibleAt(&self, row: i32, column: i32, accessible: *mut *mut IUnknown) -> HRESULT;

    /**
     * Returns the caption for the table.  The returned object could be an IAccessible or an IAccessible2.
     * `accessible` If the table has a caption, then a reference to it is returned, else a NULL pointer is returned.
     * retrieval S_FALSE if there is nothing to return, [out] value is NULL
     * */
    fn caption(&self, accessible: *mut *mut IUnknown) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Translates the given row and column indexes into the corresponding cell index.
     * `rowIndex` zero-based row index for the cell.
     * `columnIndex` zero-based column index for the cell.
     * `cellIndex` Returns the zero-based index of the cell at the specified row and column indexes.
     * retrieval E_INVALIDARG if bad [in] passed, [out] value is 0
     * @note The returned value is not necessarily a child index of the immediate parent.
     * In cases where the table cells are not direct children of the table, the index is actually the cell index, i.e., conceptually it's an index into a one-dimensional array of cells laid out in row order.
     * */
    fn childIndex(&self, rowIndex: i32, columnIndex: i32, cellIndex: *mut i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns the description text of the specified column in the table.
     * `column` The 0 based index of the column for which to retrieve the description.
     * `description` Returns the description text of the specified column in the table if such a description exists.
     * Otherwise, a NULL pointer is returned.
     * retrieval S_FALSE if there is nothing to return, [out] value is NULL
     * retrieval E_INVALIDARG if bad [in] passed, [out] value is NULL
     * */
    fn columnDescription(&self, column: i32, description: *mut BSTR) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns the number of columns occupied by the accessible object at the specified row and column in the table.
     * The result is greater than 1 if the specified cell spans multiple columns.
     * `row` zero-based row index of the accessible for which to return the column extent.
     * `column` zero-based column index of the accessible for which to return the column extent.
     * `nColumnsSpanned` Returns the one-based column extent of the specified cell.
     * retrieval E_INVALIDARG if bad [in] passed, [out] value is 0
     * */
    fn columnExtentAt(&self, row: i32, column: i32, nColumnsSpanned: *mut i32) -> HRESULT;

    /**
     * Returns the column headers as an %IAccessibleTable object.
     * Content and size of the returned table are implementation-dependent.
     * `accessibleTable` The column header
     * `startingRowIndex` The 0 based row index where the header starts, usually 0.
     * retrieval S_FALSE if there is no header, [out] values are NULL and 0 respectively
     * */
    fn columnHeader(
        &self,
        accessibleTable: *mut *mut IAccessibleTable,
        startingRowIndex: *mut i32,
    ) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Translates the given cell index into the corresponding column index.
     * `cellIndex` zero-based index of the cell in the parent or the closest ancestor table.
     * Typically, this is the value returned from IAccessible2::indexInParent.
     * But in the case where the table cells are not direct children of the table, this is the cell index specified by the "table-cell-index" object attribute obtained from parsing the attribute string returned by calling IAccessible2::attributes on the cell object.
     * `columnIndex` Returns the cell's zero-based column index of the specified child or the index of the first column if the child spans multiple columns.
     * retrieval E_INVALIDARG if bad [in] passed, [out] value is 0
     * */
    fn columnIndex(&self, cellIndex: i32, columnIndex: *mut i32) -> HRESULT;

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
     * Returns the total number of selected cells.
     * `cellCount` Numbers of cells currently selected
     * */
    fn nSelectedChildren(&self, cellCount: *mut i32) -> HRESULT;

    /**
     * Returns the total number of selected columns.
     * `columnCount` Numbers of columns currently selected
     * */
    fn nSelectedColumns(&self, columnCount: *mut i32) -> HRESULT;

    /**
     * Returns the total number of selected rows.
     * `rowCount` Numbers of rows currently selected
     * */
    fn nSelectedRows(&self, rowCount: *mut i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns the description text of the specified row in the table.
     * `row` The 0 based index of the row for which to retrieve the description.
     * `description` Returns the description text of the specified row in the table if such a description exists.
     * Otherwise, a NULL pointer is returned.
     * retrieval S_FALSE if there is nothing to return, [out] value is NULL
     * retrieval E_INVALIDARG if bad [in] passed, [out] value is NULL
     * */
    fn rowDescription(&self, row: i32, description: *mut BSTR) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns the number of rows occupied by the accessible object at the specified row and column in the table.
     * The result is greater than 1 if the specified cell spans multiple rows.
     * `row` zero-based row index of the accessible for which to return the row extent.
     * `column` zero-based column index of the accessible for which to return the row extent.
     * `nRowsSpanned` Returns the row extent of the specified cell.
     * retrieval E_INVALIDARG if bad [in] passed, [out] value is 0
     * */
    fn rowExtentAt(&self, row: i32, column: i32, nRowsSpanned: *mut i32) -> HRESULT;

    /**
     * Returns the row headers as an %IAccessibleTable object.
     * Content and size of the returned table are implementation-dependent.
     * `accessibleTable` The row header.
     * `startingColumnIndex` The 0 based column index where the header starts, usually 0.
     * retrieval S_FALSE if there is no header, [out] values are NULL and 0 respectively
     * */
    fn rowHeader(
        &self,
        accessibleTable: *mut *mut IAccessibleTable,
        startingColumnIndex: *mut i32,
    ) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Translates the given cell index into a row index.
     * `cellIndex` zero-based index of the cell in the parent or the closest ancestor table.
     * Typically, this is the value returned from IAccessible2::indexInParent.
     * But in the case where the table cells are not direct children of the table, this is the cell index specified by the "table-cell-index" object attribute obtained from parsing the attribute string returned by calling IAccessible2::attributes on the cell object.
     * `rowIndex` zero-based row index
     * retrieval E_INVALIDARG if bad [in] passed, [out] value is 0
     * */
    fn rowIndex(&self, cellIndex: i32, rowIndex: *mut i32) -> HRESULT;

    /**
     * Returns a list of cell indexes currently selected (zero-based).
     * `maxChildren` This parameter is ignored. Refer to @ref _arrayConsideration "Special Consideration when using Arrays" for more details.
     * `children` A cell index's array of selected cells (each index is zero based), allocated by the server.
     * The client must free it with CoTaskMemFree.
     * `nChildren` The number of cell indexes returned; the size of the returned array.
     * retrieval S_FALSE if there are none, [out] values are NULL and 0 respectively
     * */
    fn selectedChildren(
        &self,
        maxChildren: i32,
        children: *mut *mut i32,
        nChildren: *mut i32,
    ) -> HRESULT;

    /**
     * Returns a list of column indexes currently selected (zero-based).
     * `maxColumns` This parameter is ignored. Refer to @ref _arrayConsideration "Special Consideration when using Arrays" for more details.
     * `columns` A column index's array of selected columns (each index is zero based), allocated by the server. The client must free it with CoTaskMemFree.
     * `nColumns` The number of column indexes returned; the size of the returned array.
     * retrieval S_FALSE if there are none, [out] values are NULL and 0 respectively
     * */
    fn selectedColumns(
        &self,
        maxColumns: i32,
        columns: *mut *mut i32,
        nColumns: *mut i32,
    ) -> HRESULT;

    /**
     * Returns a list of row indexes currently selected (zero-based).
     * `maxRows` This parameter is ignored. Refer to @ref _arrayConsideration "Special Consideration when using Arrays" for more details.
     * `rows` A row index's array of selected rows (each index is zero based), allocated by the server. The client must free it with CoTaskMemFree.
     * `nRows` The number of row indexes returned; the size of the returned array.
     * retrieval S_FALSE if there are none, [out] values are NULL and 0 respectively
     * */
    fn selectedRows(&self, maxRows: i32, rows: *mut *mut i32, nRows: *mut i32) -> HRESULT;

    /**
     * Returns the summary description of the table.  The returned object could be an IAccessible or an IAccessible2.
     * `accessible` Returns a reference to an implementation-dependent accessible object representing the table's summary or a NULL pointer if the table does not support a summary.
     * retrieval S_FALSE if there is nothing to return, [out] value is NULL
     * */
    fn summary(&self, accessible: *mut *mut IUnknown) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns a boolean value indicating whether the specified column is completely selected.
     * `column` zero-based index of the column for which to determine whether it is selected.
     * `isSelected` Returns TRUE if the specified column is selected completely and FALSE otherwise.
     * retrieval E_INVALIDARG if bad [in] passed, [out] value is FALSE
     * */
    fn isColumnSelected(&self, column: i32, isSelected: *mut bool) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns a boolean value indicating whether the specified row is completely selected.
     * `row` zero-based index of the row for which to determine whether it is selected.
     * `isSelected` Returns TRUE if the specified row is selected completely and FALSE otherwise.
     * retrieval E_INVALIDARG if bad [in] passed, [out] value is FALSE
     * */
    fn isRowSelected(&self, row: i32, isSelected: *mut bool) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Returns a boolean value indicating whether the specified cell is selected.
     * `row` zero-based index of the row for the cell to determine whether it is selected.
     * `column` zero-based index of the column for the cell to determine whether it is selected.
     * `isSelected` Returns TRUE if the specified cell is selected and FALSE otherwise.
     * retrieval E_INVALIDARG if bad [in] passed, [out] value is FALSE
     * */
    fn isSelected(&self, row: i32, column: i32, isSelected: *mut bool) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Selects a row and unselects all previously selected rows.
     * `row` zero-based index of the row to be selected.
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn selectRow(&self, row: i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Selects a column and unselects all previously selected columns.
     * `column` zero-based index of the column to be selected.
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn selectColumn(&self, column: i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Unselects one row, leaving other selected rows selected (if any).
     * `row` zero-based index of the row to be unselected.
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn unselectRow(&self, row: i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Unselects one column, leaving other selected columns selected (if any).
     * `column` zero-based index of the column to be unselected.
     * retrieval E_INVALIDARG if bad [in] passed
     * */
    fn unselectColumn(&self, column: i32) -> HRESULT;

    //noinspection SpellCheckingInspection
    /**
     * Given a cell index, gets the row and column indexes and extents of a cell and whether it is selected.
     * This is a convenience function.  It is not mandatory to implement it.
     * `index` zero-based index of this cell in the table.
     * `row` zero-based row index.
     * `column` zero-based column index.
     * `rowExtents` Number of cells spanned by this cell in this row.
     * `columnExtents` Number of cells spanned by this cell in this column.
     * `isSelected` Indicates if the specified cell is selected.
     * retrieval E_INVALIDARG if bad [in] passed, [out] values are 0s and FALSE respectively
     * */
    fn rowColumnExtentsAtIndex(
        &self,
        index: i32,
        row: *mut i32,
        column: *mut i32,
        rowExtents: *mut i32,
        columnExtents: *mut i32,
        isSelected: *mut bool,
    ) -> HRESULT;

    /**
     * Returns the type and extents describing how a table changed.
     * Provided for use by the IA2_EVENT_TABLE_MODEL_CHANGED event handler.
     * This data is only guaranteed to be valid while the thread notifying the event continues.
     * Once the handler has returned, the validity of the data depends on how the server manages the life cycle of its objects.
     * Also, note that the server may have different life cycle management strategies for controls depending on whether a control manages its children.
     * Lists, trees, and tables can have a large number of children, and thus it's possible that the child objects for those controls would only be created as needed.
     * Servers should document their life cycle strategy as this will be of interest to assistive technology or script engines accessing data out of process or from other threads. Servers only need to save the most recent row and column values associated with the change, and the scope of the entire application is adequate.
     * `modelChange` A struct of (type (insert, delete, update), firstRow, lastRow, firstColumn, lastColumn).
     * retrieval S_FALSE if there is nothing to return, [out] value is NULL
     * */
    fn modelChange(&self, modelChange: *mut IA2TableModelChange) -> HRESULT;
}

/**
 * Idl file copyright information:
 *  File Name (AccessibleTable.idl)
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
