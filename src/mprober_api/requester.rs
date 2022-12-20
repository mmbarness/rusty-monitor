use std::collections::{HashMap};
use tokio::sync::oneshot::Receiver;
use tokio::sync::oneshot;
use serde_json::{Value};
use std::thread;
use crate::structs;

pub async fn default_request() -> Receiver<structs::Monitors> {
    let resp = match reqwest::get("http://100.84.247.97:8000/api/all")
        .await
        .unwrap()
        .json::<HashMap<String, Value>>()
        .await {
            Ok(v) => Monitors { cpu: Ok(v) },
            Err(e) => Monitors { cpu: Err(e) }
        };

    let (tx, rx) = oneshot::channel();

    thread::spawn(move|| {
        tx.send(resp).unwrap();
    });

    return rx;
}