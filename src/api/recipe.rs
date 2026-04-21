use rocket::futures::future::join_all;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sea_orm::EntityTrait;
use crate::db::Db;
use crate::entity::prelude::Recipes;
use crate::view::response::recipe::Recipe;

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

pub fn recipe_routes() -> Vec<rocket::Route> {
    routes![
        get_recipes,
    ]
}