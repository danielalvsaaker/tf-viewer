use crate::error::{Error, Result};
use axum::{
    extract::{Extension, Path},
    http::{self, HeaderValue, StatusCode},
    response::{Headers, IntoResponse},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use tf_database::{
    query::{GearQuery, UserQuery},
    Database,
};
use tf_macro::oauth;
use tf_models::gear::{Gear, GearType};

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_gear_index).post(post_gear_index))
        .route("/:id", get(get_gear).put(put_gear).delete(delete_gear))
}

#[oauth(scopes = ["gear:read"])]
async fn get_gear(
    Extension(db): Extension<Database<'_>>,
    Path(query): Path<GearQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.gear.gear.get(&query)?.map(Json).ok_or(Error::NotFound)
}

#[derive(Deserialize)]
struct GearForm {
    name: String,
    gear_type: GearType,
}

#[oauth(scopes = ["gear:write"])]
async fn put_gear(
    Extension(db): Extension<Database<'_>>,
    Path(query): Path<GearQuery<'_>>,
    Json(gear): Json<Gear>,
) -> Result<impl IntoResponse> {
    /*
    if db.gear.contains_gear(&query)? {
        db.gear.insert_gear(&query, gear)?;

        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
    */
    Ok("")
}

#[oauth(scopes = ["gear:write"])]
async fn post_gear_index(
    Extension(db): Extension<Database<'_>>,
    Path(query): Path<UserQuery>,
    Json(gear): Json<GearForm>,
) -> Result<impl IntoResponse> {
    let id = nanoid::nanoid!(10);
    let gear = Gear {
        owner: query.user_id.to_owned(),
        id: id.clone(),
        name: gear.name,
        gear_type: gear.gear_type,
    };

    db.gear.gear.insert(&gear)?;

    let gear_query = GearQuery::from(&gear);

    let url = format!("/user/{}/gear/{}", gear_query.user_id, gear_query.id);

    Ok(Headers(vec![(
        http::header::LOCATION,
        HeaderValue::from_str(&url).unwrap(),
    )]))
}

#[oauth(scopes = ["gear:read"])]
async fn get_gear_index(
    Extension(db): Extension<Database<'_>>,
    Path(query): Path<UserQuery>,
) -> Result<impl IntoResponse> {
    /*
    let gears = db.gear.iter_gear(&query)?.collect::<Vec<_>>();

    Ok(Json(gears))
    */
    Ok("")
}

#[oauth(scopes = ["gear:write"])]
async fn delete_gear(
    Extension(db): Extension<Database<'_>>,
    Path(query): Path<GearQuery<'_>>,
) -> Result<impl IntoResponse> {
    //db.gear.remove_gear(&query)?;

    Ok(StatusCode::NO_CONTENT)
}

/*
#[protect]
async fn get_totals(db: web::Data<Database>, query: web::Path<GearQuery<'_>>) -> Result<String> {
    let user_query: UserQuery = query.deref().into();

    let filter = |id: &Option<String>| id.as_deref() == Some(&query.id);

    /*
    let totals = db
        .activity
        .username_iter_session(&user_query)?
        .zip(db.activity.username_iter_gear(&user_query)?)
        .filter(|(_, y)| filter(y))
        .map(|(x, _)| x)
        .fold(Totals::new(unit), |acc, x| Totals::fold(acc, &x, unit));

    Ok(web::Json(totals))
    */
    todo!()
}
*/
