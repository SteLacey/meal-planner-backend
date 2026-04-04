use rocket::serde::{Deserialize, Serialize};
use crate::view::recipe_item::RecipeItem;
use super::tag::Tag;

#[derive(Serialize, Deserialize)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub time: u32,
    pub difficulty: String,
    pub recipe_items: Vec<RecipeItem>,
    pub tags: Vec<Tag>,
}