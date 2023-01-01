use reqwest::header;
#[derive(Debug, Clone)]
pub struct Client {
    pub api_key: String 
}

impl Client {
    pub fn new(&self) -> reqwest::Client {
        let api_key_header_val = match header::HeaderValue::from_str(self.api_key.as_str()) {
            Ok(v) => v,
            Err(_e) => {
                panic!("error parsing api key while initiating Client struct")
            }
        };
        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, api_key_header_val);

        match reqwest::Client::builder()
            .default_headers(headers)
            .build() {
                Ok(c) => c,
                Err(e) => {
                    panic!("error building reqwest client");
                }
            }
    }
}