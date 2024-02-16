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

use std::fmt::{Debug, Formatter};
use windows::Win32::System::Performance::PdhRemoveCounter;
pub use windows::Win32::System::Performance::{PDH_FMT_DOUBLE, PDH_FMT_LARGE, PDH_FMT_LONG};
use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{BOOLEAN, HANDLE},
        System::Performance::{
            PdhAddCounterW, PdhAddEnglishCounterW, PdhCloseQuery, PdhCollectQueryData,
            PdhCollectQueryDataEx, PdhGetCounterInfoW, PdhGetFormattedCounterValue, PdhOpenQueryW,
            PDH_COUNTER_INFO_W, PDH_FMT, PDH_FMT_COUNTERVALUE,
        },
    },
};

//noinspection SpellCheckingInspection
pub const PDH_FMT_NOSCALE: PDH_FMT = PDH_FMT(0x00001000);
pub const PDH_FMT_1000: PDH_FMT = PDH_FMT(0x00002000);
pub const PDH_FMT_NODATA: PDH_FMT = PDH_FMT(0x00004000);
//noinspection SpellCheckingInspection
pub const PDH_FMT_NOCAP100: PDH_FMT = PDH_FMT(0x00008000);
pub trait PdhFmtExt {
    fn add(&self, value: PDH_FMT) -> PDH_FMT;
}
impl PdhFmtExt for PDH_FMT {
    fn add(&self, value: PDH_FMT) -> PDH_FMT {
        PDH_FMT(self.0 | value.0)
    }
}
/**
 * 创建用于管理性能数据收集的新查询。
 * 若要使用数据源的句柄，请使用 pdh_open_query_h 函数。
 * `data_source` 以 Null 结尾的字符串，指定要从中检索性能数据的日志文件的名称。 如果 为 NULL，则从实时数据源收集性能数据。
 * `user_data` 要与此查询关联的用户定义的值。 若要稍后检索用户数据，请调用 pdh_get_counter_info 并访问 PDH_COUNTER_INFO 的 dwQueryUserData 成员。
 * */
pub fn pdh_open_query(data_source: Option<String>, user_data: usize) -> isize {
    unsafe {
        let mut handle = std::mem::zeroed();
        match data_source {
            None => PdhOpenQueryW(None, user_data, &mut handle),
            Some(d) => PdhOpenQueryW(&HSTRING::from(d), user_data, &mut handle),
        };
        handle
    }
}

/**
 * 关闭指定查询中包含的所有计数器，关闭与查询相关的所有句柄，并释放与查询关联的所有内存。
 * `h_query` 要关闭的查询的句柄。 此句柄由 pdh_open_query 函数返回。
 * */
pub fn pdh_close_query(h_query: isize) {
    unsafe { PdhCloseQuery(h_query) };
}

/**
 * 检索有关计数器的信息，例如数据大小、计数器类型、路径和用户提供的数据值。
 * `h_counter` 要从中检索信息的计数器的句柄。 pdh_add_counter 函数返回此句柄。
 * `retrieve_explain_text` 确定是否检索说明文本。 如果将此参数设置为 TRUE，则会检索计数器的说明文本。 如果将此参数设置为 FALSE，则返回的缓冲区中的字段为 NULL。
 * */
pub fn pdh_get_counter_info(
    h_counter: isize,
    retrieve_explain_text: bool,
) -> Vec<PDH_COUNTER_INFO_W> {
    unsafe {
        let mut size = std::mem::zeroed();
        PdhGetCounterInfoW(
            h_counter,
            BOOLEAN::from(retrieve_explain_text),
            &mut size,
            None,
        );
        let mut v = vec![];
        for _ in 0..size {
            v.push(PDH_COUNTER_INFO_W::default());
        }
        PdhGetCounterInfoW(
            h_counter,
            BOOLEAN::from(retrieve_explain_text),
            &mut size,
            Some(v.as_mut_ptr()),
        );
        v
    }
}

/**
 * 将指定的计数器添加到查询。
 * `h_query` 要向其添加计数器的查询的句柄。 此句柄由 pdh_open_query 函数返回。
 * `full_counter_path` 包含计数器路径的以 Null 结尾的字符串。 有关计数器路径格式的详细信息，请参阅指定计数器路径。 计数器路径的最大长度为PDH_MAX_COUNTER_PATH。
 * `user_data` 用户定义的值。 此值将成为计数器信息的一部分。 若要稍后检索此值，请调用 pdh_get_counter_info 函数并访问 PDH_COUNTER_INFO 结构的 dwUserData 成员。
 * */
pub fn pdh_add_counter(h_query: isize, full_counter_path: String, user_data: usize) -> isize {
    unsafe {
        let mut handle = std::mem::zeroed();
        PdhAddCounterW(
            h_query,
            &HSTRING::from(full_counter_path),
            user_data,
            &mut handle,
        );
        handle
    }
}

/**
 * 将指定的非特定语言计数器添加到查询。
 * `h_query` 要向其添加计数器的查询的句柄。 此句柄由 pdh_open_query 函数返回。
 * `full_counter_path` 包含计数器路径的以 Null 结尾的字符串。 有关计数器路径格式的详细信息，请参阅指定计数器路径。 计数器路径的最大长度为PDH_MAX_COUNTER_PATH。
 * `user_data` 用户定义的值。 此值将成为计数器信息的一部分。 若要稍后检索此值，请调用 pdh_get_counter_info 函数并访问 PDH_COUNTER_INFO 结构的 dwUserData 成员。
 * */
