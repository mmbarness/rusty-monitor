use crate::{
    structs::{
        Context,
        Error as CommandError
    },
    bot::{
        support::Support,
        defer::Defer,
        invocation_data::InvocationData
    },
};
use std::{convert::From};

#[poise::command(track_edits, slash_command)]
pub async fn host_name(
    ctx: Context<'_>,
    _command: Option<String>,
) -> Result<(), CommandError> {

    Support::defer(&ctx).await;

    let invo_data = InvocationData::validate(ctx).await.expect("unable to pull valid data out of invocation_data");
    let resource_api = invo_data.resource_api;
            
    let hostname = match resource_api.requester.host_name().await {
        Ok(name) => name,
        Err(e) => {
            let response = format!("error retrieving hostname: {}", e);
            ctx.say(response).await.expect("unable to send message");
            return Err(e);
        }
    };
    
    let response = 
        "```\n".to_owned() + 
        "Hostname: " +
        &hostname.data.to_string() + 
        "```";
    
    ctx.say(response).await?;

    Ok(())
}