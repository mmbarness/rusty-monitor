use crate::configs::configs::mprober_configs::MProberConfigs;

use super::mprober_api::requester;

pub struct MProberAPI {
    pub configs: MProberConfigs,
    pub request: requester,
}

impl MProberAPI {
    pub fn load() -> MProberAPI {
        let configs = MProberConfigs::load();
        let request = requester;

        MProberAPI { configs, request }
    }
}