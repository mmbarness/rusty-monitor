use async_trait::async_trait;
use serde::Deserialize;
use reqwest::Response;
use size::Size;
use crate::{mprober_api::schemas::MProberResponse};
use super::shared_traits::{Resource, Load, NumStringOrSize};
#[derive(Debug, Deserialize)]
pub struct Memory<T:NumStringOrSize> {
    pub available: T,
    pub buffers: T,
    pub cache: T,
    pub free: T,
    pub shared: T,
    pub total: T,
    pub used: T
}

#[derive(Debug, Deserialize)]
pub struct Swap<T:NumStringOrSize> {
    pub cache: T,
    pub free: T,
    pub total: T,
    pub used: T,
}

#[derive(Debug, Deserialize)]
pub struct MemoryAndSwap<T:NumStringOrSize> {
    pub memory: Memory<T>,
    pub swap: Swap<T>,
}

impl Resource for MemoryAndSwap<u64> {}

#[async_trait]
impl Load for MemoryAndSwap<u64> {
    async fn load(data: Response) -> MemoryAndSwap<u64> {
        let mprober_response = match data.json::<MProberResponse<MemoryAndSwap<u64>>>().await {
            Ok(memory) => memory,
            Err(e) => {
                panic!("error parsing cpu data: #{}", e);
            }
        };
        let memory = mprober_response.data;

        return memory;
    }
}

impl Memory<u64> {
    pub fn format_all_fields(&self) -> Memory<String> {
        let formatted_memory = Memory { 
            available: Size::from_bytes(self.available.clone()),
            buffers: Size::from_bytes(self.buffers.clone()),
            cache: Size::from_bytes(self.cache.clone()),
            free: Size::from_bytes(self.free.clone()),
            shared: Size::from_bytes(self.shared.clone()),
            total: Size::from_bytes(self.total.clone()),
            used: Size::from_bytes(self.used.clone()),
        };
        Memory {
            available: formatted_memory.available.to_string(),
            buffers: formatted_memory.buffers.to_string(),
            cache: formatted_memory.cache.to_string(),
            free: formatted_memory.free.to_string(),
            shared: formatted_memory.shared.to_string(),
            total: formatted_memory.total.to_string(),
            used: formatted_memory.used.to_string(),
        }
    }
}

impl Swap<u64> {
    pub fn format_all_fields(&self) -> Swap<String> {
        let formatted_swap = Swap { 
            cache: Size::from_bytes(self.cache.clone()),
            free: Size::from_bytes(self.free.clone()),
            total: Size::from_bytes(self.total.clone()),
            used: Size::from_bytes(self.used.clone()),
        };
        Swap {
            cache: formatted_swap.cache.to_string(),
            free: formatted_swap.free.to_string(),
            total: formatted_swap.total.to_string(),
            used: formatted_swap.used.to_string(),
        }
    }
}

impl MemoryAndSwap<u64> {
    pub fn format_all_fields(&self) -> MemoryAndSwap<String> {
        MemoryAndSwap { 
            memory: (self.memory.format_all_fields()),
            swap: (self.swap.format_all_fields()) 
        }
    }
}
