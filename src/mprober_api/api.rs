use crate::configs::configs::mprober_configs::MProberConfigs;
use super::requester;

pub struct MProberAPI {
    pub configs: MProberConfigs,
    pub requester: requester,
}

impl MProberAPI {
    pub fn load() -> MProberAPI {
        let configs = MProberConfigs::load();
        let request = requester;

        MProberAPI { configs, requester: requester }
    }
}