use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Tag {
    pub id: i32,
    pub tag: String,
}