use anyhow::Result;

#[derive(Clone)]
pub struct Client {
    api_token: String,
    host: String,
}

impl Client {
    pub fn new(api_token: Option<String>, host: String) -> Self {
        Client {
            api_token: api_token.unwrap_or_else(|| "".into()),
            host,
        }
    }

    pub fn get(&self, endpoint: &str, request: &str) -> Result<String> {
        let mut url: String = format!("{}{}", self.host, endpoint);
        if !request.is_empty() {
            url.push_str(format!("?{}", request).as_str());
        }

        Ok(url)
    }
}
