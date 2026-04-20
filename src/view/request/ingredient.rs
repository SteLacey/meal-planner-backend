use rocket::serde::Deserialize;

#[derive(Deserialize)]
pub struct IngredientRequest {
    pub name: String,
}
