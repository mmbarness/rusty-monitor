use crate::{structs::Context, Error, bot_support::bot_support::BotSupport, mprober_api_resources::{memory::MemoryAndSwap, shared_traits::{Compute, FieldsToArray}}};
use std::convert::From;

#[poise::command(track_edits, slash_command, subcommands("all", "free", "cache", "swap", "in_the_red"))]
pub async fn memory(
    _ctx: Context<'_>,
    #[description = "give me information about my memory"]
    _command: Option<String>,
) -> Result<(), Error> {
    // Running this function directly, without any subcommand, doesn't do anything
    // Discord doesn't permit invoking the root command of a slash command if it has subcommands.
    Ok(())
}

#[poise::command(track_edits, slash_command)]
pub async fn all(
    ctx: Context<'_>,
    #[description = "give me all available information about my memory"]
    _command: Option<String>,
) -> Result<(), Error> {

    BotSupport::defer(&ctx).await;

    let api_configs = &ctx.data().mprober_configs;
    let mprober_api = &ctx.data().mprober_api;
            
    let memory_and_swap = mprober_api.requester.memory(&api_configs).await;

    let formatted_fields = memory_and_swap.create_responses();
    let fields_array = formatted_fields.fields_to_array();
    
    let initial = "```\n".to_owned();
    let response= fields_array.into_iter().fold(initial, |acc, f| {
        acc + &f.to_string() + " | "
    }) + "```";
    
    ctx.say(response).await?;

    Ok(())
}

#[poise::command(track_edits, slash_command)]
pub async fn free(
    ctx: Context<'_>,
    #[description = "give me information about my memory"]
    _command: Option<String>,
) -> Result<(), Error> {

    BotSupport::defer(&ctx).await;

    let api_configs = &ctx.data().mprober_configs;
    let mprober_api = &ctx.data().mprober_api;
            
    let memory_and_swap = mprober_api.requester.memory(&api_configs).await;
    let formatted_mem_and_swap = memory_and_swap.create_responses();
    
    let response = 
        "```\n".to_owned() + 
        "Memory - " +
        &formatted_mem_and_swap.memory.available + 
        " | " +
        "Swap - " +
        &formatted_mem_and_swap.swap.free +
        "```";
    
    ctx.say(response).await?;

    Ok(())
}

#[poise::command(track_edits, slash_command)]
pub async fn cache(
    ctx: Context<'_>,
    #[description = "give me information about my memory"]
    _command: Option<String>,
) -> Result<(), Error> {

    BotSupport::defer(&ctx).await;

    let api_configs = &ctx.data().mprober_configs;
    let mprober_api = &ctx.data().mprober_api;
            
    let memory_and_swap = mprober_api.requester.memory(&api_configs).await;
    let formatted_mem_and_swap = memory_and_swap.format_all_fields();

    let available = &formatted_mem_and_swap.memory.available;
    let available_resp = format!("available memory: {available}");

    let response = "```\n".to_owned() + &available_resp.to_string() + "```";
    
    ctx.say(response).await?;

    Ok(())
}

#[poise::command(track_edits, slash_command)]
pub async fn swap(
    ctx: Context<'_>,
    #[description = "give me information about my memory"]
    _command: Option<String>,
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
    _command: Option<String>,
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