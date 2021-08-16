use crate::{Unit, Value};
use serde::Serialize;

use uom::si::length::{foot, kilometer, meter, mile};
use uom::si::velocity::{kilometer_per_hour, mile_per_hour};

use crate::backend;
use std::time::Duration;

#[derive(Serialize)]
pub struct Lap<'a> {
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

    pub lat_start: Option<f64>,
    pub lon_start: Option<f64>,
    pub lat_end: Option<f64>,
    pub lon_end: Option<f64>,
}

impl<'a> Lap<'a> {
    pub fn from_backend_iter<I>(i: I, unit: &Unit) -> Vec<Self>
    where
        I: IntoIterator<Item = backend::Lap>,
    {
        i.into_iter().map(|x| Self::from_backend(x, unit)).collect()
    }

    pub fn from_backend(l: crate::backend::Lap, unit: &Unit) -> Self {
        Self {
            duration: l.duration,
            duration_active: l.duration_active,

            distance: l
                .distance
                .map(|x| Value::<f64>::from_length::<kilometer, mile>(x, unit)),

            cadence_avg: l.cadence_avg.map(|x| Value::new(x, "rpm")),
            cadence_max: l.cadence_max.map(|x| Value::new(x, "rpm")),

            heartrate_avg: l.heartrate_avg.map(|x| Value::new(x, "bpm")),
            heartrate_max: l.heartrate_max.map(|x| Value::new(x, "bom")),

            speed_avg: l
                .speed_avg
                .map(|x| Value::<f64>::from_velocity::<kilometer_per_hour, mile_per_hour>(x, unit)),
            speed_max: l
                .speed_max
                .map(|x| Value::<f64>::from_velocity::<kilometer_per_hour, mile_per_hour>(x, unit)),

            power_avg: l.power_avg.map(Value::from_power),
            power_max: l.power_max.map(Value::from_power),

            ascent: l
                .ascent
                .map(|x| Value::<u32>::from_length::<meter, foot>(x, unit)),
            descent: l
                .descent
                .map(|x| Value::<u32>::from_length::<meter, foot>(x, unit)),

            calories: l.calories.map(|x| Value::new(x, "kcal")),

            lat_start: l.lat_start,
            lon_start: l.lon_start,
            lat_end: l.lat_end,
            lon_end: l.lon_end,
        }
    }
}
