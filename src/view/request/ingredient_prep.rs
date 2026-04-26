use rocket::serde::Deserialize;

#[derive(Deserialize)]
pub struct IngredientPrepRequest {
    pub pre_prep: Option<String>,
    pub amount: u32,
    pub unit: String,
    pub ingredient_id: u32,
}