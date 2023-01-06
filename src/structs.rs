use sqlx::{Postgres, PgPool};

use crate::{configs::{bot_configs::Config}, resource_api::{api::ResourceApi}, bot::{support::Support,Bot}};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Command = poise::Command<Bot, CommandError>;
pub type Context<'a> = poise::Context<'a, Bot, Error>;
pub type PrefixContext<'a> = poise::PrefixContext<'a, Bot, CommandError>;
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, Bot, CommandError>;

pub type CommandError = Error;
pub type CommandResult<E=Error> = Result<(), E>;
pub type Framework = poise::Framework<Bot, CommandError>;
pub type FrameworkContext<'a> = poise::FrameworkContext<'a, Bot, CommandError>;
