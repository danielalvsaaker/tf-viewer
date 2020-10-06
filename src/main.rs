mod database;
mod error;
mod models;
pub mod parser;
use std::fs;
use std::sync::Mutex;

pub use database::Database;
pub use models::{Activity, User, Gear};
pub use parser::*;
pub use error::{Error, Result};

use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(Mutex::new(Database::load_or_create().expect("Failed to load")));

    println!("Running at 127.0.0.1:2000");

    HttpServer::new(move || {
        App::new()
        .app_data(data.clone())
    })
    .bind("127.0.0.1:2000")?
    .run()
    .await

}
