use crate::entity::tags::Model;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TagResponse {
    pub id: i32,
    pub tag: String,
}

impl TagResponse {
    pub fn from_model(model: &Model) -> TagResponse {
        TagResponse {
            id: model.id,
            tag: model.tag.clone(),
        }
    }
}
