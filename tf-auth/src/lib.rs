use actix::{Actor, Addr, Message, Context, Handler};
use actix_web::{
    web, App, HttpRequest, HttpServer, rt,
};
use actix_identity::Identity;
use oxide_auth::{
    endpoint::{Endpoint, OwnerConsent, OwnerSolicitor, Solicitation, WebResponse},
    frontends::simple::endpoint::{ErrorInto, FnSolicitor, Generic, Vacant},
    primitives::prelude::{AuthMap, Client, ClientMap, RandomGenerator, Scope, TokenMap},
};
use serde::Deserialize;
use oxide_auth_actix::{
    Authorize, OAuthMessage, OAuthOperation, OAuthRequest, OAuthResource, OAuthResponse, Refresh,
    Resource, Token, WebError,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/signin")
            .route(web::get().to(routes::get_signin))
    )
    /*
    .service(
        web::resource("/signup")
            .route(web::get().to(routes::get_signup))
    )
    */
    .service(
        web::resource("/authorize")
            .route(web::get().to(get_authorize))
            .route(web::post().to(post_authorize))
    )
    .route("/token", web::post().to(token))
    .route("/refresh", web::post().to(refresh));
}

mod routes;

pub struct AuthServer {
    endpoint: Generic<
        ClientMap,
        AuthMap<RandomGenerator>,
        TokenMap<RandomGenerator>,
        Vacant,
        Vec<Scope>,
        fn() -> OAuthResponse,
    >,
}

pub enum Extras {
    AuthGet,
    AuthPost {
        consent: Consent,
        username: String,
    },
    Nothing,
}

#[derive(Deserialize)]
#[serde(tag = "consent", rename_all = "lowercase")]
pub enum Consent {
    Allow,
    Deny,
}

async fn get_authorize(
    (req, state): (OAuthRequest, web::Data<Addr<AuthServer>>),
) -> Result<OAuthResponse, WebError> {
    state.send(Authorize(req).wrap(Extras::AuthGet)).await?
}

async fn post_authorize(
    id: Identity,
    consent: web::Query<Consent>,
    req: OAuthRequest,
    state: web::Data<Addr<AuthServer>>
) -> Result<OAuthResponse, WebError> {
    if let Some(username) = id.identity() {
        state.send(
            Authorize(req)
                .wrap(Extras::AuthPost { consent: consent.into_inner(), username })
        )
        .await?
    } else {
        let mut response = OAuthResponse::ok();
        response.unauthorized("Bearer");
        Ok(response)
    }
}

async fn token((req, state): (OAuthRequest, web::Data<Addr<AuthServer>>)) -> Result<OAuthResponse, WebError> {
    state.send(Token(req).wrap(Extras::Nothing)).await?
}

async fn refresh(
    (req, state): (OAuthRequest, web::Data<Addr<AuthServer>>),
) -> Result<OAuthResponse, WebError> {
    state.send(Refresh(req).wrap(Extras::Nothing)).await?
}

async fn index(
    (req, state): (OAuthResource, web::Data<Addr<AuthServer>>),
) -> Result<OAuthResponse, WebError> {
    match state
        .send(Resource(req.into_request()).wrap(Extras::Nothing))
        .await?
    {
        Ok(_grant) => Ok(OAuthResponse::ok()
            .content_type("text/plain")?
            .body("Hello world!")),
        Err(Ok(e)) => Ok(e),
        Err(Err(e)) => Err(e),
    }
}

impl AuthServer {
    pub fn preconfigured() -> Self {
        Self {
            endpoint: Generic {
                // A registrar with one pre-registered client
                registrar: vec![Client::confidential(
                    "LocalClient",
                    "http://localhost:8021/endpoint"
                        .parse::<url::Url>()
                        .unwrap()
                        .into(),
                    "default-scope".parse().unwrap(),
                    "password".as_bytes(),
                )]
                .into_iter()
                .collect(),
                // Authorization tokens are 16 byte random keys to a memory hash map.
                authorizer: AuthMap::new(RandomGenerator::new(16)),
                // Bearer tokens are also random generated but 256-bit tokens, since they live longer
                // and this example is somewhat paranoid.
                //
                // We could also use a `TokenSigner::ephemeral` here to create signed tokens which can
                // be read and parsed by anyone, but not maliciously created. However, they can not be
                // revoked and thus don't offer even longer lived refresh tokens.
                issuer: TokenMap::new(RandomGenerator::new(16)),

                solicitor: Vacant,

                // A single scope that will guard resources for this endpoint
                scopes: vec!["default-scope".parse().unwrap()],

                response: OAuthResponse::ok,
            },
        }
    }

    pub fn with_solicitor<'a, S>(
        &'a mut self, solicitor: S,
    ) -> impl Endpoint<OAuthRequest, Error = WebError> + 'a
    where
        S: OwnerSolicitor<OAuthRequest> + 'static,
    {
        ErrorInto::new(Generic {
            authorizer: &mut self.endpoint.authorizer,
            registrar: &mut self.endpoint.registrar,
            issuer: &mut self.endpoint.issuer,
            solicitor,
            scopes: &mut self.endpoint.scopes,
            response: OAuthResponse::ok,
        })
    }
}

impl Actor for AuthServer {
    type Context = Context<Self>;
}

impl<Op> Handler<OAuthMessage<Op, Extras>> for AuthServer
where
    Op: OAuthOperation,
{
    type Result = Result<Op::Item, Op::Error>;

    fn handle(&mut self, msg: OAuthMessage<Op, Extras>, _: &mut Self::Context) -> Self::Result {
        let (op, ex) = msg.into_inner();

        match ex {
            Extras::AuthGet => {
                let solicitor = FnSolicitor(move |_: &mut OAuthRequest, pre_grant: Solicitation| {
                    // This will display a page to the user asking for his permission to proceed. The submitted form
                    // will then trigger the other authorization handler which actually completes the flow.
                    OwnerConsent::InProgress(
                        OAuthResponse::ok()
                            .content_type("text/html")
                            .unwrap()
                            .body("test")
                    )
                });

                op.run(self.with_solicitor(solicitor))
            }
            Extras::AuthPost { consent, username } => {
                let solicitor = FnSolicitor(move |_: &mut OAuthRequest, _: Solicitation| {
                    match consent {
                        Consent::Allow => OwnerConsent::Authorized(username.to_owned()),
                        Consent::Deny => OwnerConsent::Denied,
                    }
                });

                op.run(self.with_solicitor(solicitor))
            }
            _ => op.run(&mut self.endpoint),
        }
    }
}
