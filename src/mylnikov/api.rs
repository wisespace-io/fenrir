use crate::client::*;
use crate::error::*;
use anyhow::{bail, Result};
use serde_json::from_str;
use crate::mylnikov::model::*;
use crate::mylnikov::model::WifiNetworkStatus::*;

#[derive(Clone)]
pub struct Mylnikov {
    pub client: Client,
}

impl Mylnikov {
    // BSSID of Wi-Fi AP (MAC address of network's AP card)
    pub async fn search_bssid<S>(&self, bssid: S) -> Result<WifiNetworkResponse>
    where
        S: Into<String>,
    {
        let request = format!("?v=1.1&data=open&bssid={}", bssid.into());
        let data: String = self.client.get("geolocation/wifi", &request).await?;
        let search_response: WifiNetworkStatus = from_str(data.as_str())?;  
        match search_response {
            WifiNetworkSuccess(success) => Ok(success),
            WifiNetworkError(error) => bail!(FenrirError {
                code: 3001,
                message: error.desc,
            }),
        }
    } 
}