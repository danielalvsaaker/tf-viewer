use chrono::{offset::Local, DateTime};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use uom::si::f64::{Length as Length_f64, Velocity};
use uom::si::u16::Power;

#[derive(Default, Serialize, Deserialize)]
pub struct Record {
    pub cadence: Vec<Option<u8>>,
    pub distance: Vec<Option<Length_f64>>,
    pub altitude: Vec<Option<Length_f64>>,
    pub speed: Vec<Option<Velocity>>,
    pub heartrate: Vec<Option<u8>>,
    pub power: Vec<Option<Power>>,
    pub lat: Vec<Option<f64>>,
    pub lon: Vec<Option<f64>>,
    pub timestamp: Vec<Option<DateTime<Local>>>,
    pub duration: Vec<Duration>,
}
