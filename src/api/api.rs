use crate::db::Db;
use crate::entity::prelude::Recipes;
use crate::view::ingredient::Ingredient;
use crate::view::ingredient_prep::IngredientPrep;
use crate::view::recipe::Recipe;
use crate::view::tag::Tag;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sea_orm::EntityTrait;

#[get("/get_recipes")]
async fn get_recipes(conn: Connection<Db>) -> Result<Json<Recipe>, Status> {
    let db = conn.into_inner();
    let recipe = Recipes::find()
        .one(&db)
        .await
        .expect("Error finding recipes");

    match recipe {
        Some(recipe) =>
            match Recipe::from_model(&recipe, &db).await {
                Ok(view) => Ok(Json(view)),
                Err(_) => Err(Status::InternalServerError)
            },
        None => Err(Status::NotFound)
    }
}

pub fn api_routes() -> Vec<rocket::Route> {
    routes![get_recipes]
}