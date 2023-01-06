use crate::bot::{
    support::Support,
    invocation_data::InvocationData
};
use crate::Error;
use crate::bot::defer::Defer;
use crate::resource_api_structs;
use resource_api_structs::{
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

    Support::defer(&ctx).await;

    let invo_data = InvocationData::validate(ctx).await.expect("unable to pull valid data out of invocation_data");
    let resource_api = invo_data.resource_api;
            
    let memory_and_swap = resource_api.requester.memory().await;

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

    Support::defer(&ctx).await;

    let invo_data = InvocationData::validate(ctx).await.expect("unable to pull valid data out of invocation_data");
    let resource_api = invo_data.resource_api;
            
    let memory_and_swap = resource_api.requester.memory().await;
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

    Support::defer(&ctx).await;

    let invo_data = InvocationData::validate(ctx).await.expect("unable to pull valid data out of invocation_data");
    let resource_api = invo_data.resource_api;
            
    let memory_and_swap = resource_api.requester.memory().await;
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

    Support::defer(&ctx).await;

    let invo_data = InvocationData::validate(ctx).await.expect("unable to pull valid data out of invocation_data");
    let resource_api = invo_data.resource_api;
            
    let memory_and_swap = resource_api.requester.memory().await;
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
    #[description = "Threshold - defaults to 10% (of total). Enter as a number, e.g. 10"] threshold: Option<u8>,
    _command: Option<String>,
) -> Result<(), Error> {

    let (validated_threshold, default_used) = match threshold {
        Some(num) => {
            let as_f32 = f32::from(num);
            let divided = f32::from(as_f32 / 100.0);
            println!("_threshold: {}", num);
            println!("dived: {}", divided);
            (divided, false)
        },
        None => (0.5, true),
    };

    Support::defer(&ctx).await;

    let invo_data = InvocationData::validate(ctx).await.expect("unable to pull valid data out of invocation_data");
    let resource_api = invo_data.resource_api;
            
    let memory_and_swap = resource_api.requester.memory().await;
    
    let memory = &memory_and_swap.memory;
    let memory_ratio =  Memory::ratio(&memory.used, &memory.total);

    let swap = &memory_and_swap.swap;
    let swap_ratio = Swap::ratio(&swap.used, &swap.total);

    let response = match [(memory_ratio > validated_threshold), (swap_ratio > validated_threshold)] {
        [true, true] => {
            match default_used {
                true => {
                    format!(
                        "No threshold passed. Used default of 50%, which both memory and swap exceed. Memory is at {}% and Swap is {}%",
                        (memory_ratio * 100.0).round(),
                        (swap_ratio * 100.0).round()
                    )
                }, 
                false => {
                    format!(
                        "Both! Memory is at {}% and Swap is {}%",
                        (memory_ratio * 100.0).round(),
                        (swap_ratio * 100.0).round()
                    )
                }
            }
        },
        [true, false] => {
            match default_used {
                true => {
                    format!(
                        "No threshold passed, used default of 50%. Swap is okay ({}%), but memory is at {}%",
                        (swap_ratio * 100.0).round(),
                        (memory_ratio * 100.0).round(),
                    )
                }, 
                false => {
                    format!(
                        "Swap is okay ({}%), but memory is at {}%",
                        (swap_ratio * 100.0).round(),
                        (memory_ratio * 100.0).round(),
                    )
                }
            }
        }, 
        [false, true] => {
            match default_used {
                true => {
                    format!(
                        "No threshold passed, used default of 50%. Memory is ok ({}%), but swap is at {}%",
                        (memory_ratio * 100.0).round(),
                        (swap_ratio * 100.0).round(),
                    )
                }, 
                false => {
                    format!(
                        "Memory is ok ({}%), but swap is at {}%",
                        (memory_ratio * 100.0).round(),
                        (swap_ratio * 100.0).round(),
                    )
                }
            }
        },
        [false, false] => {
            match default_used {
                true => {
                    format!("Neither swap nor memory exceeds default threshold of {}%", (validated_threshold * 100.0).round())
                }
                false => {
                    format!("Neither swap nor memory exceeds {}%", (validated_threshold * 100.0).round())
                }
            }
        }
    };
    
    ctx.say(response).await?;

    Ok(())
}