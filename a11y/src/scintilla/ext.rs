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

pub(crate) trait VecExt<T> {
    fn to_vec(&self, len: usize) -> Vec<T>;
}

impl<T> VecExt<T> for *const T {
    fn to_vec(&self, len: usize) -> Vec<T> {
        unsafe {
            let mut p = *self;
            let mut v = Vec::with_capacity(len);
            for _ in 0..len {
                v.push(p.read());
                p = p.add(1);
            }
            v
        }
    }
}
