pub use sea_orm_migration::prelude::*;

mod m20260204_000001_create_ingredients_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20260204_000001_create_ingredients_table::Migration)]
    }
}
