use reqwest::header;
#[derive(Debug, Clone)]
pub struct Client {
    pub auth: bool,
    pub auth_key: Option<i32> 
}

impl Client {
    pub fn new(&self) -> reqwest::Client {
        let headers = match self.auth {
            true => match self.auth_key {
                Some(val) => self.auth_header(val),
                None => panic!("auth required but no api key found")
            },
            false => header::HeaderMap::new()
        };

        match reqwest::Client::builder()
            .default_headers(headers)
            .build() {
                Ok(c) => c,
                Err(_) => {
                    panic!("error building reqwest client");
                }
            }
    }

    fn auth_header(&self, validated_auth_key: i32) -> header::HeaderMap {
        let api_key_header_val = match header::HeaderValue::from_str(&validated_auth_key.to_string()) {
            Ok(v) => v,
            Err(_e) => {
                panic!("error parsing api key while initiating Client struct")
            }
        };
        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, api_key_header_val);
        headers
    }
}