use async_trait::async_trait;
use serde::Deserialize;
use reqwest::Response;
use tokio::sync::oneshot;
use crate::{mprober_api::schemas::MProberResponse, channel_wrap::wrapper::{Wrap, Wrapper}};

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

pub type CpusStat = Vec<f64>;

#[derive(Debug, Deserialize)]
pub struct CPUsDetect {
    pub cpus: Vec<CPU>,
    pub load_average: LoadAverage,
    pub cpus_stat: CpusStat,
}

impl Resource for CPUsDetect {}

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

impl Wrap for CPUs {
    fn wrap<'a, CPUs>() -> Wrapper<CPUs> {
        let (tx, rx) = oneshot::channel::<CPUs>();
        return Wrapper { tx, rx, }
    }
}

#[async_trait]
impl Load for CPUsDetect {
    async fn load(data: Response) -> CPUsDetect {
        let mprober_response = match data.json::<MProberResponse<CPUsDetect>>().await {
            Ok(cpu) => cpu,
            Err(e) => {
                panic!("error parsing cpu data: #{}", e);
            }
        };
        let cpu = mprober_response.data;

        return cpu;
    }
}

impl Wrap for CPUsDetect {
    fn wrap<'a, CPUsDetect>() -> Wrapper<CPUsDetect> {
        let (tx, rx) = oneshot::channel::<CPUsDetect>();
        return Wrapper { tx, rx, }
    }
}