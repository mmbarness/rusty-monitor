use serde::Deserialize;
use crate::mprober_api_resources::{shared_traits::Resource};

pub trait Print {
    fn print(&self) -> String;
}

#[derive(Debug, Deserialize)]
pub struct MProberResponse<D: Resource> {
    pub code: u64,
    pub data: D,
}

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