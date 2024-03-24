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

pub use crate::IAccessible2Lib::{
    Accessible2::IA2Locale,
    AccessibleStates::AccessibleStates,
    IA2CommonTypes::{IA2CoordinateType, IA2ScrollType},
};
use crate::{
    ia2::relation::AccessibleRelation,
    IAccessible2Lib::{
        Accessible2::IAccessible2, Accessible2_2::IAccessible2_2, Accessible2_3::IA2Range,
        Accessible2_3::IAccessible2_3,
    },
};
use std::{
    ffi::c_void,
    fmt::{Debug, Formatter},
};
use win_wrap::msaa::object::AccessibleObject;
use windows::{
    core::{Error, IUnknown, Interface, Result, Type, BSTR, VARIANT},
    Win32::{
        Foundation::{HWND, S_FALSE},
        System::Com::{CoTaskMemFree, IServiceProvider},
        UI::Accessibility::IAccessible,
    },
};

pub struct Accessible2Object {
    _ia2: IAccessible2,
    _ia2_2: Option<IAccessible2_2>,
    _ia2_3: Option<IAccessible2_3>,
}

impl Accessible2Object {
    pub fn from_accessible_object(obj: AccessibleObject) -> Result<Self> {
        if let Ok(sp) = obj.get_raw().cast::<IServiceProvider>() {
            let ia2 = match unsafe { sp.QueryService::<IAccessible2>(&IAccessible::IID) } {
                Err(e) => return Err(e),
                Ok(x) => x,
            };
            let ia2_3 = match unsafe { sp.QueryService::<IAccessible2_3>(&IAccessible::IID) } {
                Err(_) => None,
                Ok(x) => Some(x),
            };
            let ia2_2 = match unsafe { sp.QueryService::<IAccessible2_2>(&IAccessible::IID) } {
                Err(_) => None,
                Ok(x) => Some(x),
            };
            return Ok(Self {
                _ia2: ia2,
                _ia2_2: ia2_2,
                _ia2_3: ia2_3,
            });
        }
        Err(Error::new(S_FALSE, "Not supported."))
    }

