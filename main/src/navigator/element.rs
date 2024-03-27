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

use std::{
    hash::{DefaultHasher, Hash, Hasher},
    sync::Arc,
};
use crate::performer::Speakable;
use a11y::{
    ia2::{object::Accessible2Object, relation::AccessibleRelation},
    jab::context::AccessibleContext,
};
use win_wrap::{common::RECT, msaa::object::AccessibleObject, uia::element::UiAutomationElement};

/**
 * UI元素。
 * */
pub(crate) enum UiElement<'a> {
    IA2(Option<Accessible2Object>, Option<AccessibleRelation>),
    JAB(AccessibleContext<'a>),
    MSAA(AccessibleObject, i32),
    UIA(UiAutomationElement),
}

#[allow(dead_code)]
impl<'a> UiElement<'a> {
    /**
     * 获取子对象数量。
     * */
    pub(crate) fn get_child_count(&self) -> i32 {
        match self {
            Self::IA2(Some(x), _) => x.n_relations(),
            Self::IA2(None, Some(x)) => x.n_targets(),
            Self::IA2(None, None) => 0,
            Self::JAB(x) => x.get_child_count(),
            Self::MSAA(x, _) => x.child_count() as i32,
            Self::UIA(x) => x.get_child_count(),
        }
    }
    pub(crate) fn get_child(&self, index: i32) -> Option<Arc<Self>> {
        let child = match self {
            Self::IA2(Some(x), _) => match x.relation(index) {
                Ok(y) => Some(Self::IA2(None, Some(y))),
                Err(_) => None,
            },
            Self::IA2(None, Some(x)) => Some(Self::IA2(x.target(index), None)),
            Self::IA2(None, None) => None,
            Self::JAB(x) => match x.get_child(index) {
                None => None,
                Some(y) => Some(Self::JAB(y)),
            },
            Self::MSAA(x, _) => match x.get_child(index) {
                Ok(y) => Some(Self::MSAA(y, 0)),
                Err(_) => None,
            },
            Self::UIA(x) => match x.get_child(index) {
                None => None,
                Some(y) => Some(Self::UIA(y)),
            },
        };
        match child {
            None => None,
            Some(x) => Some(x.into()),
        }
    }
    pub(crate) fn get_rect(&self) -> Option<RECT> {
        match self {
            Self::IA2(_, _) => None,
            Self::JAB(x) => {
                if let Some(r) = x.get_bound_rectangle() {
                    Some(RECT {
                        left: r.0,
                        top: r.1,
                        right: r.0 + r.2,
                        bottom: r.1 + r.3,
                    })
                } else {
                    None
                }
            }
            Self::MSAA(x, y) => {
                let r = x.location(*y);
                Some(RECT {
                    left: r.0,
                    top: r.1,
                    right: r.0 + r.2,
                    bottom: r.1 + r.3,
                })
            }
            Self::UIA(x) => Some(x.get_bounding_rectangle()),
        }
    }
}

impl<'a> Hash for UiElement<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let unique_id = match self {
            Self::IA2(Some(x), _) => format!("ia2:{}", x.unique_id().unwrap_or(0)),
            Self::JAB(x) => format!("jab:{}", x.get_unique_id()),
            Self::MSAA(x, y) => {
                let (x, y, w, h) = x.location(*y);
                format!("msaa:{},{},{},{}", x, y, w, h)
            }
            Self::UIA(x) => format!("uia:{}", x.get_automation_id()),
            _ => "None".to_string()
        };
        state.write(unique_id.as_bytes())
    }
}

impl<'a> Eq for UiElement<'a> {}

impl<'a> PartialEq for UiElement<'a> {
    fn eq(&self, other: &Self) -> bool {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher) == other.hash(&mut hasher)
    }
}

unsafe impl<'a> Sync for UiElement<'a> {}

unsafe impl<'a> Send for UiElement<'a> {}

impl<'a> From<UiAutomationElement> for UiElement<'a> {
    fn from(value: UiAutomationElement) -> Self {
        Self::UIA(value)
    }
}

impl<'a> From<(AccessibleObject, i32)> for UiElement<'a> {
    fn from(value: (AccessibleObject, i32)) -> Self {
        Self::MSAA(value.0, value.1)
    }
}

impl<'a> Speakable for UiElement<'a> {
    fn get_sentence(&self) -> String {
        match self {
            Self::IA2(x, _) => match x {
                None => String::new(),
                Some(y) => y.get_sentence(),
            },
            Self::JAB(x) => Arc::new(x).get_states_en_us().unwrap(),
            Self::MSAA(x, y) => (x.clone(), *y).get_sentence(),
            Self::UIA(x) => x.get_sentence(),
        }
    }
}
