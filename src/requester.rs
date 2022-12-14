use std::error::Error;
use std::collections::HashMap;

pub async fn make_request() -> Result<(), Box<dyn Error>>{
    let resp = reqwest::get("http://100.84.247.97:8000/api/volume-detect")
        .await
        .unwrap()
        .json::<HashMap<String, String>>()
        .await;
    
    match resp {
        Ok(_) => Ok(()),
        Err(_) => Err("error".into())
    }
    
}