pub use sea_orm_migration::prelude::*;

mod m20260402_000001_create_ingredients_table;
mod m20260403_000002_create_recipes_table;
mod m20260403_000003_create_recipe_items_table;
mod m20260403_000004_create_tags_table;
mod m20260403_000005_create_recipe_tags_table;
mod m20260416_000006_drop_recipe_tags_table;
mod m20260416_000007_create_ingredient_preps_table;
mod m20260416_000008_create_recipe_ingredients_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260402_000001_create_ingredients_table::Migration),
            Box::new(m20260403_000002_create_recipes_table::Migration),
            Box::new(m20260403_000003_create_recipe_items_table::Migration),
            Box::new(m20260403_000004_create_tags_table::Migration),
            Box::new(m20260403_000005_create_recipe_tags_table::Migration),
            Box::new(m20260416_000006_drop_recipe_tags_table::Migration),
            Box::new(m20260416_000007_create_ingredient_preps_table::Migration),
            Box::new(m20260416_000008_create_recipe_ingredients_table::Migration),
        ]
    }
}
