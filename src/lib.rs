#[allow(dead_code)]
#[warn(unused_variables)]
mod client;
mod config;
mod info;
mod utils;

pub fn print_crate_info() {
    println!("-- CRATE INFO --\r\n{}", info::get_crate_info());
}

pub fn set_kraken_api_credentials(api_key: String, api_secret: String) {
    config::set_kraken_api_key(api_key);
    config::set_kraken_api_secret(api_secret);
}

pub async fn get_kraken_api_response(api_method: String, url_encoded_body: String) -> String {
    match client::api_request(&api_method, &url_encoded_body).await {
        Ok(result) => result,
        Err(error) => error.to_string(),
    }
}
