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

use crate::common::{FARPROC, HOOKPROC, LPARAM};
use std::intrinsics::transmute;

/**
 * 对FARPROC类型的扩展操作。
 * */
pub trait FarProcExt {
    fn to_hook_proc(self) -> HOOKPROC;
}

impl FarProcExt for FARPROC {
    /**
     * 转换到HOOKPROC类型。
     * */
    fn to_hook_proc(self) -> HOOKPROC {
        unsafe { transmute(self) }
    }
}

/**
 * 对LPARAM类型的扩展操作。
 * */
pub trait LParamExt {
    /**
     * 转换到T的引用类型。
     * */
    fn to<T>(&self) -> &T;
}

impl LParamExt for LPARAM {
    fn to<T>(&self) -> &T {
        let ptr = self.0 as *const T;
        unsafe { &*ptr }
    }
}

/**
 * 对字符串的扩展操作。
 * */
pub trait StringExt {
    /**
     * 转换到utf16字符串。
     * */
    fn to_string_utf16(self) -> String;
}

impl StringExt for *const u8 {
    fn to_string_utf16(self) -> String {
        let mut r = self as *const u16;
        let mut s = Vec::new();
        unsafe {
            while !r.is_null() && *r != 0 {
                s.push(*r);
                r = r.wrapping_add(1);
            }
        }
        String::from_utf16_lossy(&s)
    }
}

impl StringExt for *const u16 {
    fn to_string_utf16(self) -> String {
        let mut r = self;
        let mut s = Vec::new();
        unsafe {
            while !r.is_null() && *r != 0 {
                s.push(*r);
                r = r.wrapping_add(1);
            }
        }
        String::from_utf16_lossy(&s)
    }
}

/// Vec的扩展操作。
pub trait VecExt<T> {
    fn to_vec(self) -> Vec<T>;
}
