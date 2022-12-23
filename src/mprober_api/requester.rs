use std::{collections::{HashMap}, hash::Hash};
use reqwest::{Response};
use reqwest::header;
use serde_json::Value;
use tokio::sync::oneshot::Receiver;
use tokio::sync::oneshot;
use std::thread;
use crate::mprober_api_resource_structs::shared_traits::Load;
use crate::mprober_api_resource_structs::cpu::CPUs;

pub struct Requester {}

impl Requester {
    pub async fn cpu() -> Receiver<CPUs> {

        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, header::HeaderValue::from_static("5563"));

        // get a client builder
        let client = match reqwest::Client::builder()
            .default_headers(headers)
            .build() {
                Ok(c) => c,
                Err(e) => {
                    panic!("error building reqwest client");
                }
            };

        let resp:Response = match client.get("http://100.84.247.97:8000/api/cpu")
            .send()
            .await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        println!("success!");
                    }
                    let string = stringify!(resp);
                    println!("#{string}");
                    resp
                },
                Err(e) => {
                    println!("error making request to /cpu");
                    panic!("oopsie daisy: #{}", e);
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
