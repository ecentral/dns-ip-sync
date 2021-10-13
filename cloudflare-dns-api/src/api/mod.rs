use reqwest::{Method, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::env;

pub mod zones;
pub mod records;

#[derive(Deserialize, Debug)]
pub struct ResponseError {
    pub code: i32,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct Response<T> {
    pub success: bool,
    pub errors: Option<Vec<ResponseError>>,
    pub messages: Option<Vec<String>>,
    pub result: Option<Vec<T>>,
}

#[derive(Deserialize, Debug)]
pub struct ZonesResponse<T> {
    pub success: bool,
    pub errors: Option<Vec<ResponseError>>,
    pub messages: Option<Vec<String>>,
    pub result: Vec<T>,
}

pub fn cloudflare_client(path: &str, method: Method) -> RequestBuilder {
    let key = env::var("CLOUDFLARE_KEY").unwrap();
    let email = env::var("CLOUDFLARE_EMAIL").unwrap();

    #[cfg(not(test))]
    let url = "https://api.cloudflare.com/client/v4";
    #[cfg(test)]
    let url = &mockito::server_url();

    let url = format!("{}/{}", url, path);
    reqwest::Client::new()
        .request(method, url)
        .header("X-Auth-Key", key.as_str())
        .header("X-Auth-Email", email.as_str())
        .header("Content-Type", "application/json")
}
