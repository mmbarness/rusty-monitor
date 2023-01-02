use sqlx::{PgPool, postgres::PgQueryResult};
use crate::configs::bot_configs::Environment;

use super::schemas::TableSchemas;

#[derive(Debug, Clone)]
pub struct Database {
    pub database: PgPool
}

impl Database {
    pub async fn load(username: &str, password: &str, env: &Environment) -> Self {
        let database = Self::connect(username, password, env).await;
        Self::migrate(&database).await;
        Self::create_all_tables(&database).await;
        Database {
            database,
        }
    }

    pub fn full_name(env: &Environment) -> String {
        match env {
            Environment::Dev => "rusty-monitor-dev".to_string(),
            Environment::Prod => "rusty-monitor-prod".to_string(),
            Environment::Test => "rusty-monitor-test".to_string(),
        }
    }

    pub async fn create_all_tables(database: &PgPool) -> () {
        let schemas = TableSchemas::get_all();
        Self::create(&schemas.monitoring_server, database).await.expect("Failed to create monitoring server table");
        Self::create(&schemas.users, database).await.expect("Failed to create users table");
        Self::create(&schemas.monitor, database).await.expect("failed to create monitor table");
    }

    async fn connect(username: &str, db_password: &str, env: &Environment) -> PgPool {
        let database_name = &Self::full_name(env);
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(3)
            .connect_with(
                sqlx::postgres::PgConnectOptions::new()
                    .database(database_name)
                    .username(username)
                    .password(&db_password)
                )
            .await
            .expect("Couldn't connect to database")
    }

    async fn migrate(database: &PgPool) {
        sqlx::migrate!("./migrations").run(database).await.expect("Couldn't run database migrations"); 
    }

    async fn create(query: &str, database: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        let create_table_query = "create table if not exists ".to_string() + &query.to_string();
        sqlx::query(&create_table_query)
            .execute(database)
            .await
    }
}