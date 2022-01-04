mod sport;
pub use sport::{Sport, SPORTS};
pub mod activity;
pub mod gear;
pub mod user;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Activity {
    pub id: String,
    pub gear_id: Option<String>,
    pub session: activity::Session,
    pub record: activity::Record,
    pub lap: Vec<activity::Lap>,
}
