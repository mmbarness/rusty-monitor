use std::{borrow::Cow, collections::BTreeMap, sync::Arc};
use poise::serenity_prelude::{self as serenity, json::prelude as json};


pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub struct BotData {
    pub mprober_configs: MProberConfigs,
    pub bot_configs: BotConfig,
}

pub type Command = poise::Command<BotData, CommandError>;
pub type Context<'a> = poise::Context<'a, BotData, Error>;
pub type PrefixContext<'a> = poise::PrefixContext<'a, BotData, CommandError>;
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, BotData, CommandError>;

pub type CommandError = Error;
pub type CommandResult<E=Error> = Result<(), E>;
pub type Framework = poise::Framework<BotData, CommandError>;
pub type FrameworkContext<'a> = poise::FrameworkContext<'a, BotData, CommandError>;