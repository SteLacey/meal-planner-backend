use super::ingredient::IngredientResponse;
use crate::entity::ingredient_preps::Model;
use crate::entity::ingredients;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::{DbConn, DbErr, ModelTrait};

#[derive(Serialize, Deserialize)]
pub struct IngredientPrepResponse {
    pub pre_prep: Option<String>,
    pub amount: u32,
    pub unit: String,
    pub ingredient: IngredientResponse,
}

impl IngredientPrepResponse {
    pub async fn from_model(model: &Model, db: &DbConn) -> Result<IngredientPrepResponse, DbErr> {
        Ok(IngredientPrepResponse {
            pre_prep: model.pre_prep.clone(),
            amount: model.amount as u32,
            unit: model.unit.clone(),
            ingredient: IngredientResponse::from_model(
                &model
                    .find_related(ingredients::Entity)
                    .one(db)
                    .await?
                    .unwrap(),
            ),
        })
    }
}
