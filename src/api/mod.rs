use reqwest::{Method, RequestBuilder};
use std::env;

#[derive(Deserialize, Debug)]
struct Record {
    #[serde(rename="type")]
    record_type: String,
    pub(crate) id: String,
    created: String,
    modified: String,
    zone_id: String,
    pub(crate) name: String,
    value: String,
}

#[derive(Deserialize, Serialize)]
struct RecordCreate {
    #[serde(rename="type")]
    record_type: String,
    zone_id: String,
    name: String,
    value: String,
}

#[derive(Deserialize, Debug)]
pub struct Records {
    pub(crate) records: Vec<Record>
}

#[derive(Deserialize, Debug)]
pub struct Zone {
    pub(crate) id: String,
    name: String,
    ttl: i32,
    registrar: String,
    legacy_dns_host: String,
    legacy_ns: Vec<String>,
    ns: Vec<String>,
    created: String,
    verified: String,
    modified: String,
    project: String,
    owner: String,
    permission: String,
    status: String,
    paused: bool,
    is_secondary_dns: bool,
    records_count: i32,
}

#[derive(Deserialize, Debug)]
pub struct Zones {
    pub(crate) zones: Vec<Zone>
}

pub fn hetzner_client(url: &str, method: Method) -> RequestBuilder {
    let token = env::var("HETZNER_TOKEN").unwrap();
    reqwest::Client::new()
        .request(method, url)
        .header("Auth-API-Token", token.as_str())
}

pub mod records;
pub mod zones;