use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{ItemFn, ReturnType};

pub(crate) struct Protect {
    func: ItemFn,
}

impl Protect {
    pub fn new(func: ItemFn) -> syn::Result<Self> {
        Ok(Self { func })
    }
}

impl ToTokens for Protect {
    fn to_tokens(&self, output: &mut TokenStream2) {
        let func_vis = &self.func.vis;
        let func_block = &self.func.block;

        let fn_sig = &self.func.sig;
        let fn_attrs = &self.func.attrs;
        let fn_name = &fn_sig.ident;
        let fn_generics = &fn_sig.generics;
        let fn_args = &fn_sig.inputs;
        let fn_async = &fn_sig.asyncness.unwrap();
        let fn_output = match &fn_sig.output {
            ReturnType::Type(ref _arrow, ref ty) => ty.to_token_stream(),
            ReturnType::Default => {
                quote! {()}
            }
        };

        let stream = quote! {
            #(#fn_attrs)*
            #func_vis #fn_async fn #fn_name #fn_generics(
                _state_: actix_web::web::Data<actix::Addr<tf_auth::AuthServer>>,
                _req_: actix_web::HttpRequest,
                #fn_args
            ) -> actix_web::Either<actix_web::Either<#fn_output, impl Responder>, actix_web::HttpResponse> {
                let auth = tf_auth::authorize(&_state_, &_req_).await;

                if let Ok(grant) = auth {
                    let f = || async move #func_block;
                    actix_web::Either::Left(actix_web::Either::Left(f().await))
                } else if let Err(Ok(e)) = auth {
                    actix_web::Either::Left(actix_web::Either::Right(e))
                } else {
                    actix_web::Either::Right(actix_web::HttpResponse::InternalServerError().finish())
                }
            }
        };

        output.extend(stream);
    }
}
