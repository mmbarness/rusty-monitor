use entity::{target_server::ActiveModel};
use crate::{
    configs::resource_api_configs::ResourceApiConfigs, structs::Error,
};
use super::{requester::Request, client::Client};

#[derive(Debug, Clone)]
pub struct ResourceApi {
    pub configs: ResourceApiConfigs,
    pub requester: Request,
}

impl ResourceApi {
    pub fn load(target_server: &ActiveModel) -> Result<ResourceApi, Error> {
        let configs = match ResourceApiConfigs::load(target_server) {
            Ok(c) => c,
            Err(e) => {
                return Err(e);
            },
        };
        let client = Self::client(&configs);
        let requester = Self::requester(client, configs.clone());
        Ok(ResourceApi { configs, requester })
    }

    fn client(configs: &ResourceApiConfigs) -> Client {
        Client { auth: configs.auth,  auth_key: configs.api_key.clone() }
    }

    fn requester(client: Client, configs: ResourceApiConfigs) -> Request {
        Request { client, configs }
    }
}