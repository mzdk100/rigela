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

use crate::IAccessible2Lib::{
    Accessible2::IAccessible2, Accessible2_2::IAccessible2_2, Accessible2_3::IA2Range,
    Accessible2_3::IAccessible2_3,
};
use std::ffi::c_void;
use std::fmt::{Debug, Formatter};
use win_wrap::msaa::object::AccessibleObject;
use windows::{
    core::IUnknown,
    core::{ComInterface, BSTR},
    Win32::{
        System::{
            Com::{CoTaskMemFree, IServiceProvider},
            Variant::{VariantClear, VariantInit, VARIANT, VT_BOOL, VT_BSTR, VT_I4},
        },
        UI::Accessibility::IAccessible,
    },
};

pub struct Accessible2Object {
    _ia2: Option<IAccessible2>,
    _ia2_2: Option<IAccessible2_2>,
    _ia2_3: Option<IAccessible2_3>,
}

impl Accessible2Object {
    pub fn from_accessible_object(obj: AccessibleObject) -> Option<Self> {
        if let Ok(sp) = obj.get_raw().cast::<IServiceProvider>() {
            let ia2 = match unsafe { sp.QueryService::<IAccessible2_3>(&IAccessible::IID) } {
                Err(_) => None,
                Ok(x) => Some(x),
            };
            if !ia2.is_none() {
                return Some(Self {
                    _ia2: None,
                    _ia2_2: None,
                    _ia2_3: ia2,
                });
            }
            let ia2 = match unsafe { sp.QueryService::<IAccessible2_2>(&IAccessible::IID) } {
                Err(_) => None,
                Ok(x) => Some(x),
            };
            if !ia2.is_none() {
                return Some(Self {
                    _ia2: None,
                    _ia2_3: None,
                    _ia2_2: ia2,
                });
            }
            let ia2 = match unsafe { sp.QueryService::<IAccessible2>(&IAccessible::IID) } {
                Err(_) => None,
                Ok(x) => Some(x),
            };
            if !ia2.is_none() {
                return Some(Self {
                    _ia2: ia2,
                    _ia2_2: None,
                    _ia2_3: None,
                });
            }
        }
        None
    }

    /**
     * 返回可访问的中的选择的范围数组。
     * */
    pub fn selection_ranges(&self) -> Vec<IA2Range> {
        if self._ia2_3.is_none() {
            return vec![];
        }
        let ia2 = self._ia2_3.as_ref().unwrap();
        unsafe {
            let mut ranges = std::mem::zeroed();
            let mut num = 0;
            if ia2.selectionRanges(&mut ranges, &mut num).is_err() {
                return vec![];
            }
            let mut vec = vec![];
            for i in 0..num {
                vec.push(*ranges.wrapping_add(i as usize));
            }
            CoTaskMemFree(Some(ranges as *const c_void));
            vec
        }
    }

    /**
     * 获取对象特定的属性值。
     * `name` 属性名称。
     * */
    pub fn attribute(&self, name: &str) -> IA2ResultType {
        if self._ia2_2.is_none() {
            return IA2ResultType::None;
        }
        let ia2 = self._ia2_2.as_ref().unwrap();
        unsafe {
            let mut out = VariantInit();
            if ia2.attribute(BSTR::from(name), &mut out).is_err() {
                VariantClear(&mut out).unwrap_or(());
                return IA2ResultType::None;
            }
            out.into()
        }
    }

    /**
     * 返回此对象子树中可访问的最深超文本，以及其中的插入符号偏移量。
     * */
    pub fn accessible_with_caret(&self) -> Option<(IUnknown, i32)> {
        if self._ia2_2.is_none() {
            return None;
        }
        let ia2 = self._ia2_2.as_ref().unwrap();
        unsafe {
            let mut acc = std::mem::zeroed();
            let mut caret = 0;
            let val = ia2
                .accessibleWithCaret(&mut acc, &mut caret)
                .from_abi(acc as *mut c_void);
            if val.is_err() {
                return None;
            }
            Some((val.unwrap(), caret))
        }
    }

    /**
     * 返回指定目标类型的关系目标。
     * `type` 请求的@ref grpRelations“关系类型”。
     * `max_targets`请求的目标数。零表示应返回所有目标。
     * */
    pub fn relation_targets_of_type(&self, r#type: &str, max_targets: i32) -> Vec<IUnknown> {
        if self._ia2_2.is_none() {
            return vec![];
        }
        let ia2 = self._ia2_2.as_ref().unwrap();
        unsafe {
            let mut targets = std::mem::zeroed();
            let mut num = 0;
            let res =
                ia2.relationTargetsOfType(BSTR::from(r#type), max_targets, &mut targets, &mut num);
            if res.is_err() {
                return vec![];
            }
            let mut v = Vec::new();
            for i in 0..num {
                let it = *targets.wrapping_add(i as usize);
                v.push(res.from_abi(it as *mut c_void).unwrap());
            }
            CoTaskMemFree(Some(targets as *const c_void));
            v
        }
    }

    /**
     * 返回此对象的可访问关系数。
     * */
    pub fn n_relations(&self) -> i32 {
        if self._ia2.is_none() {
            return 0;
        }
        let ia2 = self._ia2.as_ref().unwrap();
        unsafe {
            let mut num = 0;
            if ia2.nRelations(&mut num).is_err() {
                return 0;
            }
            num
        }
    }
}

impl Debug for Accessible2Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Accessible2Object(n_relations:{})", self.n_relations())
    }
}

pub enum IA2ResultType {
    Str(String),
    Num(i32),
    Bool(bool),
    None,
}

impl From<VARIANT> for IA2ResultType {
    fn from(mut value: VARIANT) -> Self {
        unsafe {
            let v = match value.Anonymous.Anonymous.vt {
                VT_I4 => Self::Num(value.Anonymous.Anonymous.Anonymous.intVal),
                VT_BSTR => Self::Str(value.Anonymous.Anonymous.Anonymous.bstrVal.to_string()),
                VT_BOOL => Self::Bool(value.Anonymous.Anonymous.Anonymous.boolVal.as_bool()),
                _ => Self::None,
            };
            VariantClear(&mut value).unwrap_or(());
            v
        }
    }
}
