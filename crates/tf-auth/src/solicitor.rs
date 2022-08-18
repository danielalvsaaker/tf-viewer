use crate::database::{
    resource::{
        client::{ClientName, EncodedClient},
        user::User,
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
    id: UserQuery,
}

impl Solicitor {
    pub fn new(db: Database, id: UserQuery) -> Self {
        Self { db, id }
    }
}

#[async_trait::async_trait]
impl OwnerSolicitor<OAuthRequest> for Solicitor {
    async fn check_consent(
        &mut self,
        req: &mut OAuthRequest,
        solicitation: Solicitation<'_>,
    ) -> OwnerConsent<<OAuthRequest as WebRequest>::Response> {
        let client_id = match solicitation.pre_grant().client_id.parse::<ClientQuery>() {
            Ok(id) => id,
            Err(err) => return OwnerConsent::Error(WebError::InternalError(Some(err.to_string()))),
        };

        let (client, user) = {
            let result = tokio::task::spawn_blocking({
                let db = self.db.clone();
                let id = self.id;
                move || {
                    let client = db
                        .root::<EncodedClient>()?
                        .traverse::<ClientName>()?
                        .get(&client_id)?;

                    let user = db.root::<User>()?.get(&id)?;

                    Ok::<_, Error>((client, user))
                }
            })
            .await
            .map_err(|err| WebError::InternalError(Some(err.to_string())))
            .map(|res| res.map_err(|err| WebError::InternalError(Some(err.to_string()))))
            .and_then(std::convert::identity);

            match result {
                Ok(result) => result,
                Err(err) => return OwnerConsent::Error(err),
            }
        };

        if let Some((client, user)) = client.zip(user) {
            let body = Authorize::new(req, &solicitation, &user.username, &client.inner);

            match body.render() {
                Ok(inner) => OwnerConsent::InProgress(
                    OAuthResponse::default()
                        .content_type("text/html")
                        .unwrap()
                        .body(&inner),
                ),
                Err(inner) => OwnerConsent::Error(WebError::InternalError(Some(inner.to_string()))),
            }
        } else {
            OwnerConsent::Denied
        }
    }
}
