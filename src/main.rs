mod timer;
mod requester;

fn main() {
    timer::run_timer(requester::make_request);
}