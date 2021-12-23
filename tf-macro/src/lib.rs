extern crate proc_macro;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemFn, LitStr};

mod oauth;

#[proc_macro_attribute]
pub fn oauth(scopes: TokenStream, route: TokenStream) -> TokenStream {
    let scopes = parse_macro_input!(scopes as LitStr).value();
    let scopes = scopes.split(", ").collect();
    let route = parse_macro_input!(route as ItemFn);

    oauth::OAuth::new(route, scopes)
        .unwrap()
        .into_token_stream()
        .into()
}
