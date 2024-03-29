use core::panic;
use std::{collections::HashMap, error, fmt};
use dotenv::{dotenv};
use entity::target_server::{ActiveModel};
use serde::Deserialize;
use tokio::{time};
use crate::{resource_api::{schemas::Endpoints}, Error};
#[derive(Debug, Deserialize, Clone)]
pub struct ResourceApiConfigs {
    pub address: String,
    pub api_key: Option<i32>,
    pub auth: bool,
    pub port: i32,
    pub polling_frequency: time::Duration,
}

#[derive(Debug, Clone)]
struct ActiveModelParseError;

impl fmt::Display for ActiveModelParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for ActiveModelParseError {}

impl ResourceApiConfigs {
    pub fn load(target_server: &ActiveModel) -> Result<ResourceApiConfigs, Error> {
        Self::env_vars();

        let address = target_server.address.clone().unwrap();
        let auth = target_server.auth.clone().unwrap();
        let api_key = match auth {
            true =>  target_server.auth_key.clone().unwrap(),
            false => None,
        };
        let port = target_server.port.clone().unwrap();
        let polling_frequency = Self::polling_frequency();

        Ok(ResourceApiConfigs { 
            address,
            api_key,
            auth,
            port,
            polling_frequency,
        })
    }

    pub fn build_address(address: &String, port: &String, endpoint: Endpoints) -> String {
        let binding = Self::endpoints();
        let endpoint_str = match binding.get(&endpoint) {
            Some(str) => str,
            None => {
                panic!("endpoint not in endpoints hashmap")
            }
        };
        let full_address = address.clone() + ":" + &port.clone() + &endpoint_str.clone();
        println!("address: {}", full_address);
        address.clone() + ":" + &port.clone() + &endpoint_str.clone()
    }

    fn endpoints() -> HashMap<Endpoints, String> {
        HashMap::from([
            (Endpoints::Hostname, "/api/hostname".to_string()),
            (Endpoints::Kernel, "/api/kernel".to_string()),
            (Endpoints::Uptime, "/api/uptime".to_string()),
            (Endpoints::Time, "/api/time".to_string()),
            (Endpoints::CPU, "/api/cpu".to_string()),
            (Endpoints::CpuDetect, "/api/cpu-detect".to_string()),
            (Endpoints::Memory, "/api/memory".to_string()),
            (Endpoints::NetworkDetect, "/api/network-detect".to_string()),
            (Endpoints::Volume, "/api/volume".to_string()),
            (Endpoints::All, "/api/all".to_string()),
        ])
    }

    fn env_vars() {
        match dotenv() {
            Ok(_) => {
                println!(".env file found, using...")
            }
            Err(_) => {
                println!(".env file not found")
            }
        };
    }

    fn polling_frequency() -> time::Duration {    
        let ok_val = match std::env::var("POLLING_FREQUENCY") {
            Ok(address) => address,
            Err(_) => {
                panic!("Error accessing server address in .env")
            }
        };

        let as_number = match ok_val.parse::<u64>() {
            Ok(num) => num,
            Err(_) => {
                panic!("Error parsing num from POLLING_FREQUENCY value");
            }
        };

        let duration = time::Duration::from_secs(as_number);

        return duration;
    }

}