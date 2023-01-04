use entity::users::{ActiveModel as User};
use entity::target_server::{ActiveModel as TargetServer};
use crate::models::target_server::Find as ServerFind;
use crate::models::users::Find as UserFind;
use crate::{structs::Context, Error};
use std::convert::From;

#[poise::command(track_edits, slash_command)]
pub async fn who_am_i(
    ctx: Context<'_>,
) -> Result<(), Error> {

    let bot = ctx.data();
    let db_connection = &bot.configs.database.connection;
    let user_discord_id = &ctx.author().id;
    let id_as_u64 = user_discord_id.as_u64().clone();
    println!("discord id: {}", user_discord_id);
    let id_as_i64 = match i64::try_from(id_as_u64) {
        Ok(id) => id,
        Err(e) => {
            panic!("error converting discord id sourced from ctx to i32 used in database: {}", e);
        }
    };

    let user = match &bot.data.user {
        Some(u) => u.clone(),
        None => {
            match User::find_by_discord_id(id_as_i64, db_connection).await {
                Some(u) => u,
                None => {
                    ctx.say("didn't find you, have you registered?").await.expect("unable to send message");
                    return Ok(());
                }
            }
        }
    };
    
    let target_server = match &bot.data.target_server {
        Some(u) => u.clone(),
        None => {
            match TargetServer::find_by_user_model(&user, db_connection).await {
                Some(u) => u,
                None => {
                    ctx.say("didn't find you, have you registered?").await.expect("unable to send message");
                    return Ok(());
                }
            }
        }
    };

    let user_name = user.user_name;
    let created_at = user.created_at;
    let server_address = target_server.address;

    ctx.say(
        format!(
            "hello {}, you\'ve been registered since {} and your server is at {}",
            user_name.to_string(),
            created_at.format("%Y-%m-%d").to_string(),
            server_address.to_string()
        )
    ).await.expect("unable to send message");

    Ok(())
}