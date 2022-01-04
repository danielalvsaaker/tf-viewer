use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub heartrate_rest: u8,
    pub heartrate_max: u8,
}
