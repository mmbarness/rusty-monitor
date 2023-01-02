use sqlx::{PgPool, postgres::PgQueryResult};
use super::schemas::TableSchemas;

#[derive(Debug, Clone)]
pub struct Database {
    pub database: PgPool
}

impl Database {
    pub async fn load(password: &str) -> Self {
        let database = Self::connect(password).await;
        Self::migrate(&database).await;
        Self::create_all_tables(&database).await;
        Database {
            database,
        }
    }

    pub async fn create_all_tables(database: &PgPool) -> () {
        let schemas = TableSchemas::get_all();
        Self::create(&schemas.monitoring_server, database).await.expect("Failed to create monitoring server table");
        Self::create(&schemas.users, database).await.expect("Failed to create users table");
        Self::create(&schemas.monitor, database).await.expect("failed to create monitor table");
    }

    async fn connect(db_password: &str) -> PgPool {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(3)
            .connect_with(
                sqlx::postgres::PgConnectOptions::new()
                    .database("rusty-monitor-test")
                    .username("postgres")
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