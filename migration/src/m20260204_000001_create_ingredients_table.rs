use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260204_000001_create_ingredients_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Ingredients::Table)
                    .col(
                        ColumnDef::new(Ingredients::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Ingredients::Name)
                            .string()
                            .not_null(),
                    )
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
    Name
}