use crate::client::*;
use crate::wigle::api::*;
use crate::mylnikov::api::*;

static WIGLE_HOST: &str = "https://api.wigle.net/api/v2/";
static MYLNIKOV_HOST: &str = "https://api.mylnikov.org/";

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

impl Fenrir for Mylnikov {
    fn new(api_token: Option<String>) -> Mylnikov {
        Mylnikov {
            client: Client::new(api_token, MYLNIKOV_HOST.to_string()),
        }
    }
}
