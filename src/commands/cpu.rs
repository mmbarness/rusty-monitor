use crate::{structs::Context, Error, bot_support::{bot_support::BotSupport, defer::Defer}, mprober_api_resources::{cpu::CPULoad, shared_traits::Compute}};
use std::convert::From;

#[poise::command(track_edits, slash_command)]
pub async fn cpu_info(
    ctx: Context<'_>,
    _command: Option<String>,
) -> Result<(), Error> {

    let mprober_api = &ctx.data().mprober_api;

    let cpus = mprober_api.requester.cpus().await;
    // going to for now not handle multi-cpu systems
    let cpu_1 = match cpus.cpus.first() {
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

#[poise::command(track_edits, slash_command)]
pub async fn cpu_load(
    ctx: Context<'_>,
    _command: Option<String>,
) -> Result<(), Error> {

    BotSupport::defer(&ctx).await;

    let mprober_api = &ctx.data().mprober_api;
            
    let cpus = mprober_api.requester.cpu_load().await;
    let cpus_stat = &cpus.cpus_stat;
    let cpus_average = CPULoad::avg(cpus_stat);
    let cpus_average_resp = format!("average load across cores: {}", CPULoad::percentage(&cpus_average));
  
    let response = "```\n".to_owned() + &cpus_average_resp.to_string() + "```";
    
    ctx.say(response).await?;

    Ok(())
}