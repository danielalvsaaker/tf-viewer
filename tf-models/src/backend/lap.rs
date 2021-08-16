use serde::{Deserialize, Serialize};
use std::time::Duration;

use uom::si::f64::{Length as Length_f64, Velocity};
use uom::si::u16::Power;
use uom::si::u32::Length as Length_u32;

#[derive(Default, Serialize, Deserialize)]
pub struct Lap {
    pub cadence_avg: Option<u8>,
    pub cadence_max: Option<u8>,
    pub heartrate_avg: Option<u8>,
    pub heartrate_max: Option<u8>,
    pub speed_avg: Option<Velocity>,
    pub speed_max: Option<Velocity>,
    pub power_avg: Option<Power>,
    pub power_max: Option<Power>,
    pub lat_start: Option<f64>,
    pub lon_start: Option<f64>,
    pub lat_end: Option<f64>,
    pub lon_end: Option<f64>,
    pub ascent: Option<Length_u32>,
    pub descent: Option<Length_u32>,
    pub calories: Option<u16>,
    pub distance: Option<Length_f64>,
    pub duration: Duration,
    pub duration_active: Duration,
}
