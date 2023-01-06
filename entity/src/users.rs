//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub discord_id: i64,
    pub user_name: String,
    pub admin: bool,
    pub premium: bool,
    pub created_at: Date,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::target_server::Entity")]
    TargetServer,
}

impl Related<super::target_server::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TargetServer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}