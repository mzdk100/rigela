extern crate proc_macro;
mod talent;

use proc_macro::TokenStream;
use talent::parse_talent;


#[proc_macro_attribute]
pub fn talent(args: TokenStream, item: TokenStream) -> TokenStream {
    let ret = parse_talent(args.into(), item.into()).into();
    println!("{}", ret);
    ret
}