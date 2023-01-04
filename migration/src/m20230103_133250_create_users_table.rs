use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .unique_key()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::DiscordId)
                            .big_unsigned()
                            .not_null()
                    )
                    .col(ColumnDef::new(Users::UserName).string().not_null())
                    .col(ColumnDef::new(Users::Admin).boolean().not_null())
                    .col(ColumnDef::new(Users::Premium).boolean().not_null())
                    .col(ColumnDef::new(Users::CreatedAt).date().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Users {
    Table,
    Id,
    Admin,
    DiscordId,
    Premium,
    UserName,
    CreatedAt,
}