

#[poise::command(track_edits, slash_command)]
pub async fn cpu_info(
    ctx: Context<'_>,
    _command: Option<String>,
) -> Result<(), Error> {

    Ok(())
}