use super::thread_channel::Enqueue;

impl Enqueue<CPUs> for ThreadChannel<CPUs> {
    async fn enqueue(callback: fn() -> dyn Future<Output = Receiver<CPUs>>) -> Receiver<CPUs> {
        let (enqueue_monitors, monitor_queue) = oneshot::channel();
            
        tokio::spawn(async move {
            let request_channel_receiver = callback().await;
            let resp = request_channel_receiver.await.unwrap();
            enqueue_monitors.send(resp).unwrap();
        });

        let res = monitor_queue.await.unwrap();
        res
    }
}