use actix_identity::RequestIdentity;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
    task::{Context, Poll},
};

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<A, B> Either<A, B> {
    fn project(self: Pin<&mut Self>) -> Either<Pin<&mut A>, Pin<&mut B>> {
        unsafe {
            match self.get_unchecked_mut() {
                Either::Left(a) => Either::Left(Pin::new_unchecked(a)),
                Either::Right(b) => Either::Right(Pin::new_unchecked(b)),
            }
        }
    }
}

impl<A, B> Future for Either<A, B>
where
    A: Future,
    B: Future<Output = A::Output>,
{
    type Output = A::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project() {
            Either::Left(x) => x.poll(cx),
            Either::Right(x) => x.poll(cx),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Redirect {
    Public,
    Private,
}

pub struct RedirectMiddleware<S> {
    service: S,
    visibility: Redirect,
}

impl<S> Transform<S, ServiceRequest> for Redirect
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = RedirectMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RedirectMiddleware {
            service,
            visibility: *self,
        }))
    }
}

impl<S> Service<ServiceRequest> for RedirectMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
{
    type Response = ServiceResponse;
    type Error = Error;

    #[allow(clippy::type_complexity)]
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&self, ctx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        macro_rules! execute {
            (send $self:ident) => {
                Either::Left(self.service.call(req));
            };

            (redirect $path:expr) => {
                Either::Right(ready(Ok(req.into_response(
                    HttpResponse::Found()
                        .append_header(("Location", $path))
                        .finish(),
                ))))
            };
        }

        let id = req.get_identity();

        match (id, self.visibility) {
            (Some(_), Redirect::Public) => execute!(redirect "index"),
            (None, Redirect::Public) => execute!(send self),
            (Some(_), Redirect::Private) => execute!(send self),
            (None, Redirect::Private) => execute!(redirect "signin"),
        }
    }
}
