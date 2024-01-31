extern crate proc_macro;
mod talent;
mod utils;

use proc_macro::TokenStream;
use talent::parse_talent;

#[proc_macro_attribute]
pub fn talent(args: TokenStream, item: TokenStream) -> TokenStream {
    parse_talent(args.into(), item.into()).into()
}
