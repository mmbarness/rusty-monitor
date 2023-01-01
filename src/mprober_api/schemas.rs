use serde::Deserialize;
use strum_macros::EnumString;
use crate::mprober_api_resources::{shared_traits::Resource};

#[derive(Debug, Deserialize, Clone)]
pub struct MProberResponse<D: Resource> {
    pub code: u64,
    pub data: D,
}

#[derive(Eq, Hash, Debug, PartialEq, EnumString, Clone, Copy)]
#[strum(ascii_case_insensitive)]
pub enum Endpoints {
    Hostname,
    Kernel,
    Uptime,
    Time,
    CPU,
    CpuDetect,
    Memory,
    NetworkDetect,
    Volume,
    All,
}