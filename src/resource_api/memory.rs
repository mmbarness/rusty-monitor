use reqwest::{Response};
use crate::configs::resource_api_configs::ResourceApiConfigs;
use crate::resource_api_structs::memory::{MemoryAndSwap};
use crate::{resource_api_structs::shared_traits::Load};
use super::requester::{Request};
use super::schemas::Endpoints;

impl Request {
    pub async fn memory(&self) -> MemoryAndSwap<u64> {
        let client = self.client.new();
        let configs = &self.configs;
        let address = ResourceApiConfigs::build_address(&configs.address, &configs.port.to_string(), Endpoints::Memory);
        let resp:Response = match client.get(address)
            .send()
            .await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        println!("api request successful");
                    };
                    resp
                },
                Err(e) => {
                    panic!("error making request to /memory: #{}", e)
                }
            };
        let memory = MemoryAndSwap::load(resp).await;
        return memory;
    }
}