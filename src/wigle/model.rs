use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetworkGeocodingResponse {
    pub address: Vec<String>,
    pub lat: i32,
    pub lon: i32,
    pub importance: i32,
    pub place_id: i32,
    pub licence: String,
    pub osm_type: String,
    pub display_name: String,
    pub boundingbox: Vec<i32>
}
