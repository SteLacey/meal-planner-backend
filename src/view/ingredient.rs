use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
}