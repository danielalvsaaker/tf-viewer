use super::{templates::authorize_template, Consent, Extras};
use actix::{Actor, Context, Handler};
use oxide_auth::{
    endpoint::{Endpoint, OwnerConsent, OwnerSolicitor, Solicitation},
    frontends::simple::endpoint::{ErrorInto, FnSolicitor, Generic, Vacant},
    primitives::prelude::{AuthMap, Client, ClientMap, RandomGenerator, Scope, TokenMap},
};
use oxide_auth_actix::{OAuthMessage, OAuthOperation, OAuthRequest, OAuthResponse, WebError};
type AuthEndpoint = Generic<
    ClientMap,
    AuthMap<RandomGenerator>,
    TokenMap<RandomGenerator>,
    Vacant,
    Vec<Scope>,
    fn() -> OAuthResponse,
>;

pub struct AuthServer {
    endpoint: AuthEndpoint,
}

impl AuthServer {
    pub fn preconfigured() -> Self {
        Self {
            endpoint: Generic {
                // A registrar with one pre-registered client
                registrar: vec![Client::public(
                    "LocalClient",
                    "http://localhost:8021/endpoint"
                        .parse::<url::Url>()
                        .unwrap()
                        .into(),
                    "default-scope".parse().unwrap(),
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

    pub fn with_solicitor<S>(
        &mut self,
        solicitor: S,
    ) -> impl Endpoint<OAuthRequest, Error = WebError> + '_
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
            Extras::AuthGet { username } => {
                let solicitor =
                    FnSolicitor(move |_: &mut OAuthRequest, pre_grant: Solicitation| {
                        // This will display a page to the user asking for his permission to proceed. The submitted form
                        // will then trigger the other authorization handler which actually completes the flow.
                        OwnerConsent::InProgress(
                            OAuthResponse::ok()
                                .content_type("text/html")
                                .unwrap()
                                .body(&authorize_template(pre_grant, &username)),
                        )
                    });

                op.run(self.with_solicitor(solicitor))
            }
            Extras::AuthPost { consent, username } => {
                let solicitor =
                    FnSolicitor(move |_: &mut OAuthRequest, _: Solicitation| match consent {
                        Consent::Allow => OwnerConsent::Authorized(username.to_owned()),
                        Consent::Deny => OwnerConsent::Denied,
                    });

                op.run(self.with_solicitor(solicitor))
            }
            _ => op.run(&mut self.endpoint),
        }
    }
}
