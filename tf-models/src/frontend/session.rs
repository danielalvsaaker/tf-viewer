use crate::{ActivityType, Unit, Value};
use serde::{Deserialize, Serialize};

use chrono::{offset::Local, DateTime};
use std::time::Duration;
use uom::si::length::{foot, kilometer, meter, mile};
use uom::si::velocity::{kilometer_per_hour, mile_per_hour};

#[derive(Serialize, Deserialize)]
pub struct Session<'a> {
    pub start_time: Option<DateTime<Local>>,

    pub activity_type: ActivityType,

    pub duration: Duration,
    pub duration_active: Duration,

    pub distance: Option<Value<'a, f64>>,

    pub cadence_avg: Option<Value<'a, u8>>,
    pub cadence_max: Option<Value<'a, u8>>,

    pub heartrate_avg: Option<Value<'a, u8>>,
    pub heartrate_max: Option<Value<'a, u8>>,

    pub speed_avg: Option<Value<'a, f64>>,
    pub speed_max: Option<Value<'a, f64>>,

    pub power_avg: Option<Value<'a, u16>>,
    pub power_max: Option<Value<'a, u16>>,

    pub ascent: Option<Value<'a, u32>>,
    pub descent: Option<Value<'a, u32>>,

    pub calories: Option<Value<'a, u16>>,

    pub nec_lat: Option<f64>,
    pub nec_lon: Option<f64>,
    pub swc_lat: Option<f64>,
    pub swc_lon: Option<f64>,

    pub laps: Option<u16>,
}

impl<'a> Session<'a> {
    pub fn from_backend(s: crate::backend::Session, unit: &Unit) -> Self {
        Self {
            start_time: s.start_time,
            activity_type: s.activity_type,
            duration: s.duration,
            duration_active: s.duration_active,

            distance: s
                .distance
                .map(|x| Value::<f64>::from_length::<kilometer, mile>(x, unit)),

            cadence_avg: s.cadence_avg.map(|x| Value::new(x, "rpm")),
            cadence_max: s.cadence_max.map(|x| Value::new(x, "rpm")),

            heartrate_avg: s.heartrate_avg.map(|x| Value::new(x, "bpm")),
            heartrate_max: s.heartrate_max.map(|x| Value::new(x, "bpm")),

            speed_avg: s
                .speed_avg
                .map(|x| Value::<f64>::from_velocity::<kilometer_per_hour, mile_per_hour>(x, unit)),
            speed_max: s
                .speed_max
                .map(|x| Value::<f64>::from_velocity::<kilometer_per_hour, mile_per_hour>(x, unit)),

            power_avg: s.power_avg.map(Value::from_power),
            power_max: s.power_max.map(Value::from_power),

            ascent: s
                .ascent
                .map(|x| Value::<u32>::from_length::<meter, foot>(x, unit)),
            descent: s
                .descent
                .map(|x| Value::<u32>::from_length::<meter, foot>(x, unit)),

            calories: s.calories.map(|x| Value::new(x, "kcal")),

            nec_lat: s.nec_lat,
            nec_lon: s.nec_lon,
            swc_lat: s.swc_lat,
            swc_lon: s.swc_lon,

            laps: s.laps,
        }
    }
}
