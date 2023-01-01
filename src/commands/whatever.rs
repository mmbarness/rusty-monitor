use std::{convert::From, sync::Arc};
use gnomeutils::serenity::Message;
use tokio::{time};
use crate::{structs::{Context, BotData}, Error, timer::run_timer::cpu_monitor};

#[poise::command(track_edits, slash_command)]
pub async fn who_cares(
    ctx: poise::Context<'_, BotData, Error>,
    _command: Option<String>,
    msg: Message,
) -> Result<(), Error> {
    // let ctx_ = ctx.serenity_context().clone();
    // let mprober_api = ctx.data().mprober_api.clone();
    
    // msg.reply(&ctx, "hello").await;
    // tokio::spawn(async move {
    //     cpu_monitor(ctx_, mprober_api, 0.01).await;
    // });
    ctx.say("began monitoring, i'll message you if something goes off the rails (or if i crash)").await;
    Ok(())
}