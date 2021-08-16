use crate::{Unit, Value};
use chrono::{offset::Local, DateTime};
use serde::Serialize;
use std::time::Duration;

use uom::si::length::{foot, kilometer, meter, mile};
use uom::si::velocity::{kilometer_per_hour, mile_per_hour};

#[derive(Serialize)]
pub struct Record<'a> {
    pub timestamp: Vec<Option<DateTime<Local>>>,
    pub duration: Vec<Duration>,

    pub distance: Value<'a, Vec<Option<f64>>>,

    pub cadence: Value<'a, Vec<Option<u8>>>,

    pub altitude: Value<'a, Vec<Option<f64>>>,

    pub speed: Value<'a, Vec<Option<f64>>>,

    pub heartrate: Value<'a, Vec<Option<u8>>>,

    pub power: Value<'a, Vec<Option<u16>>>,

    pub lat: Vec<Option<f64>>,
    pub lon: Vec<Option<f64>>,
}

impl<'a> Record<'a> {
    pub fn from_backend(r: crate::backend::Record, unit: &Unit) -> Self {
        Self {
            timestamp: r.timestamp,
            duration: r.duration,

            distance: Value::<Vec<Option<f64>>>::from_length_iter::<kilometer, mile>(
                r.distance, unit,
            ),

            cadence: Value::new(r.cadence, "rpm"),

            altitude: Value::<Vec<Option<f64>>>::from_length_iter::<meter, foot>(r.altitude, unit),

            speed: Value::<Vec<Option<f64>>>::from_velocity_iter::<kilometer_per_hour, mile_per_hour>(
                r.speed, unit,
            ),

            heartrate: Value::new(r.heartrate, "bpm"),

            power: Value::<Vec<Option<u16>>>::from_power_iter(r.power),

            lat: r.lat,
            lon: r.lon,
        }
    }
}
