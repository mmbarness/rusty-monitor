use async_trait::async_trait;

use reqwest::Response;
use serde::Deserialize;

#[async_trait]
pub trait Load {
    async fn load(data: Response) -> Self;
}

pub trait Print {
    fn print(&self) -> String;
}

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

pub trait Resource {}
#[derive(Debug, Deserialize)]
pub struct MProberResponse<D: Resource> {
    code: u64,
    data: D,
}

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
#[derive(Debug, Deserialize)]
pub struct LoadAverage {
    fifteen: f32,
    five: f32,
    one: f32,
}

impl Resource for LoadAverage {}

struct Memory {
    available: u128,
    buffers: u128,
    cache: u128,
    free: u128,
    shared: u128,
    total: u128,
    used: u128
}


impl Resource for Memory {}

struct Network {
    download_rate: f32,
    download_total: u128,
    interface: String,
    upload_rate: f32,
    upload_total: u128
}


impl Resource for Network {}

struct RTCTime {
    date: String,
    time: String,
}


impl Resource for RTCTime {}

struct Swap {
    cache: u128,
    free: u128,
    total: u128,
    used: u128,
}


impl Resource for Swap {}

struct Volume {
    device: String,
    mount_points: Vec<String>,
    read_rate: f32,
    read_total: u128,
    size: u128,
    used: u128,
    write_rate: f32,
    write_total: u128
}

pub struct All {
    cpus: Vec<CPU>,
    cpus_stat: Vec<u128>,
    hostname: String,
    kernel: String,
    load_average: LoadAverage,
    memory: Memory,
    network: Vec<Network>,
    rtc_time: RTCTime,
    swap: Swap,
    uptime: u128,
    volumes: Vec<Volume>
}

pub struct Hostname {
    code: u8,
    data: String,
}

pub struct Kernel {
    code: u8,
    data: String,
}

pub struct Uptime {
    code: u8,
    data: u64,
}

struct Date {
    date: String,
    time: String,
}

pub struct Time {
    code: u8,
    data: Date,
}

pub enum Endpoints {
    Hostname,
    Kernel,
    Uptime,
    Time,
    CPU,
    CpuDetect,
    Memory,
    NetworkDetect,
    Volume,
    All,
}