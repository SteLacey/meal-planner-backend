mod db;
mod entity;
mod api;
mod view;

#[macro_use] extern crate rocket;

use rocket::{fairing, Build, Rocket};
use rocket::fairing::AdHoc;
use rocket_db_pools::{Database};
use db::Db;
use api::api::api_routes;
use migration::MigratorTrait;

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
        .launch()
        .await?;

    Ok(())
}
