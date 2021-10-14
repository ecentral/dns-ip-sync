use reqwest::StatusCode;
use super::*;

#[derive(Deserialize, Debug)]
pub struct Record {
    pub id: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub name: String,
    pub content: String,
    pub proxiable: Option<bool>,
    pub proxied: Option<bool>,
    pub ttl: i16,
    pub locked: bool,
    pub zone_id: String,
    pub zone_name: String,
    pub created_on: String,
    pub modified_on: String,
}

#[derive(Deserialize, Serialize)]
pub struct RecordCreate {
    #[serde(rename = "type")]
    pub record_type: Option<String>,
    pub name: Option<String>,
    pub content: Option<String>,
    pub ttl: Option<i16>,
}

pub async fn get_all_records(zone_id: String) -> Response<Record> {
    let url = format!("zones/{}/dns_records", zone_id);
    let client = cloudflare_client(url.as_str(), Method::GET);
    client.send().await.unwrap().json::<Response<Record>>().await.unwrap()
}

pub async fn create_record(name: &str, record_type: &str, value: &str, zone_id: String) -> SingleResult<Record> {
    let url = format!("zones/{}/dns_records", zone_id);
    let new_record = RecordCreate {
        record_type: Option::Some(String::from(record_type)),
        content: Option::Some(String::from(value)),
        name: Option::Some(String::from(name)),
        ttl: Option::None,
    };
    let client = cloudflare_client(url.as_str(), Method::POST)
        .json(&new_record);
    client.send().await.unwrap().json::<SingleResult<Record>>().await.unwrap()
}

pub async fn delete_record(zone_id: &str, record_id: &str) -> StatusCode {
    let url = format!("zones/{}/dns_records/{}", zone_id, record_id);
    let client = cloudflare_client(&*url, Method::DELETE);
    client.send().await.unwrap().status()
}


pub async fn update_record(record_id: &str, zone_id: String, name: Option<&str>, record_type: Option<&str>, value: Option<&str>, ttl: Option<i16>) -> SingleResult<Record> {
    let url = format!("zones/{}/dns_records/{}", zone_id, record_id);
    let update_record = RecordCreate {
        record_type: match record_type.is_some() {
            true => Option::Some(String::from(record_type.unwrap())),
            _ => Option::None,
        },
        content: match value.is_some() {
            true => Option::Some(String::from(value.unwrap())),
            _ => Option::None,
        },
        name: match name.is_some() {
            true => Option::Some(String::from(name.unwrap())),
            _ => Option::None,
        },
        ttl,
    };
    let method = if name.is_some() && record_type.is_some() && value.is_some() && ttl.is_some() {
        Method::PUT
    } else {
        Method::PATCH
    };
    let client = cloudflare_client(url.as_str(), method).json(&update_record);
    client.send().await.unwrap().json::<SingleResult<Record>>().await.unwrap()
}
