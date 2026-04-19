use crate::entity::ingredients::Model;
use rocket::serde::{Deserialize, Serialize};

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