use crate::client::*;
use anyhow::Result;
use serde_json::from_str;
use crate::wigle::model::*;

#[derive(Clone)]
pub struct Wigle {
    pub client: Client,
}

impl Wigle {
    pub async fn geocode(&self) -> Result<NetworkGeocodingResponse> {
        let data: String = self.client.get("network/geocode", "").await?;
        let geocode_response = from_str(data.as_str())?;

        Ok(geocode_response)
    }
}
