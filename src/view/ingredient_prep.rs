use super::ingredient::Ingredient;
use crate::entity::ingredient_preps::Model;
use crate::entity::ingredients;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::{DbConn, DbErr, ModelTrait};

#[derive(Serialize, Deserialize)]
pub struct IngredientPrep {
    pub pre_prep: Option<String>,
    pub amount: u32,
    pub unit: String,
    pub ingredient: Ingredient
}

impl IngredientPrep {
    pub async fn from_model(model: &Model, db: &DbConn) -> Result<IngredientPrep, DbErr> {
        Ok(
            IngredientPrep {
                pre_prep: model.pre_prep.clone(),
                amount: model.amount as u32,
                unit: model.unit.clone(),
                ingredient: Ingredient::from_model(&model
                    .find_related(ingredients::Entity)
                    .one(db)
                    .await?
                    .unwrap())
            }
        )
    }
}