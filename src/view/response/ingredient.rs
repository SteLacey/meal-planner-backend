use crate::entity::ingredients::Model;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IngredientResponse {
    pub id: i32,
    pub name: String,
}

impl IngredientResponse {
    pub fn from_model(model: &Model) -> IngredientResponse {
        IngredientResponse {
            id: model.id,
            name: model.name.clone(),
        }
    }
}
