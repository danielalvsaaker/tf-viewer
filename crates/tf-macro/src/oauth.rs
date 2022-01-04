use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{ItemFn, ReturnType};

pub(crate) struct OAuth<'a> {
    scopes: Vec<&'a str>,
    func: ItemFn,
}

impl<'a> OAuth<'a> {
    pub fn new(func: ItemFn, scopes: Vec<&'a str>) -> syn::Result<Self> {
        Ok(Self { func, scopes })
    }
}

impl<'a> ToTokens for OAuth<'a> {
    fn to_tokens(&self, output: &mut TokenStream2) {
        let func_vis = &self.func.vis;
        let func_block = &self.func.block;

        let fn_sig = &self.func.sig;
        let fn_attrs = &self.func.attrs;
        let fn_name = &fn_sig.ident;
        let fn_generics = &fn_sig.generics;
        let fn_args = &fn_sig.inputs;
        let fn_async = &fn_sig.asyncness.unwrap_or_default();
        let fn_output = match &fn_sig.output {
            ReturnType::Type(ref _arrow, ref ty) => ty.to_token_stream(),
            ReturnType::Default => {
                quote! {()}
            }
        };

        let scopes = &self.scopes;

        let stream = quote! {
            #(#fn_attrs)*
            #func_vis #fn_async fn #fn_name #fn_generics(
                ::axum::extract::Extension(_state_): ::axum::extract::Extension<::tf_auth::State>,
                _req_: ::oxide_auth_axum::OAuthResource,
                #fn_args
            ) -> ::std::result::Result<
                #fn_output,
                ::std::result::Result<::oxide_auth_axum::OAuthResponse, ::oxide_auth_axum::WebError>
            > {
                let _auth_ = _state_
                    .endpoint()
                    .with_scopes(
                        &[#(#scopes.parse().unwrap()),*]
                    )
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
