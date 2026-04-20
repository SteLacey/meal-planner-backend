#[macro_use]
extern crate rocket;
mod api;
mod db;
mod entity;
mod view;

use crate::api::ingredient::ingredient_routes;
use api::api::api_routes;
use db::Db;
use migration::MigratorTrait;
use rocket::fairing::AdHoc;
use rocket::{Build, Rocket, fairing};
use rocket_db_pools::Database;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Database Migrations", run_migrations))
        .mount("/", routes![index])
        .mount("/api", api_routes())
        .mount("/api", ingredient_routes())
        .launch()
        .await?;

    Ok(())
}
