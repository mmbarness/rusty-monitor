use reqwest::{Response};
use crate::configs::resource_api_configs::ResourceApiConfigs;
use crate::{resource_api_structs::shared_traits::Load};
use crate::resource_api_structs::cpu::{CPUs, CPULoad};
use super::requester::{Request};
use super::schemas::Endpoints;

impl Request {
    pub async fn cpus(&self) -> CPUs {
        let client = self.client.new();
        let configs = &self.configs;
        let address = ResourceApiConfigs::build_address(&configs.address, &configs.port.to_string(), Endpoints::CPU);
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
                    panic!("error making request to /cpu: #{}", e)
                }
            };
        
        let cpu = CPUs::load(resp).await;
        return cpu;
    }

    pub async fn cpu_load(&self) -> CPULoad{
        let client = self.client.new();
        let configs = &self.configs;
        let address = ResourceApiConfigs::build_address(&configs.address, &configs.port.to_string(), Endpoints::CpuDetect);
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
                    panic!("error making request to /cpu: #{}", e)
                }
            };
        CPULoad::load(resp).await
    }
}