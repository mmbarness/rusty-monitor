use sea_orm::TryIntoModel;

use crate::{structs::Context, Error, mprober_api_resources::{cpu::CPULoad, shared_traits::Compute}};
use crate::{models::{users::Create, target_server::Create as CreateTargetServer}};
use std::convert::From;

#[poise::command(track_edits, slash_command)]
pub async fn new_user(
    ctx: Context<'_>,
    server_address: String,
    server_port: i32,
    auth_key: Option<i32>,
) -> Result<(), Error> {

    let data = ctx.data();
    let db_connection = &data.configs.database.connection;
    let user_discord_id = &ctx.author().id;
    let id_as_u64 = user_discord_id.as_u64().clone();
    let id_as_i64 = match i64::try_from(id_as_u64) {
        Ok(id) => id,
        Err(e) => {
            panic!("error converting discord id sourced from ctx to i32 used in database: {}", e);
        }
    };
    let user_name = &ctx.author().name;

    let admin = match ctx.author().id.as_u64() {
        393426394723385344 => true,
        _ => false,
    };

    let user = match entity::users::ActiveModel::create(&id_as_i64, user_name, &admin, &false, db_connection).await {
        Ok(user) => user,
        Err(e) => {
            ctx.say(format!("error while registering user: {}", e)).await.expect("error sending message");
            panic!("error while registering user: {}", e);
        }
    };

    let user_model = match user.try_into_model() {
        Ok(model) => model,
        Err(e) => {
            ctx.say(format!("error while registering user: {}", e)).await.expect("error sending message");
            panic!("error while registering user: {}", e);
        }
    };

    let auth = match auth_key {
        Some(_) => true,
        None => false,
    };

    match entity::target_server::ActiveModel::create(
        &user_model.id,
        &server_address,
        &server_port,
        &auth,
        &auth_key,
        &db_connection
    ).await {
        Ok(_) => {
            ctx.say("registered").await.expect("unable to send message");
        },
        Err(e) => {
            ctx.say(format!("error while registering user: {}", e)).await.expect("error sending message");
            panic!("error while registering user: {}", e);
        }
    };

    Ok(())
}