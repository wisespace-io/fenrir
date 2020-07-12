use crate::client::*;
use anyhow::Result;
use std::collections::BTreeMap;
use serde_json::from_str;
use crate::wigle::model::*;

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
        let data: String = self.client.get("network/geocode", &request).await?;
        let geocode_response = from_str(data.as_str())?;

        Ok(geocode_response)
    }
}
