use rocket::serde::{Deserialize, Serialize};
use crate::entity::tags::Model;

#[derive(Serialize, Deserialize)]
pub struct Tag {
    pub id: i32,
    pub tag: String,
}

impl Tag {
    pub fn from_model(model: &Model) -> Tag {
        Tag {
            id: model.id,
            tag: model.tag.clone(),
        }
    }
}