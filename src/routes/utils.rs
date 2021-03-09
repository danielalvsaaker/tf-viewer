use super::PasswordEnum;
use crate::{
    error::{Error, ErrorKind, Result},
    models::{Duration, Record, Unit},
};
use actix_web::web;
use plotly::{
    common::Mode,
    layout::{Axis, Layout},
    Plot, Scatter,
};
use staticmap::{
    tools::{Color, LineBuilder},
    StaticMapBuilder,
};
use uom::si::length::{foot, kilometer, meter, mile};
use uom::si::velocity::{kilometer_per_hour, mile_per_hour};

pub fn validate_form(form: &super::PasswordEnum, data: &web::Data<crate::Database>) -> Result<()> {
    let verify_hash = |username, password| data.users.verify_hash(username, password);

    let valid_username = |username| {
        let username_regex = regex::Regex::new(r#"^[a-zA-Z0-9-_]{2,15}$"#).unwrap();
        if username_regex.is_match(username) {
            Ok(())
        } else {
            Err(Error::BadRequest(
                ErrorKind::BadRequest,
                "Invalid username supplied",
            ))
        }
    };

    let valid_password = |password| {
        let password_regex =
            regex::Regex::new(r#"^(.{0,13}|[^0-9]*|[^A-Z]*|[^a-z]*|[a-zA-Z0-9]*)$"#).unwrap();
        if password_regex.is_match(password) {
            Err(Error::BadRequest(
                ErrorKind::BadRequest,
                "Invalid password supplied",
            ))
        } else {
            Ok(())
        }
    };

    let user_exists = |username| {
        if data.users.exists(username).is_ok() {
            Err(Error::BadRequest(
                ErrorKind::BadRequest,
                "Username is not available",
            ))
        } else {
            Ok(())
        }
    };

    let password_compare = |password, confirm_password| {
        if password == confirm_password {
            Ok(())
        } else {
            Err(Error::BadRequest(
                ErrorKind::BadRequest,
                "Passwords do not match",
            ))
        }
    };

    if let PasswordEnum::Signup(form) = form {
        valid_username(&form.username)?;
        user_exists(&form.username)?;
        valid_password(&form.password)?;
        password_compare(&form.password, &form.confirm_password)?;
    } else if let PasswordEnum::Settings(username, form) = form {
        verify_hash(&username, &form.current_password)?;
        valid_password(&form.new_password)?;
        password_compare(&form.new_password, &form.confirm_password)?;
    }

    Ok(())
}

pub fn plot(record: &Record, unit: &Unit) -> Result<String> {
    // x-axis
    let distance = record.distance.iter().map(|x| match unit {
        Unit::Metric => x.map(|y| format!("{:.2}", y.get::<kilometer>())),
        Unit::Imperial => x.map(|y| format!("{:.2}", y.get::<mile>())),
    });

    // Heart rate
    let heartrate = Scatter::new(distance.clone(), record.heartrate.clone())
        .mode(Mode::Lines)
        .name("Heart rate");

    // Speed
    let speed_map = record.speed.iter().map(|x| match unit {
        Unit::Metric => x.map(|y| format!("{:.2}", y.get::<kilometer_per_hour>())),
        Unit::Imperial => x.map(|y| format!("{:.2}", y.get::<mile_per_hour>())),
    });

    let speed = Scatter::new(distance.clone(), speed_map)
        .mode(Mode::Lines)
        .name("Speed");

    // Altitude
    let altitude_map = record.altitude.iter().map(|x| match unit {
        Unit::Metric => x.map(|y| format!("{:.2}", y.get::<meter>())),
        Unit::Imperial => x.map(|y| format!("{:.2}", y.get::<foot>())),
    });

    let altitude = Scatter::new(distance, altitude_map)
        .mode(Mode::Lines)
        .name("Altitude");

    let mut plot = Plot::new();

    let tick_suffix = match unit {
        Unit::Metric => " km",
        Unit::Imperial => " mi",
    };

    let axis = Axis::new().tick_suffix(tick_suffix);
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

pub fn zone_duration(
    record: &Record,
    heartrate: &Option<(u8, u8)>,
) -> Result<Option<Vec<Duration>>> {
    let mut zones: Vec<u8> = Vec::with_capacity(7);
    let mut zones_duration: Vec<Duration> = vec![
        Duration::default(),
        Duration::default(),
        Duration::default(),
        Duration::default(),
        Duration::default(),
        Duration::default(),
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
