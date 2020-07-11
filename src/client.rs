use anyhow::Result;
use crate::error::*;

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

    pub async fn get(&self, endpoint: &str, request: &str) -> Result<String> {
        let mut url: String = format!("{}{}", self.host, endpoint);
        if !request.is_empty() {
            url.push_str(format!("?{}", request).as_str());
        }

        let response: String = surf::get(url)
            .set_header("authorization", self.api_token.as_str())
            .recv_string()
            .await
            .map_err(|err| FenrirError {
                code: 1001,
                message: format!("{}", err),
            })?;

        Ok(response)
    }
}
