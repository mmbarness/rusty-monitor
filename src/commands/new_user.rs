use sea_orm::{TryIntoModel, ActiveModelTrait, IntoActiveModel};
use std::convert::From;
use crate::{
    structs::Context,
    Error,
    models::{
        users::{Create, Find},
        target_server::Create as CreateTargetServer
    }, resource_api::api::ResourceApi, bot::{support::Support, defer::Defer}
};

#[poise::command(track_edits, slash_command)]
pub async fn new_user(
    ctx: Context<'_>,
    server_address: String,
    server_port: i32,
    auth_key: Option<i32>,
) -> Result<(), Error> {
    let data = ctx.data();
    let db_connection = &data.database.connection;
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

    match entity::users::ActiveModel::is_user_all_set(id_as_i64, db_connection).await {
        Some((user, target_server)) => {
            let user_name = user.user_name;
            let address = target_server.address;
            let response = "```\n".to_owned()
            + &"You\'ve already registered, ".to_string() + &user_name.to_string() + &".".to_string()
            + &"The server address we have for you on file is: ".to_string() + &address.to_string() + &":".to_string() + &target_server.port.to_string()
            + &"```\n".to_string();
            ctx.say(response).await.expect("unable to send message");
            return Ok(())
        },
        None => ()
    };

    let user = match entity::users::ActiveModel::find_by_discord_id(id_as_i64, db_connection).await {
        Some(u) => u.into_active_model(),
        None => {
            match entity::users::ActiveModel::create(&id_as_i64, user_name, &admin, &false, db_connection).await {
                Ok(user) => user,
                Err(e) => {
                    ctx.say(format!("error while registering user: {}", e)).await.expect("error sending message");
                    panic!("error while registering user: {}", e);
                }
            }
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

    let target_server_model = entity::target_server::ActiveModel::new(
        &user_model.id,
        &server_address,
        &server_port,
        &auth,
        &auth_key,
    );

    let resource_api = match ResourceApi::load(&target_server_model) {
        Ok(api) => api,
        Err(e) => {
            ctx.say("error parsing provided configurations, please check them").await.expect("unable to send message");
            return Err(e)
        }
    };
    
    Support::defer(&ctx).await;

    match resource_api.requester.host_name().await {
        Ok(hostname) => {
            let response = format!("made a successful request to your server, hostname \"{}\". now saving your info...", hostname.data);
            ctx.say(response).await.expect("unable to send message");
            match <entity::target_server::ActiveModel as ActiveModelTrait>::save(target_server_model, db_connection).await {
                Ok(m) => m,
                Err(e) => {
                    ctx.say(format!("error while registering user: {}", e)).await.expect("error sending message");
                    panic!("error while registering user: {}", e);
                }
            };
        },
        Err(e) => {
            let end_of_response = match auth_key {
                Some(key) => format!(", Auth key: {}", key).to_string() + &"```".to_string(),
                None => ", and you didn't provide an auth key - maybe you need to?".to_string() + &"```".to_string()
            };

            let response = "```\n".to_owned()
            + &"Failed to make a request to your server. These are the details you provided, please check them. ".to_string()
            + &"Address: ".to_string() + &server_address.to_string() + ", "
            + &"Port: ".to_string() + &server_port.to_string() + ""
            + &end_of_response;
            ctx.say(response).await.expect("unable to send message");
            return Err(e);
        }
    }

    Ok(())
}