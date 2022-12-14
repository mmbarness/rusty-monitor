mod timer;
mod requester;

fn callback() {
    requester::make_request();
}

fn main() {
    timer::run_timer(Box::new(callback));
}
