use std::collections::HashMap;

use reqwest::{Response};
use crate::Error;
use crate::configs::resource_api_configs::ResourceApiConfigs;
use crate::resource_api_structs::hostname::HostName;
use super::requester::{Request};
use super::schemas::Endpoints;

impl Request {
    pub async fn host_name(&self) -> Result<HostName, Error> {
        let client = self.client.new();
        let configs = &self.configs;
        let address = ResourceApiConfigs::build_address(&configs.address, &configs.port.to_string(), Endpoints::Hostname);
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
                   return Err(Box::new(e))
                }
            };
            Ok(HostName::load(resp).await)
    }
}