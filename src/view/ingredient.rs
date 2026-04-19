use rocket::serde::{Deserialize, Serialize};
use crate::entity::ingredients::Model;

#[derive(Serialize, Deserialize)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
}

impl Ingredient {
    pub fn from_model(model: &Model) -> Ingredient {
        Ingredient {
            id: model.id,
            name: model.name.clone(),
        }
    }
}