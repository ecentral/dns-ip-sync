use reqwest::{Method, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::env;

pub mod records;
pub mod zones;

pub fn hetzner_client(path: &str, method: Method) -> RequestBuilder {
    let token = env::var("HETZNER_TOKEN").unwrap();

    #[cfg(not(test))]
        let url = "https://dns.hetzner.com/api/v1";
    #[cfg(test)]
        let url = &mockito::server_url();

    let url = format!("{}/{}", url, path);
    reqwest::Client::new()
        .request(method, url)
        .header("Auth-API-Token", token.as_str())
}
