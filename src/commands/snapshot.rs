use crate::{structs::Context, Error};
use crate::{configs::configs::mprober_configs::MProberConfigs, mprober_api::mprober_api};
use tokio::sync::oneshot;

#[poise::command(track_edits, slash_command)]
async fn snapshot(
    ctx: Context<'_>,
    #[description = "Give me a snapshot of my resources"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    let bot_data = ctx.data().clone();
    let mprober:MProberConfigs = bot_data.mprober_configs;
    let rx = mprober_api::requester;
            
    tokio::spawn(async move {
        let request_channel_receiver = rx().await;
        let resp = request_channel_receiver.await.unwrap();
        enqueue_monitors.send(resp).unwrap();
    });

    interval.tick().await;

    let res = monitor_queue.await.unwrap();
    
    println!("Queue: {:?}", &res);
}