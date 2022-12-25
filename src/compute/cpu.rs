use crate::mprober_api_resources::cpu::CpusStat;
pub struct Compute{}

impl Compute {
    pub fn avg_load(cpus_stat: &CpusStat) -> f64 {
        cpus_stat.into_iter().fold(0.0, |acc, x| acc + x)
    }

    pub fn percentage(num:&f64) -> String {
        let rounded = f64::trunc(num  * 100.0) / 100.0; //
        (rounded * 100.0).to_string() + "%"
    }
}