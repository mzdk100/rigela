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


use heck::ToUpperCamelCase;
use proc_macro2::{Ident, Span};

pub(crate) fn get_struct_name(ident: &Ident, suffix: &str) -> Ident {
    let s = ident.to_string();
    let s = s.to_upper_camel_case() + suffix;
    Ident::new(s.as_str(), Span::call_site())
}
