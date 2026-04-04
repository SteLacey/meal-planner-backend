use rocket::figment::Figment;
use rocket_db_pools::Database;
use sea_orm::{DatabaseConnection, DbErr};

const DATABASE_URL: &str = "postgres://postgres:dev@localhost:5432/meal-planner_dev"; // TODO: Get from config

#[derive(Database)]
#[database("sea_orm")]
pub struct Db(SeaOrmPool);

pub struct SeaOrmPool {
    pub conn: DatabaseConnection,
}

#[rocket::async_trait]
impl rocket_db_pools::Pool for SeaOrmPool {
    type Connection = DatabaseConnection;
    type Error = DbErr;

    async fn init(figment: &Figment) -> Result<Self, Self::Error> {
        let conn = sea_orm::Database::connect(DATABASE_URL).await?;
        Ok(SeaOrmPool { conn })
    }

    async fn get(&self) -> Result<Self::Connection, Self::Error> {
        Ok(self.conn.clone())
    }

    // DatabaseConnection auto closes on drop
    async fn close(&self) {}
}