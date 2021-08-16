use super::backend;
use crate::{Unit, Value};
use serde::Serialize;

use uom::si::length::{kilometer, mile};

#[derive(Serialize)]
pub struct Totals<'a> {
    pub distance: Value<'a, f64>,
    pub duration: std::time::Duration,
    pub count: usize,
}

impl<'a> Totals<'a> {
    pub fn new(unit: &Unit) -> Self {
        Self {
            distance: Value::<f64>::from_length::<kilometer, mile>(Default::default(), unit),
            duration: Default::default(),
            count: Default::default(),
        }
    }

    pub fn fold(mut self, session: &backend::Session, unit: &Unit) -> Self {
        if let Some(distance) = session.distance {
            self.distance += Value::<f64>::from_length::<kilometer, mile>(distance, unit);
        }
        self.duration += session.duration;
        self.count += 1;
        self
    }
}
