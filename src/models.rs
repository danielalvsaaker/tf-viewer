use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Activity {
    pub id: u8,
    pub gear_id: u8,
    pub session: crate::parser::Session,
    pub record: crate::parser::Record,
    pub lap: Vec<crate::parser::Lap>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gear {
    pub name: String,
    pub kind: String,
    pub fixed_distance: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub heartrate_rest: u32,
    pub heartrate_max: u32,
    pub age: u32,
    pub height: u32,
    pub weight: u32,
    pub standard_gear: String,
}
