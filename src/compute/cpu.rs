use crate::mprober_api_resources::cpu::CpusStat;
use std::f32;
pub struct Compute{}

impl Compute {
    pub fn avg_load(cpus_stat: &CpusStat) -> f32 {
        cpus_stat.into_iter().fold(0.0, |acc, x| acc + x.round() as f32)
    }

    pub fn percentage(num:&f32) -> String {
        (num * 100.0).to_string()
    }
}