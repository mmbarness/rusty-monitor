use sea_orm_migration::prelude::*;

use crate::m20230103_133250_create_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
        .create_table(
            Table::create()
                .table(TargetServer::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(TargetServer::Id)
                        .primary_key()
                        .unique_key()
                        .integer()
                        .not_null()
                        .auto_increment(),
                )
                .col(
                    ColumnDef::new(TargetServer::UserId)
                        .unique_key()
                        .integer()
                        .not_null()
                )
                .col(
                    ColumnDef::new(TargetServer::Address)
                        .string()
                        .not_null()
                )
                .col(
                    ColumnDef::new(TargetServer::Port)
                        .integer()
                        .not_null()
                )
                .col(
                    ColumnDef::new(TargetServer::Auth)
                        .boolean()
                        .not_null()
                )
                .col(
                    ColumnDef::new(TargetServer::AuthKey)
                        .integer()
                )
                .foreign_key(
                    & mut ForeignKey::create()
                        .name("FK_owner")
                        .from(TargetServer::Table, TargetServer::UserId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                        .to_owned()
                )
                .to_owned(),
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TargetServer::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum TargetServer {
    Table,
    Id,
    Address,
    Port,
    Auth,
    AuthKey,
    UserId,
}
