use oxide_auth::endpoint::QueryParameter;
use std::borrow::Cow;

pub fn base_template(title: &str, template: &str) -> String {
    format!(
        include_str!("../templates/base.html"),
        switcher_script = include_str!("../static/js/minimal-switcher.js"),
        template = template,
        title = title
    )
}

pub fn index_template() -> String {
    base_template("Index", include_str!("../templates/index.html"))
}

pub fn signin_template(query: &str) -> String {
    base_template(
        "Sign in",
        &format!(include_str!("../templates/signin.html"), query = query),
    )
}

pub fn signup_template(query: &str) -> String {
    base_template(
        "Sign up",
        &format!(include_str!("../templates/signup.html"), query = query),
    )
}

pub fn authorize_template(
    req: &oxide_auth_actix::OAuthRequest,
    solicitation: oxide_auth::endpoint::Solicitation,
    user_id: &str,
) -> String {
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

    base_template(
        "Authorize",
        &format!(
            include_str!("../templates/authorize.html"),
            query = query,
            client_id = grant.client_id,
            user_id = user_id,
            scopes = grant.scope.iter().collect::<Vec<_>>().join(", ")
        ),
    )
}
