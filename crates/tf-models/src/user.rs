use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "graphql", graphql(name = "_User"))]
pub struct User {
    #[serde(default)]
    pub name: String,
    pub heartrate_rest: u8,
    pub heartrate_max: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Password<'a> {
    pub password: std::borrow::Cow<'a, str>,
}
