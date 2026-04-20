use crate::db::Db;
use crate::entity::prelude::{Recipes, Tags};
use crate::entity::prelude::Ingredients;
use crate::view::response::recipe::Recipe;
use rocket::futures::future::join_all;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, NotSet, Set, TryIntoModel};
use crate::entity::ingredients;
use crate::view::request::ingredient::{IngredientRequest};
use crate::view::response::ingredient::IngredientResponse;
use crate::view::response::tag::Tag;

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

#[get("/get_ingredients")]
async fn get_ingredients(conn: Connection<Db>) -> Result<Json<Vec<IngredientResponse>>, Status> {
    let db = conn.into_inner();

    match Ingredients::find().all(&db).await {
        Ok(ingredients) => Ok(Json(
            ingredients
                .iter()
                .map(|i| IngredientResponse::from_model(i))
                .collect()
        )),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/ingredient", data = "<ingredient>")]
async fn add_ingredient(ingredient: Json<IngredientRequest>, conn: Connection<Db>) -> Result<Json<IngredientResponse>, Status> {
    let db = conn.into_inner();

    let new_ingred = ingredients::ActiveModel {
        id: NotSet,
        name: Set(ingredient.name.clone()),
    }.save(&db).await;

    match new_ingred {
        Ok(ingred) => {
            match ingred.try_into_model() {
                Ok(model) => Ok(Json(IngredientResponse::from_model(&model))),
                Err(_) => Err(Status::InternalServerError),
            }
        },
        Err(_) => Err(Status::InternalServerError)
    }
}

#[put("/ingredient/<id>", data = "<ingredient>")]
async fn update_ingredient(id: i32, ingredient: Json<IngredientRequest>, conn: Connection<Db>) -> Result<Json<IngredientResponse>, Status> {
    let db = conn.into_inner();

    let ingred_query = Ingredients::find_by_id(id)
        .one(&db)
        .await;

    match ingred_query {
        Ok(result) => match result {
            Some(ingred) => {
                let mut am = ingred.into_active_model();
                am.name = Set(ingredient.name.clone());
                match am.save(&db).await {
                    Ok(am) => match am.try_into_model() {
                        Ok(model) => Ok(Json(IngredientResponse::from_model(&model))),
                        Err(_) => Err(Status::InternalServerError)
                    },
                    Err(_) => Err(Status::InternalServerError)
                }
            },
            None => Err(Status::NotFound)
        },
        Err(_) => Err(Status::InternalServerError)
    }
}

#[get("/get_tags")]
async fn get_tags(conn: Connection<Db>) -> Result<Json<Vec<Tag>>, Status> {
    let db = conn.into_inner();

    match Tags::find().all(&db).await {
        Ok(ingredients) => Ok(Json(
            ingredients
                .iter()
                .map(|t| Tag::from_model(t))
                .collect()
        )),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub fn api_routes() -> Vec<rocket::Route> {
    routes![get_recipes, get_ingredients, get_tags, add_ingredient, update_ingredient,]
}
