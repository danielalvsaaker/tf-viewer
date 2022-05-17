use askama::Template;

use oxide_auth::endpoint::WebRequest;
use std::borrow::Cow;
use tf_database::query::UserQuery;

#[derive(Template)]
#[template(path = "signin.html")]
pub struct SignIn<'a> {
    pub query: &'a str,
}

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignUp<'a> {
    pub query: &'a str,
}

#[derive(Template)]
#[template(path = "authorize.html")]
pub struct Authorize<'a> {
    pub query: String,
    pub client_id: String,
    pub user_id: &'a UserQuery,
    pub scopes: String,
}

#[derive(Template)]
#[template(path = "client.html")]
pub struct Client;

impl<'a> Authorize<'a> {
    pub fn new(
        req: &mut oxide_auth_axum::OAuthRequest,
        solicitation: oxide_auth::endpoint::Solicitation<'a>,
        user_id: &'a UserQuery,
    ) -> Self {
        macro_rules! to_string {
            ($query:expr) => {
                $query.unwrap_or(Cow::Borrowed("")).to_string()
            };
        }

        let query = req.query().unwrap();
        let grant = solicitation.pre_grant();
        let state = solicitation.state();
        let code_challenge = to_string!(query.unique_value("code_challenge"));
        let method = to_string!(query.unique_value("code_challenge_method"));

        let mut extra = vec![
            ("response_type", "code"),
            ("client_id", grant.client_id.as_str()),
            ("redirect_uri", grant.redirect_uri.as_str()),
            ("code_challenge", &code_challenge),
            ("code_challenge_method", &method),
        ];

        if let Some(state) = state {
            extra.push(("state", state));
        }

        let query = serde_urlencoded::to_string(extra).unwrap();

        Self {
            query,
            client_id: grant.client_id.to_owned(),
            user_id,
            scopes: grant.scope.iter().collect::<Vec<_>>().join(", "),
        }
    }
}
