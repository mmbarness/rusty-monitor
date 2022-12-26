use reqwest::{Response};
use crate::configs::mprober_configs::MProberConfigs;
use crate::{mprober_api_resources::shared_traits::Load};
use crate::mprober_api_resources::cpu::{CPUs, CPULoad};
use super::requester::{Request};
use super::schemas::Endpoints;

impl Request {
    pub async fn cpus(&self, configs: &MProberConfigs) -> CPUs {
        let client = self.client.new();
        let address = (configs.build_address)(&Endpoints::CPU);
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

    pub async fn cpu_load(&self, configs: &MProberConfigs) -> CPULoad{
        let client = self.client.new();
        let address = (configs.build_address)(&Endpoints::CpuDetect);
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