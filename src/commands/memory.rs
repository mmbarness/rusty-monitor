use crate::bot_support::bot_support::BotSupport;
use crate::Error;
use crate::mprober_api_resources;
use mprober_api_resources::{
    memory::{
        Memory,
        Threshold,
        Swap
    }, 
    shared_traits::{
        FieldsToArray
    }
};
use std::convert::From;
use crate::structs::Context;

#[poise::command(track_edits, slash_command, subcommands("all", "free", "cache", "swap", "in_the_red"))]
pub async fn memory(
    _ctx: Context<'_>,
    _command: Option<String>,
) -> Result<(), Error> {
    // Running this function directly, without any subcommand, doesn't do anything
    // Discord doesn't permit invoking the root command of a slash command if it has subcommands.
    Ok(())
}

#[poise::command(track_edits, slash_command)]
async fn all(
    ctx: Context<'_>,
    _command: Option<String>,
) -> Result<(), Error> {

    BotSupport::defer(&ctx).await;

    let api_configs = &ctx.data().mprober_configs;
    let mprober_api = &ctx.data().mprober_api;
            
    let memory_and_swap = mprober_api.requester.memory(&api_configs).await;

    let formatted_fields = memory_and_swap.responses();
    let fields_array = formatted_fields.fields_to_array();
    
    let initial = "```\n".to_owned();
    let response= fields_array.into_iter().fold(initial, |acc, f| {
        acc + &f.to_string() + " | "
    }) + "```";
    
    ctx.say(response).await?;

    Ok(())
}

#[poise::command(track_edits, slash_command)]
async fn free(
    ctx: Context<'_>,
    _command: Option<String>,
) -> Result<(), Error> {

    BotSupport::defer(&ctx).await;

    let api_configs = &ctx.data().mprober_configs;
    let mprober_api = &ctx.data().mprober_api;
            
    let memory_and_swap = mprober_api.requester.memory(&api_configs).await;
    let formatted_mem_and_swap = memory_and_swap.responses();
    
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
async fn cache(
    ctx: Context<'_>,
    _command: Option<String>,
) -> Result<(), Error> {

    BotSupport::defer(&ctx).await;

    let api_configs = &ctx.data().mprober_configs;
    let mprober_api = &ctx.data().mprober_api;
            
    let memory_and_swap = mprober_api.requester.memory(&api_configs).await;
    let formatted_mem_and_swap = memory_and_swap.format_all_fields();

    let response = 
        "```\n".to_owned() +
        "Cache use in memory - " +
        &formatted_mem_and_swap.memory.cache + 
        " | " +
        "Cache use in swap - " +
        &formatted_mem_and_swap.swap.cache +
        "```";
    
    ctx.say(response).await?;

    Ok(())
}

#[poise::command(track_edits, slash_command)]
async fn swap(
    ctx: Context<'_>,
    _command: Option<String>,
) -> Result<(), Error> {

    BotSupport::defer(&ctx).await;

    let api_configs = &ctx.data().mprober_configs;
    let mprober_api = &ctx.data().mprober_api;
            
    let memory_and_swap = mprober_api.requester.memory(&api_configs).await;
    let formatted_fields = memory_and_swap.swap.responses();
    let fields_array = formatted_fields.fields_to_array();
    
    let initial = "```\n".to_owned();
    let response= fields_array.into_iter().fold(initial, |acc, f| {
        acc + &f.to_string() + " | "
    }) + "```";
    
    ctx.say(response).await?;

    Ok(())
}

#[poise::command(track_edits, slash_command)]
async fn in_the_red(
    ctx: Context<'_>,
    #[description = "Threshold - defaults to 10% (of total)"] t: Option<u8>,
    _command: Option<String>,
) -> Result<(), Error> {
    
    let threshold = match t {
        Some(num) => {
            let as_f32 = f32::from(num);
            f32::from(as_f32 / 100.0)
        },
        None => 0.1,
    };

    BotSupport::defer(&ctx).await;

    let api_configs = &ctx.data().mprober_configs;
    let mprober_api = &ctx.data().mprober_api;
            
    let memory_and_swap = mprober_api.requester.memory(&api_configs).await;
    
    let memory = &memory_and_swap.memory;
    let memory_ratio =  Memory::ratio(&memory.used, &memory.total);

    let swap = &memory_and_swap.swap;
    let swap_ratio = Swap::ratio(&swap.used, &swap.total);

    let response = match [(memory_ratio > threshold), (swap_ratio > threshold)] {
        [true, true] => {
            format!(
                "Both! Memory is at {}% and Swap is {}%",
                (memory_ratio * 100.0).round(),
                (swap_ratio * 100.0).round()
            )
        },
        [true, false] => {
            format!(
                "Swap is okay ({}%), but memory is at {}%",
                (swap_ratio * 100.0).round(),
                (memory_ratio * 100.0).round(),
            )
        }, 
        [false, true] => {
            format!(
                "Mmory is ok ({}%), but swap is at {}%",
                (memory_ratio * 100.0).round(),
                (swap_ratio * 100.0).round(),
            )
        },
        [false, false] => {
            "Nope, all good!"
        }
    };
    
    ctx.say(response).await?;

    Ok(())
}