    pub(crate) fn from_raw(obj: &IUnknown) -> Result<Self> {
        let ia2 = match obj.cast::<IAccessible2>() {
            Err(e) => return Err(e),
            Ok(x) => x,
        };
        let ia2_3 = match obj.cast::<IAccessible2_3>() {
            Err(_) => None,
            Ok(x) => Some(x),
        };
        let ia2_2 = match obj.cast::<IAccessible2_2>() {
            Err(_) => None,
            Ok(x) => Some(x),
        };
        return Ok(Self {
            _ia2: ia2,
            _ia2_2: ia2_2,
            _ia2_3: ia2_3,
        });
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
            let mut out = std::mem::zeroed();
            if ia2.attribute(BSTR::from(name), &mut out).is_err() {
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
                .and_then(|| Type::from_abi(acc as *mut c_void));
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
            if ia2
                .relationTargetsOfType(BSTR::from(r#type), max_targets, &mut targets, &mut num)
                .is_err()
            {
                return vec![];
            }
            let mut v = Vec::new();
            for i in 0..num {
                let it = *targets.wrapping_add(i as usize);
                v.push(Type::from_abi(it as *mut c_void).unwrap());
            }
            CoTaskMemFree(Some(targets as *const c_void));
            v
        }
    }

    /**
     * 返回此对象的可访问关系数。
     * */
    pub fn n_relations(&self) -> i32 {
        unsafe {
            let mut num = 0;
            if self._ia2.nRelations(&mut num).is_err() {
                return 0;
            }
            num
        }
    }

    //noinspection StructuralWrap
    /**
     * 返回此对象的一个可访问关系。
     * `index` 从0开始的关系对象索引。
     * */
    pub fn relation(&self, index: i32) -> Result<AccessibleRelation> {
        unsafe {
            let mut relation = std::mem::zeroed();
            if let Ok(relation) = self
                ._ia2
                .relation(index, &mut relation)
                .and_then(|| Type::from_abi(relation as *mut c_void))
            {
                Ok(AccessibleRelation::from_raw(&relation))
            } else {
                Err(Error::new(S_FALSE, "Not supported."))
            }
        }
    }

    //noinspection StructuralWrap
    /**
     * 返回此对象的多个可访问关系。
     * `max_relations` 要获取关系对象的数量。
     * */
    pub fn relations(&self, max_relations: i32) -> Vec<AccessibleRelation> {
        unsafe {
            let mut relations = std::mem::zeroed();
            let mut num = 0;
            if self
                ._ia2
                .relations(max_relations, &mut relations, &mut num)
                .is_err()
            {
                return vec![];
            }
            let mut v = vec![];
            for i in 0..num {
                v.push(AccessibleRelation::from_raw(
                    &Type::from_abi(relations.wrapping_add(i as usize) as *mut c_void).unwrap(),
                ));
            }
            v
        }
    }

    //noinspection SpellCheckingInspection
    /**
     * 返回%IAccessible2对象的角色。
     * 为了方便起见，MSAA角色也通过此方法传递，因此AT不必也通过MSAA的get_role获取角色。
     * %IAccessible2角色不应通过MSAA的get_accRole传递。
     * 为了与未启用IAccessible2的AT兼容，IAccessible 2应用程序还应添加对get_accRole的支持，以返回最接近的MSAA角色或role_SYSTEM_CLIENT（MSAA定义的默认角色）（如果不匹配）。
     * 此方法在IDL中缺少[propget]前缀。结果是该方法在生成的C++代码中被命名为role，而不是get_role。
     * */
    pub fn role(&self) -> i32 {
        unsafe {
            let mut role = std::mem::zeroed();
            self._ia2.role(&mut role).and_then(|| Type::from_abi(role))
        }
        .unwrap_or(0)
    }

    //noinspection StructuralWrap
    /**
     * 使对象在屏幕上可见。
     * `scroll_type` 定义对象应放在屏幕上的位置。
     * */
    pub fn scroll_to(&self, scroll_type: IA2ScrollType) -> bool {
        unsafe { self._ia2.scrollTo(scroll_type).is_ok() }
    }

    /**
     * 将对象的左上角移动到指定位置。
     * `coordinate_type` 指定坐标是相对于屏幕还是相对于父对象。
     * `x` 定义x坐标。
     * `y` 定义y坐标。*/
    pub fn scroll_to_point(&self, coordinate_type: IA2CoordinateType, x: i32, y: i32) -> bool {
        unsafe { self._ia2.scrollToPoint(coordinate_type, x, y).is_ok() }
    }

    //noinspection StructuralWrap
    /**
     * 返回分组信息(group_level, similar_items_in_group, position_in_group)。
     * 用于树项目、列表项目、选项卡面板标签、单选按钮等。也用于非文本对象的集合。
     * `group_level`从1开始，0表示此值不适用。
     * `similar_items_in_group` 从1开始，0表示此值不适用。
     * `position_in_group` 从1开始，0表示此值不适用。这是对当前组中对象的索引，而不是对同一组级别的所有对象的索引。如果至少有一个值有效。
     * 这个方法是用来描述对象包含结构的性质。它由树、树网格、嵌套列表、嵌套菜单公开，但不由使用级别对象属性的标题公开。它也通过单选按钮（group_level==0）显示。
     * 一般的，这通常不会在组合框上实现，以描述其内容的性质。
     * */
    pub fn group_position(&self) -> (i32, i32, i32) {
        unsafe {
            let (mut group_level, mut similar_items_in_group, mut position_in_group) =
                std::mem::zeroed();
            if self
                ._ia2
                .groupPosition(
                    &mut group_level,
                    &mut similar_items_in_group,
                    &mut position_in_group,
                )
                .is_err()
            {
                return (0, 0, 0);
            }
            (group_level, similar_items_in_group, position_in_group)
        }
    }

    /**
     * 返回包含任何IAccessible2状态的位带。
     * IAccessible2状态是MSAA状态之外的状态，在IA2States枚举中定义。
     * */
    pub fn states(&self) -> Result<AccessibleStates> {
        unsafe {
            let mut states = std::mem::zeroed();
            self._ia2
                .states(&mut states)
                .and_then(|| Type::from_abi(states))
        }
    }

    /**
     * 返回扩展角色。
     * 扩展角色是由应用程序动态生成的角色。
     * 它不是由%IAccessible2规范预定义的。
     * */
    pub fn extended_role(&self) -> Result<String> {
        unsafe {
            let mut role = std::mem::zeroed();
            let res = self._ia2.extendedRole(&mut role);
            if res.is_err() {
                return Err(Error::new(S_FALSE, res.message()));
            }
            Ok(role.to_string())
        }
    }

    /**
     * 返回本地化的扩展角色。
     * */
    pub fn localized_extended_role(&self) -> Result<String> {
        unsafe {
            let mut role = std::mem::zeroed();
            let res = self._ia2.localizedExtendedRole(&mut role);
            if res.is_err() {
                return Err(Error::new(S_FALSE, res.message()));
            }
            Ok(role.to_string())
        }
    }

    /**
     * 返回扩展状态的数目。
     * */
    pub fn n_extended_states(&self) -> Result<i32> {
        unsafe {
            let mut states = std::mem::zeroed();
            self._ia2
                .nExtendedStates(&mut states)
                .and_then(|| Type::from_abi(states))
        }
    }

    /**
     * 返回扩展状态（字符串数组）。扩展状态是由应用程序动态生成的状态。它不是由%IAccessible2规范预定义的。
     * */
    pub fn extended_states(&self) -> Vec<String> {
        unsafe {
            let (mut states, mut num) = std::mem::zeroed();
            let res = self._ia2.extendedStates(0, &mut states, &mut num);
            if res.is_err() {
                return vec![];
            }
            let mut v = vec![];
            for i in 0..num {
                v.push(&*states.wrapping_add(i as usize));
            }
            let v = v.iter().map(|i| i.to_string()).collect();
            CoTaskMemFree(Some(states as *const c_void));
            v
        }
    }

    /**
     * 返回本地化的扩展状态（字符串数组）。
     * */
    pub fn localized_extended_states(&self) -> Vec<String> {
        unsafe {
            let (mut states, mut num) = std::mem::zeroed();
            let res = self._ia2.localizedExtendedStates(0, &mut states, &mut num);
            if res.is_err() {
                return vec![];
            }
            let mut v = vec![];
            for i in 0..num {
                v.push(&*states.wrapping_add(i as usize));
            }
            let v = v.iter().map(|i| i.to_string()).collect();
            CoTaskMemFree(Some(states as *const c_void));
            v
        }
    }

    /**
     * 返回唯一的ID。
     * uniqueID是此对象的标识符，在当前窗口中是唯一的，并且在可访问对象的生存期内保持不变。
     * uniqueID与以下内容无关：
     * - MSAA对象ID，服务器使用它来消除每个HWND的IAccessibles之间的歧义，或
     * - MSAA childID，用于消除由IAccessible管理的子级之间的歧义。
     * 提供该值是为了使AT即使在不处理对象的事件时也可以访问唯一的运行时持久标识符。
     * 这个值何时有用的一个例子是，如果AT想要建立一个缓存。AT除了缓存其他数据之外，还可以缓存uniqueID。
     * 当事件被触发时，AT可以将uniqueId映射到其内部模型。
     * 因此，如果存在REORDER/SHOW/HIDE事件，AT就会知道内部结构的哪一部分已经失效，并且可以只重新蚀刻该部分。
     * AT也可以使用该值来确定当前控件是否已经改变。如果选项卡顺序中相邻的两个控件的角色相同，则可用于检测新控件。
     * AT对该值的另一个使用是识别分组对象何时改变，例如，何时从一个组中的单选按钮移动到不同组中的按钮。
     * 实现这一点的一种方法是创建一个具有32位数字生成器和重用池的工厂。数字生成器将发出从1开始的数字。每次对象的生命周期结束时，其编号都会保存到重用池中。只要重用池为空，就会使用数字生成器。
     * 创建唯一ID的另一种方法是根据指针值（例如对象的地址）生成唯一ID。这将是唯一的，因为没有两个活动对象可以使用相同的分配内存空间。
     * */
    pub fn unique_id(&self) -> Result<i32> {
        unsafe {
            let mut id = std::mem::zeroed();
            self._ia2.uniqueID(&mut id).and_then(|| Type::from_abi(id))
        }
    }

    //noinspection SpellCheckingInspection
    /**
     * 辅助技术（AT）将不得不在每个IAccessible上使用WindowFromAccessibleObject。
     * 它很慢，因为它涉及一个循环，该循环向上爬祖先链并搜索ROLE_WINDOW对象，将其映射回窗口句柄。这是由oleacc.dll实现的。
     * 然而，有了windowHandle的可用性，可以避免这个过程。
     * */
    pub fn window_handle(&self) -> HWND {
        unsafe {
            let mut h_wnd = std::mem::zeroed();
            self._ia2
                .windowHandle(&mut h_wnd)
                .and_then(|| Type::from_abi(h_wnd))
                .unwrap()
        }
    }

    /**
     * 返回此对象在其父对象中的索引。
     * 从0开始，-1表示没有父级；
     * */
    pub fn index_in_parent(&self) -> i32 {
        unsafe {
            let mut index = std::mem::zeroed();
            let res = self
                ._ia2
                .indexInParent(&mut index)
                .and_then(|| Type::from_abi(index));
            if res.is_err() {
                -1
            } else {
                res.unwrap()
            }
        }
    }

    /**
     * 返回可访问对象的IA2Locale。
     * */
    pub fn locale(&self) -> Result<IA2Locale> {
        unsafe {
            let mut locale = std::mem::zeroed();
            let res = self._ia2.locale(&mut locale);
            if res.is_err() {
                return Err(Error::new(S_FALSE, res.message()));
            }
            Ok(locale)
        }
    }

    /**
     * 返回特定于此对象的属性，例如单元格的公式。
     * */
    pub fn attributes(&self) -> String {
        unsafe {
            let mut attributes = std::mem::zeroed();
            if self._ia2.attributes(&mut attributes).is_err() {
                return String::new();
            }
            attributes.to_string()
        }
    }
}

impl Debug for Accessible2Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Accessible2Object(role:{}, states:{}, attributes:{}, n_relations:{}, window:{})",
            self.role(),
            self.states().unwrap_or(0),
            self.attributes(),
            self.n_relations(),
            self.window_handle().0
        )
    }
}

pub enum IA2ResultType {
    Str(String),
    Num(i32),
    Bool(bool),
    None,
}

impl From<VARIANT> for IA2ResultType {
    fn from(value: VARIANT) -> Self {
        if let Ok(d) = i32::try_from(&value) {
            return Self::Num(d);
        }
        if let Ok(d) = BSTR::try_from(&value) {
            return Self::Str(d.to_string());
        }
        if let Ok(d) = bool::try_from(&value) {
            return Self::Bool(d);
        }
        Self::None
    }
}
