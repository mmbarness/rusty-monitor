use async_trait::async_trait;
use futures::Future;
use tokio::sync::oneshot::Receiver;

use crate::resource_api_structs::cpu::CPUs;

use super::{wrapper::Wrapper};

pub struct ThreadChannel<T> {
    wrapper: Wrapper<T>,
}

#[async_trait]
pub trait Enqueue<T> {
    async fn enqueue(callback: fn() -> dyn Future<Output = Receiver<CPUs>>) -> Receiver<T>;
}