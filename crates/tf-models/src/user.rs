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

#[derive(Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(
    feature = "graphql",
    derive(async_graphql::SimpleObject, async_graphql::InputObject)
)]
#[cfg_attr(
    feature = "graphql",
    graphql(input_name = "ZonesInput", name = "Zones")
)]
pub struct Zones {
    z1: f32,
    z2: f32,
    z3: f32,
    z4: f32,
    z5: f32,
}

impl Default for Zones {
    fn default() -> Self {
        Self {
            z1: 0.55,
            z2: 0.72,
            z3: 0.82,
            z4: 0.87,
            z5: 0.92,
        }
    }
}
