use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    #[cfg(not(feature="embedded_static"))]
    cfg.service(actix_files::Files::new("/static", "static"))
    .service(web::resource("/static").name("static"));

    #[cfg(feature="embedded_static")]
    cfg.service(
        web::resource("/static/css/{file}")
            .route(web::get().to(serve_css))
    )
    .service(
        web::resource("/static/js/{file}")
            .route(web::get().to(serve_js))
    )
    .service(
        web::resource("/static/img/favicon.png")
            .route(web::get().to(serve_favicon))
    )
    .service(actix_files::Files::new("/static/img/activity", "static/img/activity"))
    .service(web::resource("/static").name("static"));
}

#[cfg(feature="embedded_static")]
use {
    actix_web::HttpResponse,
    crate::error::{Error, ErrorKind, Result}
};

#[cfg(feature="embedded_static")]
async fn serve_css(file: web::Path<String>) -> Result<HttpResponse> {
    static DATATABLES_CSS: &[u8] = include_bytes!("../static/css/datatables.min.css");
    static DROPZONE_CSS: &[u8] = include_bytes!("../static/css/dropzone.min.css");
    static LEAFLET_CSS: &[u8] = include_bytes!("../static/css/leaflet.css");
    static SPECTRE_CSS: &[u8] = include_bytes!("../static/css/spectre.min.css");
    static SPECTRE_ICONS_CSS: &[u8] = include_bytes!("../static/css/spectre-icons.min.css");
    static STYLESHEET_CSS: &[u8] = include_bytes!("../static/css/stylesheet.css");

    let body = match file.into_inner().as_str() {
        "datatables.min.css" => Ok(DATATABLES_CSS),
        "dropzone.min.css" => Ok(DROPZONE_CSS),
        "leaflet.css" => Ok(LEAFLET_CSS),
        "spectre.min.css" => Ok(SPECTRE_CSS),
        "spectre-icons.min.css" => Ok(SPECTRE_ICONS_CSS),
        "stylesheet.css" => Ok(STYLESHEET_CSS),
        _ => Err(Error::BadRequest(ErrorKind::NotFound, "File not found")),
    };


    Ok(
        HttpResponse::Ok()
        .body(body?)
    )
}

#[cfg(feature="embedded_static")]
async fn serve_js(file: web::Path<String>) -> Result<HttpResponse> {
    static DATATABLES_JS: &[u8] = include_bytes!("../static/js/datatables.min.js");
    static DROPZONE_JS: &[u8] = include_bytes!("../static/js/dropzone.min.js");
    static LEAFLET_JS: &[u8] = include_bytes!("../static/js/leaflet.js");
    static PLOTLY_JS: &[u8] = include_bytes!("../static/js/plotly-basic.min.js");

    let body = match file.into_inner().as_str() {
        "datatables.min.js" => Ok(DATATABLES_JS),
        "dropzone.min.js" => Ok(DROPZONE_JS),
        "leaflet.js" => Ok(LEAFLET_JS),
        "plotly-basic.min.js" => Ok(PLOTLY_JS),
        _ => Err(Error::BadRequest(ErrorKind::NotFound, "File not found")),
    };

    Ok(
        HttpResponse::Ok()
        .body(body?)
    )
}

#[cfg(feature="embedded_static")]
async fn serve_favicon() -> HttpResponse {
    static FAVICON: &[u8] = include_bytes!("../static/img/favicon.png");

    HttpResponse::Ok()
        .body(FAVICON)
}
