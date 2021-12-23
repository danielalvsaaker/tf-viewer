use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GearType {
    RoadBike,
    HybridBike,
    TimeTrialBike,
    OffroadBike,
    RunningShoes,
}

#[derive(Serialize, Deserialize)]
pub struct Gear {
    pub id: String,
    pub name: String,
    pub gear_type: GearType,
}
