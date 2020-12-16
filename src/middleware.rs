use std::task::{Context, Poll};

use actix_identity::Identity;
use actix_service::{Service, Transform};
use actix_web::dev::{Payload, ServiceRequest, ServiceResponse};
use actix_web::{http, Error, FromRequest, HttpRequest, HttpResponse};
use futures::future::{ok, Either, Ready, Future};
use std::pin::Pin;

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

pub struct Restricted;

impl<S, B> Transform<S> for Restricted
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RestrictedMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RestrictedMiddleware { service })
    }
}

pub struct RestrictedMiddleware<S> {
    service: S,
}

impl<S, B> Service for RestrictedMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let (r, mut pl) = req.into_parts();

        let token = auto_login(&r, &mut pl);
        let req = ServiceRequest::from_parts(r, pl).ok().unwrap();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let owner = res.request().match_info().query("username");
            if owner == token.unwrap() {
                Ok(res)
            }
            else {
                let new_body = HttpResponse::Found()
                    .header(http::header::LOCATION, "/")
                    .finish()
                    .into_body();
                let res = res.into_response(new_body);
                 
                Ok(res)
            }

        })
    }
}

/// Login middleware

#[derive(Clone, Copy)]
pub enum AuthType {
    Restricted,
    Public,
}

pub struct CheckLogin(AuthType);

impl CheckLogin {
    pub fn new(auth_type: AuthType) -> Self {
        CheckLogin(auth_type)
    }
}

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
        ok(CheckLoginMiddleware { service, auth_type: self.0, })
    }
}


pub struct CheckLoginMiddleware<S> {
    service: S,
    auth_type: AuthType,
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

        let auth_type = self.auth_type;
        let path = match self.auth_type {
            AuthType::Restricted => "/signin",
            AuthType::Public => "/",
        };

        let mut send = |request: Self::Request| Either::Left(self.service.call(request));
        let redirect = |path: &str, request: Self::Request| {
            Either::Right(ok(request.into_response(
                        HttpResponse::Found()
                        .header(http::header::LOCATION, path)
                        .finish()
                        .into_body()
                        )
                    )
                )
        };

        if token.is_some() {
            match auth_type {
                AuthType::Restricted => send(req),
                AuthType::Public => redirect(path, req),
            }
        }
        else {
            match auth_type {
                AuthType::Restricted => redirect(path, req),
                AuthType::Public => send(req),
            }
        }
    }
}
