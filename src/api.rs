use crate::client::*;
use crate::wigle::api::*;

static WIGLE_HOST: &str = "https://api.wigle.net/api/v2/";

//#[derive(Clone)]
pub trait Fenrir {
    fn new(api_token: Option<String>) -> Self;
}

impl Fenrir for Wigle {
    fn new(api_token: Option<String>) -> Wigle {
        Wigle {
            client: Client::new(api_token, WIGLE_HOST.to_string()),
        }
    }
}
