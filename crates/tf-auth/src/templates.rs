use askama::Template;

use oxide_auth::endpoint::WebRequest;

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
    pub client_name: &'a str,
    pub username: &'a str,
    pub scopes: String,
}

#[derive(Template)]
#[template(path = "client.html")]
pub struct Client;

impl<'a> Authorize<'a> {
    pub fn new(
        req: &mut oxide_auth_axum::OAuthRequest,
        solicitation: &oxide_auth::endpoint::Solicitation<'a>,
        username: &'a str,
        client_name: &'a str,
    ) -> Self {
        let query = req.query().unwrap();
        let grant = solicitation.pre_grant();
        let state = solicitation.state();
        let code_challenge = query
            .unique_value("code_challenge")
            .unwrap_or_default()
            .to_string();
        let method = query
            .unique_value("code_challenge_method")
            .unwrap_or_default()
            .to_string();
        let scope = grant.scope.to_string();

        let mut extra = vec![
            ("response_type", "code"),
            ("client_id", grant.client_id.as_str()),
            ("redirect_uri", grant.redirect_uri.as_str()),
            ("code_challenge", &code_challenge),
            ("code_challenge_method", &method),
            ("scope", &scope),
        ];

        if let Some(state) = state {
            extra.push(("state", state));
        }

        let query = serde_urlencoded::to_string(extra).unwrap();

        Self {
            query,
            client_name,
            username,
            scopes: grant.scope.iter().collect::<Vec<_>>().join(", "),
        }
    }
}
