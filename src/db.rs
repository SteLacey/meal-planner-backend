use rocket::figment::Figment;
use rocket_db_pools::{Config, Database};
use sea_orm::{DatabaseConnection, DbErr};

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
        let config: Config = figment.extract().unwrap();
        let conn = sea_orm::Database::connect(config.url).await?;
        Ok(SeaOrmPool { conn })
    }

    async fn get(&self) -> Result<Self::Connection, Self::Error> {
        Ok(self.conn.clone())
    }

    // DatabaseConnection auto closes on drop
    async fn close(&self) {}
}
