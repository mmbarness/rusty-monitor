use crate::configs::bot_configs::Config;
use self::load::Load;

pub mod support;
pub mod data;
pub mod defer;
pub mod manage_target_server;
pub mod manage_user;
pub mod load;

#[derive(Debug, Clone)]
pub struct Bot {
    pub data: data::Data,
    pub support: support::Support,
    pub configs: Config,
}

impl Load for Bot {}