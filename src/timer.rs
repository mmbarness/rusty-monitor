use timer::Timer;
// use std::time::Duration;
use std::thread;

pub fn run_timer(timer_callback: Box<dyn FnMut() + Send>) {
    let timer = Timer::new();

    let guard = timer.schedule_repeating(chrono::Duration::seconds(5), timer_callback);
    thread::sleep(::std::time::Duration::new(10, 0));

    drop(guard);
}