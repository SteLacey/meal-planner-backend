mod db;
mod entity;

#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket_db_pools::{Connection, Database};
use sea_orm::EntityTrait;
use db::Db;
use entity::{recipes, recipes::Entity as Recipe};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/get_recipes")]
async fn get_recipes(conn: Connection<Db>) -> Json<Vec<recipes::Model>> {
    let db = conn.into_inner();
    Json(Recipe::find().all(&db).await.expect("Error finding recipes"))
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![index, get_recipes])
        .launch()
        .await?;

    Ok(())
}
