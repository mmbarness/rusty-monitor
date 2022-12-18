
#![warn(clippy::str_to_string)]
mod configs;
mod commands;
mod structs;
use configs::configs::{bot_configs, mprober_configs};
use poise::serenity_prelude as serenity;
use std::{collections::HashMap, env::var, sync::Mutex, time::Duration};
use poise::serenity_prelude::GatewayIntents;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
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
    let configs = configs::configs;
    let bot_configs = bot_configs::BotConfig::load();
    let mprober_configs = mprober_configs::MProberConfigs::load();
    let data = structs::BotData { 
        bot_configs,
        mprober_configs
    };
    
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    poise::Framework::builder()
        .token(&bot_configs.token)
        .user_data_setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(data)
            })
        })
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::commands::help(),
                commands::commands::register(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~~~~~".into()),
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