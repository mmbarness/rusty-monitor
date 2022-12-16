use std::{collections::HashMap, sync::Mutex};
use bot_configs::BotConfig;
use poise::serenity_prelude;
type Error = Box<dyn std::error::Error + Send + Sync>;
use crate::structs::Data;
mod timer;
mod requester;
mod mprober_schemas;
mod mprober_configs;
mod bot_configs;
mod commands;
mod structs;
mod traits;

#[allow(unused_doc_comments)]
#[tokio::main]
async fn main() {
    let mprober_configs = mprober_configs::MProberConfigs::load();
    let bot_configs = BotConfig::load();

    let intents = serenity_prelude::GatewayIntents::non_privileged()
        | serenity_prelude::GatewayIntents::MESSAGE_CONTENT;

    let data = Data {
        mprober_configs,
    };
    // Build our client.
    let client = poise::Framework::builder()
        .token(&bot_configs.token)
        .intents(intents)
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::help::help(),
            ],
            ..Default::default()
        })
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(data)
            })
        })
        .build()
        .await
        .expect("Error creating client");


    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

// fn main() {
    // also going to need to pass the timer a Sender to message the discord client the monitor data
    // maybe a group of senders
    // let configs = mprober_configs::MProberConfigs::load();
    // timer::run_timer(requester::default_request);
// }