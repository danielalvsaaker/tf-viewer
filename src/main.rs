mod database;
mod error;
mod models;
pub mod parser;
use std::fs;

pub use database::Database;
pub use models::{Activity, User, Gear};
pub use parser::*;
pub use error::{Error, Result};

use actix_web::{App, HttpServer};

/*
fn main() {
    let data = Database::load_or_create().expect("Failed to load");

    let parsed = parser::parse(&fs::read("test.fit").unwrap());

    let user = User {
        heartrate_rest: 50_u32,
        heartrate_max: 205u32,
        age: 20u32,
        height: 190u32,
        weight: 80u32,
        standard_gear: String::from("Specialized Venge"),
    };

    let hash = String::from("password hash");


    data.users.insert(user, String::from("daniel"), hash);
    let get = data.users.get(String::from("daniel")).unwrap();

    //data.gear.insert(new, String::from("daniel")).unwrap();
    //let get = data.gear.get(String::from("daniel"), String::from("Specialized Venge")).unwrap();
    println!("{:#?}", get);

}*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
    })
    .bind("127.0.0.1:2000")?
    .run()
    .await

}
