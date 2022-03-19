use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum))]
#[serde(rename_all = "snake_case")]
pub enum GearType {
    RoadBike,
    HybridBike,
    TimeTrialBike,
    OffroadBike,
    RunningShoes,
}

impl Default for GearType {
    fn default() -> Self {
        Self::RoadBike
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(feature = "graphql", derive(async_graphql::SimpleObject))]
#[cfg_attr(feature = "graphql", graphql(name = "_Gear"))]
pub struct Gear {
    pub name: String,
    pub gear_type: GearType,
}
