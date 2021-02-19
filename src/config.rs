static mut KRAKEN_API_KEY: String = String::new();
static mut KRAKEN_API_SECRET: String = String::new();
const KRAKEN_API_URL: &str = "https://api.kraken.com";
const KRAKEN_API_VERSION: &str = "0";
const KRAKEN_API_TIMEOUT: u64 = 5000;

use std::time::Duration;

pub fn set_kraken_api_key(api_key: String) {
    unsafe {
        KRAKEN_API_KEY = api_key;
    }
}

pub fn get_kraken_api_key() -> String {
    unsafe { KRAKEN_API_KEY.to_string() }
}

pub fn set_kraken_api_secret(api_secret: String) {
    unsafe {
        KRAKEN_API_SECRET = api_secret;
    }
}

pub fn get_kraken_api_secret() -> String {
    unsafe { KRAKEN_API_SECRET.to_owned() }
}

pub fn get_kraken_api_url() -> String {
    KRAKEN_API_URL.to_string()
}

pub fn get_kraken_api_version() -> String {
    KRAKEN_API_VERSION.to_string()
}

pub fn get_kraken_api_timeout() -> Duration {
    Duration::from_millis(KRAKEN_API_TIMEOUT)
}
