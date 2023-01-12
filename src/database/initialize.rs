use sea_orm::{Database as SeaDatabase, DatabaseConnection};
use migration::{Migrator, MigratorTrait};

use crate::configs::bot_configs::Environment;

#[derive(Debug, Clone)]
pub struct Database {
    pub connection: DatabaseConnection,
}

#[derive(Debug, Clone)]
pub struct DatabaseCredentials {
    pub username: String,
    pub password: String,
}

impl Database {
    pub async fn load(credentials: &DatabaseCredentials, env: &Environment) -> Self {
        let db_url = Self::url(&credentials, &env);
        let database = Self::connect(&db_url).await;
        Self::migrate(&database).await;
        Database {
            connection: database,
        }
    }

    fn url(credentials: &DatabaseCredentials, env: &Environment) -> String {
        match env {
            Environment::Prod => format!("postgres://{}:{}@localhost/rusty-monitor-prod", credentials.username, credentials.password),
            Environment::Dev => format!("postgres://{}:{}@localhost/rusty-monitor-dev", credentials.username, credentials.password),
            Environment::Test => format!("postgres://{}:{}@localhost/rusty-monitor-test", credentials.username, credentials.password),
        }
    }

    async fn connect(db_url: &str) -> DatabaseConnection {
        SeaDatabase::connect(db_url).await.expect("unable to connect to db")
    }

    async fn migrate(database: &DatabaseConnection) {
        Migrator::up(database, None).await.expect("unable to run migrations")
    }
}