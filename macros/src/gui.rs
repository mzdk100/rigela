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


use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{ItemFn, MetaNameValue, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use crate::utils::get_struct_name;

struct Metadata {
    doc: MetaNameValue,
    title: String
}
impl Parse for Metadata {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let r=Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;
        let iter = r.iter();
        let mut doc: Option<MetaNameValue> = None;
        let mut title = String::new();
        for i in iter {
            let name = i.path
                .get_ident()
                .unwrap()
                .to_string();
            if name == "doc" {
                doc = Some(i.clone())
            } else if name == "title" {
                title = i.value.to_token_stream().to_string();
            }
        }
        Ok(Self {
            doc: doc.unwrap().clone(),
            title
        })
    }
}
pub fn parse_gui(args: TokenStream, item: TokenStream) -> TokenStream {
    let metadata: Metadata = syn::parse2(args).unwrap();
    let doc = metadata.doc;
    let title = metadata.title;
    let input: ItemFn = syn::parse2(item.clone()).unwrap();
    let id0 = &input.sig.ident;
    let id = get_struct_name(id0, "FrameUi");
    let id2 = format!("get_{}_frame_ui", input.sig.ident.to_string());
    let id2 = Ident::new(id2.as_str(), Span::call_site());
    let id3 = get_struct_name(id0, "FrameUiApp");
    quote! {
        #[#doc]
        struct #id3(Arc<Context>);
        impl #id3 {
            #item
        }
        impl eframe::App for #id3 {
            #[allow(unused_must_use)]
            fn update(&mut self, ctx: &GuiContext, _frame: &mut eframe::Frame) {
                Self::#id0(self.0.clone(), ctx)
            }
        }
        #[#doc]
        pub(crate) struct #id;
        impl crate::gui::FrameUi for #id {
            fn show(&self, context: std::sync::Arc<Context>) {
                std::thread::spawn(move || Self::run(context.clone(), #title, #id3(context)));
            }
        }
        impl crate::gui::GuiAccessor {
            #[#doc]
            pub fn #id2(&self) -> #id {#id}
        }
    }

}