use std::{convert::From};
use crate::{structs::{BotData}, Error, timer::run_timer::cpu_monitor};
#[poise::command(track_edits, slash_command)]
pub async fn start_monitor(
    ctx: poise::Context<'_, BotData, Error>,
    _command: Option<String>,
    threshold: f64,
) -> Result<(), Error> {
    let serenity_ctx = ctx.serenity_context().clone();
    let channel_id = ctx.channel_id().clone();
    let mprober_api = ctx.data().mprober_api.clone();
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