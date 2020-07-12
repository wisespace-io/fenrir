use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkGeocodingResponse {
    pub success: bool,
    pub results: Vec<NetworkGeocoding>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkGeocoding {
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
