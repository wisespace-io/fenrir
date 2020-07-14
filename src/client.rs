use anyhow::Result;
use crate::error::*;
use std::collections::BTreeMap;

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
        let response: String = surf::get(self.get_url(endpoint, request))
            .set_header("Accept", "application/json")
            .recv_string()
            .await
            .map_err(|err| FenrirError {
                code: 1001,
                message: format!("{}", err),
            })?;

        Ok(response)
    }
    
    pub async fn get_auth(&self, endpoint: &str, request: &str) -> Result<String> {
        let response: String = surf::get(self.get_url(endpoint, request))
            .set_header("Accept", "application/json")
            .set_header("Authorization", format!("{} {}", "Basic", self.api_token.as_str()))
            .recv_string()
            .await
            .map_err(|err| FenrirError {
                code: 1002,
                message: format!("{}", err),
            })?;

        Ok(response)
    }

    pub fn build_request(&self, parameters: &BTreeMap<String, String>) -> String {
        let mut request = String::new();
        for (key, value) in parameters {
            let param = format!("{}={}&", key, value);
            request.push_str(param.as_ref());
        }
        request.pop(); // remove last &
    
        request
    }

    fn get_url(&self, endpoint: &str, request: &str) -> String {
        let mut url: String = format!("{}{}", self.host, endpoint);
        if !request.is_empty() {
            url.push_str(format!("?{}", request).as_str());
        }
        url    
    }
}
