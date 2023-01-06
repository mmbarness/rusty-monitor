use crate::configs::resource_api_configs::ResourceApiConfigs;

use super::client::Client;
#[derive(Debug, Clone)]
pub struct Request {
    pub client: Client,
    pub configs: ResourceApiConfigs,
}