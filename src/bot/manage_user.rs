use crate::{structs::Context, models::users::Find};
use entity::users::{ActiveModel as User};
use super::support::Support;

#[async_trait::async_trait]
pub trait ManageUser {
    async fn get_user_if_exists(ctx: &Context<'_>) -> Option<entity::users::Model> {
        let user_discord_id = ctx.author().id;
        let id_as_u64 = user_discord_id.as_u64().clone();
        let id_as_i64 = match i64::try_from(id_as_u64) {
            Ok(id) => id,
            Err(e) => {
                panic!("error converting discord id sourced from ctx to i32 used in database: {}", e);
            }
        };
        let db_connection = &ctx.data().configs.database.connection;
        User::find_by_discord_id(id_as_i64, db_connection).await
    }

    async fn exists(ctx: &Context<'_>) -> bool {
        let user_discord_id = ctx.author().id;
        let id_as_u64 = user_discord_id.as_u64().clone();
        let id_as_i64 = match i64::try_from(id_as_u64) {
            Ok(id) => id,
            Err(e) => {
                panic!("error converting discord id sourced from ctx to i32 used in database: {}", e);
            }
        };
        let db_connection = &ctx.data().configs.database.connection;
        match User::find_by_discord_id(id_as_i64, db_connection).await {
            Some(_) => true,
            None => false,
        }
    }
}

impl ManageUser for Support {}