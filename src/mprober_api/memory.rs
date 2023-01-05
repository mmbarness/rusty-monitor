use entity::target_server::Model;
use reqwest::{Response};
use crate::configs::mprober_configs::MProberConfigs;
use crate::mprober_api_resources::memory::{MemoryAndSwap};
use crate::{mprober_api_resources::shared_traits::Load};
use super::requester::{Request};
use super::schemas::Endpoints;

impl Request {
    pub async fn memory(&self, server:&Model) -> MemoryAndSwap<u64> {
        let client = self.client.new();
        let address = MProberConfigs::build_address(&server.address, &server.port.to_string(), Endpoints::Memory);
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