use rocket::futures::future::join_all;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sea_orm::{ActiveModelTrait, EntityTrait, NotSet, Set, TransactionTrait};
use crate::db::Db;
use crate::entity::prelude::Recipes;
use crate::entity::{ingredient_preps, recipe_tags, recipes, recipe_ingredients};
use crate::view::request::recipe::RecipeRequest;
use crate::view::response::recipe::RecipeResponse;

#[get("/recipe/all")]
async fn get_recipes(conn: Connection<Db>) -> Result<Json<Vec<RecipeResponse>>, Status> {
    let db = conn.into_inner();
    let recipe = Recipes::find().all(&db).await;

    match recipe {
        Ok(recipes) => Ok(Json(
            join_all(recipes.iter().map(|recipe| RecipeResponse::from_model(recipe, &db)))
                .await
                .into_iter()
                .filter_map(|r| r.map_err(|e| println!("{:?}", e)).ok())
                .collect(),
        )),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/recipe", data = "<recipe_request>")]
async fn create_recipe(recipe_request: Json<RecipeRequest>, conn: Connection<Db>) -> Result<Json<RecipeResponse>, Status> {
    let db = conn.into_inner();

    let txn = match db.begin().await {
        Ok(txn) => txn,
        Err(_) => return Err(Status::InternalServerError),
    };

    let new_recipe = recipes::ActiveModel {
        id: NotSet,
        name: Set(recipe_request.name.clone()),
        time: Set(recipe_request.time as i32),
        difficulty: Set(recipe_request.difficulty.clone()),
    }.insert(&txn)
        .await.unwrap();

    join_all(recipe_request.tag_ids
        .iter()
        .map(async |tag_id| recipe_tags::ActiveModel {
            recipe_id: Set(new_recipe.id),
            tag_id: Set(*tag_id as i32)
        }
            .insert(&txn)
            .await)
    ).await;

    let ingredient_prep_results = join_all(
        recipe_request.ingredient_preps
        .iter()
            .map(async |ip| ingredient_preps::ActiveModel {
                id: NotSet,
                pre_prep: Set(ip.pre_prep.clone()),
                amount: Set(ip.amount as i32),
                unit: Set(ip.unit.clone()),
                ingredient_id: Set(ip.ingredient_id as i32)
            }
                .insert(&txn)
                .await)
    ).await;

    join_all(
        ingredient_prep_results
            .into_iter()
            .filter_map(|r| r.map_err(|e| println!("{:?}", e)).ok())
            .map(async |ip| recipe_ingredients::ActiveModel {
                recipe_id: Set(new_recipe.id),
                ingredient_prep_id: Set(ip.id)
            }
                .insert(&txn)
                .await)
    ).await;

    match txn.commit().await {
        Ok(_) => match RecipeResponse::from_model(&new_recipe, &db).await {
            Ok(recipe) => Ok(Json(recipe)),
            Err(_) => Err(Status::InternalServerError),
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

pub fn recipe_routes() -> Vec<rocket::Route> {
    routes![
        get_recipes,
        create_recipe
    ]
}