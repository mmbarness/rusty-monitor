use sea_orm::{Database as SeaDatabase, DatabaseConnection};
use migration::{Migrator, MigratorTrait};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Database {
    pub connection: DatabaseConnection,
}

impl Database {
    pub async fn load(db_url: &str) -> Self {
        let database = Self::connect(db_url).await;
        Self::migrate(&database).await;
        Database {
            connection: database,
        }
    }

    async fn connect(db_url: &str) -> DatabaseConnection {
        SeaDatabase::connect(db_url).await.expect("unable to connect to db")
    }

    async fn migrate(database: &DatabaseConnection) {
        Migrator::up(database, None).await.expect("unable to run migrations")
    }
}