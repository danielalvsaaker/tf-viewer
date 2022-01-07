extern crate proc_macro;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemFn};

mod oauth;
use oauth::OAuthArguments;

#[proc_macro_attribute]
pub fn oauth(attr: TokenStream, route: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as OAuthArguments);
    let route = parse_macro_input!(route as ItemFn);

    oauth::OAuth::new(route, attr).into_token_stream().into()
}
