use std::{borrow::Cow, collections::BTreeMap, sync::Arc};

use poise::serenity_prelude::{self as serenity, json::prelude as json};

pub use anyhow::{Error, Result};

use crate::mprober_configs::MProberConfigs;
pub struct Data {
    pub mprober_configs: MProberConfigs,
}

impl Data {

}

pub type Command = poise::Command<Data, CommandError>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type PrefixContext<'a> = poise::PrefixContext<'a, Data, CommandError>;
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, CommandError>;

pub type CommandError = Error;
pub type CommandResult<E=Error> = Result<(), E>;
pub type Framework = poise::Framework<Data, CommandError>;
pub type FrameworkContext<'a> = poise::FrameworkContext<'a, Data, CommandError>;