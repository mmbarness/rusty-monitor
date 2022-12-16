use std::{collections::HashMap, io::Seek};

use serde_json::de::Read;

struct CPU {    
    cores: u8,
    mhz: Vec<f64>,
    model_name: String,
    threads: u8
}

struct LoadAverage {
    fifteen: f32,
    five: f32,
    one: f32,
}

struct Memory {
    available: u128,
    buffers: u128,
    cache: u128,
    free: u128,
    shared: u128,
    total: u128,
    used: u128
}

struct Network {
    download_rate: f32,
    download_total: u128,
    interface: String,
    upload_rate: f32,
    upload_total: u128
}

struct RTCTime {
    date: String,
    time: String,
}

struct Swap {
    cache: u128,
    free: u128,
    total: u128,
    used: u128,
}

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

struct All {
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


pub trait Request {
    
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