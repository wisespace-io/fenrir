use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkGeocodingStatus {
    NetworkGeocodingSuccess(NetworkGeocodingResponse),
    NetworkGeocodingError(ErrorMessage),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkGeocodingResponse {
    pub success: bool,
    pub results: Vec<NetworkGeocoding>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkGeocoding{
    #[serde(flatten)]
    pub address: HashMap<String, Value>,
    pub lat: f32,
    pub lon: f32,
    pub importance: f32,
    pub place_id: Option<i32>,
    pub licence: String,
    pub osm_type: Option<String>,
    pub display_name: Option<String>,
    pub boundingbox: Vec<f32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Address {
    pub road: String,
    pub suburb: String,
    pub city_district: String,
    pub city: String,
    pub county: String,
    pub postcode: String,
    pub country: String,
    pub country_code: String
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorMessage {
    pub success: bool,
    pub message: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WifiNetworkStatus {
    WifiNetworkSuccess(WifiNetworkResponse),
    WifiNetworkError(ErrorMessage),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WifiNetworkResponse {
    pub success: bool,
    #[serde(rename = "totalResults")]
    pub total_results: u32,
    pub first: u32,
    pub last: u32,
    #[serde(rename = "resultCount")]
    pub result_count: u32,
    pub results: Vec<WifiNetwork>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WifiNetwork {
    pub trilat: f32,
    pub trilong: f32,
    pub ssid: String,
    pub qos: u32,
    pub transid: String,
    pub firsttime: String,
    pub lasttime: String,
    pub lastupdt: String,
    pub netid: String,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub type_name: String,
    pub comment: Option<String>,
    pub wep: String,
    pub bcninterval: u32,
    pub freenet: String,
    pub dhcp: String,
    pub paynet: String,
    pub userfound: bool,
    pub channel: u32,
    pub encryption: String,
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub housenumber: Option<String>,
    pub road: Option<String>,
    pub postalcode: Option<String>,
}