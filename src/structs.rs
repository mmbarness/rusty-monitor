use std::collections::HashMap;

use crate::{configs::{bot_configs::BotConfig,mprober_configs::MProberConfigs}, mprober_api::{api::MProberAPI}, mprober_api_resources::cpu::CPU, bot_support::bot_support::BotSupport};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
#[derive(Debug, Clone)]
pub struct BotData {
    pub bot_support: BotSupport,
    pub bot_configs: BotConfig,
    pub mprober_api: MProberAPI,
}

pub type Command = poise::Command<BotData, CommandError>;
pub type Context<'a> = poise::Context<'a, BotData, Error>;
pub type PrefixContext<'a> = poise::PrefixContext<'a, BotData, CommandError>;
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, BotData, CommandError>;

pub type CommandError = Error;
pub type CommandResult<E=Error> = Result<(), E>;
pub type Framework = poise::Framework<BotData, CommandError>;
pub type FrameworkContext<'a> = poise::FrameworkContext<'a, BotData, CommandError>;
