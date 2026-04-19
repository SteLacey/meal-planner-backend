use crate::m20260403_000002_create_recipes_table::Recipes;
use crate::m20260403_000004_create_tags_table::Tags;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RecipeTags::Table)
                    .col(integer(RecipeTags::RecipeId))
                    .col(integer(RecipeTags::TagId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipe-tags-recipe-id")
                            .from_col(RecipeTags::RecipeId)
                            .to(Recipes::Table, Recipes::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipe-tags-tag-id")
                            .from_col(RecipeTags::TagId)
                            .to(Tags::Table, Tags::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(RecipeTags::RecipeId)
                            .col(RecipeTags::TagId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RecipeTags::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum RecipeTags {
    Table,
    RecipeId,
    TagId,
}
