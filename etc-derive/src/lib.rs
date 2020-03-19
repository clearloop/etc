//! proc-macro derive for etc
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use syn::{parse, DeriveInput};
// use quote::quote;

#[proc_macro_attribute]
pub fn etc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _attr = parse::<DeriveInput>(attr).unwrap();

    item
}
