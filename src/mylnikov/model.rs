use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WifiNetworkStatus {
    WifiNetworkSuccess(WifiNetworkResponse),
    WifiNetworkError(ErrorMessage),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WifiNetworkResponse {
    pub result: u16,
    pub data: WifiNetwork
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WifiNetwork {
    pub lat: f32,
    pub range: f32,
    pub lon: f32,
    pub time: Option<u32> 
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorMessage {
    pub result: u16,
    pub data: Data,
    pub message: u16,
    pub desc: String,
    pub time: u32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {}
