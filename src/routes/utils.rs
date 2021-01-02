use crate::{
    error::{Error, Result},
    models::{Duration, Record},
};
use plotly::{
    common::Mode,
    layout::{Axis, Layout},
    Plot, Scatter,
};
use staticmap::{Color, Line, StaticMap};

pub fn plot(record: &Record) -> Result<String> {
    let heartrate = Scatter::new(record.distance.clone(), record.heartrate.clone())
        .mode(Mode::Lines)
        .name("Heart rate");
    let speed = Scatter::new(record.distance.clone(), record.speed.clone())
        .mode(Mode::Lines)
        .name("Speed");
    let altitude = Scatter::new(record.distance.clone(), record.altitude.clone())
        .mode(Mode::Lines)
        .name("Altitude");

    let mut plot = Plot::new();

    let axis = Axis::new().tick_suffix(" km");
    let layout = Layout::new().x_axis(axis);

    plot.set_layout(layout);
    plot.add_trace(heartrate);
    plot.add_trace(speed);
    plot.add_trace(altitude);

    Ok(plot.to_inline_html(None))
}

pub fn generate_thumb(record: Record, path: &std::path::PathBuf) -> Result<()> {
    if record.lon.is_empty() {
        return Ok(());
    }

    let mut map = StaticMap {
        width: 200,
        height: 200,
        padding: (0, 0), // (x, y)
        x_center: 0.,
        y_center: 0.,
        //url_template: "https://a.tile.openstreetmap.org/%z/%x/%y.png".to_string(),
        url_template: "http://a.tile.komoot.de/komoot-2/%z/%x/%y.png".to_string(),
        tile_size: 256,
        lines: Vec::new(),
        zoom: 0,
    };

    let coordinates: Vec<(f64, f64)> = record
        .lon
        .into_iter()
        .flatten()
        .zip(record.lat.into_iter().flatten())
        .collect();

    let line = Line {
        coordinates,
        color: Color {
            r: 255u8,
            g: 0u8,
            b: 0u8,
            a: 255u8,
        },
        width: 6.,
        simplify: true,
    };

    map.add_line(line);

    let image = map
        .render()
        .map_err(|_| Error::BadServerResponse("Failed to render activity thumbnail"))?;
    image
        .save(path)
        .map_err(|_| Error::BadServerResponse("Failed to save rendered activity thumbnail"))?;
    Ok(())
}

pub fn zone_duration(
    record: &Record,
    heartrate: &Option<(u8, u8)>,
) -> Result<Option<Vec<Duration>>> {
    let mut zones: Vec<u8> = Vec::with_capacity(7);
    let mut zones_duration: Vec<Duration> = vec![
        Duration::new(),
        Duration::new(),
        Duration::new(),
        Duration::new(),
        Duration::new(),
        Duration::new(),
    ];

    if let Some(x) = heartrate {
        zones.push(x.0);
        zones.push((x.1 as f32 * 0.55).round() as u8);
        zones.push((x.1 as f32 * 0.72).round() as u8);
        zones.push((x.1 as f32 * 0.82).round() as u8);
        zones.push((x.1 as f32 * 0.87).round() as u8);
        zones.push((x.1 as f32 * 0.92).round() as u8);
        // Safety measure, considering that measured heartrate can spike
        zones.push((x.1 as f32 * 3.00).round() as u8);
    } else {
        return Ok(None);
    }

    let duration_iter = record.duration.iter();
    let heartrate_iter = record.heartrate.iter();

    // This can probably be done in a prettier way
    for ((i, duration), heartrate) in duration_iter.enumerate().zip(heartrate_iter) {
        if i > 0 {
            let time_diff = *duration - record.duration[i - 1];
            if let Some(x) = heartrate {
                if time_diff < Duration::from_secs_f64(30.0) {
                    for j in 0..=5 {
                        if x <= &zones[j + 1] && x >= &zones[j] {
                            zones_duration[j] += time_diff;
                        }
                    }
                }
            }
        }
    }

    Ok(Some(zones_duration))
}
