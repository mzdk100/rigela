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

use crate::{
    jab::context::AccessibleContext,
    JabLib::{
        JabLib,
        packages::{AccessibleContext as AC, AccessibleTableCellInfo, AccessibleTableInfo},
    },
};

#[derive(Debug)]
pub struct AccessibleTable<'lib> {
    _lib: &'lib JabLib,
    _vm_id: i32,
    _table: AccessibleTableInfo,
    _caption: AccessibleContext<'lib>,
    _summary: AccessibleContext<'lib>,
}

impl<'lib> AccessibleTable<'lib> {
    /**
     * 创建一个实例。
     * `lib` 库引用。
     * `vm_id` 虚拟机ID。
     * `ac` 原始上下文对象。
     * */
    pub(crate) fn new(lib: &'lib JabLib, vm_id: i32, ac: AC) -> Option<Self> {
        let Some(table) = lib.get_accessible_table_info(vm_id, ac) else {
            return None;
        };
        return Some(Self {
            _lib: &lib,
            _vm_id: vm_id,
            _table: table.clone(),
            _caption: AccessibleContext::new(&lib, vm_id, table.caption),
            _summary: AccessibleContext::new(&lib, vm_id, table.summary),
        });
    }

    /**
     * 获取标题对象。
     * */
    pub fn get_caption(&self) -> &AccessibleContext {
        &self._caption
    }

    /**
     * 获取摘要对象。
     * */
    pub fn get_summary(&self) -> &AccessibleContext {
        &self._summary
    }

    /**
     * 返回指定表中指定列的说明。列说明符是从零开始的。
     * `column` 列索引。
     * */
    pub fn get_column_description(&self, column: i32) -> Option<AccessibleContext<'lib>> {
        if let Some(ac) = self._lib.get_accessible_table_column_description(self._vm_id, self._table.accessibleContext, column) {
            return Some(AccessibleContext::new(self._lib, self._vm_id, ac));
        }
        None
    }

    /**
     * 返回指定表中指定行的描述。行说明符是从零开始的。
     * `row` 行索引。
     * */
    pub fn get_row_description(&self, row: i32) -> Option<AccessibleContext<'lib>> {
        if let Some(ac) = self._lib.get_accessible_table_row_description(self._vm_id, self._table.accessibleContext, row) {
            return Some(AccessibleContext::new(self._lib, self._vm_id, ac));
        }
        None
    }

    /**
     * 将指定表格的表行标题作为表格对象返回。
     * */
    pub fn get_row_header(&self) -> Option<AccessibleTable<'lib>> {
        let Some(info) = self._lib.get_accessible_table_row_header(self._vm_id, self._table.accessibleContext) else {
            return None;
        };
        Some(Self {
            _lib: self._lib,
            _vm_id: self._vm_id,
            _table: info.clone(),
            _caption: AccessibleContext::new(self._lib, self._vm_id, info.caption),
            _summary: AccessibleContext::new(self._lib, self._vm_id, info.summary),
        })
    }

    /**
     * 将指定表格的表列标题作为表格对象返回。
     * */
    pub fn get_column_header(&self) -> Option<AccessibleTable<'lib>> {
        let Some(info) = self._lib.get_accessible_table_column_header(self._vm_id, self._table.accessibleContext) else {
            return None;
        };
        Some(Self {
            _lib: self._lib,
            _vm_id: self._vm_id,
            _table: info.clone(),
            _caption: AccessibleContext::new(self._lib, self._vm_id, info.caption),
            _summary: AccessibleContext::new(self._lib, self._vm_id, info.summary),
        })
    }

    /**
     * 判断表格某行是否被选中。如果选择了指定的从零开始的行，则返回true。
     * `row` 行索引。
     * */
    pub fn is_row_selected(&self, row: i32) -> bool {
        self._lib.is_accessible_table_row_selected(self._vm_id, self._table.accessibleTable, row)
    }

    /**
     * 判断表格某列是否被选中。如果选择了指定的从零开始的行，则返回true。
     * `column` 列索引。
     * */
    pub fn is_column_selected(&self, column: i32) -> bool {
        self._lib.is_accessible_table_column_selected(self._vm_id, self._table.accessibleTable, column)
    }

    /**
     * 返回有关指定表单元格的信息。行和列说明符是从零开始的。
     * `row` 行索引。
     * `column` 列索引。
     * */
    pub fn get_cell(&self, row: i32, column: i32) -> Option<AccessibleTableCell> {
        let Some(info) = self._lib.get_accessible_table_cell_info(self._vm_id, self._table.accessibleTable, row, column) else {
            return None;
        };
        Some(AccessibleTableCell { _lib: &self._lib, _vm_id: self._vm_id, _info: info })
    }

    /**
     * 返回指定单元格索引处单元格的列编号。这些值以零为基础。
     * `index` 索引。
     * */
    pub fn get_column(&self, index: i32) -> i32 {
        self._lib.get_accessible_table_column(self._vm_id, self._table.accessibleTable, index)
    }

    /**
     * 返回指定单元格索引处单元格的行号。这些值以零为基础。
     * `index` 索引。
     * */
    pub fn get_row(&self, index: i32) -> i32 {
        self._lib.get_accessible_table_row(self._vm_id, self._table.accessibleTable, index)
    }

    /**
     * 返回表中选定的列数。
     * */
    pub fn get_column_selection_count(&self) -> i32 {
        self._lib.get_accessible_table_column_selection_count(self._vm_id, self._table.accessibleTable)
    }

    /**
     * 返回表中选定的行数。
     * */
    pub fn get_row_selection_count(&self) -> i32 {
        self._lib.get_accessible_table_row_selection_count(self._vm_id, self._table.accessibleTable)
    }

    /**
     * 返回表中指定行和列偏移量的索引。这些值以零为基础。
     * `row` 行索引。
     * `column` 列索引。
     * */
    pub fn get_index(&self, row: i32, column: i32) -> i32 {
        self._lib.get_accessible_table_index(self._vm_id, self._table.accessibleTable, row, column)
    }

    /**
     * 返回所选列的从零开始的索引数组。
     * `count` 数组长度。
     * */
    pub fn get_column_selections(&self, count: i32) -> Vec<i32> {
        if let Some(v) = self._lib.get_accessible_table_column_selections(self._vm_id, self._table.accessibleTable, count) {
            return v;
        }
        vec![]
    }

    /**
     * 返回所选行的从零开始的索引数组。
     * `count` 数组长度。
     * */
    pub fn get_row_selections(&self, count: i32) -> Vec<i32> {
        if let Some(v) = self._lib.get_accessible_table_row_selections(self._vm_id, self._table.accessibleTable, count) {
            return v;
        }
        vec![]
    }
}

