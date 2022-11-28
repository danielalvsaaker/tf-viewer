use crate::database::{
    resource::{
        client::{ClientName, EncodedClient},
        user::{Authorization, AuthorizationQuery, User},
    },
    Database,
};
use crate::templates::Authorize;
use askama::Template;
use oxide_auth::endpoint::{OwnerConsent, Solicitation, WebRequest};
use oxide_auth_async::endpoint::OwnerSolicitor;
use oxide_auth_axum::{OAuthRequest, OAuthResponse, WebError};
use tf_database::error::Error;
use tf_models::query::{ClientQuery, UserQuery};

pub struct Solicitor {
    db: Database,
    user: UserQuery,
}

impl Solicitor {
    pub fn new(db: Database, user: UserQuery) -> Self {
        Self { db, user }
    }
}

#[async_trait::async_trait]
impl OwnerSolicitor<OAuthRequest> for Solicitor {
    async fn check_consent(
        &mut self,
        req: &mut OAuthRequest,
        solicitation: Solicitation<'_>,
    ) -> OwnerConsent<<OAuthRequest as WebRequest>::Response> {
        fn map_err<E: std::error::Error>(
            err: E,
        ) -> OwnerConsent<<OAuthRequest as WebRequest>::Response> {
            OwnerConsent::Error(WebError::InternalError(Some(err.to_string())))
        }

        let client_id = match solicitation
            .pre_grant()
            .client_id
            .parse::<ClientQuery>()
            .map_err(map_err)
        {
            Ok(id) => id,
            Err(err) => return err,
        };

        let authorization = {
            let inner = tokio::task::spawn_blocking({
                let db = self.db.clone();
                let query = AuthorizationQuery {
                    user: self.user,
                    client: client_id,
                };
                move || db.root::<User>()?.traverse::<Authorization>()?.get(&query)
            })
            .await
            .map_err(map_err)
            .map(|res| res.map_err(map_err))
            .and_then(std::convert::identity);

            match inner {
                Ok(inner) => inner,
                Err(err) => return err,
            }
        };

        match authorization {
            Some(Authorization { scope }) if scope >= solicitation.pre_grant().scope => {
                return OwnerConsent::Authorized(self.user.to_string())
            }
            _ => (),
        }

        let (client, user) = {
            let inner = tokio::task::spawn_blocking({
                let db = self.db.clone();
                let user = self.user;

                move || {
                    let client = db
                        .root::<EncodedClient>()?
                        .traverse::<ClientName>()?
                        .get(&client_id)?;

                    let user = db.root::<User>()?.get(&user)?;

                    Ok::<_, Error>((client, user))
                }
            })
            .await
            .map_err(map_err)
            .map(|res| res.map_err(map_err))
            .and_then(std::convert::identity);

            match inner {
                Ok(inner) => inner,
                Err(err) => return err,
            }
        };

        if let Some((client, user)) = client.zip(user) {
            let body = Authorize::new(req, &solicitation, &user.username, &client.inner);

            match body.render().map_err(map_err) {
                Ok(inner) => OwnerConsent::InProgress(
                    OAuthResponse::default()
                        .content_type("text/html")
                        .unwrap()
                        .body(&inner),
                ),
                Err(err) => err,
            }
        } else {
            OwnerConsent::Denied
        }
    }
}
