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
    solicitation: oxide_auth::endpoint::Solicitation,
    user_id: &str,
) -> String {
    let grant = solicitation.pre_grant();
    let state = solicitation.state();

    let mut extra = vec![
        ("response_type", "code"),
        ("client_id", grant.client_id.as_str()),
        ("redirect_uri", grant.redirect_uri.as_str()),
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
            user_id = user_id
        ),
    )
}
