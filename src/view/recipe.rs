use super::tag::Tag;
use crate::entity::ingredient_preps::Model as IngredientPrepModel;
use crate::entity::recipes::Model;
use crate::entity::{ingredient_preps, tags};
use crate::view::ingredient_prep::IngredientPrep;
use rocket::futures::future::join_all;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::{DatabaseConnection, DbErr, ModelTrait};

#[derive(Serialize, Deserialize)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub time: u32,
    pub difficulty: String,
    pub ingredient_preps: Vec<IngredientPrep>,
    pub tags: Vec<Tag>,
}

impl Recipe {
    pub async fn from_model(model: &Model, db: &DatabaseConnection) -> Result<Recipe, DbErr> {
        let ingredient_preps = model.find_related(ingredient_preps::Entity).all(db).await?;

        let tags = model.find_related(tags::Entity).all(db).await?;
        let tags_views = tags.iter().map(|t| Tag::from_model(t)).collect();

        Ok(Recipe {
            id: model.id,
            name: model.name.clone(),
            time: model.time as u32,
            difficulty: model.difficulty.clone(),
            tags: tags_views,
            ingredient_preps: Recipe::get_ingredient_prep_views(&ingredient_preps, db).await?,
        })
    }

    async fn get_ingredient_prep_views(
        ingredient_preps: &Vec<IngredientPrepModel>,
        db: &DatabaseConnection,
    ) -> Result<Vec<IngredientPrep>, DbErr> {
        let ingredient_preps_views = join_all(
            ingredient_preps
                .iter()
                .map(|ip| IngredientPrep::from_model(ip, &db)),
        )
        .await;

        let mut errored_ip_views = vec![];

        let success_ip_views = ingredient_preps_views
            .into_iter()
            .filter_map(|ip| ip.map_err(|e| errored_ip_views.push(e)).ok())
            .collect();

        if errored_ip_views.is_empty() {
            Ok(success_ip_views)
        } else {
            Err(DbErr::Custom(format!("{:?}", errored_ip_views)))
        }
    }
}
