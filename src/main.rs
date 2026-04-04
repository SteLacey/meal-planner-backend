mod db;
mod entity;
mod api;
mod view;

#[macro_use] extern crate rocket;

use rocket_db_pools::{Database};
use db::Db;
use api::api::api_routes;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![index])
        .mount("/api", api_routes())
        .launch()
        .await?;

    Ok(())
}
