use async_trait::async_trait;
use serde::Deserialize;
use reqwest::Response;
use crate::{mprober_api::schemas::MProberResponse};
use super::shared_traits::{Resource, Load};
#[derive(Debug, Deserialize)]
pub struct Memory {
    pub available: u64,
    pub buffers: u64,
    pub cache: u64,
    pub free: u64,
    pub shared: u64,
    pub total: u64,
    pub used: u64
}
#[derive(Debug, Deserialize)]
pub struct Swap {
    pub cache: u64,
    pub free: u64,
    pub total: u64,
    pub used: u64,
}
#[derive(Debug, Deserialize)]
pub struct MemoryAndSwap {
    pub memory: Memory,
    pub swap: Swap,
}

impl Resource for MemoryAndSwap {}

#[async_trait]
impl Load for MemoryAndSwap {
    async fn load(data: Response) -> MemoryAndSwap {
        let mprober_response = match data.json::<MProberResponse<MemoryAndSwap>>().await {
            Ok(memory) => memory,
            Err(e) => {
                panic!("error parsing cpu data: #{}", e);
            }
        };
        let memory = mprober_response.data;

        return memory;
    }
}