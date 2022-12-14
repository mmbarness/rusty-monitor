
use tokio::{time, task};
use std::collections::HashMap;
use serde_json::{Value, Map};

#[tokio::main]
pub async fn run_timer( ) -> (){
    let forever = task::spawn(async {
        let mut interval = time::interval(time::Duration::from_secs(10));
        loop {
            interval.tick().await;
            let resp = reqwest::get("http://100.84.247.97:8000/api/cpu-detect")
            .await
            .unwrap()
            .json::<HashMap<String, Value>>()
            .await;
        match resp {
            Ok::<HashMap<String, Value>, reqwest::Error>(resp) => {
                println!("{:?}", resp);
                // resp;
            },
            Err(_) => {
                // Err("error".into());
                println!("{:?}", resp);
                return ();
            }
        }
        }
    });
    forever.await;
}
