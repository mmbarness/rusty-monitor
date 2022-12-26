use crate::{ structs, mprober_api };
use tokio::sync::oneshot::Receiver;
use tokio::{ time, task };
use tokio::sync::oneshot;
use std::future::Future;

#[tokio::main()]
pub async fn run_timer<T>(callback: fn() -> T) -> () where T: Future<Output = Receiver<structs::Monitors>> + Send + Sync + 'static {
    let forever = task::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(10));
        loop {
            let (enqueue_monitors, monitor_queue) = oneshot::channel();
            
            tokio::spawn(async move {
                let request_channel_receiver = callback().await;
                let resp = request_channel_receiver.await.unwrap();
                enqueue_monitors.send(resp).unwrap();
            });
            interval.tick().await;
            let res = monitor_queue.await.unwrap();
            println!("Queue: {:?}", &res);
        }
    });

    match forever.await {
        Ok(()) => (),
        Err(some_error) => println!("{}", some_error),
    };
}
