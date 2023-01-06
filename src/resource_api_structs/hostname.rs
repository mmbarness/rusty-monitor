use reqwest::Response;
use serde::Deserialize;
use super::shared_traits::Resource;

#[derive(Debug, Deserialize, Clone)]
pub struct HostName {
    pub code: u8,
    pub data: String,
}

impl Resource for HostName {}

impl HostName {
    pub async fn load(data: Response) -> HostName {
        let mprober_response = match data.json().await {
            Ok(hostname) => hostname,
            Err(e) => {
                panic!("error parsing hostname data: #{}", e);
            }
        };
        return mprober_response;
    }
}