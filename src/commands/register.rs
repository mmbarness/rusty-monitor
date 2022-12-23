use crate::structs::{Context, Error};

/// Registers or unregisters application commands in this guild or globally
#[poise::command(prefix_command, hide_in_help)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
