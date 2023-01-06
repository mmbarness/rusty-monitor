use serde::Deserialize;
use strum_macros::EnumString;
use crate::resource_api_structs::{shared_traits::Resource};

#[derive(Debug, Deserialize, Clone)]
pub struct ResourceApiResponse<D: Resource> {
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