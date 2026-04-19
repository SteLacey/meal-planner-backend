use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sea_orm::{EntityTrait};
use crate::db::Db;
use crate::entity::prelude::Recipes;
use crate::view::ingredient::Ingredient;
use crate::view::recipe::Recipe;
use crate::view::ingredient_prep::IngredientPrep;
use crate::view::tag::Tag;

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

#[get("/test_recipe")]
async fn test_recipe() -> Json<Recipe> {
    let i = Ingredient { id: 1, name: "Chicken".to_owned() };
    let ri = IngredientPrep { pre_prep: Some("Diced".to_owned()), amount: 100, unit: "g".to_owned(), ingredient: i };
    let t = Tag { id:1, tag: "Easy".to_owned() };
    Json(Recipe { id:1, name: "Chicken Recipe".to_owned(), time: 10, difficulty: "easy".to_owned(), ingredient_preps: vec![ri], tags: vec![t] })
}

pub fn api_routes() -> Vec<rocket::Route> {
    routes![get_recipes, test_recipe]
}