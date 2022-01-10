use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Bracket,
    ItemFn, LitStr, Result, ReturnType, Token,
};

mod kw {
    syn::custom_keyword!(scopes);
}

struct Scopes {
    scopes: Punctuated<LitStr, Token![,]>,
}

impl Parse for Scopes {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;

        let _: kw::scopes = input.parse()?;
        let _: Token![=] = input.parse()?;
        let _: Bracket = bracketed!(content in input);

        Ok(Self {
            scopes: Punctuated::parse_separated_nonempty(&content)?,
        })
    }
}

impl ToTokens for Scopes {
    fn to_tokens(&self, output: &mut TokenStream) {
        let scopes = self.scopes.iter();

        output.extend(quote! { &[#(#scopes.parse().unwrap()),*] })
    }
}

pub struct OAuthArguments {
    scopes: Scopes,
}

impl Parse for OAuthArguments {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::scopes) {
            Ok(Self {
                scopes: input.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

pub(crate) struct OAuth {
    func: ItemFn,
    args: OAuthArguments,
}

impl OAuth {
    pub fn new(func: ItemFn, args: OAuthArguments) -> Self {
        Self { func, args }
    }
}

impl ToTokens for OAuth {
    fn to_tokens(&self, output: &mut TokenStream) {
        let func_vis = &self.func.vis;
        let func_block = &self.func.block;

        let fn_sig = &self.func.sig;
        let fn_attrs = &self.func.attrs;
        let fn_name = &fn_sig.ident;
        let fn_generics = &fn_sig.generics;
        let fn_args = &fn_sig.inputs;
        let fn_output = match &fn_sig.output {
            ReturnType::Type(ref _arrow, ref ty) => ty.to_token_stream(),
            ReturnType::Default => {
                quote! {()}
            }
        };

        let scopes = &self.args.scopes;

        let stream = quote! {
            #(#fn_attrs)*
            #func_vis async fn #fn_name #fn_generics(
                ::axum::extract::Extension(_state_): ::axum::extract::Extension<::tf_auth::State>,
                _req_: ::oxide_auth_axum::OAuthResource,
                #fn_args
            ) -> ::std::result::Result<
                #fn_output,
                ::std::result::Result<::oxide_auth_axum::OAuthResponse, ::oxide_auth_axum::WebError>
            > {
                let _auth_ = _state_
                    .endpoint()
                    .with_scopes(#scopes)
                    .resource_flow()
                    .execute(_req_.into());

                match _auth_ {
                    Ok(grant) => {
                        let f = || async move #func_block;
                        Ok(f().await)
                    },
                    Err(inner) => Err(inner),
                }
            }
        };

        output.extend(stream);
    }
}
