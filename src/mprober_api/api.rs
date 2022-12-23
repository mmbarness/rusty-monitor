use crate::{configs::mprober_configs::MProberConfigs};

use super::{requester::Requester};

pub struct MProberAPI {
    pub configs: MProberConfigs,
    pub requester: Requester,
}

impl MProberAPI {
    pub fn load() -> MProberAPI {
        let configs = MProberConfigs::load();
        MProberAPI { configs, requester: Requester {} }
    }
}