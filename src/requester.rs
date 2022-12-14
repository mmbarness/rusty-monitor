use std::error::Error;

#[tokio::main]
pub async fn make_request() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::get("http://localhost:8000/api/volume-detect")
        .await?;
    // println!("{:#?}", resp);

    Ok(())
}