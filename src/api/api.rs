use crate::db::Db;
use crate::entity::prelude::{Tags};
use crate::view::response::tag::Tag;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sea_orm::{EntityTrait};

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
    routes![get_tags,]
}
