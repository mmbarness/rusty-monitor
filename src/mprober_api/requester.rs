use reqwest::{Response};
use tokio::sync::oneshot::Receiver;
use tokio::sync::oneshot;
use std::thread;
use crate::configs::mprober_configs::MProberConfigs;
use crate::{mprober_api_resources::shared_traits::Load};
use crate::mprober_api_resources::cpu::CPUs;
use super::client::Client;

pub struct Request {
    pub client: Client
}

impl Request {
    pub async fn cpus(&self, configs: &MProberConfigs) -> Receiver<CPUs> {
        let client = self.client.new();
        // "http://100.84.247.97:8000/api/cpu"
        let address = configs.address.clone() + "/api/cpu";
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

        let (tx, rx) = oneshot::channel();
    
        thread::spawn(move|| {
            tx.send(cpu).unwrap();
        });
    
        return rx;
    }
}
