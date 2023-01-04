use crate::{mprober_api::api::MProberAPI};

#[derive(Debug, Clone)]
pub struct Data {
    pub mprober_api: Option<MProberAPI>,
    pub user: Option<entity::users::Model>,
    pub target_server: Option<entity::target_server::Model>,
}
