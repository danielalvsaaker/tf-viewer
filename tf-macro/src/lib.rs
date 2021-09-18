extern crate proc_macro;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemFn};

use crate::expand::Protect;

mod expand;

#[proc_macro_attribute]
pub fn protect(_: TokenStream, input: TokenStream) -> TokenStream {
    check_permissions(input)
}

fn check_permissions(input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);

    match Protect::new(func) {
        Ok(has_permissions) => has_permissions.into_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
