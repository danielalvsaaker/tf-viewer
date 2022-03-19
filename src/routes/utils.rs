/*
use crate::{
    error::{Error, ErrorKind, Result},
    models::{Duration, Record, Unit},
};
use actix_web::web;
use staticmap::{
    tools::{Color, LineBuilder},
    StaticMapBuilder,
};
use uom::si::length::{foot, kilometer, meter, mile};
use uom::si::velocity::{kilometer_per_hour, mile_per_hour};


pub fn generate_thumb(record: Record, path: &std::path::PathBuf) -> Result<()> {
    if record.lon.is_empty() {
        return Ok(());
    }

    let mut map = StaticMapBuilder::default()
        .width(200)
        .height(200)
        .url_template("https://a.tile.openstreetmap.org/{z}/{x}/{y}.png")
        .build()
        .unwrap();

    let line = LineBuilder::default()
        .width(3.)
        .simplify(true)
        .lon_coordinates(record.lon.into_iter().flatten().collect::<Vec<f64>>())
        .lat_coordinates(record.lat.into_iter().flatten().collect::<Vec<f64>>())
        .color(Color::new(true, 255, 0, 0, 255))
        .tolerance(2)
        .build()
        .unwrap();

    map.add_line(line);

    if !path.exists() {
        std::fs::create_dir_all(path.parent().unwrap())
            .map_err(|_| Error::BadServerResponse("Failed to create thumbnail directory"))?;
    }

    map.save_png(path)
        .map_err(|_| Error::BadServerResponse("Failed to save rendered activity thumbnail"))?;

    Ok(())
}
*/
use std::time::Duration;
use tf_models::activity::Record;

pub fn zone_duration(record: &Record, rest: u8, max: u8) -> Option<[Duration; 6]> {
    let (rest, max): (f32, f32) = (rest.into(), max.into());
    let zones: [f32; 7] = [
        rest,
        max * 0.55,
        max * 0.72,
        max * 0.82,
        max * 0.87,
        max * 0.92,
        max * 3.00,
    ];
    let zones_duration: [Duration; 6] = [Duration::default(); 6];

    let zones_duration = record
        .duration
        .as_slice()
        .windows(2)
        .map(|x| [x[0].as_ref(), x[1].as_ref()])
        .zip(record.heartrate.iter())
        .fold(
            zones_duration,
            |mut acc: [Duration; 6], (d, h): ([&Duration; 2], &Option<u8>)| {
                if let Some(h) = h {
                    let time_diff = *d[1] - *d[0];
                    if time_diff < Duration::from_secs_f64(30.0) {
                        for (i, z) in zones.windows(2).enumerate() {
                            if (z[0]..z[1]).contains(&f32::from(*h)) {
                                acc[i] += time_diff;
                            }
                        }
                    }
                }
                acc
            },
        );

    Some(zones_duration)
}
