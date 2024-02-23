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

extern crate proc_macro;

mod gui;
mod talent;
mod utils;

use proc_macro::TokenStream;
use talent::parse_talent;
use crate::gui::parse_gui;

#[proc_macro_attribute]
pub fn talent(args: TokenStream, item: TokenStream) -> TokenStream {
    parse_talent(args.into(), item.into()).into()
}

#[proc_macro_derive(GuiFormImpl)]
pub fn gui_form_impl(input: TokenStream) -> TokenStream {
    parse_gui(input.into()).into()
}
