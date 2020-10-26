use std::task::{Context, Poll};

use actix_identity::Identity;
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse, Payload};
use actix_web::{http, Error, HttpResponse, HttpRequest, FromRequest, http::StatusCode};
use futures::future::{ok, Either, Ready};

/// Login middleware

pub struct CheckLogin;

impl<S, B> Transform<S> for CheckLogin
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CheckLoginMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckLoginMiddleware { service })
    }
}

pub fn auto_login(req: &HttpRequest, pl: &mut Payload) -> Option<String> {
    if let Some(identity) = Identity::from_request(req, pl)
        .into_inner()
        .map(|x| x.identity())
        .unwrap()
    {
        Some(identity)
    } else {
        None
    }
}

pub struct CheckLoginMiddleware<S> {
    service: S,
}

impl<S, B> Service for CheckLoginMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let (r, mut pl) = req.into_parts();

        let token = auto_login(&r, &mut pl);
        let req = ServiceRequest::from_parts(r, pl).ok().unwrap();
        if token.is_some() {
            Either::Left(self.service.call(req))
        } else {
            Either::Right(ok(req.into_response(
                HttpResponse::Found()
                    .header(http::header::LOCATION, "/login")
                    .finish()
                    .into_body(),
            )))
        }
    }
}
