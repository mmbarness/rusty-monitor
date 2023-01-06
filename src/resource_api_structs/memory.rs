use async_trait::async_trait;
use serde::Deserialize;
use reqwest::Response;
use size::Size;
use crate::{resource_api::schemas::ResourceApiResponse};
use super::shared_traits::{Resource, Load, NumStringOrSize, FieldsToArray};
#[derive(Debug, Deserialize, Clone)]
pub struct Memory<T:NumStringOrSize> {
    pub available: T,
    pub buffers: T,
    pub cache: T,
    pub free: T,
    pub shared: T,
    pub total: T,
    pub used: T
}

#[derive(Debug, Deserialize, Clone)]
pub struct Swap<T:NumStringOrSize> {
    pub cache: T,
    pub free: T,
    pub total: T,
    pub used: T,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MemoryAndSwap<T:NumStringOrSize> {
    pub memory: Memory<T>,
    pub swap: Swap<T>,
}

impl Resource for MemoryAndSwap<u64> {}

#[async_trait]
impl Load for MemoryAndSwap<u64> {
    async fn load(data: Response) -> MemoryAndSwap<u64> {
        let mprober_response = match data.json::<ResourceApiResponse<MemoryAndSwap<u64>>>().await {
            Ok(memory) => memory,
            Err(e) => {
                panic!("error parsing cpu data: #{}", e);
            }
        };
        let memory = mprober_response.data;

        return memory;
    }
}

pub trait Threshold {
    fn ratio(part: &u64, whole: &u64) -> f32 {
        (part.clone()) as f32 / (whole.clone() as f32)
    }
}

impl Threshold for Memory<u64>{
    fn ratio(part: &u64, whole: &u64) -> f32 {
        (part.clone()) as f32 / (whole.clone() as f32)
    }
}
impl Threshold for Swap<u64> {
    fn ratio(part: &u64, whole: &u64) -> f32 {
        (part.clone()) as f32 / (whole.clone() as f32)
    }
}

impl Memory<u64> {
    pub fn responses(&self) -> Memory<String> {
        let formatted_memory = &self.format_all_fields();
        Memory {
            available: format!("Available Memory: {}", formatted_memory.available),
            buffers: format!("Buffers: {}", formatted_memory.buffers),
            cache: format!("Cache: {}", formatted_memory.cache),
            free: format!("Free: {}", formatted_memory.free),
            shared: format!("Shared: {}", formatted_memory.shared),
            total: format!("Total: {}", formatted_memory.total),
            used: format!("Used: {}", formatted_memory.used),
        }
    }
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

impl FieldsToArray for Memory<String> {
    fn fields_to_array(&self) -> Vec<String> {
        vec![
            self.available.to_owned(),
            self.buffers.to_owned(),
            self.cache.to_owned(),
            self.free.to_owned(),
            self.shared.to_owned(),
            self.total.to_owned(),
            self.used.to_owned()
        ]
    }
}

impl FieldsToArray for Swap<String> {
    fn fields_to_array(&self) -> Vec<String> {
        vec![
            self.cache.to_owned(),
            self.free.to_owned(),
            self.total.to_owned(),
            self.used.to_owned()
        ]
    }
}

impl FieldsToArray for MemoryAndSwap<String> {
    fn fields_to_array(&self) -> Vec<String> {
        let memory_fields = self.memory.fields_to_array();
        let swap_fields = self.swap.fields_to_array();
        [memory_fields, swap_fields].concat()
    }
}

impl Swap<u64> {
    pub fn responses(&self) -> Swap<String> {
        let formatted_swap = &self.format_all_fields();
        Swap {
            cache: format!("Cache: {}", formatted_swap.cache),
            free: format!("Free: {}", formatted_swap.free),
            total: format!("Total: {}", formatted_swap.total),
            used: format!("Used: {}", formatted_swap.used),
        }
    }
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

    pub fn responses(&self) -> MemoryAndSwap<String> {
        MemoryAndSwap {
            memory: self.memory.responses(),
            swap: self.swap.responses()
        }
    }
}