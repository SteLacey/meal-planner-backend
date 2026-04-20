use crate::db::Db;
use crate::entity::prelude::{Recipes, Tags};
use crate::view::response::recipe::Recipe;
use crate::view::response::tag::Tag;
use rocket::futures::future::join_all;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sea_orm::{EntityTrait};

#[get("/get_recipes")]
async fn get_recipes(conn: Connection<Db>) -> Result<Json<Vec<Recipe>>, Status> {
    let db = conn.into_inner();
    let recipe = Recipes::find().all(&db).await;

    match recipe {
        Ok(recipes) => Ok(Json(
            join_all(recipes.iter().map(|recipe| Recipe::from_model(recipe, &db)))
                .await
                .into_iter()
                .filter_map(|r| r.map_err(|e| println!("{:?}", e)).ok())
                .collect(),
        )),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/get_tags")]
async fn get_tags(conn: Connection<Db>) -> Result<Json<Vec<Tag>>, Status> {
    let db = conn.into_inner();

    match Tags::find().all(&db).await {
        Ok(ingredients) => Ok(Json(
            ingredients.iter().map(|t| Tag::from_model(t)).collect(),
        )),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub fn api_routes() -> Vec<rocket::Route> {
    routes![get_recipes, get_tags,]
}
