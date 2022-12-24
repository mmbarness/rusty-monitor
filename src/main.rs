#![feature(async_fn_in_trait)]
#![warn(clippy::str_to_string)]
mod configs;
mod commands;
mod structs;
mod mprober_api;
mod mprober_api_resources;
mod timer;
mod thread_channel;
use mprober_api::api::MProberAPI;
use configs::{bot_configs::BotConfig, mprober_configs::MProberConfigs};
use structs::{BotData};
use std::time::Duration;
use poise::serenity_prelude::GatewayIntents;
type Error = Box<dyn std::error::Error + Send + Sync>;

async fn on_error(error: poise::FrameworkError<'_, BotData, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

fn main() -> Result<(), Error>{
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(_main());
    Ok(())
}

async fn _main() {

    tracing_subscriber::fmt::init();
    let bot_configs = BotConfig::load();
    let token_copy = bot_configs.token.clone();
    let mprober_configs = MProberConfigs::load();
    let mprober_api = MProberAPI::load();
    let data = structs::BotData { 
        bot_configs,
        mprober_configs,
        mprober_api,
    };
    
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    poise::Framework::builder()
        .token(token_copy)
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(data)
            })
        })
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::help::help(),
                commands::cpu::cpu_info(),
                commands::register::register(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
                additional_prefixes: vec![
                    poise::Prefix::Literal("hey bot"),
                ],
                ..Default::default()
            },
            /// This code is run before every command
            pre_command: |ctx| {
                Box::pin(async move {
                    println!("Executing command {}...", ctx.command().qualified_name);
                })
            },
            on_error: |error| Box::pin(on_error(error)),
            /// This code is run after a command if it was successful (returned Ok)
            post_command: |ctx| {
                Box::pin(async move {
                    println!("Executed command {}!", ctx.command().qualified_name);
                })
            },
            /// Every command invocation must pass this check to continue execution
            command_check: Some(|ctx| {
                Box::pin(async move {
                    return Ok(true);
                })
            }),
            ..Default::default()
        })
        .intents(intents)
        .run()
        .await
        .unwrap();

}