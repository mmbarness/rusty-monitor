use crate::bot::invocation_data::InvocationData;
use crate::{structs::Context, Error};
use std::convert::From;

#[poise::command(track_edits, slash_command)]
pub async fn who_am_i(
    ctx: Context<'_>,
) -> Result<(), Error> {

    let invo_data = InvocationData::validate(ctx).await.expect("unable to pull valid data out of invocation_data");

    let user = &invo_data.user;
    let target_server = &invo_data.target_server;

    ctx.say(
        format!(
            "hello {}, you\'ve been registered since {} and your server is at {}",
            user.user_name.to_string(),
            user.created_at.format("%Y-%m-%d").to_string(),
            target_server.address.to_string()
        )
    ).await.expect("unable to send message");

    Ok(())
}