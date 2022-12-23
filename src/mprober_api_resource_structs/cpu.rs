use async_trait::async_trait;
use serde::Deserialize;
use reqwest::Response;
use crate::mprober_api::schemas::MProberResponse;

use super::shared_traits::{Resource, Load};

#[derive(Debug, Deserialize)]
pub struct CPU {
    pub cores: u8,
    pub mhz: Vec<f64>,
    pub model_name: String,
    pub threads: u8
}

#[derive(Debug, Deserialize)]
pub struct CPUs {
    pub cpus: Vec<CPU>,
    pub load_average: LoadAverage,
}

impl Resource for CPUs {}

#[derive(Debug, Deserialize)]
pub struct LoadAverage {
    fifteen: f32,
    five: f32,
    one: f32,
}


#[async_trait]
impl Load for CPUs {
    async fn load(data: Response) -> CPUs {
        let mprober_response = match data.json::<MProberResponse<CPUs>>().await {
            Ok(cpu) => cpu,
            Err(e) => {
                panic!("error parsing cpu data: #{}", e);
            }
        };
        let cpu = mprober_response.data;

        return cpu;
    }
}