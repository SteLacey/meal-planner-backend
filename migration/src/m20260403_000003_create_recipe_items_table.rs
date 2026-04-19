use crate::m20260402_000001_create_ingredients_table::Ingredients;
use crate::m20260403_000002_create_recipes_table::Recipes;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RecipeItems::Table)
                    .col(string_null(RecipeItems::PrePrep))
                    .col(integer(RecipeItems::Amount))
                    .col(string(RecipeItems::Unit))
                    .col(integer(RecipeItems::RecipeId))
                    .col(integer(RecipeItems::IngredientId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipe-items-recipe-id")
                            .from_col(RecipeItems::RecipeId)
                            .to(Recipes::Table, Recipes::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipe-items-ingredient-id")
                            .from_col(RecipeItems::IngredientId)
                            .to(Ingredients::Table, Ingredients::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(RecipeItems::RecipeId)
                            .col(RecipeItems::IngredientId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RecipeItems::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum RecipeItems {
    Table,
    RecipeId,
    IngredientId,
    PrePrep,
    Amount,
    Unit,
}
