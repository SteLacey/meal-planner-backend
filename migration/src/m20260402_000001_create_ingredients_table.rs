use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Ingredients::Table)
                    .col(pk_auto(Ingredients::Id))
                    .col(string(Ingredients::Name))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Ingredients::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Ingredients {
    Table,
    Id,
    Name,
}
