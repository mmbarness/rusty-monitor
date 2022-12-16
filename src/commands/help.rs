
use crate::structs::{Command, CommandResult, Data};
use anyhow::Error;
use poise::Context;
// type Error = Box<dyn std::error::Error + Send + Sync>;
use serenity::model::user::User;

/// Displays your or another user's account creation date
#[poise::command(slash_command)]
pub async fn help(
    ctx: Context<'_, Data, Error>,
    #[description = "Selected user"] user: Option<User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

pub fn commands() -> [Command; 1] {
    [help()]
}