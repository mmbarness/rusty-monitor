use crate::{structs::Context, Error};
use std::convert::From;
use tokio::sync::oneshot;

#[poise::command(track_edits, slash_command)]
pub async fn cpu_info(
    ctx: Context<'_>,
    #[description = "whats up with my cpu?"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    let bot_data = &ctx.data().bot_configs;
    let api_configs = &ctx.data().mprober_configs;
    let mprober_api = &ctx.data().mprober_api;

    let (enqueue_monitors, monitor_queue) = oneshot::channel();
            
    let request_channel_receiver = mprober_api.requester.cpus(&api_configs).await;
    let resp = request_channel_receiver.await.unwrap();
    enqueue_monitors.send(resp).unwrap();

    let res = monitor_queue.await.unwrap();

    let cpus = res.cpus;
    let cpu_1 = match cpus.first() {
        Some(cpu) => cpu,
        None => {
            panic!("no cpus returned in response");
        }
    };

    let cores = cpu_1.cores.to_string();
    let cores_response = format!("num of cores: {cores}");
    
    let mhz_average = cpu_1.mhz.iter().fold(0.00, |sum, val| sum + val);
    let mhz_response = format!("mhz average: {mhz_average}");

    let model_name = cpu_1.model_name.to_string();
    let model_name_response = format!("cpu model: {model_name}");

    let threads = cpu_1.threads;
    let threads_response = format!("threads: {threads}");

    let response = "```\n".to_owned()
            + &cores_response.to_string() + ", "
            + &mhz_response.to_string() + ", "
            + &model_name_response.to_string() + ", "
            + &threads_response.to_string()
            + "```";

    ctx.say(response).await?;

    Ok(())
}