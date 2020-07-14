use crate::client::*;
use crate::error::*;
use anyhow::{bail, Result};
use std::collections::BTreeMap;
use serde_json::from_str;
use crate::wigle::model::*;
use crate::wigle::model::{NetworkGeocodingStatus::*, WifiNetworkStatus::*};

#[derive(Clone)]
pub struct Wigle {
    pub client: Client,
}

impl Wigle {
    pub async fn geocode<S>(&self, address: S) -> Result<NetworkGeocodingResponse>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("addresscode".into(), address.into());
        let request = self.client.build_request(&parameters);
        let data: String = self.client.get_auth("network/geocode", &request).await?;
        let geocode_response: NetworkGeocodingStatus = from_str(data.as_str())?;

        match geocode_response {
            NetworkGeocodingSuccess(success) => Ok(success),
            NetworkGeocodingError(error) => bail!(FenrirError {
                code: 2001,
                message: error.message,
            }),
        }
    }

    // Include only networks matching the string network BSSID,
    // e.g. ‘0A:2C:EF:3D:25:1B’ or '0A:2C:EF’. The first three octets are required.
    pub async fn search_bssid<S>(&self, bssid: S) -> Result<WifiNetworkResponse>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("netid".into(), bssid.into());
        match self.search(parameters).await? {
            WifiNetworkSuccess(success) => Ok(success),
            WifiNetworkError(error) => bail!(FenrirError {
                code: 2002,
                message: error.message,
            }),
        }
    }
    
    // Include only networks exactly matching the string network name.
    pub async fn search_ssid<S>(&self, ssid: S) -> Result<WifiNetworkResponse>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("ssid".into(), ssid.into());
        match self.search(parameters).await? {
            WifiNetworkSuccess(success) => Ok(success),
            WifiNetworkError(error) => bail!(FenrirError {
                code: 2003,
                message: error.message,
            }),
        }
    } 

    pub async fn search(&self, parameters: BTreeMap<String, String>) -> Result<WifiNetworkStatus>
    {
        let request = self.client.build_request(&parameters);
        let data: String = self.client.get_auth("network/search", &request).await?;
        let search_response = from_str(data.as_str())?;        
        Ok(search_response)
    }    
}
