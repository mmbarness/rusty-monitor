use crate::{configs::bot_configs::Config, database::initialize::Database};
use self::load::Load;

pub mod support;
pub mod invocation_data;
pub mod defer;
pub mod query_db;
pub mod load;

#[derive(Debug, Clone)]
pub struct Bot {
    pub support: support::Support,
    pub configs: Config,
    pub database: Database,
}

impl Load for Bot {}