use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum))]
#[serde(rename_all = "snake_case")]
pub enum GearType {
    #[default]
    RoadBike,
    HybridBike,
    TimeTrialBike,
    OffroadBike,
    RunningShoes,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(
    feature = "graphql",
    derive(async_graphql::SimpleObject, async_graphql::InputObject)
)]
#[cfg_attr(feature = "graphql", graphql(input_name = "GearInput", name = "_Gear"))]
pub struct Gear {
    pub name: String,
    pub gear_type: GearType,
}
