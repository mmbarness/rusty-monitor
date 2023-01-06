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