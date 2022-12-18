use core::panic;

use dotenv::{ dotenv };
use crate::mprober_schemas::Endpoints;
use tokio::{time};
pub struct MProberConfigs {
    pub address: String,
    pub port: u64,
    pub polling_frequency: time::Duration,
}

impl MProberConfigs {
    pub fn load() -> MProberConfigs {
        Self::env_vars();

        let address = Self::address();
        let port = Self::port();
        let polling_frequency = Self::polling_frequency();

        MProberConfigs { 
            address,
            port,
            polling_frequency,
        }
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

pub fn endpoint(endpoint: Endpoints) -> &'static str {
    match endpoint {
        Endpoints::Hostname => "api/hostname",
        Endpoints::Kernel => "api/kernel",
        Endpoints::Uptime => "api/uptime",
        Endpoints::Time => "api/time",
        Endpoints::CPU => "api/cpu",
        Endpoints::CpuDetect => "api/cpu-detect",
        Endpoints::Memory => "api/memory",
        Endpoints::NetworkDetect => "api/network-detect",
        Endpoints::Volume => "api/volume",
        Endpoints::All => "api/all",
    }
}