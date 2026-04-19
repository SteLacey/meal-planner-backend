use sea_orm_migration::{prelude::*, schema::*};
use crate::m20260403_000002_create_recipes_table::Recipes;
use crate::m20260416_000007_create_ingredient_preps_table::IngredientPreps;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RecipeIngredients::Table)
                    .col(integer(RecipeIngredients::RecipeId))
                    .col(integer(RecipeIngredients::IngredientPrepId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipe-ingredients-recipe-id")
                            .from_col(RecipeIngredients::RecipeId)
                            .to(Recipes::Table, Recipes::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipe-ingredients-ingredient-prep-id")
                            .from_col(RecipeIngredients::IngredientPrepId)
                            .to(IngredientPreps::Table, IngredientPreps::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(RecipeIngredients::RecipeId)
                            .col(RecipeIngredients::IngredientPrepId)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RecipeIngredients::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum RecipeIngredients {
    Table,
    RecipeId,
    IngredientPrepId
}
