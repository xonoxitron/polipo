use hmac::{Hmac, Mac, NewMac};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Error,
};
use sha2::{Digest, Sha256, Sha512};
use std::time::{SystemTime, UNIX_EPOCH};
type HmacSha512 = Hmac<Sha512>;

fn get_signature(
    api_path: String,
    nonce: String,
    url_encoded_body: String,
    api_secret: String,
) -> String {
    // API-Sign = Message signature using HMAC-SHA512 of (URI path + SHA256(nonce + POST data)) and base64 decoded secret API key
    let hash_digest = Sha256::digest(format!("{}{}", nonce, url_encoded_body).as_bytes());
    let private_key = base64::decode(&api_secret).unwrap();
    let mut mac = HmacSha512::new_varkey(&private_key).unwrap();
    let mut hmac_data = api_path.into_bytes();
    hmac_data.append(&mut hash_digest.to_vec());
    mac.update(&hmac_data);
    base64::encode(mac.finalize().into_bytes())
}

fn get_headers(signature: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "API-Key",
        HeaderValue::from_str(&crate::config::get_kraken_api_key()).unwrap(),
    );
    headers.insert(
        "API-Sign",
        HeaderValue::from_str(&signature).unwrap()
    );
    headers
}

pub async fn api_request(method: &str, url_encoded_body: &str) -> Result<String, Error> {
    let method_type: &str = crate::utils::get_method_type(method);
    let api_path = format!(
        "/{}/{}/{}",
        crate::config::get_kraken_api_version(),
        method_type,
        method
    );
    let mut api_endpoint = format!("{}{}", crate::config::get_kraken_api_url(), api_path);
    let api_timeout = crate::config::get_kraken_api_timeout();
    let api_response = match method_type {
        "public" => {
            if !url_encoded_body.is_empty() {
                api_endpoint = api_endpoint + "?" + url_encoded_body;
            }
            reqwest::Client::new()
                .get(&api_endpoint)
                .timeout(api_timeout)
                .send()
                .await
        }
        "private" => {
            if crate::config::get_kraken_api_key().is_empty()
                || crate::config::get_kraken_api_secret().is_empty()
            {
                panic!("void credentials");
            }
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            let payload_nonce = format!("nonce={}", &nonce.to_string());
            let payload_body = if !url_encoded_body.is_empty() {
                format!("{}&{}", payload_nonce, url_encoded_body)
            } else {
                payload_nonce
            };
            let signature = get_signature(
                api_path,
                nonce.to_string(),
                payload_body.to_owned(),
                crate::config::get_kraken_api_secret(),
            );
            reqwest::Client::new()
                .post(&api_endpoint)
                .headers(get_headers(signature))
                .timeout(api_timeout)
                .body(payload_body)
                .send()
                .await
        }
        _ => panic!("{} method is not supported", method),
    };
    match api_response {
        Ok(result) => result.text().await,
        Err(error) => Err(error),
    }
}
