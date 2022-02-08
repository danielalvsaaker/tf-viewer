use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
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

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Gear {
    pub owner: String,
    pub id: String,
    pub name: String,
    pub gear_type: GearType,
}