pub fn pdh_add_english_counter(
    h_query: isize,
    full_counter_path: String,
    user_data: usize,
) -> isize {
    unsafe {
        let mut handle = std::mem::zeroed();
        PdhAddEnglishCounterW(
            h_query,
            &HSTRING::from(full_counter_path),
            user_data,
            &mut handle,
        );
        handle
    }
}

/**
 * 使用单独的线程收集指定查询中所有计数器的当前原始数据值。 然后，函数向应用程序定义的事件发出信号，并在返回之前等待指定的时间间隔。
 * `h_query` 查询的句柄。 查询标识要收集的计数器。 pdh_open_query 函数返回此句柄。
 * `interval_time` 等待的时间间隔（以秒为单位）。
 * `h_new_data_event` 希望 PDH 在时间间隔过期后发出信号的事件的句柄。 若要创建事件对象，请调用 create_event 函数。
 * */
pub fn pdh_collect_query_data_ex(h_query: isize, interval_time: u32, h_new_data_event: HANDLE) {
    unsafe {
        PdhCollectQueryDataEx(h_query, interval_time, h_new_data_event);
    }
}

/**
 * 收集指定查询中所有计数器的当前原始数据值，并更新每个计数器的状态代码。
 * */
pub fn pdh_collect_query_data(h_query: isize) {
    unsafe {
        PdhCollectQueryData(h_query);
    }
}

/**
 * 计算指定计数器的可显示值。
 * `h_counter` 要计算其可显示值的计数器的句柄。 PdhAddCounter 函数返回此句柄。
 * `format` 确定格式化值的数据类型。 指定以下值之一。
 * - PDH_FMT_DOUBLE 以双精度浮点实数的形式返回数据。
 * - PDH_FMT_LARGE 以 64 位整数的形式返回数据。
 * - PDH_FMT_LONG 以长整数的形式返回数据。
 * */
pub fn pdh_get_formatted_counter_value(
    h_counter: isize,
    r#format: PDH_FMT,
) -> (u32, PDH_FMT_COUNTERVALUE) {
    unsafe {
        let mut r#type = std::mem::zeroed();
        let mut value = std::mem::zeroed();
        PdhGetFormattedCounterValue(h_counter, r#format, Some(&mut r#type), &mut value);
        (r#type, value)
    }
}

/**
 * 从查询中删除计数器。
 * `h_counter` 要从其查询中删除的计数器的句柄。 pdh_add_counter 函数返回此句柄。
 * */
pub fn pdh_remove_counter(h_counter: isize) {
    unsafe {
        PdhRemoveCounter(h_counter);
    }
}

pub struct PdhCounter(isize);
impl Debug for PdhCounter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (r, v) = self.get_value();
        write!(f, "PdhCounter({}, {})", r, v)
    }
}

impl Drop for PdhCounter {
    fn drop(&mut self) {
        pdh_remove_counter(self.0)
    }
}

#[derive(Debug)]
pub struct PdhQuery(isize);
impl PdhQuery {
    /**
     * 创建一个性能数据查询器。
     * */
    pub fn new() -> Self {
        Self(pdh_open_query(None, 0))
    }

    /**
     * 将指定的计数器添加到查询。
     * `full_counter_path` 包含计数器路径的字符串。 有关计数器路径格式的详细信息，请参阅指定计数器路径。
     * */
    pub fn add_counter(&self, full_counter_path: String) -> PdhCounter {
        PdhCounter(pdh_add_counter(self.0, full_counter_path, 0))
    }

    /**
     * 将指定的非特定语言计数器添加到查询。
     * `full_counter_path` 包含计数器路径的字符串。 有关计数器路径格式的详细信息，请参阅指定计数器路径。
     * */
    pub fn add_english_counter(&self, full_counter_path: String) -> PdhCounter {
        PdhCounter(pdh_add_english_counter(self.0, full_counter_path, 0))
    }

    /// 收集指定查询中所有计数器的当前原始数据值，并更新每个计数器的状态代码。
    pub fn collect_data(&self) -> &Self {
        pdh_collect_query_data(self.0);
        self
    }
}

impl Drop for PdhQuery {
    fn drop(&mut self) {
        pdh_close_query(self.0)
    }
}

pub trait PdhCounterExt {
    fn get_value(&self) -> (u32, f64);
}

impl PdhCounterExt for PdhCounter {
    /**
     * 查询指定计数器的可显示值。
     * */
    fn get_value(&self) -> (u32, f64) {
        let (t, v) = pdh_get_formatted_counter_value(self.0, PDH_FMT_DOUBLE);
        unsafe { (t, v.Anonymous.doubleValue) }
    }
}

#[cfg(test)]
mod test_pdh {
    use crate::pdh::PdhQuery;

    #[test]
    fn main() {
        let pdh = PdhQuery::new();
        let counter = pdh.add_counter(format!(
            r"\Processor Information({})\% Processor Time",
            "_Total"
        ));
        for _ in 0..10 {
            pdh.collect_data();
            dbg!(&counter);
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
        dbg!(pdh);
    }
}
