use crate::{structs::Context, Error, bot_support::bot_support::BotSupport, mprober_api_resources::{memory::MemoryAndSwap, shared_traits::Compute}};
use std::convert::From;

#[poise::command(track_edits, slash_command, subcommands("all", "free", "cache", "swap", "in_the_red"))]
pub async fn memory(
    ctx: Context<'_>,
    #[description = "give me information about my memory"]
    command: Option<String>,
) -> Result<(), Error> {

    BotSupport::defer(&ctx).await;

    let api_configs = &ctx.data().mprober_configs;
    let mprober_api = &ctx.data().mprober_api;
            
    let memory_and_swap = mprober_api.requester.memory(&api_configs).await;

    let available = MemoryAndSwap::size(&memory_and_swap.memory.available);
    let available_resp = format!("availalbe memory: {available}");

    let response = "```\n".to_owned() + &available_resp.to_string() + "```";
    
    ctx.say(response).await?;

    Ok(())
}

#[poise::command(track_edits, slash_command)]
pub async fn all(
    ctx: Context<'_>,
    #[description = "give me information about my memory"]
    command: Option<String>,
) -> Result<(), Error> {

    BotSupport::defer(&ctx).await;

    let api_configs = &ctx.data().mprober_configs;
    let mprober_api = &ctx.data().mprober_api;
            
    let memory_and_swap = mprober_api.requester.memory(&api_configs).await;

    let available = MemoryAndSwap::size(&memory_and_swap.memory.available);
    let available_resp = format!("availalbe memory: {available}");

    let response = "```\n".to_owned() + &available_resp.to_string() + "```";
    
    ctx.say(response).await?;

    Ok(())
}

#[poise::command(track_edits, slash_command)]
pub async fn free(
    ctx: Context<'_>,
    #[description = "give me information about my memory"]
    command: Option<String>,
) -> Result<(), Error> {

    BotSupport::defer(&ctx).await;

    let api_configs = &ctx.data().mprober_configs;
    let mprober_api = &ctx.data().mprober_api;
            
    let memory_and_swap = mprober_api.requester.memory(&api_configs).await;

    let available_memory = MemoryAndSwap::size(&memory_and_swap.memory.available);
    let available_memory_resp = format!("availalbe memory: {available_memory}");

    let available_swap = MemoryAndSwap::size(&memory_and_swap.swap.free);
    let available_swap_resp = format!("availalbe memory: {available_swap}");

    let response = "```\n".to_owned()
    + &available_memory_resp.to_string() 
    + &available_swap_resp.to_string() 
    + "```";
    
    ctx.say(response).await?;

    Ok(())
}

#[poise::command(track_edits, slash_command)]
pub async fn cache(
    ctx: Context<'_>,
    #[description = "give me information about my memory"]
    command: Option<String>,
) -> Result<(), Error> {

    BotSupport::defer(&ctx).await;

    let api_configs = &ctx.data().mprober_configs;
    let mprober_api = &ctx.data().mprober_api;
            
    let memory_and_swap = mprober_api.requester.memory(&api_configs).await;

    let available = MemoryAndSwap::size(&memory_and_swap.memory.available);
    let available_resp = format!("availalbe memory: {available}");

    let response = "```\n".to_owned() + &available_resp.to_string() + "```";
    
    ctx.say(response).await?;

    Ok(())
}

#[poise::command(track_edits, slash_command)]
pub async fn swap(
    ctx: Context<'_>,
    #[description = "give me information about my memory"]
    command: Option<String>,
) -> Result<(), Error> {

    BotSupport::defer(&ctx).await;

    let api_configs = &ctx.data().mprober_configs;
    let mprober_api = &ctx.data().mprober_api;
            
    let memory_and_swap = mprober_api.requester.memory(&api_configs).await;

    let available = MemoryAndSwap::size(&memory_and_swap.memory.available);
    let available_resp = format!("availalbe memory: {available}");

    let response = "```\n".to_owned() + &available_resp.to_string() + "```";
    
    ctx.say(response).await?;

    Ok(())
}

#[poise::command(track_edits, slash_command)]
pub async fn in_the_red(
    ctx: Context<'_>,
    #[description = "give me information about my memory"]
    command: Option<String>,
) -> Result<(), Error> {

    BotSupport::defer(&ctx).await;

    let api_configs = &ctx.data().mprober_configs;
    let mprober_api = &ctx.data().mprober_api;
            
    let memory_and_swap = mprober_api.requester.memory(&api_configs).await;

    let available = MemoryAndSwap::size(&memory_and_swap.memory.available);
    let available_resp = format!("availalbe memory: {available}");

    let response = "```\n".to_owned() + &available_resp.to_string() + "```";
    
    ctx.say(response).await?;

    Ok(())
}