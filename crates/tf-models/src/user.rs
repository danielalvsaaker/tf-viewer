use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
#[cfg_attr(
    feature = "graphql",
    derive(async_graphql::SimpleObject, async_graphql::InputObject)
)]
#[cfg_attr(feature = "graphql", graphql(input_name = "UserInput", name = "_User"))]
pub struct User {
    #[serde(default)]
    pub name: String,
    pub heartrate_rest: u8,
    pub heartrate_max: u8,
}
