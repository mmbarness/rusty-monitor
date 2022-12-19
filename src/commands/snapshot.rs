/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
async fn snapshot(
    ctx: Context<'_>,
    #[description = "Give me a snapshot of my resources"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {

}