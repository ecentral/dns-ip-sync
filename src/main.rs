use serde::{Deserialize, Serialize};
use reqwest::{Method, RequestBuilder};
use local_ip_address::local_ip;
use std::env;

#[derive(Deserialize, Debug)]
struct Zones {
    zones: Vec<Zone>
}

#[derive(Deserialize, Debug)]
struct Zone {
    id: String,
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
struct Record {
    #[serde(rename="type")]
    record_type: String,
    id: String,
    created: String,
    modified: String,
    zone_id: String,
    name: String,
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
struct Records {
    records: Vec<Record>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_local_ip = local_ip().unwrap();
    let zone = env::var("HETZNER_ZONE").unwrap();
    let domain = env::var("HETZNER_DOMAIN").unwrap();
    create_update_record(zone.as_str(),domain.as_str(), my_local_ip.to_string().as_str(), "A").await;
    let response = get_all_records(zone.as_str()).await;
    println!("{:#?}", response.records);
    Ok(())
}

fn hetzner_client(url: &str, method: Method) -> RequestBuilder {
    let token = env::var("HETZNER_TOKEN").unwrap();
    reqwest::Client::new()
        .request(method, url)
        .header("Auth-API-Token", token.as_str())
}

async fn get_zones(name: Option<&str>) -> Result<Zones, Box<dyn std::error::Error>> {
    let mut client = hetzner_client("https://dns.hetzner.com/api/v1/zones", Method::GET);
    if name.is_some() {
        client = client.query(&[("name", name.unwrap())]);
    }
    Ok(client.send().await?.json::<Zones>().await?)
}

async fn get_zone_by_name(name: &str) -> Zone {
    let zones = get_zones(Option::from(name)).await.unwrap();
    zones.zones.into_iter().next().unwrap()
}

async fn get_all_records(zone_name: &str) -> Records {
    let zone = get_zone_by_name(zone_name).await;
    let client = hetzner_client("https://dns.hetzner.com/api/v1/records", Method::GET)
        .query(&[("zone_id", zone.id)]);
    client.send().await.unwrap().json::<Records>().await.unwrap()
}

async fn add_new_record(name: &str, record_type: &str, value: &str, zone_name: &str) -> String {
    let zone = get_zone_by_name(zone_name).await;
    let new_record = RecordCreate {
        record_type: String::from(record_type),
        value: String::from(value),
        name: String::from(name),
        zone_id: zone.id,
    };
    let client = hetzner_client("https://dns.hetzner.com/api/v1/records", Method::POST)
        .json(&new_record);
    client.send().await.unwrap().text().await.unwrap()
}

async fn update_record(id: &str, name: &str, record_type: &str, value: &str, zone_name: &str) -> String {
    let url = format!("{}{}", "https://dns.hetzner.com/api/v1/records/", id);
    let zone = get_zone_by_name(zone_name).await;
    let new_record = RecordCreate {
        record_type: String::from(record_type),
        value: String::from(value),
        name: String::from(name),
        zone_id: zone.id,
    };
    let client = hetzner_client(url.as_str(), Method::PUT)
        .json(&new_record);
    client.send().await.unwrap().text().await.unwrap()
}

async fn delete_record(record_id: &str) {
    let url = format!("{}{}", "https://dns.hetzner.com/api/v1/records/", record_id);
    let client = hetzner_client(&*url, Method::DELETE);
    let result = client.send().await.unwrap().status();
    println!("{:?}", result);
}

async fn delete_records_by_name(zone: &str, record_name: &str) {
    let data = get_all_records(zone)
        .await
        .records
        .into_iter()
        .filter(|record| record.name == record_name);
    for record in data {
        delete_record(record.id.as_str()).await;
    }
}

async fn create_update_record(zone: &str, record_name: &str, value: &str, record_type: &str) {
    let count = get_all_records(zone)
        .await
        .records
        .into_iter()
        .filter(|record| record.name == record_name).count();
    if count > 1 {
        println!("delete");
        delete_records_by_name(zone, record_name).await;
    }
    if count == 1 {
        println!("update");
        let mut data = get_all_records(zone).await.records.into_iter()
            .filter(|record| record.name == record_name);
        let record = data.next().unwrap();
        update_record(record.id.as_str(), record_name, record_type, value, zone).await;
        return;
    }
    println!("add");
    add_new_record(record_name, record_type, value, zone).await;
}