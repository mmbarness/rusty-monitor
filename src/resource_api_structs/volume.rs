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