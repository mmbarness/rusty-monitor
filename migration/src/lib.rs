pub use sea_orm_migration::prelude::*;

mod m20230103_104900_create_target_servers_table;
mod m20230103_133250_create_users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230103_133250_create_users_table::Migration),
            Box::new(m20230103_104900_create_target_servers_table::Migration),
        ]
    }
}
