use crate::db::Db;
use crate::entity::ingredients;
use crate::entity::prelude::Ingredients;
use crate::view::request::ingredient::IngredientRequest;
use crate::view::response::ingredient::IngredientResponse;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, NotSet, Set, TryIntoModel, QueryFilter};

#[get("/ingredient/all")]
async fn get_ingredients(conn: Connection<Db>) -> Result<Json<Vec<IngredientResponse>>, Status> {
    let db = conn.into_inner();

    match Ingredients::find().all(&db).await {
        Ok(ingredients) => Ok(Json(
            ingredients
                .iter()
                .map(|i| IngredientResponse::from_model(i))
                .collect(),
        )),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/ingredient/<id>")]
async fn get_ingredient(id: i32, conn: Connection<Db>) -> Result<Json<IngredientResponse>, Status> {
    let db = conn.into_inner();

    match Ingredients::find_by_id(id).one(&db).await {
        Ok(opt) => match opt {
            Some(found) => Ok(Json(IngredientResponse::from_model(&found))),
            None => Err(Status::NotFound),
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/ingredient?<name>")]
async fn find_ingredient(name: &str, conn: Connection<Db>) -> Result<Json<Vec<IngredientResponse>>, Status> {
    let db = conn.into_inner();

    match Ingredients::find()
        .filter(ingredients::Column::Name.ilike(format!("%{}%", name)))
        .all(&db)
        .await {
        Ok(ingredients) => {
            Ok(Json(ingredients
                .iter()
                .map(|i| IngredientResponse::from_model(i))
                .collect()))
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/ingredient", data = "<ingredient>")]
async fn add_ingredient(
    ingredient: Json<IngredientRequest>,
    conn: Connection<Db>,
) -> Result<Json<IngredientResponse>, Status> {
    let db = conn.into_inner();

    let new_ingred = ingredients::ActiveModel {
        id: NotSet,
        name: Set(ingredient.name.clone()),
    }
    .save(&db)
    .await;

    match new_ingred {
        Ok(ingred) => match ingred.try_into_model() {
            Ok(model) => Ok(Json(IngredientResponse::from_model(&model))),
            Err(_) => Err(Status::InternalServerError),
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/ingredient/<id>", data = "<ingredient>")]
async fn update_ingredient(
    id: i32,
    ingredient: Json<IngredientRequest>,
    conn: Connection<Db>,
) -> Result<Json<IngredientResponse>, Status> {
    let db = conn.into_inner();

    let ingred_query = Ingredients::find_by_id(id).one(&db).await;

    match ingred_query {
        Ok(result) => match result {
            Some(ingred) => {
                let mut am = ingred.into_active_model();
                am.name = Set(ingredient.name.clone());
                match am.save(&db).await {
                    Ok(am) => match am.try_into_model() {
                        Ok(model) => Ok(Json(IngredientResponse::from_model(&model))),
                        Err(_) => Err(Status::InternalServerError),
                    },
                    Err(_) => Err(Status::InternalServerError),
                }
            }
            None => Err(Status::NotFound),
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

pub fn ingredient_routes() -> Vec<rocket::Route> {
    routes![
        get_ingredients,
        get_ingredient,
        find_ingredient,
        update_ingredient,
        add_ingredient
    ]
}
