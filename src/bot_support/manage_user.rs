use crate::{structs::Context, models::users::Find};
use entity::users::ActiveModel as User;

use super::bot_support::BotSupport;

#[async_trait::async_trait]
pub trait ManageUser {
    async fn pre_command_check(ctx: &Context<'_>) -> Option<entity::users::Model> {
        let user_discord_id = ctx.author().id;
        let id_as_u64 = user_discord_id.as_u64().clone();
        println!("discord_id:{}", &id_as_u64);
        let id_as_i64 = match i64::try_from(id_as_u64) {
            Ok(id) => id,
            Err(e) => {
                panic!("error converting discord id sourced from ctx to i32 used in database: {}", e);
            }
        };
        let db_connection = &ctx.data().bot_configs.database.connection;
        User::find_by_discord_id(id_as_i64, db_connection).await
    }
}

impl ManageUser for BotSupport {}