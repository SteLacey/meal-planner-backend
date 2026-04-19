use sea_orm_migration::{prelude::*, schema::*};
use crate::m20260402_000001_create_ingredients_table::Ingredients;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IngredientPreps::Table)
                    .col(pk_auto(IngredientPreps::Id))
                    .col(string_null(IngredientPreps::PrePrep))
                    .col(integer(IngredientPreps::Amount))
                    .col(string(IngredientPreps::Unit))
                    .col(integer(IngredientPreps::IngredientId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipe-items-ingredient-id")
                            .from_col(IngredientPreps::IngredientId)
                            .to(Ingredients::Table, Ingredients::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IngredientPreps::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum IngredientPreps {
    Table,
    Id,
    IngredientId,
    PrePrep,
    Amount,
    Unit
}