use core::panic;
use std::{collections::HashMap};
use dotenv::{dotenv};
use tokio::{time};
use crate::{mprober_api::{schemas::Endpoints}};
#[derive(Debug, Clone)]
pub struct MProberConfigs {
    pub address: String,
    pub api_key: String,
    pub port: u64,
    pub polling_frequency: time::Duration,
}

impl MProberConfigs {
    pub fn load() -> MProberConfigs {
        Self::env_vars();

        let address = Self::address();
        let api_key = Self::api_key();
        let port = Self::port();
        let polling_frequency = Self::polling_frequency();

        MProberConfigs { 
            address,
            api_key,
            port,
            polling_frequency,
        }
    }

    pub fn build_address(endpoint: Endpoints) -> String {
        let binding = Self::endpoints();
        let endpoint_str = match binding.get(&endpoint) {
            Some(str) => str,
            None => {
                panic!("endpoint not in endpoints hashmap")
            }
        };
        Self::address().clone() + &endpoint_str.clone()
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

    fn address() -> String {    
        match std::env::var("ADDRESS") {
            Ok(address) => address,
            Err(_) => {
                panic!("Error accessing server address in .env")
            }
        }
    }

    fn api_key() -> String {    
        match std::env::var("API_KEY") {
            Ok(api_key) => api_key,
            Err(_) => {
                panic!("Error accessing api key in .env")
            }
        }
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
    
    fn port() -> u64 {    
        return match std::env::var("PORT") {
            Ok(address) => address.parse::<u64>().expect("Error parsing num from PORT value"),
            Err(_) => {
                panic!("Error accessing server address in .env")
            }
        }
    }
}