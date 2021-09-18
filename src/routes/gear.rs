use crate::error::{Error, Result};
use actix_web::{http, web, HttpRequest, HttpResponse, Responder};
use std::ops::Deref;
use tf_database::{
    query::{GearQuery, UserQuery},
    Database,
};
use tf_macro::protect;
use tf_models::{backend::Gear, frontend::Totals, Unit};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{user_id}/gear/{id}")
            .name("gear")
            .route(web::get().to(get_gear))
            .route(web::put().to(put_gear))
            .route(web::delete().to(delete_gear)),
    )
    .service(
        web::resource("/{user_id}/gear/{id}/totals")
            .name("gear_totals")
            .route(web::get().to(get_totals)),
    )
    .service(
        web::resource("/{user_id}/gear")
            .name("gear_index")
            .route(web::get().to(get_gear_index))
            .route(web::post().to(post_gear_index)),
    );
}

#[protect]
async fn get_gear(
    db: web::Data<Database>,
    query: web::Path<GearQuery<'_>>,
) -> Result<impl Responder> {
    let gear = db.gear.get_gear(&query)?.ok_or(Error::NotFound)?;

    Ok(web::Json(gear))
}

#[protect]
async fn put_gear(
    db: web::Data<Database>,
    query: web::Path<GearQuery<'_>>,
    gear: web::Json<Gear>,
) -> Result<impl Responder> {
    if db.gear.contains_gear(&query)? {
        db.gear
            .insert_gear(&query.into_inner(), gear.into_inner())?;

        Ok(HttpResponse::NoContent())
    } else {
        Ok(HttpResponse::NotFound())
    }
}

#[protect]
async fn post_gear_index(
    db: web::Data<Database>,
    query: web::Path<UserQuery<'_>>,
    gear: web::Json<Gear>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let id = nanoid::nanoid!(10);
    let gear_query = GearQuery::from((query.deref(), id.as_str()));

    db.gear.insert_gear(&gear_query, gear.into_inner())?;

    let url = req
        .url_for("gear", &[&gear_query.user_id, &gear_query.id])
        .unwrap();

    Ok(HttpResponse::Created()
        .insert_header((http::header::LOCATION, url.to_string()))
        .finish())
}

#[protect]
async fn get_gear_index(
    db: web::Data<Database>,
    query: web::Path<UserQuery<'_>>,
) -> Result<impl Responder> {
    let gears = db.gear.iter_gear(&query)?;
    let ids = db.gear.iter_id(&query)?;

    let gears: Vec<_> = gears
        .zip(ids)
        .map(|(x, y)| tf_models::frontend::Gear::from_backend(x, y))
        .collect();

    Ok(web::Json(gears))
}

#[protect]
async fn delete_gear(
    db: web::Data<Database>,
    query: web::Path<GearQuery<'_>>,
) -> Result<impl Responder> {
    db.gear.remove_gear(&query)?;

    Ok(HttpResponse::NoContent())
}

#[protect]
async fn get_totals(
    db: web::Data<Database>,
    query: web::Path<GearQuery<'_>>,
    unit: Option<web::Query<Unit>>,
) -> Result<impl Responder> {
    let user_query: UserQuery = query.deref().into();

    let filter = |id: &Option<String>| id.as_deref() == Some(&query.id);

    let unit = unit.as_deref().unwrap_or_default();
    let totals = db
        .activity
        .username_iter_session(&user_query)?
        .zip(db.activity.username_iter_gear(&user_query)?)
        .filter(|(_, y)| filter(y))
        .map(|(x, _)| x)
        .fold(Totals::new(unit), |acc, x| Totals::fold(acc, &x, unit));

    Ok(web::Json(totals))
}
