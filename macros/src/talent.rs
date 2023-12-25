/*
 * Copyright (c) 2023. The RigelA open source project team and
 * its contributors reserve all rights.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and limitations under the License.
 */use heck::ToUpperCamelCase;

use proc_macro2::{Delimiter::Bracket, Group, Punct, Spacing, Span, TokenStream};
use proc_macro2::Delimiter::Parenthesis;
use quote::{quote, TokenStreamExt, ToTokens};
use syn::{parse::{Parse, ParseStream}, Ident, ItemFn, Token, MetaNameValue, punctuated::Punctuated};

struct Metadata {
    doc: MetaNameValue,
    cmd_list: TokenStream
}
impl Parse for Metadata {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let r=Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;
        let mut iter = r.iter();
        let doc = iter
            .next()
            .unwrap()
            .clone();
        let mut cmd_list = TokenStream::new();
        cmd_list.append(Ident::new("vec", Span::call_site()));
        cmd_list.append(Punct::new('!', Spacing::Alone));
        cmd_list.append(Group::new(Bracket, {
            let mut items = TokenStream::new();
            for i in iter {
                let cmd_type = i
                    .path
                    .get_ident()
                    .unwrap()
                    .to_string()
                    .to_upper_camel_case()
                    .to_string();
                let cmd_type = Ident::new(cmd_type.as_str(), Span::call_site());
                let cmd_type = quote! {
                    crate::commander::CommandType::#cmd_type
                };
                cmd_type.to_tokens(&mut items);
                let value: Group = syn::parse2(i.value.to_token_stream())?;
                let mut data = TokenStream::new();
                data.append(Ident::new("vec", Span::call_site()));
                data.append(Punct::new('!', Spacing::Alone));
                data.append(Group::new(Bracket, value.stream()));
                items.append(Group::new(Parenthesis, data));
            }
            items
        }));
        Ok(Self {
            doc,
            cmd_list
        })
    }
}
fn get_struct_name(ident: &Ident) -> Ident {
    let s = ident.to_string();
    let s = s.to_upper_camel_case() + "Talent";
    Ident::new(s.as_str(), Span::call_site())
}

pub fn parse_talent(args: TokenStream, item: TokenStream) -> TokenStream {
    let metadata: Metadata = syn::parse2(args).unwrap();
    let doc = metadata.doc;
    let cmd_list = metadata.cmd_list;
    let input: ItemFn = syn::parse2(item).unwrap();
    let id = get_struct_name(&input.sig.ident);
    let body = input.block.to_token_stream();
    quote! {
        #[#doc]
        pub(crate) struct #id;
        impl crate::talent::Talented for #id {
            fn get_supported_cmd_list(&self) -> Vec<crate::commander::CommandType> {
                #cmd_list
            }
            fn perform(&self, context: std::sync::Arc<Context>) {
                let main_handler = context.main_handler.clone();
                main_handler.spawn(async move #body);
            }
        }
    }

}