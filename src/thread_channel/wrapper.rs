use tokio::sync::oneshot::{Receiver, Sender};

pub struct Wrapper<T>{
    pub tx: Sender<T>,
    pub rx: Receiver<T>
}

pub trait Wrap {
    fn wrap<T>() -> Wrapper<T>;
}