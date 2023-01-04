use std::{convert::From};
use crate::{Error, timer::run_timer::cpu_monitor, structs::Context};
use crate::mprober_api::api::MProberAPI;
#[poise::command(track_edits, slash_command)]
pub async fn start_monitor(
    ctx: Context<'_>,
    threshold: f64,
) -> Result<(), Error> {
    let serenity_ctx = ctx.serenity_context().clone();
    let channel_id = ctx.channel_id().clone();
    let mprober_api = match MProberAPI::validate_from_invocation_data(ctx).await {
        Ok(api) => api.clone(),
        Err(e) => {
            ctx.say("we weren\t able to get your server info. Maybe try again.").await;
            return Err("error parsing server info from db".into())
        }
    };
    tokio::spawn(async move {
        cpu_monitor(serenity_ctx, mprober_api, channel_id, threshold).await;
    });
    match ctx.say("began monitoring, i'll message you if something goes off the rails (or if i crash)").await {
        Ok(_) => {
            println!("successfully sent message")
        }, 
        Err(e) => {
            panic!("error sending message - {}", e)
        }
    }
    Ok(())
}