use entity::target_server::Model;
use crate::{
    configs::resource_api_configs::ResourceApiConfigs,
};

use super::{requester::Request, client::Client};
#[derive(Debug, Clone)]
pub struct ResourceApi {
    pub configs: ResourceApiConfigs,
    pub requester: Request,
}

impl ResourceApi {
    pub fn load(target_server: &Model) -> ResourceApi {
        let configs = ResourceApiConfigs::load(target_server);
        let client = Self::client(&configs);
        ResourceApi { configs, requester: Self::requester(client) }
    }

    fn client(configs: &ResourceApiConfigs) -> Client {
        Client { auth: configs.auth,  auth_key: configs.api_key.clone() }
    }

    fn requester(client: Client) -> Request {
        Request { client }
    }
}