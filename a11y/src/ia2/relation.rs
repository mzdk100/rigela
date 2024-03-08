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
    ia2::object::Accessible2Object,
    IAccessible2Lib::AccessibleRelation::IAccessibleRelation,
};
use std::ffi::c_void;
use windows::core::Type;

pub struct AccessibleRelation(IAccessibleRelation);

impl AccessibleRelation {
    pub(crate) fn from_raw(raw: &IAccessibleRelation) -> Self {
        Self(raw.clone())
    }

    /**
     * 返回关系的类型。
     * */
    pub fn relation_type(&self) -> String {
        unsafe {
            let mut res = std::mem::zeroed();
            if self.0.relationType(&mut res).is_err() {
                return String::new();
            }
            res.to_string()
        }
    }

    /**
     * 返回关系类型的本地化版本。
     * */
    pub fn localized_relation_type(&self) -> String {
        unsafe {
            let mut res = std::mem::zeroed();
            if self.0.localizedRelationType(&mut res).is_err() {
                return String::new();
            }
            res.to_string()
        }
    }

    /**
     * 返回此关系的目标数。
     * */
    pub fn n_targets(&self) -> i32 {
        unsafe {
            let mut num = std::mem::zeroed();
            if self.0.nTargets(&mut num).is_err() {
                return 0;
            }
            num
        }
    }

    /**
     * 返回一个可访问的关系目标。
     * `target_index` 从零开始的索引
     * */
    pub fn target(&self, target_index: i32) -> Option<Accessible2Object> {
        unsafe {
            let mut target = std::mem::zeroed();
            if self.0.target(target_index, &mut target).is_err() {
                return None;
            }
            match Accessible2Object::from_raw(&Type::from_abi(target as *mut c_void).unwrap()) {
                Ok(x) => Some(x),
                Err(_) => None
            }
        }
    }

    /**
     * 返回多个可访问的关系目标
     * `max_targets`客户端分配的数组的最大大小
     * */
    pub fn targets(&self, max_targets: i32) -> Vec<Accessible2Object> {
        unsafe {
            let mut targets = std::mem::zeroed();
            let mut num = 0;
            if self.0.targets(max_targets, &mut targets, &mut num).is_err() {
                return vec![];
            }
            let mut v = vec![];
            for i in 0..num {
                let Ok(o) = Accessible2Object::from_raw(
                    &Type::from_abi(targets.wrapping_add(i as usize) as *mut c_void).unwrap(),
                ) else {
                    continue;
                };
                v.push(o);
            }
            v
        }
    }
}
