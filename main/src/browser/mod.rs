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

use std::sync::Arc;
use a11y::{
    ia2::{
        object::Accessible2Object,
        relation::AccessibleRelation,
    },
    jab::context::AccessibleContext,
};
use win_wrap::{
    msaa::object::AccessibleObject,
    uia::element::UiAutomationElement,
};
use win_wrap::common::RECT;
use crate::performer::Speakable;

pub(crate) mod form_browser;

pub(crate) enum BrowserElement<'a> {
    IA2(Option<Accessible2Object>, Option<AccessibleRelation>),
    JAB(AccessibleContext<'a>),
    MSAA(AccessibleObject, i32),
    UIA(UiAutomationElement),
}

impl<'a> BrowserElement<'a> {
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
            Self::UIA(x) => x.get_child_count()
        }
    }
    pub(crate) fn get_child(&self, index: i32) -> Option<Arc<Self>> {
        let child = match self {
            Self::IA2(Some(x), _) => match x.relation(index) {
                Ok(y) => Some(Self::IA2(None, Some(y))),
                Err(_) => None,
            }
            Self::IA2(None, Some(x)) => Some(Self::IA2(x.target(index), None)),
            Self::IA2(None, None) => None,
            Self::JAB(x) => match x.get_child(index) {
                None => None,
                Some(y) => Some(Self::JAB(y))
            }
            Self::MSAA(x, _) => match x.get_child(index) {
                Ok(y) => Some(Self::MSAA(y, 0)),
                Err(_) => None
            }
            Self::UIA(x) => match x.get_child(index) {
                None => None,
                Some(y) => Some(Self::UIA(y))
            }
        };
        match child {
            None => None,
            Some(x) => Some(x.into())
        }
    }
    pub(crate) fn get_rect(&self) -> Option<RECT> {
        match self {
            Self::IA2(_, _) => None,
            Self::JAB(x) => if let Some(r) = x.get_bound_rectangle() {
                Some(RECT {
                    left: r.0,
                    top: r.1,
                    right: r.0 + r.2,
                    bottom: r.1 + r.3,
                })
            } else {
                None
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
            Self::UIA(x) => Some(x.get_rect())
        }
    }
}

unsafe impl<'a> Sync for BrowserElement<'a> {}

unsafe impl<'a> Send for BrowserElement<'a> {}

impl<'a> From<UiAutomationElement> for BrowserElement<'a> {
    fn from(value: UiAutomationElement) -> Self {
        Self::UIA(value)
    }
}

impl<'a> Speakable for BrowserElement<'a> {
    fn get_sentence(&self) -> String {
        match self {
            Self::IA2(x, _) => match x {
                None => String::new(),
                Some(y) => y.get_sentence()
            }
            Self::JAB(x) => Arc::new(x).get_states_en_us().unwrap(),
            Self::MSAA(x, y) => (x.clone(), *y).get_sentence(),
            Self::UIA(x) => x.get_sentence()
        }
    }
}