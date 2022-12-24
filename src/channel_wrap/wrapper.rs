use async_trait::async_trait;
use tokio::sync::oneshot::{Receiver, Sender};
use tokio::sync::oneshot;
use std::thread;
use crate::mprober_api_resources::{cpu::CPUs};

pub struct Wrapper<T>{
    pub tx: Sender<T>,
    pub rx: Receiver<T>
}

pub trait Wrap {
    fn wrap<T>() -> Wrapper<T>;
}