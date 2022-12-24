use async_trait::async_trait;
use reqwest::Response;

#[async_trait]
pub trait Load {
    async fn load(data: Response) -> Self;
}

pub trait Resource {}