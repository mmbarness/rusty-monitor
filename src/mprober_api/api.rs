use entity::target_server::Model;
use crate::{
    configs::mprober_configs::MProberConfigs,
};

use super::{requester::Request, client::Client};
#[derive(Debug, Clone)]
pub struct MProberAPI {
    pub configs: MProberConfigs,
    pub requester: Request,
}

impl MProberAPI {
    pub fn load(target_server: &Model) -> MProberAPI {
        let configs = MProberConfigs::load(target_server);
        let client = Self::client(&configs);
        MProberAPI { configs, requester: Self::requester(client) }
    }

    fn client(configs: &MProberConfigs) -> Client {
        Client { auth: configs.auth,  auth_key: configs.api_key.clone() }
    }

    fn requester(client: Client) -> Request {
        Request { client }
    }
}