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


use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

pub fn parse_gui(input: TokenStream) -> TokenStream {
    let input: ItemStruct = syn::parse2(input).unwrap();
    let name = input.ident;

    quote! {
        impl crate::gui::GuiForm for #name {
            fn set_context(&self, context: Arc<Context>) {
                self.context.set(context.clone()).unwrap();
            }

            fn get_show_notice_sender(&self) -> NoticeSender {
                self.show_notice.sender().clone()
            }

            fn get_exit_notice_sender(&self) -> NoticeSender {
                self.exit_notice.sender().clone()
            }
        }
    }
}
