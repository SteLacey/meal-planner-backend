use sea_orm::*;

const DATABASE_URL: &str = "postgres://postgres:dev@localhost:5432/meal-planner_dev"; // TODO: Get from config
pub(super) async fn get_db_connection() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    Ok(db)
}