use rocket::serde::{Deserialize, Serialize};
use super::ingredient::Ingredient;

#[derive(Serialize, Deserialize)]
pub struct RecipeItem {
    pub pre_prep: Option<String>,
    pub amount: u32,
    pub unit: String,
    pub ingredient: Ingredient
}