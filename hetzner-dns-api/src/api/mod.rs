use reqwest::{Method, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::env;

pub mod records;
pub mod zones;

pub fn hetzner_client(url: &str, method: Method) -> RequestBuilder {
    let token = env::var("HETZNER_TOKEN").unwrap();
    reqwest::Client::new()
        .request(method, url)
        .header("Auth-API-Token", token.as_str())
}
