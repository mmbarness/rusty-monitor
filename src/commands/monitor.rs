use std::{convert::From};
use crate::bot::invocation_data::InvocationData;
use crate::{Error, timer::run_timer::cpu_monitor, structs::Context};

#[poise::command(track_edits, slash_command)]
pub async fn start_monitor(
    ctx: Context<'_>,
    threshold: f64,
) -> Result<(), Error> {
    let serenity_ctx = ctx.serenity_context().clone();
    let channel_id = ctx.channel_id().clone();
    let invo_data = InvocationData::validate(ctx).await.expect("unable to pull valid data out of invocation_data");
    tokio::spawn(async move {
        cpu_monitor(serenity_ctx, invo_data, channel_id, threshold).await;
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