use futures::future::BoxFuture;

mod timer;
mod requester;

fn main() {
    timer::run_timer();
}