impl<'lib> Drop for AccessibleTable<'lib> {
    fn drop(&mut self) {
        self._lib.release_java_object(self._vm_id, self._table.accessibleContext);
    }
}

#[derive(Debug)]
pub struct AccessibleTableCell<'lib> {
    _lib: &'lib JabLib,
    _vm_id: i32,
    _info: AccessibleTableCellInfo,
}

impl<'lib> AccessibleTableCell<'lib> {
    /**
     * 获取上下文对象。
     * */
    pub fn get_context(&self) -> AccessibleContext<'lib> {
        AccessibleContext::new(self._lib, self._vm_id, self._info.accessibleContext)
    }

    /**
     * 获取单元格索引。
     * */
    pub fn get_index(&self) -> i32 {
        self._info.index
    }

    /**
     * 获取单元格列索引。
     * */
    pub fn get_column(&self) -> i32 {
        self._info.column
    }

    /**
     * 获取单元格行索引。
     * */
    pub fn get_row(&self) -> i32 {
        self._info.row
    }

    /**
     * 获取单元格行跨度。
     * */
    pub fn get_row_extent(&self) -> i32 {
        self._info.rowExtent
    }

    /**
     * 获取单元格列跨度。
     * */
    pub fn get_column_extent(&self) -> i32 {
        self._info.columnExtent
    }

    /**
     * 判断单元格是否被选中。
     * */
    pub fn is_selected(&self) -> bool {
        self._info.isSelected != 0
    }
}

impl<'lib> Drop for AccessibleTableCell<'lib> {
    fn drop(&mut self) {
        self._lib.release_java_object(self._vm_id, self._info.accessibleContext)
    }
}