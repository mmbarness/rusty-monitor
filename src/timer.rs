use tokio::sync::oneshot::Receiver;
use tokio::{time, task};
use tokio::sync::oneshot;
use std::future::Future;

use crate::requester::Monitors;

#[tokio::main()]
pub async fn run_timer<T>(callback: fn() -> T) -> () where T: Future<Output = Receiver<Monitors>> + Send + Sync + 'static {

    task::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(10));
        loop {
            let (tx, rx) = oneshot::channel();
            
            tokio::spawn(async move {
                let idk = callback().await;
                let resp = idk.await.unwrap();
                tx.send(resp).unwrap();
            });

            interval.tick().await;

            let res = rx.await.unwrap();
            
            println!("Queue: {:?}", &res);
        }
    });

    // forever.await; 
}
