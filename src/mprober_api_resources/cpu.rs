use async_trait::async_trait;
use serde::Deserialize;
use reqwest::Response;
use size::Size;
use tokio::sync::oneshot;
use crate::{mprober_api::schemas::MProberResponse, thread_channel::wrapper::{Wrap, Wrapper}};
use super::shared_traits::{Resource, Load, Compute};

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
pub struct CPULoad {
    pub cpus: Vec<CPU>,
    pub load_average: LoadAverage,
    pub cpus_stat: CpusStat,
}
#[derive(Debug, Deserialize)]
pub struct LoadAverage {
    pub fifteen: f32,
    pub five: f32,
    pub one: f32,
}

impl Resource for CPULoad {}

impl Resource for CPUs {}

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
impl Load for CPULoad {
    async fn load(data: Response) -> CPULoad {
        let mprober_response = match data.json::<MProberResponse<CPULoad>>().await {
            Ok(cpu) => cpu,
            Err(e) => {
                panic!("error parsing cpu data: #{}", e);
            }
        };
        let cpu = mprober_response.data;

        return cpu;
    }
}

impl Wrap for CPULoad {
    fn wrap<'a, CPUsDetect>() -> Wrapper<CPUsDetect> {
        let (tx, rx) = oneshot::channel::<CPUsDetect>();
        return Wrapper { tx, rx, }
    }
}