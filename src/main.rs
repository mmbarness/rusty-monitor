#![feature(async_fn_in_trait)]
#![warn(clippy::str_to_string)]
mod bot;
mod configs;
mod commands;
mod database;
mod mprober_api;
mod mprober_api_resources;
mod structs;
mod timer;
mod thread_channel;
mod models;
use bot::{support::Support, manage_user::ManageUser, data::{Data}, Bot, load::Load};
use configs::{bot_configs::Config};
use std::time::Duration;
use poise::serenity_prelude::GatewayIntents;
type Error = Box<dyn std::error::Error + Send + Sync>;

async fn on_error(error: poise::FrameworkError<'_, Bot, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    println!("error: {}", error);
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

    let configs = Config::load().await;

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    
    poise::Framework::builder()
        .token(configs.token.clone())
        .setup(move |ctx, _ready, _framework| {
            Box::pin(async move {
                let bot = match Bot::on_setup().await {
                    Ok(bot) => bot,
                    Err(e) =>  {
                        panic!("error starting bot: {}", e);
                    }
                };
                Ok(bot)
            })
        })
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::cpu::cpu_info(),
                commands::help::help(),
                commands::cpu::cpu_load(),
                commands::memory::memory(),
                commands::monitor::start_monitor(),
                commands::new_user::new_user(),
                commands::register::register(),
                commands::who_am_i::who_am_i(),
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
                    let modified_bot = match Bot::on_pre_command(&ctx).await {
                        Ok(bot_with_models) => bot_with_models,
                        Err(e) => {
                            println!("unable to write active models to ctx data: {}", e);
                            ctx.data().clone()
                        }
                    };
                    println!("writing models to bot data");
                    ctx.set_invocation_data::<Bot>(modified_bot).await;
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
                    match Support::get_user_if_exists(&ctx).await {
                        Some(_) => {
                            Ok(true)
                        },
                        None => {
                            match ctx.command().identifying_name.as_str() {
                                "register" => Ok(true),
                                "new_user" => Ok(true),
                                "who_am_i" => Ok(true),
                                _ => {
                                    ctx.say("You\'re a new user, and I don't have your server information on file. Register yourself with the /new_user command").await.expect("unable to send message");
                                    Ok(false)
                                }
                            }
                        }
                    }
                })
            }),
            ..Default::default()
        })
        .intents(intents)
        .run()
        .await
        .unwrap();

}