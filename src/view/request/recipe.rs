use rocket::serde::Deserialize;
use crate::view::request::ingredient_prep::IngredientPrepRequest;

#[derive(Deserialize)]
pub struct RecipeRequest {
    pub name: String,
    pub time: u32,
    pub difficulty: String,
    pub ingredient_preps: Vec<IngredientPrepRequest>,
    pub tag_ids: Vec<u32>,
}