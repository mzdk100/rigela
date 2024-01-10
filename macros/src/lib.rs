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

#[proc_macro_attribute]
pub fn gui(args: TokenStream, item: TokenStream) -> TokenStream {
    parse_gui(args.into(), item.into()).into()
}