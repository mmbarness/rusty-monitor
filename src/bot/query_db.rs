use crate::models::{
    users::Find as FindUser,
    target_server::Find as FindTargetServer
};
use crate::structs::Context;
use entity::users::{ActiveModel as User};
use entity::target_server::ActiveModel as TargetServer;
use super::support::Support;

#[async_trait::async_trait]
pub trait QueryDb {
    async fn get_server_if_exists(ctx: &Context<'_>) -> Option<entity::target_server::Model> {
        let db_connection = &ctx.data().database.connection;
        let user = match Support::get_user_if_exists(&ctx).await {
            Some(u) => u,
            None => {
                panic!("user not registered to db");
            }
        };
        TargetServer::find_by_user_model(&user, db_connection).await
    }

    async fn get_user_if_exists(ctx: &Context<'_>) -> Option<entity::users::Model> {
        let user_discord_id = ctx.author().id;
        let id_as_u64 = user_discord_id.as_u64().clone();
        let id_as_i64 = match i64::try_from(id_as_u64) {
            Ok(id) => id,
            Err(e) => {
                panic!("error converting discord id sourced from ctx to i32 used in database: {}", e);
            }
        };
        let db_connection = &ctx.data().database.connection;
        User::find_by_discord_id(id_as_i64, db_connection).await
    }

    async fn user_exists(ctx: &Context<'_>) -> bool {
        let user_discord_id = ctx.author().id;
        let id_as_u64 = user_discord_id.as_u64().clone();
        let id_as_i64 = match i64::try_from(id_as_u64) {
            Ok(id) => id,
            Err(e) => {
                panic!("error converting discord id sourced from ctx to i32 used in database: {}", e);
            }
        };
        let db_connection = &ctx.data().database.connection;
        match User::find_by_discord_id(id_as_i64, db_connection).await {
            Some(_) => true,
            None => false,
        }
    }

}

impl QueryDb for Support {}