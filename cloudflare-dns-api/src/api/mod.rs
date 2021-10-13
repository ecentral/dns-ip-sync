use reqwest::{Method, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::env;

pub mod zones;

#[derive(Deserialize, Debug)]
pub struct Response<T> {
    pub success: bool,
    pub errors: Option<Vec<String>>,
    pub messages: Option<Vec<String>>,
    pub result: Vec<T>,
}

pub fn cloudflare_client(path: &str, method: Method) -> RequestBuilder {
    let key = env::var("CLOUDFLARE_KEY").unwrap();
    let email = env::var("CLOUDFLARE_EMAIL").unwrap();
    reqwest::Client::new()
        .request(method, format!("https://api.cloudflare.com/client/v4/{}", path))
        .header("X-Auth-Key", key.as_str())
        .header("X-Auth-Email", email.as_str())
        .header("Content-Type", "application/json")
}
